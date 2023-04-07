//! Contains the Buffer trait and any build in implementations.

// General implementations for file interaction and substitution of e.g. '\n'
mod substitute;
mod verify;
pub use verify::*;

// Spread out methods into multiple files
// Methods pertaining to finding things in a file
mod finding;
pub use finding::*;
// Methods for editing buffer contents
mod editing;
pub use editing::*;
// Methods regarding undo/redo
mod undo;
pub use undo::*;

// Include a general test
#[cfg(test)]
mod test;

use core::iter::Iterator;
use std::rc::Rc;

use crate::error_consts::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Line {
  tag: char,
  matched: bool,
  text: Rc<String>,
}

/// The editing Buffer built on Vec and String
///
/// It stores the entire editing history in a vector of history states.
/// Each history state is in turn a vector of lines as they were at that time.
/// And each line is a Rc<String> (newline inclusive), to avoid data copying.
/// Regex functionality is imported from the Regex crate.
///
/// BEWARE!!! 1-indexed!
/// This means _line_ 0 doesn't exist, error if given (use verify_selection/verify_line below)
/// BUT, _index_ 0 is valid (therefore use verify_index instead)
/// Subtract 1 to get 0 indexed. It is recommended to use .saturating_sub(1)
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Buffer {
  saved: bool,
  // Chars used for tagging. No tag equates to NULL in the char
  history: Vec<Vec<Line>>,
  buffer_i: usize, // The index in history currently seen by user
  clipboard: Vec<Line>,
}
impl Default for Buffer {
  fn default() -> Self { Self::new() }
}
impl Buffer {
  /// Create a new empty buffer. It is considered saved while unchanged.
  pub fn new() -> Self
  {
    Self{
      saved: true,
      history: vec![vec![]],
      buffer_i: 0,
      clipboard: Vec::new(),
    }
  }
  pub fn len(&self) -> usize { self.history[self.buffer_i].len() }
  pub fn is_empty(&self) -> bool { self.history[self.buffer_i].is_empty() }

  pub fn set_saved(&mut self) {
    self.saved = true;
  }
  pub fn set_unsaved(&mut self) {
    self.saved = false;
  }
  pub fn saved(&self) -> bool {
    self.saved
  }

  // The output command
  pub fn get_selection<'a>(&'a self, selection: (usize, usize))
    -> Result<Box<dyn Iterator<Item = (char, &'a str)> + 'a>, &'static str>
  {
    verify_selection(self, selection)?;
    let tmp = self.history[self.buffer_i][selection.0 - 1 .. selection.1]
      .iter()
      .map(|line| (line.tag, &line.text[..]))
    ;
    Ok(Box::new(tmp))
  }
}
