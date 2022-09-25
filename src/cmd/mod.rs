use crate::{Ed, Substitution};
use crate::buffer::{Buffer, verify_selection, verify_index, verify_line};
use crate::ui::{UI, DummyUI};
use crate::error_consts::*;

mod parse_selection;
use parse_selection::*;
mod parse_expressions;
use parse_expressions::*;
mod parse_path;
use parse_path::*;
mod parse_flags;
use parse_flags::*;

mod commands;
use commands::*;

#[derive(Default)]
struct PrintingFlags {
  pub p: bool,
  pub n: bool,
  pub l: bool,
}

/// The horrifying piece that is command parsing and execution.
///
/// I tried to break it up, but since all commands require different subsequent
/// parsing it is a lost cause.
/// If someone manages to do it, a PR is more than welcome.
///
/// Important things to remember if modifying this are:
/// * If taking input, verify everything you have first. Nothing is more
///   annoying than entering a paragraph of text to be informed that the given
///   index doesn't exist...
/// * Forbid input you don't handle. This should prevent accidentally force
///   exiting with ',Q file.txt' because you pressed 'Q' instead of 'W'.
pub fn run<B: Buffer>(state: &mut Ed<'_,B>, ui: &mut dyn UI, command: &str)
  -> Result<bool, &'static str>
{
  // Declare flags for printing after the command has been executed.
  let mut pflags = PrintingFlags::default();

  // Parse out the command index and the selection
  let (cmd_i, selection) = parse_selection(command)?;

  // Use the cmd_i to get a clean selection  
  // Match the command and act upon it
  let ret = match command[cmd_i..].trim().chars().next() {
    // No command is valid. It updates selection and prints
    None => {
      if selection.is_some() {
        // Get and update the selection.
        let sel = interpret_selection(selection, state.selection, state.buffer)?;
        verify_selection(state.buffer, sel)?;
        state.selection = sel;
        pflags.p = true; // Default command is 'p'
      } else {
        scroll(state, &mut pflags, selection, 'z', "",
          state.selection.1 - state.selection.0 + 1,
        )?;
      }
      Ok(false)
    },
    Some(ch) => {
      // If command isn't excluded from undoing or state.dont_snapshot, snapshot
      match ch {
        // The following commands don't modify the buffer, therefore creating
        // undo snapshots in the buffer for them is only confusing
        'q' | 'Q' |
        'h' | 'H' |
        '#' |
        '=' |
        'N' | 'L' |
        'f' |
        'e' | 'E' | 'r' |
        'w' | 'W' |
        'p' | 'n' | 'l' |
        'z' | 'Z' |
        // If undo creates snapshots then history is changed by "viewing", which
        // becomes too complex
        'u' | 'U' => {},
        // If not in match arm above we check if we may snapshot
        _ => {
          if ! state.dont_snapshot {
            state.buffer.snapshot()?;
          }
        },
      }
      let tail = {
        let mut x = 1;
        while ! command.is_char_boundary(cmd_i + x) { x += 1; }
        &command[cmd_i + x ..]
      };
      let clean = tail.trim();
      match ch {
        // Quit commands
        'q' | 'Q' => {
          if selection.is_some() { return Err(SELECTION_FORBIDDEN); }
          parse_flags(clean, "")?;
          if state.buffer.saved() || ch == 'Q' {
            Ok(true)
          }
          else {
            Err(UNSAVED_CHANGES)
          }
        }
        // Help commands
        'h' => {
          if selection.is_some() { return Err(SELECTION_FORBIDDEN); }
          // If 'help' was entered, print help
          if clean == "elp" {
            ui.print_message(HELP_TEXT)?;
          }
          // Else no flags accepted and print last error
          else {
            parse_flags(clean, "")?;
            ui.print_message(state.error.unwrap_or(NO_ERROR))?;
          }
          Ok(false)
        },
        'H' => {
          if selection.is_some() { return Err(SELECTION_FORBIDDEN); }
          parse_flags(clean, "")?;
          state.print_errors = !state.print_errors; // Toggle the setting
          Ok(false)
        }
        // Non-editing commands
        '#' => {
          if selection.is_some() { return Err(SELECTION_FORBIDDEN); }
          Ok(false)
        },
        '=' => { // Print selection (can set selection)
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          verify_selection(state.buffer, sel)?;
          state.selection = sel;
          ui.print_message(&format!("({},{})", sel.0, sel.1) )?;
          Ok(false)
        },
        // Toggles printing with/without numbering/literal by default
        'N' => {
          if selection.is_some() { return Err(SELECTION_FORBIDDEN); }
          parse_flags(clean, "")?;
          state.n = !state.n;
          Ok(false)
        },
        'L' => {
          if selection.is_some() { return Err(SELECTION_FORBIDDEN); }
          parse_flags(clean, "")?;
          state.l = !state.l;
          Ok(false)
        },
        // File commands
        'f' => { // Set or print filename
          if selection.is_some() { return Err(SELECTION_FORBIDDEN); }
          // Print or update filename
          filename(state, ui, clean)?;
          Ok(false)
        }
        'e' | 'E' | 'r' => {
          read_from_file(state, selection, ch, clean)?;
          Ok(false)
        },
        'w' | 'W' => {
          write_to_file(state, selection, ch, clean)
        },
        // Print commands
        'p' | 'n' | 'l' => {
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          verify_selection(state.buffer, sel)?;
          // Get the flags
          let mut flags = parse_flags(&command[cmd_i..], "pnl")?;
          // Set the global print pflags (safe to unwrap since parse_flags never removes a key)
          pflags.p = flags.remove(&'p').unwrap();
          pflags.n = flags.remove(&'n').unwrap();
          pflags.l = flags.remove(&'l').unwrap();
          state.selection = sel;
          Ok(false)
        },
        'z' | 'Z' => {
          scroll(state, &mut pflags, selection, ch, clean, 3)?;
          Ok(false)
        },
        // Basic editing commands
        'a' | 'i' | 'A' | 'I' => {
          input(state, ui, &mut pflags, selection, ch, clean)?;
          Ok(false)
        },
        'c' | 'C' => {
          change(state, ui, &mut pflags, selection, ch, clean)?;
          Ok(false)
        },
        'd' => { // Cut
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          // Since selection after execution can be 0 it isn't allowed to auto print after
          parse_flags(clean, "")?;
          state.buffer.cut(sel)?;
          // Try to figure out a selection after the deletion
          state.selection = 
            // If we just deleted up to start of buffer then 
            // sel.0 == 1 then this resolves to (1,0)
            // Otherwise selects nearest line before selection
            (1.max(sel.0 - 1), sel.0 - 1)
          ;
          Ok(false)
        },
        'y' => { // Copy to clipboard
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          let mut flags = parse_flags(clean, "pnl")?;
          state.buffer.copy(sel)?;
          // Save the selection and export the flags
          state.selection = sel;
          pflags.p = flags.remove(&'p').unwrap();
          pflags.n = flags.remove(&'n').unwrap();
          pflags.l = flags.remove(&'l').unwrap();
          Ok(false)
        },
        'x' | 'X' => { // Append/prepend (respectively) clipboard contents to selection
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          let mut flags = parse_flags(clean, "pnl")?;
          pflags.p = flags.remove(&'p').unwrap();
          pflags.n = flags.remove(&'n').unwrap();
          pflags.l = flags.remove(&'l').unwrap();
          // Append or prepend based on command
          let index = 
            if ch == 'X' { sel.0.saturating_sub(1) }
            else { sel.1 }
          ;
          let length = state.buffer.paste(index)?;
          state.selection =
            if length != 0 {
              (index + 1, index + length)
            }
            else { sel }
          ;
          Ok(false)
        },
        'u' | 'U' => {
          if selection.is_some() {return Err(SELECTION_FORBIDDEN); }
          // A undo steps parsing not unlike index parsing would be good later
          // ie. relative AND shorthand for start and end of history
          let steps = if clean.is_empty() { 1 }
          else { clean.parse::<isize>().map_err(|_| INTEGER_PARSE)? };
          if ch == 'U' {
            state.buffer.undo( -steps )?;
          } else {
            state.buffer.undo( steps )?;
          }
          Ok(false)
        },
        // Advanced editing commands
        'k' | 'K' => { // Tag first (k) or last (K) line in selection
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          // Expect only the tag, no flags
          if clean.len() > 1 { return Err(INVALID_TAG); }
          let index = if ch == 'k' { sel.0 } else { sel.1 };
          state.buffer.tag_line(index, clean.chars().next().unwrap_or('\0'))?;
          state.selection = sel;
          Ok(false)
        },
        'm' | 't' => {
          transfer(state, &mut pflags, selection, ch, clean)?;
          Ok(false)
        }
        'j' => {
          // Calculate the selection
          let selection = interpret_selection(selection, state.selection, state.buffer)?;
          let mut flags = parse_flags(clean, "pnl")?;
          pflags.p = flags.remove(&'p').unwrap();
          pflags.n = flags.remove(&'n').unwrap();
          pflags.l = flags.remove(&'l').unwrap();
          state.buffer.join(selection)?;
          state.selection = (selection.0, selection.0);
          Ok(false)
        },
        'J' => {
          let selection = interpret_selection(selection, state.selection, state.buffer)?;
          let nr_end = clean.find( | c: char | !c.is_numeric() ).unwrap_or(clean.len());
          let width = if nr_end == 0 {
            80
          } else {
            clean[.. nr_end].parse::<usize>().map_err(|_| INTEGER_PARSE)?
          };
          let mut flags = parse_flags(&clean[nr_end ..], "pnl")?;
          pflags.p = flags.remove(&'p').unwrap();
          pflags.n = flags.remove(&'n').unwrap();
          pflags.l = flags.remove(&'l').unwrap();
          let end = state.buffer.reflow(selection, width)?;
          // selection.0 must be less than or equal end and bigger be than 1, to
          // handle a reflow without any words, which may delete the selection
          state.selection = (selection.0.min(end).max(1), end);
          Ok(false)
        },
        // Pattern commands
        's' => {
          substitute(state, &mut pflags, selection, tail)?;
          Ok(false)
        },
        'g' | 'v' => {
          // Disable snapshotting during execution
          state.dont_snapshot = true;
          let res = global(state, ui, selection, ch, clean);
          state.dont_snapshot = false;
          res?;
          Ok(false)
        },
        'G' | 'V' => {
          // Disable snapshotting during execution
          state.dont_snapshot = true;
          let res = global_inv(state, ui, selection, ch, clean);
          state.dont_snapshot = false;
          res?;
          Ok(false)
        },
        ':' => {
          let selection = interpret_selection(selection, state.selection, state.buffer)?;
          verify_selection(state.buffer, selection)?;
          match state.macros.get(clean) {
            Some(m) => {
              // Disable undo snapshotting during macro execution
              state.dont_snapshot = true;
              let mut dummy = DummyUI{
                input: m.lines().map(|x| format!("{}\n",x)).collect(),
                print_ui: Some(ui),
              };
              state.selection = selection;
              let res = state.run_macro(&mut dummy);
              // Re-enable snapshotting after
              state.dont_snapshot = false;
              res?;
            },
            None => return Err(UNDEFINED_MACRO),
          }
          Ok(false)
        },
        _cmd => {
          Err(UNDEFINED_COMMAND)
        }
      }
    }
  }?;

  // If print flags are set, print
  if pflags.p | pflags.n | pflags.l {
    verify_selection(state.buffer, state.selection)?;
    ui.print_selection(
      state.see_state(),
      state.selection,
      state.n^pflags.n,
      state.l^pflags.l
    )?;
  }

  Ok(ret)
}
