use super::*;

impl Buffer {
  pub fn inline_insert<S: Into<String>>(&mut self,
    mut data: Vec<S>,
    index: usize,
  ) -> Result<(), &'static str> {
    verify_line(self, index)?;
    self.saved = false;
    let mut tail = self.history[self.buffer_i].split_off(index);
    let indexed_line = self.history[self.buffer_i].split_off(index - 1);
    let last_data_line = data.pop().map(|s| s.into());
    for line in data.drain(..) {
      self.history[self.buffer_i].push(
        Line{tag: '\0', matched: false, text: Rc::new(line.into())}
      );
    }
    // The inline part means we join the indexed line with the last data-line
    let mut joined_text = last_data_line.unwrap_or(String::new());
    joined_text.pop(); // Get rid of newline
    joined_text.push_str(&indexed_line[0].text[..]);
    self.history[self.buffer_i].push(
      Line{tag: '\0', matched: false, text: Rc::new(joined_text)}
    );
    // As indexed line was changed, we save the old state in clipboard
    self.clipboard = indexed_line;
    // And we put back the tail afterwards
    self.history[self.buffer_i].append(&mut tail);
    Ok(())
  }
  pub fn inline_append<S: Into<String>>(&mut self,
    mut data: Vec<S>,
    index: usize,
  ) -> Result<(), &'static str> {
    verify_line(self, index)?;
    self.saved = false;
    let mut tail = self.history[self.buffer_i].split_off(index);
    let indexed_line = self.history[self.buffer_i].split_off(index - 1);
    let mut data_iter = data.drain(..);
    // Since inline means we join indexed line with first data line
    let mut joined_text = String::from(&indexed_line[0].text[..]);
    if let Some(txt) = data_iter.next() {
      joined_text.pop(); // Remove newline
      joined_text.push_str(txt.into().as_str());
      self.history[self.buffer_i].push(
        Line{tag: '\0', matched: false, text: Rc::new(joined_text)}
      );
    }
    for line in data_iter {
      self.history[self.buffer_i].push(
        Line{tag: '\0', matched: false, text: Rc::new(line.into())}
      );
    }
    // As indexed line was changed, we save the old state in clipboard
    self.clipboard = indexed_line;
    // And we put back the tail afterwards
    self.history[self.buffer_i].append(&mut tail);
    Ok(())
  }
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
  pub fn replace_buffer<'a, S: Into<String>>(&mut self,
    mut data: Vec<S>,
  ) -> Result<(), &'static str> {
    self.saved = false;
    self.history[self.buffer_i].clear();
    for line in data.drain(..) {
      self.history[self.buffer_i].push(
        Line{tag: '\0', matched: false, text: Rc::new(line.into())}
      );
    }
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
    // Take out all the selected lines
    let mut tail = self.history[self.buffer_i].split_off(selection.1);
    let data = self.history[self.buffer_i].split_off(selection.0.saturating_sub(1));
    // Put in the first selected line, to reserve a spot, before appending tail
    self.history[self.buffer_i].push(data[0].clone());
    self.history[self.buffer_i].append(&mut tail);
    // Construct contents of new line
    let mut text = String::from(
      &self.history[self.buffer_i][selection.0.saturating_sub(1)].text[..]
    );
    // Add the contents of data (excluding first line) to it
    for line in &data[1..] {
      text.pop(); // Get rid of newline
      text.push_str(&line.text[..]);
    }
    // Construct replacement line from this
    self.history[self.buffer_i][selection.0.saturating_sub(1)].text = Rc::new(text);
    // And finally save the selections prior state to clipboard
    self.clipboard = data;
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
    for line in &data {
      for ch in line.text.chars() {
        joined.push(match ch {
          '\n' => ' ',
          c => c,
        })
      }
    }
    // When we are done with original data we save it to clipboard
    self.clipboard = data;
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
}
