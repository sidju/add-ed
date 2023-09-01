//! The text storage structures

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use super::*;

/// Error type for [`LineText`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineTextError {
  pub text: String,
}
impl std::fmt::Display for LineTextError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Invalid line text given: {}", self.text)
  }
}
impl std::error::Error for LineTextError {}

/// An immutable text data container for a single line of text
///
/// Upon creation verifies that the text is newline terminated and contains no
/// other newlines. Also uses reference counting to prevent data duplication
/// when cloning (as this will be done very often within `add-ed`'s logic).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineText {
  inner: Rc<String>,
}
impl LineText {
  /// Create new LineText instance
  ///
  /// Returns error if the text you are creating it from isn't a single valid
  /// line (Exactly one newline should be in it, the last character).
  pub fn new<T: Into<String>>(
    text: T,
  ) -> Result<Self, LineTextError> {
    let text: String = text.into();
    // We can safely subtract 1 from len after verifying it contains a '\n'
    if !text.ends_with('\n') || text[..text.len()-1].contains('\n') {
      Err(LineTextError{text})
    } else {
      Ok(Self{ inner: Rc::new(text) })
    }
  }
}
impl std::ops::Deref for LineText {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &(*self.inner)
  }
}
impl TryFrom<&str> for LineText {
  type Error = LineTextError;
  fn try_from(t: &str) -> Result<Self, Self::Error> {
    Self::new(t)
  }
}

/// Text data and metadata for a single line of text
///
/// Note that the internal field accessed by `.tag()` and `.set_tag()` is shared
/// throughout the historical instances of the Line.
///
/// The main way to create, move around or clone Line instances is through
/// [`PubLine`]. For this purpose PubLine implements From<&Line> and Line
/// implements From<PubLine>. (Creating a PubLine from a Line is basically free,
/// creating a Line from a PubLine includes some allocations but still cheap.)
/// This is to ensure that the internal pointers that Line uses internally
/// (sharing objects between historical states) aren't pointing to somewhere
/// they shouldn't.

// We don't derive Clone, since it wouldn't be  what library users expect.
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
  // The tag set on the given line
  //
  // Rc<Cell> makes it so we can have the same tag throughout all snapshots of
  // the same line, but also requires us to hide the variable (so library users
  // can't clone the Rc and cause strange behaviour.
  tag: Rc<Cell<char>>,
  /// The text data for a given line
  ///
  /// [`LineText`] ensures that the text data is valid for a single line and
  /// implements reference counted cloning, allowing easy re-use of the same
  /// data (through history, clipboard and even multiple identical lines
  /// (depending on how they are created)).
  pub text: LineText,
}
impl Line {
  /// Create a new line with given text data
  ///
  /// Returns LineTextError if the data is not newline terminated or contains
  /// other newlines than the terminating one.
  ///
  /// Sets the tag to `'\0'`, which is the data representation of not tagged.
  pub (crate) fn new<T: Into<String>>(
    text: T,
  ) -> Result<Self, LineTextError> {
    Ok(Self{
      matched: Rc::new(RefCell::new(Vec::new())),
      tag: Rc::new(Cell::new('\0')),
      text: LineText::new(text)?,
    })
  }
  /// Get the current value of the tag field.
  pub fn tag(&self) -> char {
    self.tag.get()
  }
  /// Set the tag to given character
  ///
  /// Note that this changes all historical states of this line.
  pub fn set_tag(&self, new: char) {
    self.tag.set(new)
  }
}
// Our internal-only Clone implementation, to enable snapshotting without
// misleading library users that they can Clone Lines.
impl Snapshot for Line {
  fn create_snapshot(&self) -> Self {
    Line{
      tag: self.tag.clone(),
      matched: self.matched.clone(),
      text: self.text.clone(),
    }
  }
}
impl From<&PubLine> for Line {
  fn from(l: &PubLine) -> Self {
    Self{
      text: l.text.clone(),
      tag: Rc::new(Cell::new(l.tag)),
      matched: Rc::new(RefCell::new(Vec::new())),
    }
  }
}
impl TryFrom<&str> for Line {
  type Error = LineTextError;
  fn try_from(t: &str) -> Result<Self, Self::Error> {
    Self::new(t)
  }
}

/// A fully public version of the [`Line`] struct
///
/// Intended for API interaction, since it cannot represent the internal state
/// in Line which could cause trouble if invalid.
///
/// [`From`] is implemented both ways, to make it easy to convert into and from
/// [`Line`]. Some TryFrom implementations that may be useful also exist.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PubLine {
  /// The tag set on the line
  ///
  /// See [`Line`] `.tag()` and `.set_tag()`, but note that we disconnect the
  /// shared tag state through history by converting into this.
  pub tag: char,
  /// The text data for the line
  ///
  /// See [`Line'].text.
  pub text: LineText,
}
impl<'a> TryFrom<&'a str> for PubLine {
  type Error = LineTextError;
  fn try_from(t: &str) -> Result<Self, Self::Error> {
    Ok(Self{tag: '\0', text: LineText::new(t)?})
  }
}
impl<'a> TryFrom<&'a &'a str> for PubLine {
  type Error = LineTextError;
  fn try_from(t: &&str) -> Result<Self, Self::Error> {
    Ok(Self{tag: '\0', text: LineText::new(*t)?})
  }
}
impl<'a> TryFrom<(char, &'a str)> for PubLine {
  type Error = LineTextError;
  fn try_from(l: (char, &str)) -> Result<Self, Self::Error> {
    (&l).try_into()
  }
}
impl<'a> TryFrom<&'a (char, &'a str)> for PubLine {
  type Error = LineTextError;
  fn try_from(l: &(char, &str)) -> Result<Self, Self::Error> {
    Ok(Self{tag: l.0, text: LineText::new(l.1)?})
  }
}
impl From<&Line> for PubLine {
  fn from(l: &Line) -> Self {
    Self{tag: l.tag.get(), text: l.text.clone()}
  }
}
