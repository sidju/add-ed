use core::iter::Iterator;

use super::Buffer;
use super::file;
use crate::error_consts::*;

//#[test]
//mod test;

pub struct VecBuffer {
  saved: bool,
  // Chars used for tagging. No tag equates to NULL in the char
  buffer: Vec<(char, String)>,
  clipboard: Vec<(char, String)>,
}
impl VecBuffer {
  pub fn new() -> Self
  {
    Self{
      saved: true,
      buffer: Vec::new(),
      clipboard: Vec::new(),
    }
  }
}
impl Buffer for VecBuffer {

  // Index operations, get and verify
  fn len(&self) -> usize {
      self.buffer.len()
  }
  fn verify_index(&self, index: usize) -> Result<(), &'static str>
  {
    if index > self.buffer.len() {
      return Err(INDEX_TOO_BIG);
    }
    Ok(())
  }
  fn verify_selection(&self, selection: (usize, usize)) -> Result<(), &'static str>
  {
    if selection.0 >= selection.1 {
      return Err(SELECTION_EMPTY);
    }
    if selection.1 > self.buffer.len() {
      return Err(INDEX_TOO_BIG);
    }
    Ok(())
  }
  fn get_tag(&self, tag: char, selection: (usize, usize))
    -> Result<usize, &'static str>
  {
    let mut index = selection.0;
    for (ltag, _line) in &self.buffer[selection.0 .. selection.1] {
      if &tag == ltag { return Ok(index); }
      index += 1;
    }
    Err(NO_MATCH)
  }
  fn get_matching(&self, pattern: &str, selection: (usize, usize))
    -> Result<usize, &'static str>
  {
    use regex::RegexBuilder;
    self.verify_selection(selection)?;
    let regex = RegexBuilder::new(pattern)
      .multi_line(true)
      .build()
      .map_err(|_| INVALID_REGEX)
    ?;
    for index in selection.0 .. selection.1 {
      if regex.is_match(&(self.buffer[index].1)) {
        return Ok(index);
      }
    }
    Err(NO_MATCH)
  }
  fn get_all_matching(&self, pattern: &str, selection: (usize, usize))
    -> Result<Vec<usize>, &'static str>
  {
    use regex::RegexBuilder;
    self.verify_selection(selection)?;
    let regex = RegexBuilder::new(pattern)
      .multi_line(true)
      .build()
      .map_err(|_| INVALID_REGEX)
    ?;
    let mut ret = Vec::new();
    for index in selection.0 .. selection.1 {
      if regex.is_match(&(self.buffer[index].1)) {
        ret.push(index);
      }
    }
    Ok(ret)
  }

