//! A module for an obfuscating iterator, to not leak too much implementation
//! through the API.

use crate::Line;

// Type shorthand for the current implementation specific iterator, to reduce
// how many times I have to write out this monstrosity of a type
type Inner<'b> = std::iter::Map<
  std::slice::Iter<'b, Line>, for<'a> fn(&'a Line) -> &'a str
>;
type TaggedInner<'b> = std::iter::Map<
  std::slice::Iter<'b, Line>, for<'a> fn(&'a Line) -> (char, &'a str)
>;

/// The iterator returned by [`Ed::get_selection`]
///
/// To simplify [`IO`] testing it implements
/// From<Box<dyn Iterator<Item = &str>>> when the "testing" feature is enabled.
pub struct LinesIter<'a> {
  // enum internal, so we can have a low cost match during testing and no
  // overhead or generics complexity at runtime.
  inner: LinesIterInner<'a>,
}
// Wrapped by struct, so we can hide the internal state
enum LinesIterInner<'a> {
  Real(Inner<'a>),
  #[cfg(any(fuzzing, test))]
  Test(Box<dyn Iterator<Item = &'a str>>),
}

impl<'a> Iterator for LinesIter<'a> {
  type Item = &'a str;

  fn next(&mut self) -> Option<Self::Item> {
    match &mut self.inner {
      LinesIterInner::Real(x) => x.next(),
      #[cfg(any(fuzzing, test))]
      LinesIterInner::Test(x) => x.next(),
    }
  }
}

impl<'a> From<Inner<'a>> for LinesIter<'a> {
  fn from(i: Inner<'a>) -> Self {
    Self{ inner: LinesIterInner::Real(i) }
  }
}

#[cfg(any(fuzzing, test))]
impl<'a, I: Iterator<Item=&'a str> + 'static> From<Box<I>> for LinesIter<'a> {
  fn from(i: Box<I>) -> Self {
    Self{ inner: LinesIterInner::Test(i) }
  }
}
/// The iterator returned by [`Ed::get_tagged_selection`]
///
/// As it isn't specifically required by anything there is no testing
/// functionality added.
pub struct TaggedLinesIter<'a> {
  inner: TaggedInner<'a>,
}

impl<'a> Iterator for TaggedLinesIter<'a> {
  type Item = (char, &'a str);

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }
}

impl<'a> From<TaggedInner<'a>> for TaggedLinesIter<'a> {
  fn from(i: TaggedInner<'a>) -> Self {
    Self{ inner: i }
  }
}
