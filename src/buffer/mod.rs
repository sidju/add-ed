//! Contains the Buffer trait and any build in implementations.

// General implementations for file interaction and substitution of e.g. '\n'
mod substitute;
mod verify;
pub use verify::*;

// Include a general test
#[cfg(test)]
mod test;

use core::iter::Iterator;
use std::rc::Rc;

use crate::error_consts::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Line {
  tag: char,
  matched: bool,
  text: Rc<String>,
}

/// The editing Buffer built on Vec and String
///
/// It stores the entire editing history in a vector of history states.
/// Each history state is in turn a vector of lines as they were at that time.
/// And each line is a Rc<String> (newline inclusive), to avoid data copying.
/// Regex functionality is imported from the Regex crate.
///
/// BEWARE!!! 1-indexed!
/// This means _line_ 0 doesn't exist, error if given (use verify_selection/verify_line below)
/// BUT, _index_ 0 is valid (therefore use verify_index instead)
/// Subtract 1 to get 0 indexed. It is recommended to use .saturating_sub(1)
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Buffer {
  saved: bool,
  // Chars used for tagging. No tag equates to NULL in the char
  history: Vec<Vec<Line>>,
  buffer_i: usize, // The index in history currently seen by user
  clipboard: Vec<Line>,
}
impl Default for Buffer {
  fn default() -> Self { Self::new() }
}
impl Buffer {
  /// Create a new empty buffer. It is considered saved while unchanged.
  pub fn new() -> Self
  {
    Self{
      saved: true,
      history: vec![vec![]],
      buffer_i: 0,
      clipboard: Vec::new(),
    }
  }

  // Index operations, get and verify
  pub fn len(&self) -> usize { self.history[self.buffer_i].len() }
  pub fn is_empty(&self) -> bool { self.history[self.buffer_i].is_empty() }
  pub fn get_tag(&self, tag: char)
    -> Result<usize, &'static str>
  {
    for (index, line) in self.history[self.buffer_i][..].iter().enumerate() {
      if tag == line.tag { return Ok(index + 1); } // Add one for 1-indexing
    }
    Err(NO_MATCH)
  }
  pub fn get_matching(&self, pattern: &str, curr_line: usize, backwards: bool)
    -> Result<usize, &'static str>
  {
    verify_index(self, curr_line)?;
    use regex::RegexBuilder;
    let regex = RegexBuilder::new(pattern)
      .multi_line(true)
      .build()
      .map_err(|_| INVALID_REGEX)
    ?;
    // Figure out how far to iterate
    let length = if ! backwards {
      self.len().saturating_sub(curr_line)
    } else {
      curr_line.saturating_sub(1)
    };

    // Since the range must be positive we subtract from bufferlen for backwards
    for index in 0 .. length {
      if backwards {
        // 1-indexed to 0-indexed conversion (-1) stacks with -1 to skip current line
        if regex.is_match(&(self.history[self.buffer_i][curr_line - 2 - index].text)) {
          return Ok(curr_line - 1 - index) // only -1 since we return 1-indexed
        }
      } else {
        // 1-indexed to 0-indexed conversion (-1) negate +1 to skip current line
        if regex.is_match(&(self.history[self.buffer_i][curr_line + index].text)) {
          return Ok(curr_line + 1 + index) // +1 since we return 1-indexed
        }
      }
    }
    Err(NO_MATCH)
  }

  // For macro commands
  pub fn mark_matching(&mut self, pattern: &str, selection: (usize, usize), inverse: bool)
    -> Result<(), &'static str>
  {
    use regex::RegexBuilder;
    verify_selection(self, selection)?;
    let regex = RegexBuilder::new(pattern)
      .multi_line(true)
      .build()
      .map_err(|_| INVALID_REGEX)
    ?;
    for index in 0 .. self.len() {
      if index >= selection.0.saturating_sub(1) && index <= selection.1.saturating_sub(1) {
        self.history[self.buffer_i][index].matched = regex.is_match(&(self.history[self.buffer_i][index].text)) ^ inverse;
      }
      else {
        self.history[self.buffer_i][index].matched = false;
      }
    }
    Ok(())
  }
  // Horrendously inefficient getter with O(N * M) N = buffer.len(), M = nr_matches
  pub fn get_marked(&mut self)
    -> Result<Option<usize>, &'static str>
  {
    for index in 0 .. self.len() {
      if self.history[self.buffer_i][index].matched {
        self.history[self.buffer_i][index].matched = false;
        return Ok(Some(index + 1));
      }
    }
    Ok(None)
  }

