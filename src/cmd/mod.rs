use crate::{Ed, Substitution, Line, Clipboard, Buffer};
use crate::ui::{UI, ScriptedUI};
use crate::error::*;
use crate::messages::*;


// Parsing helpers
mod parsing;
use parsing::*;

// Command logic in separate loosely grouped files, to manage file size
//
// The commands are usually split into a pub(super) parsing and state managing
// wrapper around an inner function that performs the text editing operation.
mod io_commands;
use io_commands::*;
mod editing_commands;
use editing_commands::*;
mod regex_commands;
use regex_commands::*;

mod undo;
use undo::*;

// Helps to hand in globally relevant flags as one &mut struct to the command
// implementations
// (pub because rusts pub fn is a bit clunky and complains otherwise)
#[derive(Default)]
pub struct PrintingFlags {
  pub p: bool,
  pub n: bool,
  pub l: bool,
}

// The horrifying piece that is command parsing and execution.
//
// I tried to break it up, but since all commands require different subsequent
// parsing it is a lost cause.
// If someone manages to do it, a PR is more than welcome.
//
// Important things to remember if modifying this or underlying functions are:
// * If taking input, verify everything you have first. Nothing is more
//   annoying than entering a paragraph of text to be informed that the given
//   index doesn't exist...
// * Forbid input you don't handle. This should prevent accidentally force
//   exiting with ',Q file.txt' because you pressed 'Q' instead of 'W'.
pub(crate) fn run(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  command: &str,
  recursion_depth: usize,
) -> Result<bool> {
  // Check current recursion depth against state.recursion limit, to prevent
  // inifinite recursion
  if recursion_depth > state.recursion_limit {
    return Err(EdError::InfiniteRecursion);
  }

  // Declare flags for printing after the command has been executed.
  let mut pflags = PrintingFlags::default();

  // Parse out the command index and the selection
  let (cmd_i, selection) = parse_selection(command)?;

  // Use the cmd_i to get a clean selection  
  // Match the command and act upon it
   // (Trim end to get None instead of '\n' or ' ' if there is no command)
   let ret = match command[cmd_i..].trim_end().chars().next() {
    // No command is valid. It updates selection and prints
    None => {
      if selection.is_some() {
        // Get and update the selection.
        let sel = interpret_selection(&state, selection, state.selection)?;
        state.history.current().verify_selection(sel)?;
        state.selection = sel;
        pflags.p = true; // Default command is 'p'
      } else {
        // Since state.selection may be invalid
        state.history.current().verify_selection(state.selection)?;
        scroll(state, &mut pflags, selection, 'z', "",
          state.selection.1 - state.selection.0 + 1,
        )?;
      }
      Ok(false)
    },
    Some(ch) => {
      let tail = {
        let mut x = 1;
        while ! command.is_char_boundary(cmd_i + x) { x += 1; }
        &command[cmd_i + x ..]
      };
      // Don't trim spaces, to allow using them as separator in expressions
      let clean = tail.trim_end_matches('\n');
      // The full command without the newline, to give as label to `history.current_mut()`
      let clean_command = command.trim_end_matches('\n');
      match ch {
        // Quit commands
        'q' | 'Q' => {
          if selection.is_some() { return Err(EdError::SelectionForbidden); }
          parse_flags(clean, "")?;
          if state.history.saved() || ch == 'Q' {
            Ok(true)
          }
          else {
            Err(EdError::UnsavedChanges)
          }
        }
        // Help commands
        'h' => {
          if selection.is_some() { return Err(EdError::SelectionForbidden); }
          // If 'help' was entered, print help
          if clean == "elp" {ui.print_message(HELP_TEXT)?;}
          // Else no flags accepted and print last error
          else {
            parse_flags(clean, "")?;
            match &state.error {
              Some(e) => {
                let msg = e.to_string();
                ui.print_message(&msg)?;
              },
              None => ui.print_message(NO_ERROR)?,
            }
          }
          Ok(false)
        },
        'H' => {
          if selection.is_some() { return Err(EdError::SelectionForbidden); }
          parse_flags(clean, "")?;
          state.print_errors = !state.print_errors; // Toggle the setting
          Ok(false)
        }
        // Non-editing commands
        '=' | '#' => {
          let sel = interpret_selection(&state, selection, state.selection)?;
          state.history.current().verify_selection(sel)?;
          if ch== '=' { parse_flags(clean, "")?; }
          state.selection = sel;
          if ch == '=' { ui.print_message(&format!("({},{})", sel.0, sel.1) )?; }
          Ok(false)
        },
        // Toggles printing with/without numbering/literal by default
        'P' => {
          if selection.is_some() { return Err(EdError::SelectionForbidden); }
          let mut flags = parse_flags(clean, "nl")?;
          // Toggle default state of the flags defined
          if flags.remove(&'l').unwrap() {
            state.l = !state.l;
          }
          if flags.remove(&'n').unwrap() {
            state.n = !state.n;
          }
          Ok(false)
        },
        // File/shell commands
        'f' => { // Set or print filename
          if selection.is_some() { return Err(EdError::SelectionForbidden); }
          // Print or update filename
          filename(state, ui, clean)?;
          Ok(false)
        },
        '!' | '|' => {
          run_command(state, ui, clean_command, selection, ch, clean)?;
          Ok(false)
        },
        'e' | 'E' | 'r' => {
          read_from_file(state, ui, clean_command, selection, ch, clean)?;
          Ok(false)
        },
        'w' | 'W' => {
          write_to_file(state, ui, selection, ch, clean)
        },
        // Print commands
        'p' | 'n' | 'l' => {
          let sel = interpret_selection(&state, selection, state.selection)?;
          state.history.current().verify_selection(sel)?;
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
          input(state, ui, &mut pflags, clean_command, selection, ch, clean)?;
          Ok(false)
        },
        'c' | 'C' => {
          change(state, ui, &mut pflags, clean_command, selection, ch, clean)?;
          Ok(false)
        },
        'd' => { // Cut
          cut(state, &mut pflags, clean_command, selection, clean)?;
          Ok(false)
        },
        'y' => { // Copy to clipboard
          copy(state, &mut pflags, selection, clean)?;
          Ok(false)
        },
        'x' | 'X' => { // Append/prepend (respectively) clipboard contents to selection
          paste(state, &mut pflags, clean_command, selection, ch, clean)?;
          Ok(false)
        },
        'U' => {
          manage_history(state, ui, selection, clean)?;
          Ok(false)
        },
        'u' => { // Undo/redo (undoing a negative number of steps redoes)
           undo(state, ui, selection, clean)?;
           Ok(false)
        },
        // Advanced editing commands
        'k' | 'K' => { // Tag first (k) or last (K) line in selection
          tag(state, selection, ch, clean)?;
          Ok(false)
        },
        'm' | 't' => {
          transfer(state, &mut pflags, clean_command, selection, ch, clean)?;
          Ok(false)
        }
        'j' => {
          join(state, &mut pflags, clean_command, selection, clean)?;
          Ok(false)
        },
        // Pattern commands
        's' => {
          substitute(state, &mut pflags, clean_command, selection, tail)?;
          Ok(false)
        },
        'g' | 'v' | 'G' | 'V' => {
          // Before disabling snapshotting, create one for this command
          state.history.snapshot(clean_command.to_string());
          // Disable snapshotting during execution, reset it after
          let orig_dont_snapshot = state.history.dont_snapshot;
          state.history.dont_snapshot = true;
          let res = if ch == 'g' || ch == 'v' {
            global(state, ui, selection, ch, clean, recursion_depth)
          } else {
            global_interactive(state, ui, selection, ch, clean, recursion_depth)
          };
          state.history.dont_snapshot = orig_dont_snapshot;
          // If snapshotting was originally enabled we should handle if no
          // mutation of the buffer occured during the dont_snapshot.
          if !orig_dont_snapshot { state.history.dedup_present(); }
          res?;
          Ok(false)
        },
        ':' => {
          let given_selection = if selection.is_some() {
            let s = interpret_selection(&state, selection, state.selection)?;
            state.history.current().verify_selection(s)?;
            Some(s)
          }
          else {
            None
          };
          // Sloppy argument parsing into list
          let mut args = clean.split(' ');
          let macro_name = args.next().unwrap_or("");
          let args: Vec<&str> = args.collect();
          match state.macro_getter.get_macro(macro_name)? {
            Some(m) => {
              // Before disabling snapshotting, create one for this command
              state.history.snapshot(clean_command.into());
              // Disable undo snapshotting during macro execution
              let orig_dont_snapshot = state.history.dont_snapshot;
              state.history.dont_snapshot = true;
              if let Some(selection) = given_selection {
                state.selection = selection;
              }
              let res = state.private_run_macro(ui, m, &args, recursion_depth+1);
              // Re-enable snapshotting after
              state.history.dont_snapshot = orig_dont_snapshot;
              // If snapshotting was originally enabled we should handle if no
              // mutation of the buffer occured during the dont_snapshot.
              if !orig_dont_snapshot { state.history.dedup_present(); }
              res
            },
            None => Err(EdError::MacroUndefined(macro_name.to_owned())),
          }?;
          Ok(false)
        },
        _cmd => {
          Err(EdError::CommandUndefined(ch))
        }
      }
    }
  }?;

  // If print flags are set, print
  if pflags.p | pflags.n | pflags.l {
    state.history.current().verify_selection(state.selection)?;
    ui.print_selection(
      state,
      state.selection,
      state.n^pflags.n,
      state.l^pflags.l
    )?;
  }

  Ok(ret)
}
