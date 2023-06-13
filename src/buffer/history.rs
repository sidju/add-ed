pub use super::*;

/// A history abstraction over the `Vec<Line>` style of buffer used by add-ed.
///
/// Handles snapshotting and undo/redo to move over the history of snapshots.
/// Currently uses a revert style of undo, more details/links on github.
///
/// Automatically manages snapshot creation upon mutable access to the current
/// point in history. Further allows pausing snapshot creation via
/// [`History.dont_snapshot`] as well as manual snapshot creation via
/// [`History.snapshot`] (for use during script/macro execution, to make each
/// snapshot correspond to a user action).
#[derive(Clone, Debug)]
pub struct History {
  snapshots: Vec<Vec<Line>>,
  viewed_i: usize,
  saved_i: Option<usize>,
  pub dont_snapshot: bool,
}
impl Default for History {
  fn default() -> Self { Self::new() }
}
impl History {
  pub fn new() -> Self {
    Self{
      snapshots: vec![vec![]],
      viewed_i: 0,
      saved_i: Some(0),
      dont_snapshot: false,
    }
  }

  /// Get if the buffer is saved
  ///
  /// It aims to be true when the viewed buffer matches the data last saved.
  pub fn saved(&self) -> bool {
    self.saved_i == Some(self.viewed_i)
  }
  /// Mark the currently viewed buffer state as saved
  ///
  /// If [`Self.dont_snapshot`] is set this instead behaves as
  /// [`Self.set_unsaved`], as we cannot be sure a snapshot will exist
  /// corresponding to the state in which the buffer was saved.
  pub fn set_saved(&mut self) {
    if !self.dont_snapshot {
      self.saved_i = Some(self.viewed_i);
    } else {
      self.saved_i = None;
    }
  }
  /// Declare that no known buffer state is saved
  ///
  /// Mainly useful for testing, but may be relevant when knowing that file
  /// was changed on disk.
  pub fn set_unsaved(&mut self) {
    self.saved_i = None;
  }

  /// Get an immutable view into the currently viewed point in history
  pub(super) fn current(&self) -> &Vec<Line> {
    &self.snapshots[self.viewed_i]
  }
  /// Get a mutable state to make new changes
  ///
  /// - If currently viewing history, will create a revert snapshot at end of
  /// history.
  /// - Unless self.dont_snapshot, will create a new snapshot.
  /// - Returns mutable access to the snapshot at the end of history.
  pub(super) fn current_mut(&mut self) -> Result<&mut Vec<Line>> {
    self.snapshot()?;
    Ok(&mut self.snapshots[self.viewed_i])
  }

  /// Move the given number of steps back into history
  pub fn undo(&mut self,
    steps: isize,
  ) -> Result<()> {
    let range = self.undo_range()?;
    if !range.contains(&steps) {
      Err(EdError::UndoStepsInvalid{undo_steps: steps, undo_range: range})?
    }
    if steps.is_negative() {
      self.viewed_i += (-steps) as usize;
    } else {
      self.viewed_i -= steps as usize;
    }
    Ok(())
  }
  /// Returns how far you can undo/redo
  pub fn undo_range(&self) -> Result<std::ops::Range<isize>> {
    // Negative is redo potential, steps between end of history and buffer_i
    // Positive is undo potential, buffer_i
    if self.snapshots.len() < isize::MAX as usize && self.viewed_i < self.snapshots.len() {
      Ok(self.viewed_i as isize - self.snapshots.len() as isize +1 .. self.viewed_i as isize + 1)
    } else {
      // When we have too much undo history to handle via the api
      Err(InternalError::UndoHistoryTooLarge.into())
    }
  }

  fn internal_create_snapshot(&mut self) {
    // Push the current index to end of history
    // (reverts if in history, snapshots if at end of history)
    self.snapshots.push(self.snapshots[self.viewed_i].clone());
    // Move to end of history
    self.viewed_i = self.snapshots.len() - 1;
  }
  /// Manually add a snapshot
  ///
  /// The only case this should be needed is before setting `dont_snapshot` for
  /// a script execution. If `dont_snapshot` isn't set snapshots are created
  /// automatically whenever [`Self.current_mut`] is executed.
  pub fn snapshot(&mut self) -> Result<()> {
    // Verify that we don't overflow history
    // Add two, since two snapshots are created for revert + snapshot
    if self.snapshots.len() + 2 > isize::MAX as usize {
      Err(InternalError::UndoHistoryTooLarge)?;
    }
    // If we are in the past, create a revert snapshot
    // This is needed even if snapshots are disabled, to not change history
    if self.viewed_i < self.snapshots.len() - 1 {
      self.internal_create_snapshot();
    }
    // If snapshots aren't disabled, create one
    if !self.dont_snapshot {
      self.internal_create_snapshot();
    }
    // Create snapshot to work on and move into future
    Ok(())
  }

  /// Checks if the last two snapshots in history are identical. If yes deletes
  /// one of them.
  ///
  /// Intended for use by macros and scripts, as they have to add a snapshot
  /// even for non-mutating scripts since they don't know if a script will
  /// modify the buffer. By running this after macro execution the snapshot will
  /// be deleted if extraneous and left if relevant.
  pub fn dedup_present(&mut self) {
    let mut last_2_iter = self.snapshots.iter().rev().take(2);
    if last_2_iter.next() == last_2_iter.next() {
      self.snapshots.pop();
      self.viewed_i = self.snapshots.len() - 1;
    }
  }
}
