//! The text storage structures

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use std::fmt::Debug;

use crate::{Result, EdError};

use crate::history::Snapshot;

pub mod iters;
use iters::*;

/// Text data and metadata for a single line of text
///
/// Note the [`Rc<Cell>`]s placed around internal variables we wish to share
/// between points in history while allowing modification, both in that Rc and
/// without [`&mut`] access to the line. This is to let [`History`] enforce use
/// of [`History::current_mut`] to edit the text, while allowing changes to
/// other data using [`History::current`] (which doesn't create undo snapshots).

// We don't derive Clone, since it usually isn't what library users expect.
// Instead we write a manual clone for Buffer, so History can do its thing.
#[derive(Debug, PartialEq, Eq)]
pub struct Line {
  // Tracks if the line has been matched in a 'g' or similar command in a shared
  // instance throughout the line's lifetime (to save on allocations)
  // (A change to BitVec would be good, TODO.)
  //
  // To support nested invocations we have a vector, where index 0 is the
  // outermost invocation and nested invocation have incrementing indices.
  //
  // Note that this has one main gotchas that must be handled where this is
  // used:
  //   There mustn't be any old data on an index when an invocation uses it,
  //   not even outside the selection acted upon.
  //   (Handled by src/cmd/regex_commands.rs : mark_matching, which resizes
  //   Vec to the correct length and overwrites the soon to be relevant index
  //   across all lines. Since mark_matching is called for each nested
  //   invocation this should be run on every relevant index before it's used.)
  //
  // Also note that this will be empty on newly created lines, but get_matching
  // handles this by defaulting to false and mark_matching explicitly resizes to
  // the size it needs.
  pub(crate) matched: Rc<RefCell<Vec<bool>>>,
  /// The tag set on the given line
  ///
  /// It is stored in an Rc<Cell> to have a shared overwriteable tag for all
  /// historical instances of the line.
  pub tag: Rc<Cell<char>>,
  /// The text data for a given line
  ///
  /// It is stored in an Rc as a CoW mechanism, since it allows a shared
  /// allocation for historical states with the same text data while
  /// preventing modification of that shared state. To modify, create a new
  /// String with the data you want and put it in a new Rc in this field.
  pub text: Rc<String>,
}
impl Line {
  pub (crate) fn new<T: Into<String>>(text: T, tag: char) -> Self {
    Self{
      matched: Rc::new(RefCell::new(Vec::new())),
      tag: Rc::new(Cell::new(tag)),
      text: Rc::new(text.into()),
    }
  }
}
impl Snapshot for Line {
  fn create_snapshot(&self) -> Self {
    Line{
      tag: self.tag.clone(),
      matched: self.matched.clone(),
      text: self.text.clone(),
    }
  }
}

/// A fully public version of the [`Line`] struct above
///
/// Intended for API interaction, since it cannot represent the metadata in Line
/// which could cause trouble if invalid.
///
/// [`From`] is implemented both ways, to make it easy to convert into and from
/// [`Line`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PubLine {
  /// The tag set on the line
  ///
  /// See [`Line.tag`], but note that we disconnect the shared tag state through
  /// history by converting into this.
  pub tag: char,
  /// The text data for the line
  ///
  /// See [`Line.text`].
  pub text: Rc<String>,
}

/// Declare a type over Vec<PubLine>, to be able to add some utility methods
///
/// Needed due to orphan rules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Clipboard {
  inner: Vec<PubLine>,
}
impl Clipboard {
  pub fn new() -> Self {
    Self{ inner: Vec::new() }
  }
}

