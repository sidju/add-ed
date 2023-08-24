use super::Buffer;
use crate::{
  Result,
  error::EdError,
};

// General index, line and selection validation functions
// These are good to run before using arguments to your buffer

/// Verify that the index is between 0 and buffer.len() inclusive.
///
/// That means it is valid to _append_ to the index in question,
/// but it may not be valid to read from.
pub fn verify_index(
  buffer: &Buffer,
  index: usize,
) -> Result<()> {
  // Indices are valid at len.
  // Needed to be able to append to the buffer via insert operations.
  if index > buffer.len() { return Err(
    EdError::IndexTooBig{index: index, buffer_len: buffer.len()}
  ); }
  Ok(())
}
/// Verify that index is between 1 and buffer.len() inclusive
///
/// This guarantees that the line exists, to both write to and read from.
/// Will always error if buffer.len() == 0, since no lines exist.
pub fn verify_line(
  buffer: &Buffer,
  index: usize,
) -> Result<()> {
  if index < 1 { Err(
    EdError::Line0Invalid
  )}
  else if index > buffer.len() { Err(
    EdError::IndexTooBig{index: index, buffer_len: buffer.len()}
  )}
  else { Ok(()) }
}

/// Verify that all lines in the selection exist and that it isn't empty.
///
/// Will always error if buffer.len() == 0, since no lines exist.
pub fn verify_selection(
  buffer: &Buffer,
  selection: (usize, usize),
) -> Result<()> {
  // Line 0 doesn't exist, even though index 0 is valid
  if selection.0 == 0 { return Err(
    EdError::Line0Invalid
  ); }
  // A selection must contain something to be valid
  if selection.0 > selection.1 { return Err(
    EdError::SelectionEmpty(selection)
  ); }
  // It cannot contain non-existent lines, such as index buffer.len() and beyond
  if selection.1 > buffer.len() { return Err(
    EdError::IndexTooBig{index: selection.1, buffer_len: buffer.len()}
  ); }
  Ok(())
}
