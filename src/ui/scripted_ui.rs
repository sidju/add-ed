//! This module is for supporting scripts.
//! The entire script is put into a vector of strings and treated as input.
//! It optionally takes a mutable UI reference, to support printing when the script requests it.

use super::{UI, UILock};
use super::EdState;

use std::collections::VecDeque;

/// This is a scripted UI. It returns the scripted input without querying user.
///
/// How to use:
/// * Put the input to simulate line-by-line in the input variable.
/// * If you want output from print commands put the UI to print with in print_ui.

// Things not derived here since they would require the same being implemented on the UI trait,
// which may be too extreme for me at this stage. If you have need, complain and I'll fix it.
pub struct ScriptedUI<'a> {
  pub input: VecDeque<String>,
  pub print_ui: Option<&'a mut dyn UI>,
}
impl <'a> UI for ScriptedUI<'a> {
  fn get_command(&mut self,
    _ed: EdState,
    _prefix: Option<char>
  ) -> Result<String, &'static str> {
    match self.input.pop_front() {
      Some(x) => Ok(x),
      // Returns from the macro execution no matter what.
      None => Ok("Q\n".to_string()),
    }
  }
  fn get_input(&mut self,
    _ed: EdState,
    terminator: char,
    #[cfg(feature = "initial_input_data")]
    initial_buffer: Option<Vec<String>>, // causes error
  ) -> Result<Vec<String>, &'static str> {
    #[cfg(feature = "initial_input_data")]
    {
      if initial_buffer.is_some() { return Err(crate::error_consts::UNSUPPORTED_INITIAL_DATA); }
    }
    let mut ret = Vec::new();
    let term = format!("{}\n", terminator);
    // Loop until we run out of data or find the terminator
    loop {
      match self.input.pop_front() {
        None => return Ok(ret), // Return what we have
        Some(x) => {
          if x == term { return Ok(ret); }
          ret.push(x)
        }
      }
    }
  }
  // Printing is handed to the print_ui if one was given, else ignored
  fn print_message(
    &mut self,
    text: &str
  ) -> Result<(), &'static str> {
    match &mut self.print_ui {
      Some(ui) => ui.print_message(text),
      None => Ok(()),
    }
  }
  fn print_selection(&mut self,
    ed: EdState,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<(), &'static str> {
    match &mut self.print_ui {
      Some(ui) => {
        ui.print_selection(ed, selection, numbered, literal)
      },
      None => Ok(()),
    }
  }
  fn lock_ui(&mut self) -> UILock<'_> {
    match self.print_ui {
      Some(ref mut i) => i.lock_ui(),
      None => UILock::new(self),
    }
  }
  // Will only be called if no inner UI, beware
  fn unlock_ui(&mut self) {}
}
