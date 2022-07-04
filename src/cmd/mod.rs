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
      // Get and update the selection.
      let sel = interpret_selection(selection, state.selection, state.buffer)?;
      verify_selection(state.buffer, sel)?;
      state.selection = sel;
      pflags.p = true; // Default command is 'p'
      Ok(false)
    },
    Some(ch) => {
      let clean = {
        let mut x = 1;
        while ! command.is_char_boundary(cmd_i + x) { x += 1; }
        &command[cmd_i + x ..].trim()
      };
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
          if clean == &"elp" {
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
        }
        // Pattern commands
        's' => {
          let selection = interpret_selection(selection, state.selection, state.buffer)?;
          // switch based on if clean was given or not
          if clean.is_empty() {
            // This means we use the arguments stored in state.s_args
            match &state.prev_s {
              None => return Err(NO_PRIOR_S),
              Some(s) => {
                pflags.p = s.p;
                pflags.n = s.n;
                pflags.l = s.l;
                let end = state.buffer.search_replace((&s.pattern, &s.substitute), selection, s.global)?;
                // If we have deleted the whole selection we start sel at end,
                // in order to select line before the deleted lines. (min(end))
                // If end is smaller than 1 we have deleted to start of
                // buffer, then we use (1,0). (max(1))
                state.selection = (selection.0.min(end).max(1), end);
              }
            }
          }
          else {
            let expressions = parse_expressions(clean)?;
            if expressions.len() != 3 { return Err(EXPRESSION_TOO_SHORT); }
            let mut flags = parse_flags(&(expressions[2]), "gpnl")?;
            let g = flags.remove(&'g').unwrap();
            pflags.p = flags.remove(&'p').unwrap();
            pflags.n = flags.remove(&'n').unwrap();
            pflags.l = flags.remove(&'l').unwrap();
            let end = state.buffer.search_replace((&expressions[0], &expressions[1]), selection, g)?;
            // If we have deleted the whole selection we start sel at end,
            // in order to select line before the deleted lines. (min(end))
            // If end is smaller than 1 we have deleted whole buffer,
            // then we use (1,0). (max(1))
            state.selection = (selection.0.min(end).max(1), end);
            // If that was valid we save all the arguments to support lone 's'
            state.prev_s = Some(Substitution{
              pattern: expressions[0].to_string(),
              substitute: expressions[1].to_string(),
              global: g,
              p: pflags.p,
              n: pflags.n,
              l: pflags.l,
            });
          }
          Ok(false)
        },
        'g' | 'v' => {
          let selection = interpret_selection(selection, state.selection, state.buffer)?;
          // Since this command may take input we need to check just as carefully as with a, i, c
          verify_selection(state.buffer, selection)?;
          let mut expressions = parse_expressions(clean)?;
          if expressions.len() < 2 { return Err(EXPRESSION_TOO_SHORT); }
          // We first try to mark all matching lines, to tell if there is any issue
          state.buffer.mark_matching(&expressions[0], selection, ch == 'v')?;
          // Then we get the script to run against them, if not already given
          // First grab commands given on command line
          let mut commands: Vec<String> = expressions.split_off(1).iter().map(|s| s.to_string()).collect();
          // If the last command in that list is not empty it means the list was not terminated, so we take more from input
          if commands.last().map(|s| s.trim()) != Some("") {
            // expressions.len() would be 0 if no char, so safe to unwrap
            let mut input = ui.get_input(
              state.see_state(),
              clean.chars().next().unwrap(),
              #[cfg(feature = "initial_input_data")]
              None,
            )?;
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
              input: commands.iter().cloned().collect(),
              print_ui: Some(ui),
            };
            state.selection = (index, index);
            state.run_macro(&mut dummy)?;
          }
          Ok(false)
        },
        'G' | 'V' => {
          let selection = interpret_selection(selection, state.selection, state.buffer)?;
          // Since this command takes input we need to check just as carefully as with a, i, c
          verify_selection(state.buffer, selection)?;
          let expressions = parse_expressions(clean)?;
          if expressions.len() != 2 { return Err(EXPRESSION_TOO_SHORT); }
          if !expressions[1].is_empty() && expressions[1] != "\n" { return Err(UNDEFINED_FLAG); }
          // Mark first, to check if the expression is valid
          state.buffer.mark_matching(&expressions[0], selection, ch == 'V')?;
          // With all data gathered we fetch and iterate over the lines
          while let Some(index) = state.buffer.get_marked()? {
            // Print the line, so the user knows what they are changing
            ui.print_selection(state.see_state(), (index, index), state.n, state.l)?;
            // Get input and create dummy-ui with it
            // expressions.len() == 2 implies that a separator was given
            let input = ui.get_input(
              state.see_state(),
              clean.chars().next().unwrap(),
              #[cfg(feature = "initial_input_data")]
              None,
            )?;
            let mut dummy = DummyUI{
              input: input.into(),
              print_ui: Some(ui),
            };
            state.selection = (index, index);
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
