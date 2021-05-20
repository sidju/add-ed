//! This module is for supporting scripts.
//! The entire script is put into a vector of strings and treated as input.
//! It optionally takes a mutable UI reference, to support printing when the script requests it.

use super::UI;
use super::Buffer;

use std::collections::VecDeque;

/// This is a dummy UI. That means it simulates an UI without interfacing with any users.
///
/// How to use:
/// * Put the input to simulate line-by-line in the input variable.
/// * If you want output from print commands put the UI to print with in print_ui.

// Eq not derived since its intent is unclear and internals are public.
// Compare the input and/or the print_ui individually instead.
#[derive(Clone, Hash, Debug)]
pub struct DummyUI<'a> {
  pub input: VecDeque<String>,
  pub print_ui: Option<&'a mut dyn UI>,
}
impl <'a> UI for DummyUI<'a> {
  fn get_command(&mut self,
    _buffer: & dyn Buffer,
  ) -> Result<String, &'static str> {
    match self.input.pop_front() {
      Some(x) => Ok(x),
      // Returns from the macro execution no matter what.
      None => Ok("Q\n".to_string()),
    }
  }
  fn get_input(&mut self,
    _buffer: & dyn Buffer,
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
  fn print(&mut self, text: &str) -> Result<(), &'static str> {
    match &mut self.print_ui {
      Some(ui) => ui.print(text),
      None => Ok(()),
    }
  }
  fn print_selection(&mut self,
    buffer: & dyn Buffer,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<(), &'static str> {
    match &mut self.print_ui {
      Some(ui) => {
        ui.print_selection(buffer, selection, numbered, literal)
      },
      None => Ok(()),
    }
  }
}
