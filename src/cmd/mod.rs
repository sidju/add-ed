use crate::error_consts::*;

use crate::buffer::Buffer;

mod parse_selection;
use parse_selection::*;

pub fn run<'a>(state: &'a mut crate::State, command: &'a mut str)
  -> Result<(), &'static str>
{
  // Parse out the command index and the selection
  let (cmd_i, selection) = parse_selection(command)?;

  // Match the command and act upon it
  match command.chars().next() {
    None => Err(NO_COMMAND_ERR),
    // Quit commands
    Some('q') => {
      if selection != Sel::Lone(Ind::Default) { return Err(SELECTION_FORBIDDEN); }
      if state.buffer.saved() {
        state.done = true;
        Ok(())
      }
      else {
        Err(UNSAVED_CHANGES)
      }
    }
    Some('Q') => {
      if selection != Sel::Lone(Ind::Default) { return Err(SELECTION_FORBIDDEN); }
      state.done = true;
      Ok(())
    }
    // Help commands
    Some('h') => {
      if selection != Sel::Lone(Ind::Default) { return Err(SELECTION_FORBIDDEN); }
      println!("{:?}", state.error);
      Ok(())
    },
    Some('H') => {
      if selection != Sel::Lone(Ind::Default) { return Err(SELECTION_FORBIDDEN); }
      state.print_errors = !state.print_errors; // Toggle the setting
      Ok(())
    }
    // Print commands
    Some('p') | Some('n') | Some('l') => {
      // Identify which flags are set
      let mut n = false;
      let mut l = false;
      for char in command[..command.len()-1].chars() {
        match char {
          'n' => { n = true; },
          'l' => { l = true; },
          'p' => { },
          _ => return Err(UNDEFINED_FLAG),
        }
      }
      // Normalise the selection and get its lines
      let sel = interpret_selection(selection, state.selection, state.buffer.len(), false);
      let output = state.buffer.get_selection(sel)?;
      // Print the output
      crate::io::format_print( state, output, sel.0, n, l );
      // And save the selection
      state.selection = Some(sel);
      Ok(())
    }
    Some(cmd) => {
      Err(UNDEFINED_COMMAND)
    }
  }
}
