use super::*;

impl Buffer {
  /// `I` command
  pub fn inline_insert<S: Into<String>>(&mut self,
    mut data: Vec<S>,
    index: usize,
  ) -> Result<()> {
    verify_line(self, index)?;
    let buffer = self.history.current_mut()?;
    let mut tail = buffer.split_off(index);
    let indexed_line = buffer.split_off(index - 1);
    let last_data_line = data.pop().map(|s| s.into());
    for line in data.drain(..) {
      buffer.push(
        Line{tag: '\0'.into(), matched: false.into(), text: Rc::new(line.into())}
      );
    }
    // The inline part means we join the indexed line with the last data-line
    let mut joined_text = last_data_line.unwrap_or(String::new());
    joined_text.pop(); // Get rid of newline
    joined_text.push_str(&indexed_line[0].text[..]);
    buffer.push(
      Line{tag: '\0'.into(), matched: false.into(), text: Rc::new(joined_text)}
    );
    // As indexed line was changed, we save the old state in clipboard
    self.clipboard = indexed_line;
    // And we put back the tail afterwards
    buffer.append(&mut tail);
    Ok(())
  }
  /// `A` command
  pub fn inline_append<S: Into<String>>(&mut self,
    mut data: Vec<S>,
    index: usize,
  ) -> Result<()> {
    verify_line(self, index)?;
    let buffer = self.history.current_mut()?;
    let mut tail = buffer.split_off(index);
    let indexed_line = buffer.split_off(index - 1);
    let mut data_iter = data.drain(..);
    // Since inline means we join indexed line with first data line
    let mut joined_text = String::from(&indexed_line[0].text[..]);
    if let Some(txt) = data_iter.next() {
      joined_text.pop(); // Remove newline
      joined_text.push_str(txt.into().as_str());
      buffer.push(
        Line{tag: '\0'.into(), matched: false.into(), text: Rc::new(joined_text)}
      );
    }
    for line in data_iter {
      buffer.push(
        Line{tag: '\0'.into(), matched: false.into(), text: Rc::new(line.into())}
      );
    }
    // As indexed line was changed, we save the old state in clipboard
    self.clipboard = indexed_line;
    // And we put back the tail afterwards
    buffer.append(&mut tail);
    Ok(())
  }
  /// `i` command
  pub fn insert<S: Into<String>>(&mut self,
    mut data: Vec<S>,
    index: usize,
  ) -> Result<()> {
    verify_index(self, index)?;
    let buffer = self.history.current_mut()?;
    // To minimise time complexity we split the vector immediately
    let mut tail = buffer.split_off(index);
    // Then append the insert data
    for line in data.drain(..) {
      buffer.push(
        Line{tag: '\0'.into(), matched: false.into(), text: Rc::new(line.into())}
      );
    }
    // And finally the cut off tail
    buffer.append(&mut tail);
    Ok(())
  }
  /// `d` command
  pub fn cut(&mut self,
    selection: (usize, usize),
  ) -> Result<()> {
    verify_selection(self, selection)?;
    let buffer = self.history.current_mut()?;
    let mut tail = buffer.split_off(selection.1);
    self.clipboard = buffer.split_off(selection.0.saturating_sub(1));
    buffer.append(&mut tail);
    Ok(())
  }
  /// `c` and `C` commands
  pub fn change<S: Into<String>>(&mut self,
    mut data: Vec<S>,
    selection: (usize, usize),
  ) -> Result<()> {
    verify_selection(self, selection)?;
    let buffer = self.history.current_mut()?;
    let mut tail = buffer.split_off(selection.1);
    self.clipboard = buffer.split_off(selection.0.saturating_sub(1));
    for line in data.drain(..) {
      buffer.push(
        Line{tag: '\0'.into(), matched: false.into(), text: Rc::new(line.into())}
      );
    }
    buffer.append(&mut tail);
    Ok(())
  }
  /// `e` command (the file interaction is in [`crate::IO`])
  // One could argue to put the old file state in clipboard, but that would
  // probably break the law of least surprise.
  pub fn replace_buffer<S: Into<String>>(&mut self,
    mut data: Vec<S>,
  ) -> Result<()> {
    let buffer = self.history.current_mut()?;
    buffer.clear();
    for line in data.drain(..) {
      buffer.push(
        Line{tag: '\0'.into(), matched: false.into(), text: Rc::new(line.into())}
      );
    }
    Ok(())
  }
  /// The `m` command
  pub fn mov(&mut self,
    selection: (usize, usize),
    index: usize,
  ) -> Result<()> {
    verify_selection(self, selection)?;
    verify_index(self, index)?;
    let buffer = self.history.current_mut()?;
    // Operation varies depending on moving forward or back
    if index < selection.0 {
      // split out the relevant parts of the buffer
      let mut tail = buffer.split_off(selection.1);
      let mut data = buffer.split_off(selection.0.saturating_sub(1));
      let mut middle = buffer.split_off(index);
      // Reassemble
      buffer.append(&mut data);
      buffer.append(&mut middle);
      buffer.append(&mut tail);
      Ok(())
    }
    else if index >= selection.1 {
      // split out the relevant parts of the buffer
      let mut tail = buffer.split_off(index);
      let mut middle = buffer.split_off(selection.1);
      let mut data = buffer.split_off(selection.0.saturating_sub(1));
      // Reassemble
      buffer.append(&mut middle);
      buffer.append(&mut data);
      buffer.append(&mut tail);
      Ok(())
    }
    else {
      Err(EdError::NoOp).into() // moving into self is not moving
    }
  }
  /// `t` command
  ///
  /// Note that its implementation also duplicates any tags on the copied lines
  pub fn mov_copy(&mut self,
    selection: (usize, usize),
    index: usize,
  ) -> Result<()> {
    verify_selection(self, selection)?;
    verify_index(self, index)?;
    let buffer = self.history.current_mut()?;
    // Get the data
    let mut data = Vec::new();
    for line in &buffer[selection.0.saturating_sub(1) .. selection.1] {
      data.push(line.clone());
    }
    let mut tail = buffer.split_off(index);
    buffer.append(&mut data);
    buffer.append(&mut tail);
    Ok(())
  }
   /// `j` command
  pub fn join(&mut self,
    selection: (usize, usize),
  ) -> Result<()> {
    verify_selection(self, selection)?;
    let buffer = self.history.current_mut()?;
    // Take out all the selected lines
    let mut tail = buffer.split_off(selection.1);
    let data = buffer.split_off(selection.0.saturating_sub(1));
    // Put in the first selected line, to reserve a spot, before appending tail
    buffer.push(data[0].clone());
    buffer.append(&mut tail);
    // Construct contents of new line
    let mut text = String::from(
      &buffer[selection.0.saturating_sub(1)].text[..]
    );
    // Add the contents of data (excluding first line) to it
    for line in &data[1..] {
      text.pop(); // Get rid of newline
      text.push_str(&line.text[..]);
    }
    // Construct replacement line from this
    buffer[selection.0.saturating_sub(1)].text = Rc::new(text);
    // And finally save the selections prior state to clipboard
    self.clipboard = data;
    Ok(())
  }
  /// `J` command
  pub fn reflow(&mut self,
    selection: (usize, usize),
    width: usize,
  ) -> Result<usize> {
    verify_selection(self, selection)?;
    let buffer = self.history.current_mut()?;
    // Take out the selected lines
    let mut tail = buffer.split_off(selection.1);
    let data = buffer.split_off(selection.0.saturating_sub(1));
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
      if joined[i..].starts_with(' ') {
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
      buffer.push(
        Line{tag: '\0'.into(), matched: false.into(), text: format!("{}\n", line).into()}
      );
    }
    // Note the new end of selection to return
    let end = buffer.len();
    // Add back tail and return
    buffer.append(&mut tail);
    Ok(end)
  }
  /// `y` command
  pub fn copy(&mut self,
    selection: (usize, usize),
  ) -> Result<()> {
    verify_selection(self, selection)?;
    self.clipboard = Vec::new();
    let buffer = self.history.current();
    // copy out each line in selection
    for line in &buffer[selection.0.saturating_sub(1) .. selection.1] {
      self.clipboard.push(line.clone());
    }
    Ok(())
  }
  /// `x`/`X` command
  ///
  /// Note that it may duplicate any tags on lines in the clipboard.
  pub fn paste(&mut self,
    index: usize,
  ) -> Result<usize> {
    verify_index(self, index)?;
    let buffer = self.history.current_mut()?;
    // Cut off the tail in one go, to reduce time complexity
    let mut tmp = buffer.split_off(index);
    // Then append copies of all lines in clipboard
    for line in &self.clipboard {
      buffer.push(line.clone());
    }
    // Finally put back the tail
    buffer.append(&mut tmp);
    Ok(self.clipboard.len())
  }
  /// `s` command
  pub fn search_replace(&mut self,
    pattern: (&str, &str),
    selection: (usize, usize),
    global: bool,
  ) -> Result<usize> {
    use regex::RegexBuilder;
    // ensure that the selection is valid
    verify_selection(self, selection)?;
    // Compile the regex used to match/extract data
    let regex = RegexBuilder::new(pattern.0)
      .multi_line(true)
      .build()
      .map_err(|e|EdError::regex_error(e, pattern.0))
    ?;

    // Get view to the buffer contents and verify that there is a match
    let buffer_view = self.history.current();
    let mut joined = String::new();
    for line in &buffer_view[selection.0.saturating_sub(1) .. selection.1] {
      joined.push_str(&line.text);
    }
    if !regex.is_match(&joined) {
      // We haven't modified anything we shouldn't, so just return Err
      Err(EdError::RegexNoMatch(pattern.0.to_owned()))?
    }

    // When we know we will need to modify, get a mutable access to the buffer
    let buffer = self.history.current_mut()?;
    // Cut out the whole selection from buffer
    let mut tail = buffer.split_off(selection.1);
    let before = buffer
      .split_off(selection.0.saturating_sub(1))
    ;

    // Since we'll modify selection we save its prior state in clipboard
    self.clipboard = before;

    // Interpret escape sequences in the replace pattern
    let replace = substitute::substitute(pattern.1);
    // Run the search-replace over it
    // (We use joined string from view, since data cannot have changed)
    let after = if global {
      regex.replace_all(&joined, replace).to_string()
    }
    else {
      regex.replace(&joined, replace).to_string()
    };
    // Split on newlines and add all lines to the buffer
    // lines iterator doesn't care if the last newline is there or not
    for line in after.lines() {
      buffer.push(
        Line{tag: '\0'.into(), matched: false.into(), text: format!("{}\n", line).into()}
      );
    }
    // Get the end of the affected area from current bufferlen
    let end = buffer.len();
    // Then put the tail back
    buffer.append(&mut tail);
    Ok(end)
  }
}
