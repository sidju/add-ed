//! This module is for supporting scripts.
//! The entire script is put into a vector of strings and treated as input.
//! It optionally takes a mutable UI reference, to support printing when the script requests it.

use super::{UI, UILock};
use super::Ed;

#[cfg(feature = "initial_input_data")]
use crate::EdError;

use super::Result;

use std::collections::VecDeque;

/// Error type for Scripted UI which can only occur if you enable the feature
/// `initial_input_data` and given initial data to [`ScriptedUI::get_input`]
#[derive(Debug)]
pub struct UnsupportedInitialData{}
impl std::fmt::Display for UnsupportedInitialData {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "You cannot take input with initial data in a script.")
  }
}
impl std::error::Error for UnsupportedInitialData {}
impl crate::error::UIError for UnsupportedInitialData {}

/// This is a scripted UI. It returns the scripted input without querying user.
///
/// How to use:
/// * Put the input to simulate line-by-line in the input variable.
///   (terminating '\n' required, errors may arize if missing)
/// * If you want output from print commands put a UI to print with in
///   `print_ui`.
///   (If none given prints will be quietly ignored)

// Things not derived here since they would require the same being implemented
// on the UI trait, which is too extreme for me at this stage. If you have need,
// complain and I'll add it.
pub struct ScriptedUI<'a> {
  pub input: VecDeque<String>,
  pub print_ui: Option<&'a mut dyn UI>,
}
impl <'a> UI for ScriptedUI<'a> {
  fn get_command(&mut self,
    _ed: &Ed,
    _prefix: Option<char>
  ) -> Result<String> {
    match self.input.pop_front() {
      Some(x) => Ok(x),
      // Returns from the macro execution no matter what.
      None => Ok("Q\n".to_string()),
    }
  }
  fn get_input(&mut self,
    _ed: &Ed,
    terminator: char,
    #[cfg(feature = "initial_input_data")]
    initial_buffer: Option<Vec<String>>, // causes error
  ) -> Result<Vec<String>> {
    #[cfg(feature = "initial_input_data")]
    {
      if initial_buffer.is_some() {
        return Err(EdError::UI(Box::new(UnsupportedInitialData{})))
      }
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
  ) -> Result<()> {
    match &mut self.print_ui {
      Some(ui) => ui.print_message(text),
      None => Ok(()),
    }
  }
  fn print_selection(&mut self,
    ed: &Ed,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<()> {
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
