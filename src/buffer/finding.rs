use super::*;

impl Buffer {
  /// `k` command
  ///
  /// Set the given index's tag to given tag
  /// Use get_tag to get a line with that tag (lower indices prioritised)
  pub fn tag_line(&mut self, index: usize, tag: char)
    -> Result<(), &'static str>
  {
    verify_line(self, index)?;
    let buffer = self.history.current();
    // Overwrite current char with given char
    *buffer[index.saturating_sub(1)].tag.borrow_mut() = tag;
    Ok(())
  }
  /// `'` index resolver
  ///
  /// Prioritises lower indices
  pub fn get_tag(&self, tag: char)
    -> Result<usize, &'static str>
  {
    let buffer = self.history.current();
    for (index, line) in buffer[..].iter().enumerate() {
      if tag == *line.tag.borrow() { return Ok(index + 1); } // Add one for 1-indexing
    }
    Err(NO_MATCH)
  }

  /// `/` and `?` index resolvers
  ///
  /// Get nearest line matching regex pattern, forward or backward
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
    let buffer = self.history.current();
    // Figure out how far to iterate
    let length = if ! backwards {
      buffer.len().saturating_sub(curr_line)
    } else {
      curr_line.saturating_sub(1)
    };

    // Since the range must be positive we subtract from bufferlen for backwards
    for index in 0 .. length {
      if backwards {
        // 1-indexed to 0-indexed conversion (-1) stacks with -1 to skip current line
        if regex.is_match(&(buffer[curr_line - 2 - index].text)) {
          return Ok(curr_line - 1 - index) // only -1 since we return 1-indexed
        }
      } else {
        // 1-indexed to 0-indexed conversion (-1) negate +1 to skip current line
        if regex.is_match(&(buffer[curr_line + index].text)) {
          return Ok(curr_line + 1 + index) // +1 since we return 1-indexed
        }
      }
    }
    Err(NO_MATCH)
  }

  /// `g`, `G`, `v` and `V` commands
  ///
  ///Mark all lines in selection matching pattern (or its inverse)
  ///
  /// Call [`get_marked`] below repeatedly to get the matching indices.
  ///
  /// Note, unless you set [`History.dont_snapshot`] this will create a
  /// snapshot.
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
    let buffer = self.history.current();
    let mut match_found = false;
    for (index,item) in buffer.iter().enumerate() {
      if index >= selection.0.saturating_sub(1) && index <= selection.1.saturating_sub(1) {
        if regex.is_match(&(item.text)) ^ inverse {
          match_found = true;
          *item.matched.borrow_mut() = true;
        }
      }
      else {
        *item.matched.borrow_mut() = false;
      }
    }
    if !match_found {
      Err(NO_MATCH)
    } else {
      Ok(())
    }
  }
  /// `g`, `G`, `v` and `V` commands
  ///
  /// Returns the lowest index which is marked and unmarks it.
  ///
  /// Note, unless you set [`History.dont_snapshot`] this will create one
  /// snapshot per call.
  pub fn get_marked(&mut self)
    -> Result<Option<usize>, &'static str>
  {
    let buffer = self.history.current();
    for (index,item) in buffer.iter().enumerate() {
      let mut matched = item.matched.borrow_mut();
      if *matched {
        *matched = false;
        return Ok(Some(index + 1));
      }
    }
    Ok(None)
  }
}
