//! This module is for supporting scripts.
//! The entire script is put into a vector of strings and treated as input.
//! It optionally takes a mutable UI reference, to support printing when the script requests it.

use super::UI;
use super::Buffer;

use std::collections::VecDeque;

pub struct DummyUI<'a> {
  pub input: VecDeque<String>,
  pub print_ui: Option<&'a mut dyn UI>,
}
impl <'a> UI for DummyUI<'a> {
  /// Gets the next line of the input
  fn get_command(&mut self,
    _buffer: & dyn Buffer,
  ) -> Result<String, &'static str> {
    match self.input.pop_front() {
      Some(x) => Ok(x),
      // Returns from the macro execution no matter what.
      None => Ok("Q\n".to_string()),
    }
  }

  /// Gets lines from input until one matches terminator
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

  /// Printing is handed to the print_ui if one was given, else ignored
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
