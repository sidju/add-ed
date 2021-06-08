//! This module is for supporting scripts.
//! The entire script is put into a vector of strings and treated as input.
//! It optionally takes a mutable UI reference, to support printing when the script requests it.

use super::UI;
use super::EdState;

use std::collections::VecDeque;

/// This is a dummy UI. That means it simulates an UI without interfacing with any users.
///
/// How to use:
/// * Put the input to simulate line-by-line in the input variable.
/// * If you want output from print commands put the UI to print with in print_ui.

// Things not derived here since they would require the same being implemented on the UI trait,
// which may be too extreme for me at this stage. If you have need, complain and I'll fix it.
pub struct DummyUI<'a> {
  pub input: VecDeque<String>,
  pub print_ui: Option<&'a mut dyn UI>,
}
impl <'a> UI for DummyUI<'a> {
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
  ) -> Result<Vec<String>, &'static str> {
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
  fn print(
    &mut self,
    ed: EdState,
    text: &str
  ) -> Result<(), &'static str> {
    match &mut self.print_ui {
      Some(ui) => ui.print(ed, text),
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
}
