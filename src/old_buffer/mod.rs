//! Contains Buffer, which holds the editing buffer, clipboard and undo history.

use core::iter::Iterator;
use std::rc::Rc;
use std::cell::RefCell;

use crate::error::*;

// Data structure managing undo/redo and tracking if saved
mod history;
pub use history::*;

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

// Include a general test
//#[cfg(test)]
//mod test;

/// All data stored for every line
///
/// Note the [`RefCells`] placed around internal variables we wish to be able to
/// modify without mutable access to the Line itself. This is to let [`History`]
/// enforce use of [`History::current_mut`] to edit the text, while allowing
/// changes to other data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
  pub(crate)tag: RefCell<char>,
  pub(crate)matched: RefCell<bool>,
  pub(crate)text: Rc<String>,
}

/// Exact type of the iterator returned by [`get_selection`]
///
/// Exported to enable taking the iterator as an argument without any dyn or
/// impl, which is mainly needed by IO implementations.
///
/// This is very implementation specific, so avoid depending on it more than you
/// would on a boxed iterator.
pub type SelectionIter<'b> = std::iter::Map<
  std::slice::Iter<'b, Line>,
  for<'a> fn(&'a Line) -> &'a str
>;
/// Exact type of the iterator returned by [`get_tagged_selection`]
///
/// Exported to enable taking the iterator as an argument without any dyn or
/// impl.
///
/// This is very implementation specific, so avoid depending on it more than you
/// would on a boxed iterator.
pub type TaggedSelectionIter<'b> = std::iter::Map<
  std::slice::Iter<'b, Line>,
  for<'a> fn(&'a Line) -> (char, &'a str)
>;

/// The editing Buffer built on Vec and String
///
/// The editing methods on the buffer are mirrors of the editing commands and
/// assume every method call is a separated command, managing clipboard and undo
/// history accordingly.
///
/// It stores the entire editing history, as well as the present, in a
/// [`History`] struct.
///
/// Regex functionality is imported from the Regex crate.
///
/// BEWARE!!! 1-indexed!
/// This means _line_ 0 doesn't exist, error if given (use verify_selection/verify_line below)
/// BUT, _index_ 0 is valid (therefore use verify_index instead)
/// Subtract 1 to get 0 indexed. It is recommended to use .saturating_sub(1) to
/// avoid panicking on underflows (since they drop the data unless you catch the
/// panic).
#[derive(Clone, Debug)]
pub struct Buffer {
  /// The undo history _and_ present
  pub history: History<Line>,
  /// The current clipboard
  pub clipboard: Vec<Line>,
}
impl Default for Buffer {
  fn default() -> Self { Self::new() }
}
impl Buffer {
  /// Create a new empty buffer. It is considered saved while unchanged.
  pub fn new() -> Self
  {
    Self{
      history: History::new(),
      clipboard: Vec::new(),
    }
  }
  /// Returns the nr of lines in the buffer at the current point in history
  pub fn len(&self) -> usize { self.history.current().len() }
  /// Returns true if the buffer at the current point in history is empty
  pub fn is_empty(&self) -> bool { self.history.current().is_empty() }

  // Re-exports from history, to make them more officially part of the API

  /// Re-export of [`History.saved`]
  pub fn saved(&self) -> bool { self.history.saved() }
  /// Re-export of [`History.set_saved`]
  pub fn set_saved(&mut self) { self.history.set_saved() }

  /// Method for the undo command.
  ///
  /// Re-export of [`History.undo`]. The lone command not implemented in
  /// [`Buffer`] itself, as it modifies the internal state of [`History`]
  pub fn undo(&mut self,
    steps: isize,
  ) -> Result<()> {
    self.history.undo(steps)
  }

  /// The only real output command offered by Buffer
  ///
  /// Due to using .map() with a closure the returned iterator needs to be
  /// boxed. If this bothers you PRs are welcome.
  ///
  /// Will return error on invalid selection.
  pub fn get_selection(&self,
    selection: (usize, usize),
  ) -> Result<SelectionIter> {
    verify_selection(self, selection)?;
    let tmp = self.history.current()[selection.0 - 1 .. selection.1]
      .iter()
      .map(get_selection_helper as fn(&Line) -> &str)
    ;
    Ok(tmp)
  }

  /// Same as [`get_selection`] but with each line's tag beside the text data
  ///
  /// Will return error on invalid selection.
  pub fn get_tagged_selection(&self,
    selection: (usize, usize),
  ) -> Result<TaggedSelectionIter> {
    verify_selection(self, selection)?;
    let tmp = self.history.current()[selection.0 - 1 .. selection.1]
      .iter()
      .map(get_tagged_selection_helper as fn(&Line) -> (char, &str))
    ;
    Ok(tmp)
  }
}

/// Helper to allow [`get_selection`] to return only text of lines.
pub fn get_selection_helper(line: &Line) -> &str {
  &line.text[..]
}
/// Helper to allow [`get_tagged_selection`] to return only text and tag of lines.
pub fn get_tagged_selection_helper(line: &Line) -> (char, &str) {
  (*line.tag.borrow(), &line.text[..])
}