  // Simple buffer modifications:
  fn tag_line(&mut self, index: usize, tag: char)
    -> Result<(), &'static str>
  {
    // Overwrite current char with given char
    self.buffer[index].0 = tag;
    Ok(())
  }
  // Take an iterator over &str as data
  fn insert<'a>(&mut self, data: &mut dyn Iterator<Item = &'a str>, index: usize)
    -> Result<(), &'static str>
  {
    // Possible TODO: preallocate for the insert
    self.verify_index(index)?;
    self.saved = false;
    // To minimise time complexity we split the vector immediately
    let mut tail = self.buffer.split_off(index);
    // Then append the insert data
    for line in data {
      self.buffer.push(('\0', line.to_string()));
    }
    // And finally the cut off tail
    self.buffer.append(&mut tail);
    Ok(())
  }
  fn cut(&mut self, selection: (usize, usize)) -> Result<(), &'static str>
  {
    self.verify_selection(selection)?;
    self.saved = false;
    let mut tail = self.buffer.split_off(selection.1);
    self.clipboard = self.buffer.split_off(selection.0);
    self.buffer.append(&mut tail);
    Ok(())
  }
  fn change<'a>(&mut self, data: &mut dyn Iterator<Item = &'a str>, selection: (usize, usize))
    -> Result<(), &'static str>
  {
    self.verify_selection(selection)?;
    self.saved = false;
    let mut tail = self.buffer.split_off(selection.1);
    self.clipboard = self.buffer.split_off(selection.0);
    for line in data {
      self.buffer.push(('\0', line.to_string()));
    }
    self.buffer.append(&mut tail);
    Ok(())
  }
  fn mov(&mut self, selection: (usize, usize), index: usize) -> Result<(), &'static str> {
    self.verify_selection(selection)?;
    self.verify_index(index)?;
    // Operation varies depending on moving forward or back
    if index <= selection.0 {
      // split out the relevant parts of the buffer
      let mut tail = self.buffer.split_off(selection.1);
      let mut data = self.buffer.split_off(selection.0);
      let mut middle = self.buffer.split_off(index.saturating_sub(1));
      // Reassemble
      self.buffer.append(&mut data);
      self.buffer.append(&mut middle);
      self.buffer.append(&mut tail);
      Ok(())
    }
    else if index >= selection.1 {
      // split out the relevant parts of the buffer
      let mut tail = self.buffer.split_off(index);
      let mut middle = self.buffer.split_off(selection.1);
      let mut data = self.buffer.split_off(selection.0);
      // Reassemble
      self.buffer.append(&mut middle);
      self.buffer.append(&mut data);
      self.buffer.append(&mut tail);
      Ok(())
    }
    else {
      Err(MOVE_INTO_SELF)
    }
  }
  fn mov_copy(&mut self, selection: (usize, usize), index: usize) -> Result<(), &'static str> {
    self.verify_selection(selection)?;
    self.verify_index(index)?;
    // Get the data
    let mut data = Vec::new();
    for line in &self.buffer[selection.0 .. selection.1] {
      data.push(line.clone());
    }
    // Insert it, subtract one if copying to before selection
    let i = if index <= selection.0 {
      index.saturating_sub(1)
    }
    else {
      index
    };
    let mut tail = self.buffer.split_off(i);
    self.buffer.append(&mut data);
    self.buffer.append(&mut tail);
    Ok(())
  }
  fn join(&mut self, selection: (usize, usize)) -> Result<(), &'static str> {
    self.verify_selection(selection)?;
    // Take out the lines that should go away efficiently
    let mut tail = self.buffer.split_off(selection.1);
    let data = self.buffer.split_off(selection.0 + 1);
    self.buffer.append(&mut tail);
    // Add their contents to the line left in
    for line in data {
      self.buffer[selection.0].1.pop(); // Remove the existing newline
      self.buffer[selection.0].1.push_str(&line.1); // Add in the line
    }
    Ok(())
  }
  fn copy(&mut self, selection: (usize, usize)) -> Result<(), &'static str> {
    self.verify_selection(selection)?;
    self.clipboard = Vec::new();
    // copy out each line in selection
    for line in &self.buffer[selection.0 .. selection.1] {
      self.clipboard.push(line.clone());
    }
    Ok(())
  }
  fn paste(&mut self, index: usize) -> Result<(), &'static str> {
    self.verify_index(index)?;
    // Cut off the tail in one go, to reduce time complexity
    let mut tmp = self.buffer.split_off(index);
    // Then append copies of all lines in clipboard
    for line in &self.clipboard {
      self.buffer.push(line.clone());
    }
    // Finally put back the tail
    self.buffer.append(&mut tmp);
    Ok(())
  }
  fn search_replace(&mut self, pattern: (&str, &str), selection: (usize, usize), global: bool) -> Result<(usize, usize), &'static str>
  {
    use regex::RegexBuilder;
    // ensure that the selection is valid
    self.verify_selection(selection)?;
    self.saved = false; // TODO: actually check if changes are made
    // Compile the regex used to match/extract data
    let regex = RegexBuilder::new(pattern.0)
      .multi_line(true)
      .build()
      .map_err(|_| INVALID_REGEX)
    ?;

    let mut selection_after = selection;
    // Cut out the whole selection from buffer
    let mut tail = self.buffer.split_off(selection.1);
    let before = self.buffer.split_off(selection.0 + 1);
    // Save ourselves a little bit of copying/allocating
    let (_tag, mut tmp) = self.buffer.pop().unwrap();
    // Then join all selected lines together
    for line in before {
      tmp.push_str(&line.1);
    }
    // Run the search-replace over it
    let mut after = if global {
      regex.replace_all(&tmp, pattern.1).to_string()
    }
    else {
      regex.replace(&tmp, pattern.1).to_string()
    };
    // If there is no newline at the end, join next line
    if !after.ends_with('\n') {
      if tail.len() > 0 {
        after.push_str(&tail.remove(0).1);
      }
      else {
        after.push('\n');
      }
    }
    // Split on newlines and add all lines to the buffer
    for line in after.lines() {
      self.buffer.push(('\0', format!("{}\n", line)));
    }
    // Get the end of the affected area from current bufferlen
    selection_after.1 = self.buffer.len(); 
    // Then put the tail back
    self.buffer.append(&mut tail); 
    Ok(selection_after)
  }

  fn read_from(&mut self, path: &str, index: Option<usize>, must_exist: bool)
    -> Result<usize, &'static str>
  {
    let data = file::read_file(path, must_exist)?;
    let len = data.len();
    let mut iter = data.iter().map(| string | &string[..]);
    if let Some(i) = index {
      self.insert(&mut iter, i)?;
    }
    else {
      self.change(&mut iter, (0, self.len()))?;
      self.saved = true;
    }
    Ok(len)
  }
  fn write_to(&mut self, selection: (usize, usize), path: &str, append: bool)
    -> Result<(), &'static str>
  {
    let data = self.get_selection(selection)?;
    file::write_file(path, data, append)?;
    if selection == (0, self.len()) {
      self.saved = true;
    }
    Ok(())
  }
  fn saved(&self) -> bool {
    self.saved
  }

  // The output command
  fn get_selection<'a>(&'a self, selection: (usize, usize))
    -> Result<Box<dyn Iterator<Item = &'a str> + 'a>, &'static str>
  {
    self.verify_selection(selection)?;
    let tmp = self.buffer[selection.0 .. selection.1].iter().map(|(_tag, line)| &line[..]);
    Ok(Box::new(tmp))
  }
}
