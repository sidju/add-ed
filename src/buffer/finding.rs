use super::*;

impl Buffer {
  // Set the given index's tag to given tag
  // Use get_tag to get a line with that tag (lower indices prioritised)
  pub fn tag_line(&mut self, index: usize, tag: char)
    -> Result<(), &'static str>
  {
    verify_line(self, index)?;
    // Overwrite current char with given char
    self.history[self.buffer_i][index.saturating_sub(1)].tag = tag;
    Ok(())
  }
  pub fn get_tag(&self, tag: char)
    -> Result<usize, &'static str>
  {
    for (index, line) in self.history[self.buffer_i][..].iter().enumerate() {
      if tag == line.tag { return Ok(index + 1); } // Add one for 1-indexing
    }
    Err(NO_MATCH)
  }

  // Get nearest line matching regex pattern
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

  // Mark all lines in selection matching pattern (or its inverse)
  // Then call get_marked below, which gets matching indices (ascending order)
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
    let mut match_found = false;
    for index in 0 .. self.len() {
      if index >= selection.0.saturating_sub(1) && index <= selection.1.saturating_sub(1) {
        if regex.is_match(&(self.history[self.buffer_i][index].text)) ^ inverse {
          match_found = true;
          self.history[self.buffer_i][index].matched = true;
        }
      }
      else {
        self.history[self.buffer_i][index].matched = false;
      }
    }
    if !match_found {
      Err(NO_MATCH)
    } else {
      Ok(())
    }
  }
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
}
