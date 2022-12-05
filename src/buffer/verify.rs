use super::Buffer;

// General index, line and selection validation functions
// These are good to run before using arguments to your buffer

/// Verify that the index is between 0 and buffer.len() inclusive.
///
/// That means it is valid to _append_ to the index in question,
/// but it may not be valid to read from.
pub fn verify_index(
  buffer: &Buffer,
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
  buffer: &Buffer,
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
  buffer: &Buffer,
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
