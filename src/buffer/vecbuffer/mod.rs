//! Holds the VecBuffer, a simple Vector based buffer implementation.

use core::iter::Iterator;
use std::rc::Rc;

use super::*;
use crate::error_consts::*;

#[cfg(test)]
mod test {
  // Tests of the trickier cases
  use super::*;
  
  // Use general api test to test most functions
  #[test]
  fn api_validation() {
    let mut buf = VecBuffer::new();
    super::super::test::api_validation(&mut buf);
  }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Line {
  tag: char,
  matched: bool,
  text: Rc<String>,
}

/// VecBuffer, the default Buffer implementation
///
/// It is based on storing the text in a Vector of lines.
/// Regex functionality is imported from the Regex crate.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct VecBuffer {
  saved: bool,
  // Chars used for tagging. No tag equates to NULL in the char
  history: Vec<Vec<Line>>,
  buffer_i: usize, // The index in history currently seen by user
  clipboard: Vec<Line>,
}
impl Default for VecBuffer {
  fn default() -> Self { Self::new() }
}
impl VecBuffer {
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
}
impl Buffer for VecBuffer {
  // Index operations, get and verify
  fn len(&self) -> usize { self.history[self.buffer_i].len() }
  fn is_empty(&self) -> bool { self.history[self.buffer_i].is_empty() }
  fn get_tag(&self, tag: char)
    -> Result<usize, &'static str>
  {
    for (index, line) in self.history[self.buffer_i][..].iter().enumerate() {
      if tag == line.tag { return Ok(index + 1); } // Add one for 1-indexing
    }
    Err(NO_MATCH)
  }
  fn get_matching(&self, pattern: &str, curr_line: usize, backwards: bool)
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
  fn mark_matching(&mut self, pattern: &str, selection: (usize, usize), inverse: bool)
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
  fn get_marked(&mut self)
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
  fn tag_line(&mut self, index: usize, tag: char)
    -> Result<(), &'static str>
  {
    verify_line(self, index)?;
    // Overwrite current char with given char
    self.history[self.buffer_i][index.saturating_sub(1)].tag = tag;
    Ok(())
  }
  // Take an iterator over &str as data
  fn insert<'a>(&mut self, data: &mut dyn Iterator<Item = &'a str>, index: usize)
    -> Result<(), &'static str>
  {
    // Possible TODO: preallocate for the insert
    verify_index(self, index)?;
    self.saved = false;
    // To minimise time complexity we split the vector immediately
    let mut tail = self.history[self.buffer_i].split_off(index);
    // Then append the insert data
    for line in data {
      self.history[self.buffer_i].push(
        Line{tag: '\0', matched: false, text: line.to_owned().into()}
      );
    }
    // And finally the cut off tail
    self.history[self.buffer_i].append(&mut tail);
    Ok(())
  }
  fn cut(&mut self, selection: (usize, usize)) -> Result<(), &'static str>
  {
    verify_selection(self, selection)?;
    self.saved = false;
    let mut tail = self.history[self.buffer_i].split_off(selection.1);
    self.clipboard = self.history[self.buffer_i].split_off(selection.0.saturating_sub(1));
    self.history[self.buffer_i].append(&mut tail);
    Ok(())
  }
  fn change<'a>(&mut self, data: &mut dyn Iterator<Item = &'a str>, selection: (usize, usize))
    -> Result<(), &'static str>
  {
    verify_selection(self, selection)?;
    self.saved = false;
    let mut tail = self.history[self.buffer_i].split_off(selection.1);
    self.clipboard = self.history[self.buffer_i].split_off(selection.0.saturating_sub(1));
    for line in data {
      self.history[self.buffer_i].push(
        Line{tag: '\0', matched: false, text: line.to_owned().into()}
      );
    }
    self.history[self.buffer_i].append(&mut tail);
    Ok(())
  }
  fn mov(&mut self, selection: (usize, usize), index: usize) -> Result<(), &'static str> {
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
  fn mov_copy(&mut self, selection: (usize, usize), index: usize) -> Result<(), &'static str> {
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
  fn join(&mut self, selection: (usize, usize)) -> Result<(), &'static str> {
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
  fn reflow(&mut self,
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
  fn copy(&mut self, selection: (usize, usize)) -> Result<(), &'static str> {
    verify_selection(self, selection)?;
    self.clipboard = Vec::new();
    // copy out each line in selection
    for line in &self.history[self.buffer_i][selection.0.saturating_sub(1) .. selection.1] {
      self.clipboard.push(line.clone());
    }
    Ok(())
  }
  fn paste(&mut self, index: usize) -> Result<usize, &'static str> {
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
  fn search_replace(&mut self, pattern: (&str, &str), selection: (usize, usize), global: bool) -> Result<usize, &'static str>
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
    let replace = super::substitute::substitute(pattern.1);
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

  // File operations
  fn read_from(&mut self, path: &str, index: Option<usize>, must_exist: bool)
    -> Result<usize, &'static str>
  {
    if let Some(i) = index { verify_index(self, i)?; }
    let data = file::read_file(path, must_exist)?;
    let len = data.len();
    let mut iter = data.iter().map(| string | &string[..]);
    let mut consider_saved = false; // If opening new file it is saved until changes are made
    let i = match index {
      Some(i) => i,
      // Since .change is not safe on an empty selection and we actually just wish to delete everything
      None => {
        self.history = vec![vec![]];
        consider_saved = true;
        0
      },
    };
    self.insert(&mut iter, i)?;
    self.saved = consider_saved;
    Ok(len)
  }
  fn write_to(&mut self, selection: Option<(usize, usize)>, path: &str, append: bool)
    -> Result<(), &'static str>
  {
    let data = match selection {
      Some(sel) => self.get_selection(sel)?,
      None => Box::new(self.history[self.buffer_i][..].iter().map(
        |line| (line.tag, &line.text[..])
      )),
    };
    file::write_file(path, data.map(|(_,x)| x), append)?;
    if selection == Some((1, self.len())) || selection.is_none() {
      self.saved = true;
    }
    Ok(())
  }
  fn saved(&self) -> bool {
    self.saved
  }

  fn undo(&mut self, steps: isize)
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

  fn undo_range(&self)
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

  fn snapshot(&mut self) -> Result<(), &'static str> {
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
  fn get_selection<'a>(&'a self, selection: (usize, usize))
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
