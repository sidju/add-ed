pub use super::*;

impl Buffer {
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
}
