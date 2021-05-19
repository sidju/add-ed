use crate::Ed;
use crate::buffer::{Buffer, verify_selection, verify_index};
use crate::ui::{UI, DummyUI};
use crate::error_consts::*;

mod substitute;
mod parse_selection;
use parse_selection::*;
mod parse_expressions;
use parse_expressions::*;
mod parse_path;
use parse_path::*;
mod parse_flags;
use parse_flags::*;


/// The horrifying piece that is command parsing and execution.
///
/// I tried to break it up, but since all commands require different subsequent parsing it is a lost cause.
/// If someone manages to do it, a PR is more than welcome.
///
/// Important things to remember if modifying this are:
/// * If taking input, verify everything you have first. Nothing is more annoying than entering a paragraph of text to be informed that the given index doesn't exist...
/// * Forbid input you don't handle. This should prevent accidentally force exiting with ',Q file.txt' because you pressed 'Q' instead of 'W'.
pub fn run<B: Buffer>(state: &mut Ed<'_,B>, ui: &mut dyn UI, command: &str)
  -> Result<bool, &'static str>
{
  // Declare flags for printing after the command has been executed.
  let mut p = false;
  let mut n = false;
  let mut l = false;

  // Parse out the command index and the selection
  let (cmd_i, selection) = parse_selection(command)?;

  // Use the cmd_i to get a clean selection  
  // Match the command and act upon it
  let ret = match command[cmd_i..].trim().chars().next() {
    // No command is valid. It updates selection and thus works as a print when viewer is on
    None => {
      // Get and update the selection.
      let sel = interpret_selection(selection, state.selection, state.buffer)?;
      verify_selection(state.buffer, sel)?;
      state.selection = Some(sel);
      p = true; // Default command is 'p'
      Ok(false)
    },
    Some(ch) => {
      let clean = &command[cmd_i + 1 ..].trim();
      match ch {
        // Quit commands
        'q' => {
          if selection.is_some() { return Err(SELECTION_FORBIDDEN); }
          parse_flags(clean, "")?;
          if state.buffer.saved() {
            Ok(true)
          }
          else {
            Err(UNSAVED_CHANGES)
          }
        }
        'Q' => {
          if selection.is_some() { return Err(SELECTION_FORBIDDEN); }
          parse_flags(clean, "")?;
          Ok(true)
        }
        // Help commands
        'h' => {
          if selection.is_some() { return Err(SELECTION_FORBIDDEN); }
          // If 'help' was entered, print held
          if clean == &"elp" {
            ui.print(HELP_TEXT)?;
          }
          // Else no flags accepted and print last error
          else {
            parse_flags(clean, "")?;
            ui.print(state.error.unwrap_or(NO_ERROR))?;
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
          // Get and update selection (none given gives no change)
          if selection.is_some() {
            let sel = interpret_selection(selection, state.selection, state.buffer)?;
            verify_selection(state.buffer, sel)?;
            state.selection = Some(sel);
          }
          Ok(false)
        },
        '=' => { // Print selection (can set selection)
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          verify_selection(state.buffer, sel)?;
          state.selection = Some(sel);
          ui.print( &format!("({},{})", sel.0 + 1, sel.1 + 1) )?;
          Ok(false)
        },
        // File commands
        'f' => { // Set or print filename
          if selection.is_some() { return Err(SELECTION_FORBIDDEN); }
          match parse_path(clean) {
            None => { // Print current filename
              if state.path.len() == 0 {
                ui.print(NO_FILE)?;
              }
              else {
                ui.print(&state.path)?;
              }
            },
            Some(x) => { // Set new filename
              state.path = x.to_string();
            }
          }
          Ok(false)
        }
        'e' | 'E' | 'r' => {
          // Read the selection if 'r', else error on any selection and return 0 on none (Lone default == no input)
          let index = 
            if ch == 'r' {
              let i = interpret_selection(selection, state.selection, state.buffer)?.1;
              Ok(Some(i))
            }
            else {
              if selection.is_none() {
                Ok(None)
              }
              else {
                Err(SELECTION_FORBIDDEN)
              }
            }?;
          // Only 'e' cares if the buffer is saved
          if !state.buffer.saved() && (ch == 'e') {
            Err(UNSAVED_CHANGES)
          }
          else {
            // Get the path (cutting of the command char and the trailing newline)
            let path = parse_path(clean);
            let path = path.unwrap_or(&state.path);
            // Read the data from the file
            let datalen = state.buffer.read_from(path, index, ch == 'E')?;
            if ch != 'r' {
              state.path = path.to_string();
            }
            let index = index.unwrap_or(0);
            state.selection = Some((index, index + datalen));
            Ok(false)
          }
        },
        'w' | 'W' => {
          // Since 'w' and 'W' should default to the whole buffer rather than previous selection
          // they get some custom code here
          let sel = match selection {
            Some(s) => {
              Some( interpret_selection(Some(s), state.selection, state.buffer)? )
            },
            None => None,
          };
          // If not wq, parse path
          let (q, path) = if clean != &"q" {
            let path = parse_path(clean).unwrap_or(&state.path);
            (false, path)
          }
          // If wq, use current file path
          else {
            (true, &state.path[..])
          };
          // We need to know if the whole buffer is selected
          let whole_buffer = match sel {
            None => true,
            Some(s) => {
              if s == (0, state.buffer.len().saturating_sub(1)) {
                true
              }
              else {
                false
              }
            },
          };
          // If the 'q' flag is set the whole buffer must be selected
          if q && ! whole_buffer { return Err(UNSAVED_CHANGES); }
          // Write it into the file (append if 'W')
          let append = ch == 'W';
          state.buffer.write_to(sel, path, append)?;
          // If all was written, update state.file
          // If selection was given, save that selection
          match sel {
            None => {
              state.path = path.to_string();
            },
            Some(s) => {
              if whole_buffer { state.path = path.to_string(); }
              state.selection = Some(s);
            },
          }
          Ok(q)
        }
        // Print commands
        'p' | 'n' | 'l' => {
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          verify_selection(state.buffer, sel)?;
          // Get the flags
          let mut flags = parse_flags(&command[cmd_i..], "pnl")?;
          // Set the global print flags (safe to unwrap since parse_flags never removes a key)
          p = flags.remove(&'p').unwrap();
          n = flags.remove(&'n').unwrap();
          l = flags.remove(&'l').unwrap();
          state.selection = Some(sel);
          Ok(false)
        }
        // Basic editing commands
        'a' | 'i' | 'c' => {
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          // c requires a valid selection to exchange for the input
          // i and a only require valid indices, which are checked by interpret_selection
          if ch == 'c' { verify_selection(state.buffer, sel)? }
          let mut flags = parse_flags(clean, "pnl")?;
          p = flags.remove(&'p').unwrap();
          n = flags.remove(&'n').unwrap();
          l = flags.remove(&'l').unwrap();
          // When all possible checks have been run, get input
          // Note that this thorough checking is to make sure the command goes through before taking more input
          // commands without input don't need this, since the buffer checks its input.
          let tmp = ui.get_input(state.buffer, '.')?;
          let mut input = tmp.iter().map(|string| &string[..]);
          let new_sel = match ch {
            'a' | 'i' => {
              if input.len() != 0 {
                let start = if ch == 'a' {
                  // Only if the buffer is empty will the latter not work
                  if state.buffer.len() == 0 { 0 }
                  else { sel.1 + 1 }
                }
                else {
                  sel.0
                };
                let end = start + input.len();
                state.buffer.insert(&mut input, start)?;
                Some((start, end))
              }
              else {
                // If no input the command was cancelled, keep the old selection
                state.selection
              }
            }
            'c' => {
              let end = sel.0 + input.len();
              state.buffer.change(&mut input, sel)?;
              if input.len() != 0 {
                Some((sel.0, end))
              }
              else {
                // Same as delete, use same post-selection logic
                if state.buffer.len() == 0 { None }
                else {Some((sel.0, sel.0))}
              }
            }
            _ => { panic!("Unreachable code reached"); }
          };
          // If resulting selection is empty, set original selection?
          state.selection = new_sel;
          Ok(false)
        }
        'd' => { // Cut
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          // Since selection after execution can be 0 it isn't allowed to auto print after
          parse_flags(clean, "")?;
          state.buffer.cut(sel)?;
          // Try to figure out a selection after the deletion
          state.selection = 
            if state.buffer.len() == 0 { None }
            else {Some((sel.0, sel.0))}
          ;
          Ok(false)
        },
        'y' => { // Copy to clipboard
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          let mut flags = parse_flags(clean, "pnl")?;
          state.buffer.copy(sel)?;
          // Save the selection and export the flags
          state.selection = Some(sel);
          p = flags.remove(&'p').unwrap();
          n = flags.remove(&'n').unwrap();
          l = flags.remove(&'l').unwrap();
          Ok(false)
        },
        'x' | 'X' => { // Append/prepend (respectively) clipboard contents to index
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          let mut flags = parse_flags(clean, "pnl")?;
          let index = if ch == 'X' { sel.0 } else { sel.1 + 1 }; // Append or prepend based on command
          let length = state.buffer.paste(index)?;
          state.selection = if length != 0 { Some((index, index + length)) } else { None };
          p = flags.remove(&'p').unwrap();
          n = flags.remove(&'n').unwrap();
          l = flags.remove(&'l').unwrap();
          Ok(false)
        },
        // Advanced editing commands
        'k' | 'K' => { // Tag first (k) or last (K) line in selection
          let sel = interpret_selection(selection, state.selection, state.buffer)?;
          // Expect only the tag, no flags
          if clean.len() != 1 { return Err(INVALID_TAG); }
          let index = if ch == 'k' { sel.0 } else { sel.1 };
          state.buffer.tag_line(index, clean.chars().next().unwrap())?;
          state.selection = Some(sel);
          Ok(false)
        },
        'm' | 't' => {
          // Parse the target index, then the flags if any
          let (ind_end, ind) = parse_index(&clean)?;
          let index = interpret_index(
            ind.unwrap_or(Ind::BufferLen),
            state.buffer,
            state.selection.map(|s| s.1),
          )?;
          let mut flags = parse_flags(&clean[ind_end..], "pnl")?;
          p = flags.remove(&'p').unwrap();
          n = flags.remove(&'n').unwrap();
          l = flags.remove(&'l').unwrap();
          // Calculate the selection
          let selection = interpret_selection(selection, state.selection, state.buffer)?;
          let end = index + (selection.1 - selection.0);
          // Make the change
          if ch == 'm' {
            state.buffer.mov(selection, index)?;
          }
          else {
            state.buffer.mov_copy(selection, index)?;
          }
          // Update the selection
          state.selection = Some((index, end));
          Ok(false)
        }
        'j' => {
          // Calculate the selection
          let selection = interpret_selection(selection, state.selection, state.buffer)?;
          let mut flags = parse_flags(clean, "pnl")?;
          p = flags.remove(&'p').unwrap();
          n = flags.remove(&'n').unwrap();
          l = flags.remove(&'l').unwrap();
          state.buffer.join(selection)?;
          state.selection = Some((selection.0, selection.0)); // Guaranteed to exist, but may be wrong.
          Ok(false)
        }    
        // Pattern commands
        's' => {
          let selection = interpret_selection(selection, state.selection, state.buffer)?;
          // switch based on if clean was given or not
          if clean.len() == 0 {
            // This means we use the arguments stored in state.s_args
            match &state.s_args {
              None => return Err(NO_PRIOR_S),
              Some((pattern, replacement, global)) => {
                state.selection = Some(
                  state.buffer.search_replace((pattern, replacement), selection, *global)?
                );
              }
            }
          }
          else {
            let expressions = parse_expressions(clean);
            if expressions.len() != 3 { return Err(EXPRESSION_TOO_SHORT); }
            let mut flags = parse_flags(&(expressions[2]), "gpnl")?;
            let g = flags.remove(&'g').unwrap();
            p = flags.remove(&'p').unwrap();
            // TODO, regex in 'n' and 'l'
            let substituted = substitute::substitute(expressions[1]);
            state.selection = Some(
              state.buffer.search_replace((expressions[0], &substituted), selection, g)?
            );
            // If that was valid we save all the arguments to support lone 's'
            state.s_args = Some((expressions[0].to_string(), substituted, g));
          }
          Ok(false)
        },
        'g' | 'v' => {
          let selection = interpret_selection(selection, state.selection, state.buffer)?;
          // Since this command may take input we need to check just as carefully as with a, i, c
          verify_selection(state.buffer, selection)?;
          let mut expressions = parse_expressions(clean);
          if expressions.len() < 2 { return Err(EXPRESSION_TOO_SHORT); }
          // We first try to mark all matching lines, to tell if there is any issue
          state.buffer.mark_matching(expressions[0], selection, ch == 'v')?;
          // Then we get the script to run against them, if not already given
          // First grab commands given on command line
          let mut commands: Vec<String> = expressions.split_off(1).iter().map(|s| s.to_string()).collect();
          // If the last command in that list is not empty it means the list was not terminated, so we take more from input
          if commands.last().map(|s| s.trim()) != Some("") {
            // expressions.len() would be 0 if no char, so safe to unwrap
            let mut input = ui.get_input(state.buffer, clean.chars().next().unwrap())?;
            commands.append(&mut input);
          }
          else {
            // If the last command was empty we should pop it, since it will otherwise cause an unexpected print
            commands.pop();
          }
          // After command collection we get the matching lines to run them at and do so
          while let Some(index) = state.buffer.get_marked()? {
            // Use dummy UI to recurse while supporting text input
            let mut dummy = DummyUI{
              input: commands.iter().map(|s| s.clone()).collect(),
              print_ui: Some(ui),
            };
            state.selection = Some((index, index));
            state.run_macro(&mut dummy)?;
          }
          Ok(false)
        },
        'G' | 'V' => {
          let selection = interpret_selection(selection, state.selection, state.buffer)?;
          // Since this command takes input we need to check just as carefully as with a, i, c
          verify_selection(state.buffer, selection)?;
          let expressions = parse_expressions(clean);
          if expressions.len() != 2 { return Err(EXPRESSION_TOO_SHORT); }
          if expressions[1].len() != 0 && expressions[1] != "\n" { return Err(UNDEFINED_FLAG); }
          // Mark first, to check if the expression is valid
          state.buffer.mark_matching(expressions[0], selection, ch == 'V')?;
          // With all data gathered we fetch and iterate over the lines
          while let Some(index) = state.buffer.get_marked()? {
            // Print the line, so the user knows what they are changing
            ui.print_selection(state.buffer, (index, index), false, false)?;
            // Get input and create dummy-ui with it
            // expressions.len() == 2 implies that a separator was given
            let input = ui.get_input(state.buffer, clean.chars().next().unwrap())?;
            let mut dummy = DummyUI{
              input: input.into(),
              print_ui: Some(ui),
            };
            state.selection = Some((index, index));
            state.run_macro(&mut dummy)?;
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
  if p | n | l {
    if let Some(sel) = state.selection {
      ui.print_selection(state.buffer, sel, n, l)?;
    }
    else {
      Err(SELECTION_EMPTY)?
    }
  }

  Ok(ret)
}
