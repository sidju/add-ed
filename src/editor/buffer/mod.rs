// Include a general file handler
// Separate to enable URL based buffers and other creative solutions
mod file;

// Include the buffer implementations
mod vecbuffer;
pub use vecbuffer::*;

/// Trait that defines a buffer supporting 'ed's base commands
pub trait Buffer {

  // Functions for resolving and verifying indices in the parser
  /// Return the number of lines stored in the buffer
  fn len(&self)
    -> usize ;
  /// Check that the index is safe to operate on
  fn verify_index(&self, index: usize)
    -> Result<(), &'static str> ;
  /// Check that the selection is safe to operate on
  fn verify_selection(&self, selection: (usize, usize))
    -> Result<(), &'static str> ;
  /// Get line tagged with given letter. Not found is error
  fn get_tag(&self, tag: char)
    -> Result<usize, &'static str> ;
  /// Return the first/last index in the selection that contains the regex pattern
  fn get_matching(&self, pattern: &str, backwards: bool)
    -> Result<usize, &'static str> ;
  /// Return the indices in the selection whose lines contain the regex pattern
  fn get_all_matching(&self, pattern: &str, selection: (usize, usize))
    -> Result<Vec<usize>, &'static str> ;

  // Simple buffer modifications, but with possibly complex storage
  /// Mark a line with a letter, non letter chars should error
  fn tag_line(&mut self, index: usize, tag: char) 
    -> Result<(), &'static str> ;
  /// Takes a iterator over lines in strings and inserts at given index
  fn insert<'a>(&mut self, data: &mut dyn Iterator<Item = &'a str>, index: usize)
    -> Result<(), &'static str> ;
  /// Cut the selection from the buffer, into the clipboard
  fn cut(&mut self, selection: (usize, usize))
    -> Result<(), &'static str> ;
  /// Equal to cut of selection and insert at start of selection.
  fn change<'a>(&mut self, data: &mut dyn Iterator<Item = &'a str>, selection: (usize, usize))
    -> Result<(), &'static str> ;
  /// Move selection to index
  fn mov(&mut self, selection: (usize, usize), index: usize)
    -> Result<(), &'static str> ;
  /// Moves a copy of the selection to index
  fn mov_copy(&mut self, selection: (usize, usize), index: usize)
    -> Result<(), &'static str> ;
  /// Join all lines in selection into one line
  fn join(&mut self, selection: (usize, usize))
    -> Result<(), &'static str> ;
  /// Copy selected lines into clipboard
  fn copy(&mut self, selection: (usize, usize))
    -> Result<(), &'static str> ;
  /// Paste the clipboard contents to given index
  /// Leave clipboard unchanged
  fn paste(&mut self, index: usize)
    -> Result<usize, &'static str> ;
  /// Perform regex search and replace on the selection changing pattern.0 to pattern.1
  /// If pattern is empty, should re-use stored pattern from previous s command
  /// Returns selection, since it may delete or add lines
  fn search_replace(&mut self, pattern: (&str, &str), selection: (usize, usize), global: bool)
    -> Result<(usize, usize), &'static str> ;

  // Save/load commands. Here to enable creative Buffers, such as ssh+sed for remote editing
  /// Read to the buffer from given path
  /// If index is None replaces current buffer with read lines
  /// Return number of lines read
  fn read_from(&mut self, path: &str, index: Option<usize>, must_exist: bool)
    -> Result<usize, &'static str> ;
  /// Write the buffer to given path
  fn write_to(&mut self, selection: (usize, usize), path: &str, append: bool)
    -> Result<(), &'static str> ;
  /// Returns true if no changes have been made since last saving
  fn saved(&self)
    -> bool ;

  // Finally, the basic output command.
  /// Return the given selection without any formatting
  fn get_selection<'a>(&'a self, selection: (usize, usize))
    -> Result<Box<dyn Iterator<Item = &'a str> + 'a>, &'static str> ;
}
