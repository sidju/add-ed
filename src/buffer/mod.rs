//! Contains the Buffer trait and any build in implementations.

// General implementations for file interaction and substitution of e.g. '\n'
pub mod file;
pub mod substitute;

// Include a general test
// Doesn't test file handling to allow buffers to act on non-file paths
#[cfg(test)]
mod test;

// Include the buffer implementations based on features
#[cfg(feature = "vecbuffer")]
mod vecbuffer;
#[cfg(feature = "vecbuffer")]
pub use vecbuffer::*;

/// Trait that defines a buffer supporting 'ed's base commands
///
/// BEWARE!!! 1-indexed!
/// This means _line_ 0 doesn't exist, error if given (use verify_selection/verify_line below)
/// BUT, _index_ 0 is valid (therefore use verify_index instead)
/// Subtract 1 to get 0 indexed. It is recommended to use .saturating_sub(1)
pub trait Buffer {

  // Functions for resolving and verifying indices in the parser
  /// Return the number of lines stored in the buffer
  fn len(&self)
    -> usize ;
  /// Get line tagged with given letter. Not found is error
  fn get_tag(&self, tag: char)
    -> Result<usize, &'static str> ;
  /// Return the nearest previous/following index in the selection that contains the regex pattern
  fn get_matching(&self, pattern: &str, curr_line: usize, backwards: bool)
    -> Result<usize, &'static str> ;

  // Regex matching for the macro commands ('g', 'v', 'G', 'V')
  /// Set the matched flag on all lines matching given pattern
  fn mark_matching(&mut self, pattern: &str, selection: (usize, usize), inverse: bool)
    -> Result<(), &'static str> ;
  /// Get a line with the matched flag set, clearing that line's flag
  fn get_marked(&mut self)
    -> Result<Option<usize>, &'static str> ;

  // Simple buffer modifications, but with possibly complex storage
  /// Mark a line with a letter, mark with '\0' to clear
  fn tag_line(&mut self, index: usize, tag: char) 
    -> Result<(), &'static str> ;
  /// Takes a iterator over lines in strings and inserts after given index
  fn insert<'a>(&mut self, data: &mut dyn Iterator<Item = &'a str>, index: usize)
    -> Result<(), &'static str> ;
  /// Cut the selection from the buffer, into the clipboard
  fn cut(&mut self, selection: (usize, usize))
    -> Result<(), &'static str> ;
  /// Replace selection with input
  fn change<'a>(&mut self, data: &mut dyn Iterator<Item = &'a str>, selection: (usize, usize))
    -> Result<(), &'static str> ;
  /// Move selection to immediately after index
  fn mov(&mut self, selection: (usize, usize), index: usize)
    -> Result<(), &'static str> ;
  /// Insert a copy of the selection immediately after index
  fn mov_copy(&mut self, selection: (usize, usize), index: usize)
    -> Result<(), &'static str> ;
  /// Join all lines in selection into one line
  fn join(&mut self, selection: (usize, usize))
    -> Result<(), &'static str> ;
  /// Copy selected lines into clipboard
  fn copy(&mut self, selection: (usize, usize))
    -> Result<(), &'static str> ;
  /// Paste the clipboard contents after given index
  /// Leave clipboard unchanged
  fn paste(&mut self, index: usize)
    -> Result<usize, &'static str> ;
  /// Perform regex search and replace on the selection changing pattern.0 to pattern.1
  /// Should interpret escape sequences in both patterns. At minimum \n and \\.
  /// If pattern is empty, should re-use stored pattern from previous s command
  /// Returns new end of selection, since it may delete or add lines
  /// Beware that it may delete the whole selection just as 'c'
  fn search_replace(&mut self, pattern: (&str, &str), selection: (usize, usize), global: bool)
    -> Result<usize, &'static str> ;

  // Save/load commands. Here to enable creative Buffers, such as ssh+sed for remote editing
  /// Read to the buffer from given path
  /// If index is None replaces current buffer with read lines
  /// Else inserts read lines after given index
  /// Return number of lines read
  fn read_from(&mut self, path: &str, index: Option<usize>, must_exist: bool)
    -> Result<usize, &'static str> ;
  /// Write the buffer to given path
  fn write_to(&mut self, selection: Option<(usize, usize)>, path: &str, append: bool)
    -> Result<(), &'static str> ;
  /// Returns true if no changes have been made since last saving
  fn saved(&self)
    -> bool ;

  // Finally, the basic output command.
  /// Return the given selection without any formatting
  fn get_selection<'a>(&'a self, selection: (usize, usize))
    -> Result<Box<dyn Iterator<Item = &'a str> + 'a>, &'static str> ;
}

// General index, line and selection validation functions
// These are good to run before using arguments to your buffer

/// Verify that the index is between 0 and buffer.len() inclusive.
///
/// That means it is valid to _append_ to the index in question,
/// but it may not be valid to read from.
pub fn verify_index(
  buffer: &impl Buffer,
  index: usize,
) -> Result<(), &'static str> {
  // Indices are valid at len.
  // Needed to be able to append to the buffer via insert operations.
  if index > buffer.len() { return Err(crate::error_consts::INDEX_TOO_BIG); }
  Ok(())
}
/// Verify that index is between 1 and buffer.len() inclusive
///
/// This guarantees that the line exists, to both write to and read from.
/// Will always error if buffer.len() == 0, since no lines exist.
pub fn verify_line(
  buffer: &impl Buffer,
  index: usize,
) -> Result<(), &'static str> {
  if index < 1 { Err(crate::error_consts::INVALID_LINENR0) }
  else if index > buffer.len() { Err(crate::error_consts::INDEX_TOO_BIG) }
  else { Ok(()) }
}

/// Verify that all lines in the selection exist and that it isn't empty.
///
/// Will always error if buffer.len() == 0, since no lines exist.
pub fn verify_selection(
  buffer: &impl Buffer,
  selection: (usize, usize),
) -> Result<(), &'static str> {
  // Line 0 doesn't exist, even though index 0 is valid
  if selection.0 == 0 { return Err(crate::error_consts::INVALID_LINENR0); }
  // A selection must contain something to be valid
  if selection.0 > selection.1 { return Err(crate::error_consts::SELECTION_EMPTY); }
  // It cannot contain non-existent lines, such as index buffer.len() and beyond
  if selection.1 > buffer.len() { return Err(crate::error_consts::INDEX_TOO_BIG); }
  Ok(())
}
