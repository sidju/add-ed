use super::*;

use crate::{EdError, Result};

/// Declare a type over Vec<PubLine>, to be able to add some utility methods
///
/// Unlike [`Buffer`] it is possible to easily and safely edit a Clipboard via
/// its `AsRef<Vec<PubLine>>` implementation. You are advised to use this,
/// combined with `.into()`, to safely interact with `Buffer` instances. See
/// example code in [`Buffer`] for how this should be done. 
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
impl<'a> From<&'a [Line]> for Clipboard {
  fn from(l: &'a [Line]) -> Self {
    let mut tmp = Vec::new();
    for line in l {
      tmp.push(line.into());
    }
    Self{
      inner: tmp,
    }
  }
}
impl<'a> TryFrom<&'a [(char, &str)]> for Clipboard {
  type Error = LineTextError;
  fn try_from(l: &'a [(char,&str)]) -> core::result::Result<Self, Self::Error> {
    let mut tmp = Vec::new();
    for line in l {
      tmp.push(line.try_into()?);
    }
    Ok(Self{
      inner: tmp,
    })
  }
}
impl<'a> TryFrom<&'a [&str]> for Clipboard {
  type Error = LineTextError;
  fn try_from(l: &'a [&str]) -> core::result::Result<Self, Self::Error> {
    let mut tmp = Vec::new();
    for line in l {
      tmp.push(line.try_into()?);
    }
    Ok(Self{
      inner: tmp,
    })
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
///
/// Though `Deref<Target = Vec<Line>>` gives free access to the Buffer internals
/// the restrictions upon Line makes it a bit difficult to make use of this. The
/// intended method is to convert to and from [`Clipboard`] or [`PubLine`] as
/// shown below.
///
/// Examples of how to construct Line instances to insert into the Buffer:
/// ```
/// use add_ed::{
///   Buffer,
///   Clipboard,
///   PubLine,
/// };
///
/// // Note that all the Results we unwrap will only occur if the text is
/// // invalid for the Buffer, wherein text must be newline terminated and not
/// // contain any other newlines.
///
/// let mut buffer = Buffer::default();
/// // Note that we can create a PubLine by tag+text tuples
/// let pub_line: PubLine = ('a', "test\n").try_into().expect("Invalid line");
/// buffer.push((&pub_line).into());
/// // Or just from &str
/// let pub_line: PubLine = "data\n".try_into().expect("Invalid line");
/// buffer.push((&pub_line).into());
/// // And when you want to add a single line from &str you don't need to
/// // create an intermediate PubLine, since Line implements from &str
/// buffer.push("&str\n".try_into().expect("Invalid line"));
/// ```
///
/// Examples of how to copy out and insert multiple lines of data:
/// ```
/// use add_ed::{
///   Buffer,
///   Clipboard,
///   PubLine,
/// };
///
/// // Note that all the Results we unwrap will only occur if the text is
/// // invalid for the Buffer, wherein text must be newline terminated and not
/// // contain any other newlines.
///
/// let mut buffer = Buffer::default();
/// // Since we can construct a Clipboard from a slice of (char, &str)
/// let pub_lines: Clipboard = (&vec![('b', "more\n"),('\0', "data\n")][..])
///   .try_into().expect("Invalid line");
/// buffer.append(&mut (&pub_lines).into());
/// // And of course you don't have to give tags if you don't want to
/// let pub_lines: Clipboard = (&vec!["last\n","data\n"][..])
///   .try_into().expect("Invalid line");
/// buffer.append(&mut (&pub_lines).into());
/// // Getting data out in clipboard format is quite easy (and generally the
/// // way to go, unless you are just moving Lines around).
/// let fetched_data: Clipboard = (&buffer[..]).into();
/// // If you want you can also use the iterators on Buffer
/// let fetched_data: Vec<String> = buffer.get_lines((1,buffer.len()))
///   .expect("Invalid selection")
///   .map(|s| s.to_owned())
///   .collect()
/// ;
/// ```
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
  (line.tag(), &line.text[..])
}