  // Simple buffer modifications:
  pub fn tag_line(&mut self, index: usize, tag: char)
    -> Result<(), &'static str>
  {
    verify_line(self, index)?;
    // Overwrite current char with given char
    self.history[self.buffer_i][index.saturating_sub(1)].tag = tag;
    Ok(())
  }
  // Take an iterator over &str as data
  pub fn insert<'a, S: Into<String>>(&mut self, mut data: Vec<S>, index: usize)
    -> Result<(), &'static str>
  {
    // Possible TODO: preallocate for the insert
    verify_index(self, index)?;
    self.saved = false;
    // To minimise time complexity we split the vector immediately
    let mut tail = self.history[self.buffer_i].split_off(index);
    // Then append the insert data
    for line in data.drain(..) {
      self.history[self.buffer_i].push(
        Line{tag: '\0', matched: false, text: Rc::new(line.into())}
      );
    }
    // And finally the cut off tail
    self.history[self.buffer_i].append(&mut tail);
    Ok(())
  }
  pub fn cut(&mut self, selection: (usize, usize)) -> Result<(), &'static str>
  {
    verify_selection(self, selection)?;
    self.saved = false;
    let mut tail = self.history[self.buffer_i].split_off(selection.1);
    self.clipboard = self.history[self.buffer_i].split_off(selection.0.saturating_sub(1));
    self.history[self.buffer_i].append(&mut tail);
    Ok(())
  }
  pub fn change<'a, S: Into<String>>(&mut self, mut data: Vec<S>, selection: (usize, usize))
    -> Result<(), &'static str>
  {
    verify_selection(self, selection)?;
    self.saved = false;
    let mut tail = self.history[self.buffer_i].split_off(selection.1);
    self.clipboard = self.history[self.buffer_i].split_off(selection.0.saturating_sub(1));
    for line in data.drain(..) {
      self.history[self.buffer_i].push(
        Line{tag: '\0', matched: false, text: Rc::new(line.into())}
      );
    }
    self.history[self.buffer_i].append(&mut tail);
    Ok(())
  }
  pub fn change_no_clipboard<'a, S: Into<String>>(&mut self,
    mut data: Vec<S>,
    selection: (usize, usize)
  ) -> Result<(), &'static str> {
    verify_selection(self, selection)?;
    self.saved = false;
    let mut tail = self.history[self.buffer_i].split_off(selection.1);
    let _ = self.history[self.buffer_i].split_off(selection.0.saturating_sub(1));
    for line in data.drain(..) {
      self.history[self.buffer_i].push(
        Line{tag: '\0', matched: false, text: Rc::new(line.into())}
      );
    }
    self.history[self.buffer_i].append(&mut tail);
    Ok(())
  }
  pub fn mov(&mut self, selection: (usize, usize), index: usize) -> Result<(), &'static str> {
    verify_selection(self, selection)?;
    verify_index(self, index)?;
    self.saved = false;
    // Operation varies depending on moving forward or back
    if index < selection.0 {
      // split out the relevant parts of the buffer
      let mut tail = self.history[self.buffer_i].split_off(selection.1);
      let mut data = self.history[self.buffer_i].split_off(selection.0.saturating_sub(1));
      let mut middle = self.history[self.buffer_i].split_off(index);
      // Reassemble
      self.history[self.buffer_i].append(&mut data);
      self.history[self.buffer_i].append(&mut middle);
      self.history[self.buffer_i].append(&mut tail);
      Ok(())
    }
    else if index >= selection.1 {
      // split out the relevant parts of the buffer
      let mut tail = self.history[self.buffer_i].split_off(index);
      let mut middle = self.history[self.buffer_i].split_off(selection.1);
      let mut data = self.history[self.buffer_i].split_off(selection.0.saturating_sub(1));
      // Reassemble
      self.history[self.buffer_i].append(&mut middle);
      self.history[self.buffer_i].append(&mut data);
      self.history[self.buffer_i].append(&mut tail);
      Ok(())
    }
    else {
      Err(MOVE_INTO_SELF)
    }
  }
  pub fn mov_copy(&mut self, selection: (usize, usize), index: usize) -> Result<(), &'static str> {
    verify_selection(self, selection)?;
    verify_index(self, index)?;
    self.saved = false;
    // Get the data
    let mut data = Vec::new();
    for line in &self.history[self.buffer_i][selection.0.saturating_sub(1) .. selection.1] {
      data.push(line.clone());
    }
    let mut tail = self.history[self.buffer_i].split_off(index);
    self.history[self.buffer_i].append(&mut data);
    self.history[self.buffer_i].append(&mut tail);
    Ok(())
  }
  pub fn join(&mut self, selection: (usize, usize)) -> Result<(), &'static str> {
    verify_selection(self, selection)?;
    self.saved = false;
    // Take out the lines that should go away efficiently
    let mut tail = self.history[self.buffer_i].split_off(selection.1);
    let data = self.history[self.buffer_i].split_off(selection.0);
    self.history[self.buffer_i].append(&mut tail);
    // Construct contents of new line
    let mut text = String::from(
      &self.history[self.buffer_i][selection.0.saturating_sub(1)].text[..]
    );
    // Add the contents of data to it
    for line in data {
      text.pop(); // Get rid of newline
      text.push_str(&line.text[..]);
    }
    // Construct replacement line from this
    self.history[self.buffer_i][selection.0.saturating_sub(1)].text = Rc::new(text);
    Ok(())
  }
  pub fn reflow(&mut self,
    selection: (usize, usize),
    width: usize,
  ) -> Result<usize, &'static str> {
    verify_selection(self, selection)?;
    self.saved = false;
    // Take out the selected lines
    let mut tail = self.history[self.buffer_i].split_off(selection.1);
    let data = self.history[self.buffer_i].split_off(selection.0.saturating_sub(1));
    // First join all lines into one, replacing newlines with spaces
    let mut joined = String::new();
    for line in data {
      for ch in line.text.chars() {
        joined.push(match ch {
          '\n' => ' ',
          c => c,
        })
      }
    }
    // Remove trailing newline, which is now an unnecesary space
    joined.pop();
    // Then replace space nearest before selected width with newline
    let mut w = 0; // character width of current line
    let mut latest_space = None;
    for i in 0 .. joined.len() {
      // If not a character boundary we skip this loop
      if !joined.is_char_boundary(i) { continue; }
      w += 1;
      if &joined[i..].chars().next().unwrap() == &' ' {
        latest_space = Some(i);
      }
      if w > width {
        if let Some(s) = latest_space {
          // Split of line by replacing latest space with newline
          // Only safe because we know it is a space
          joined.replace_range(s..=s, "\n");
          w = i - s;
          latest_space = None;
        }
      }
    }
    // Finally we split into different strings on newlines and add to buffer
    for line in joined.lines() {
      self.history[self.buffer_i].push(
        Line{tag: '\0', matched: false, text: format!("{}\n", line).into()}
      );
    }
    // Note the new end of selection to return
    let end = self.history[self.buffer_i].len();
    // Add back tail and return
    self.history[self.buffer_i].append(&mut tail);
    Ok(end)
  }
  pub fn copy(&mut self, selection: (usize, usize)) -> Result<(), &'static str> {
    verify_selection(self, selection)?;
    self.clipboard = Vec::new();
    // copy out each line in selection
    for line in &self.history[self.buffer_i][selection.0.saturating_sub(1) .. selection.1] {
      self.clipboard.push(line.clone());
    }
    Ok(())
  }
  pub fn paste(&mut self, index: usize) -> Result<usize, &'static str> {
    verify_index(self, index)?;
    self.saved = false;
    // Cut off the tail in one go, to reduce time complexity
    let mut tmp = self.history[self.buffer_i].split_off(index);
    // Then append copies of all lines in clipboard
    for line in &self.clipboard {
      self.history[self.buffer_i].push(line.clone());
    }
    // Finally put back the tail
    self.history[self.buffer_i].append(&mut tmp);
    Ok(self.clipboard.len())
  }
  pub fn search_replace(&mut self, pattern: (&str, &str), selection: (usize, usize), global: bool) -> Result<usize, &'static str>
  {
    use regex::RegexBuilder;
    // ensure that the selection is valid
    verify_selection(self, selection)?;
    self.saved = false; // TODO: actually check if changes are made
    // Compile the regex used to match/extract data
    let regex = RegexBuilder::new(pattern.0)
      .multi_line(true)
      .build()
      .map_err(|_| INVALID_REGEX)
    ?;

    // Cut out the whole selection from buffer
    let mut tail = self.history[self.buffer_i].split_off(selection.1);
    let mut before = self.history[self.buffer_i]
      .split_off(selection.0.saturating_sub(1))
    ;
    let mut tmp = String::new();
    // Then join all selected lines together
    for line in &before {
      tmp.push_str(&line.text);
    }
    // Remove the trailing newline, so the whole selection cannot be deleted
    tmp.pop();

    // First check if there exists a match in the selection
    // If not we return error
    if ! regex.is_match(&tmp) {
      // Re-assemble the lines just as they were
      self.history[self.buffer_i].append(&mut before);
      self.history[self.buffer_i].append(&mut tail);
      return Err(NO_MATCH);
    }

    // Run the search-replace over it
    // Interpret escape sequences in the replace pattern
    let replace = substitute::substitute(pattern.1);
    let after = if global {
      regex.replace_all(&tmp, replace).to_string()
    }
    else {
      regex.replace(&tmp, replace).to_string()
    };
    // Put back the removed newline
    tmp.push('\n');
    // Split on newlines and add all lines to the buffer
    // lines iterator doesn't care if the last newline is there or not
    for line in after.lines() {
      self.history[self.buffer_i].push(
        Line{tag: '\0', matched: false, text: format!("{}\n", line).into()}
      );
    }
    // Get the end of the affected area from current bufferlen
    let end = self.history[self.buffer_i].len();
    // Then put the tail back
    self.history[self.buffer_i].append(&mut tail);
    Ok(end)
  }

  pub fn set_saved(&mut self) {
    self.saved = true;
  }
  pub fn saved(&self) -> bool {
    self.saved
  }

  pub fn undo(&mut self, steps: isize)
    -> Result<(), &'static str>
  {
    if !self.undo_range()?.contains(&steps) { return Err(INVALID_UNDO_STEPS); }
    self.saved = false;
    if steps.is_negative() {
      self.buffer_i += (-steps) as usize;
    } else {
      self.buffer_i -= steps as usize;
    }
    Ok(())
  }

  pub fn undo_range(&self)
    -> Result<std::ops::Range<isize>, &'static str>
  {
    // Negative is redo potential, steps between end of history and buffer_i
    // Positive is undo potential, buffer_i
    if self.history.len() < isize::MAX as usize && self.buffer_i < self.history.len() {
      Ok(self.buffer_i as isize - self.history.len() as isize +1 .. self.buffer_i as isize + 1)
    } else {
      // When we have too much undo history to handle via the api
      Err(UNDO_HISTORY_TOO_LARGE)
    }
  }

  pub fn snapshot(&mut self) -> Result<(), &'static str> {
    // We cut off history where we are
    self.history.truncate(self.buffer_i + 1);
    // Then create and move into future
    if self.history.len() < isize::MAX as usize {
      self.history.push(self.history[self.buffer_i].clone());
      self.buffer_i += 1;
      Ok(())
    } else {
      Err(UNDO_HISTORY_TOO_LARGE)
    }
  }


  // The output command
  pub fn get_selection<'a>(&'a self, selection: (usize, usize))
    -> Result<Box<dyn Iterator<Item = (char, &'a str)> + 'a>, &'static str>
  {
    verify_selection(self, selection)?;
    let tmp = self.history[self.buffer_i][selection.0 - 1 .. selection.1]
      .iter()
      .map(|line| (line.tag, &line.text[..]))
    ;
    Ok(Box::new(tmp))
  }
}
