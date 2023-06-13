//! Helper module to enable testing IO implementations
//!
//! Due to the IO trait taking [`SelectionIter`] as an argument for several
//! functions, and the type of [`SelectionIter`] basically defining the exact
//! implementation that created the iterator, some helpers are needed to
//! construct [`SelectionIter`] instances with test data.
//!
//! It is recommended to use these helpers instead of depending on the current
//! type of [`SelectionIter`], just as it is recommended to not depend on any
//! capabilities on the [`SelectionIter`] beyond `Iterator<Item=&str>`, since
//! the exact type of [`SelectionIter`] is likely to change in the future.

use crate::buffer::{
  SelectionIter,
  Line,
  get_selection_helper,
};

/// Simplest type to imitate a buffer well enough to create a
/// [`SelectionIter`] from
///
/// (Needs to be written to a variable and kept, since [`SelectionIter`]
/// borrows from its data source.)
pub struct PseudoBuf( Vec<Line> );
impl PseudoBuf {
  /// Constructor from `&str`, splits into lines using [`str::lines()`]
  pub fn new(input: &str) -> Self {
    Self(
      input
        .lines()
        .map(|s|{
          let mut line_text = s.to_owned();
          line_text.push('\n');
          Line{
            tag: '\0'.into(),
            matched: false.into(),
            text: line_text.into(),
          }
        })
        .collect()
    )
  }
  /// Creates a [`SelectionIter`] over the contents of the [`PseudoBuf`]
  pub fn as_selectioniter(&self) -> SelectionIter {
    self.0[..]
      .iter()
      .map(get_selection_helper as fn(&Line) -> &str)
  }
}