impl std::ops::Deref for Clipboard {
  type Target = Vec<PubLine>;
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}
impl std::ops::DerefMut for Clipboard {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}
impl From<&str> for PubLine {
  fn from(l: &str) -> Self {
    Self{tag: '\0', text: Rc::new(l.to_owned())}
  }
}
impl From<(char, &str)> for PubLine {
  fn from(l: (char, &str)) -> Self {
    Self{tag: l.0, text: Rc::new(l.1.to_owned())}
  }
}
impl From<&Line> for PubLine {
  fn from(l: &Line) -> Self {
    Self{tag: l.tag.get(), text: l.text.clone()}
  }
}
impl<'a, T> From<&'a [T]> for Clipboard
where
  &'a T: Into<PubLine>,
{
  fn from(l: &'a [T]) -> Self {
    let mut tmp = Vec::new();
    for line in l {
      tmp.push(line.into());
    }
    Self{
      inner: tmp,
    }
  }
}
impl From<&PubLine> for Line {
  fn from(l: &PubLine) -> Self {
    Self{
      tag: Rc::new(Cell::new(l.tag)),
      text: l.text.clone(),
      matched: Rc::new(RefCell::new(Vec::new())),
    }
  }
}
impl From<&str> for Line {
  fn from(l: &str) -> Self {
    Self{
      tag: Rc::new(Cell::new('\0')),
      text: Rc::new(l.to_owned()),
      matched: Rc::new(RefCell::new(Vec::new())),
    }
  }
}
impl Into<Vec<Line>> for &Clipboard {
  fn into(self) -> Vec<Line> {
    let mut tmp = Vec::new();
    for line in &self.inner {
      tmp.push(line.into());
    }
    tmp
  }
}

/// Declare a type over Vec<Line>, to be able to add some utility methods
#[derive(Debug, PartialEq)]
pub struct Buffer {
  pub inner: Vec<Line>,
}
impl std::ops::Deref for Buffer {
  type Target = Vec<Line>;
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}
impl std::ops::DerefMut for Buffer {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}
// Manually implement a special clone for History
impl Snapshot for Buffer {
  fn create_snapshot(&self) -> Self {
    let mut new_inner = Vec::new();
    for line in self.inner.iter() {
      new_inner.push(line.create_snapshot());
    }
    Self{ inner: new_inner }
  }
}
impl Default for Buffer {
  fn default() -> Self{ Self{ inner: Vec::new() } }
}
impl Buffer {
  /// Verify that an index is valid to operate on
  ///
  /// Doesn't mean that there exists a line at the index.
  /// Note the related [`Ed::verify_line`] and [`Ed::verify_selection`].
  pub fn verify_index(
    &self,
    index: usize,
  ) -> Result<()> {
    let buffer_len = self.len();
    if index > buffer_len {
      Err(EdError::IndexTooBig{index, buffer_len})
    } else {
      Ok(())
    }
  }
  /// Verfy that a line exists at given index
  ///
  /// Note the related [`Ed::verify_index`] and  [`Ed::verify_selection`].
  pub fn verify_line(
    &self,
    index: usize,
  ) -> Result<()> {
    if index == 0 { Err(EdError::Line0Invalid) }
    else { self.verify_index(index) }
  }
  /// Verify that all the lines in selection exist
  ///
  /// Note the related [`Ed::verify_index`] [`Ed::verify_line`].
  pub fn verify_selection(
    &self,
    selection: (usize, usize),
  ) -> Result<()> {
    self.verify_line(selection.0)?;
    self.verify_line(selection.1)?;
    if selection.0 > selection.1 {
      Err(EdError::SelectionEmpty(selection))
    } else {
      Ok(())
    }
  }

  /// Get the lines in the given selection
  ///
  /// Returns an iterator over &str to save on allocations.
  pub fn get_lines(
    &self,
    selection: (usize, usize),
  ) -> Result<LinesIter> {
    self.verify_selection(selection)?;
    Ok(self[selection.0 - 1 .. selection.1]
      .iter()
      .map(get_lines_helper as fn(&Line) -> &str)
      .into()
    )
  }
  /// Get the lines in the given selection with their tags
  ///
  /// Returns an iterator of (char, &str) to save on allocations.
  pub fn get_tagged_lines(
    &self,
    selection: (usize, usize),
  ) -> Result<TaggedLinesIter> {
    self.verify_selection(selection)?;
    Ok(self[selection.0 - 1 .. selection.1]
      .iter()
      .map(get_tagged_lines_helper as fn(&Line) -> (char, &str))
      .into()
    )
  }
}

// These functions need to be declared, because the map iterator over a closure
// has an un-name-able type (and we don't wish to use generics towards IO for 
// performance and being able to make dyn IO).
fn get_lines_helper(line: &Line) -> &str {
  &line.text[..]
}
fn get_tagged_lines_helper(line: &Line) -> (char, &str) {
  (line.tag.get(), &line.text[..])
}
