// Functions to ease interaction with the buffer

use crate::{
  Buffer,
  EdError,
  Result,
};

pub (super) fn get_tag(
  buffer: &Buffer,
  tag: char,
) -> Result<usize> {
  match buffer.iter().enumerate() // Enumerate 0-indexes our iteration
    .filter(|(_, line)| line.tag.get() == tag)
    .next()
  {
    Some((i, _)) => Ok(i + 1), // Convert to 1-indexed before returning
    None => Err(EdError::TagNoMatch(tag)),
  }
}

pub(super) enum Direction {
  Forwards,
  Backwards,
}
pub(super) fn get_matching(
  buffer: &Buffer,
  pattern: &str,
  curr_line: usize,
  direction: Direction,
) -> Result<usize> {
  buffer.verify_line(curr_line)?;
  use regex::RegexBuilder;
  let regex = RegexBuilder::new(pattern)
    .multi_line(true)
    .build()
    .map_err(|e|EdError::regex_error(e,pattern))
  ?;
  let distance = match direction {
    Direction::Forwards => buffer.len() - curr_line,
    Direction::Backwards => curr_line - 1,
  };
  // Since the range must be positive/incrementing we subtract index from
  // buffer.len() to iterate backwards
  for index in 0 .. distance { match direction {
    Direction::Forwards => {
      // Conversion to 0 indexed (-1) and skipping current line (+1) negate
      if regex.is_match(&(buffer[curr_line + index].text)) {
        return Ok(curr_line + 1 + index); // +1 to return 1-indexed
      }
    },
    Direction::Backwards => {
      // Where conversion negated before it now stacks into -2
      if regex.is_match(&(buffer[curr_line - 2 - index].text)) {
        return Ok(curr_line - 1 - index); // Only -1 to return 1-indexed
      }
    },
  }}
  Err(EdError::RegexNoMatch(pattern.to_owned()))
}
