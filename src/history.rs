//! Module for history snapshotting and management.

use crate::{EdError, Result};
use std::fmt::Debug;

/// A special type of Clone for [`History`]
///
/// Needed because [`History`] requires a Clone that re-uses as much memory as
/// possible, while normal Clone is usually expected to create unrelated copies
/// of data.
pub trait Snapshot{
  /// Create a memory efficient copy of self
  ///
  /// Beware, mutation of the created copy (if even possible) may modify the
  /// original.
  fn create_snapshot(&self) -> Self;
}

/// A history abstraction over generic objects used by add-ed.
///
/// Handles snapshotting and moving over the history of snapshots. Currently
/// uses a revert style of undo inspired by
/// [this reasoning.](https://github.com/zaboople/klonk/blob/master/TheGURQ.md)
///
/// Automatically manages snapshot creation upon mutable access to the current
/// point in history. Further allows pausing snapshot creation via
/// [`History.dont_snapshot`] as well as manual snapshot creation via
/// [`History.snapshot`] (for use during script/macro execution, to make each
/// snapshot correspond to a user action).
#[derive(Clone, Debug)]
pub struct History<T> where
  T: Default + Debug + Snapshot + PartialEq,
{
  snapshots: Vec<(String, T)>,
  viewed_i: usize,
  saved_i: Option<usize>,
  /// If true all calls to [`History::snapshot`] are ignored (including the
  /// automatic call upon running `.current_mut()`).
  ///
  /// Intended for macro execution, when it would be confusing to create
  /// multiple snapshots for what the user sees as a single action.
  ///
  /// (If a point in history is viewed a snapshot reverting to that point in
  /// history will be created before mutable access no matter if this variable
  /// is set to true.)
  pub dont_snapshot: bool,
}
impl<T> Default for History<T> where
  T: Default + Debug + Snapshot + PartialEq,
{
  fn default() -> Self { Self::new() }
}
impl <T> History<T> where
  T: Default + Debug + Snapshot + PartialEq,
{
  /// Create new [`History`] instance
  ///
  /// - Only an empty present state exists.
  /// - Considered saved at initial empty state.
  pub fn new() -> Self {
    Self{
      snapshots: vec![("Before reading in a file (empty)".to_owned(), T::default())],
      viewed_i: 0,
      saved_i: Some(0),
      dont_snapshot: false,
    }
  }

  /// Get if the buffer is saved
  ///
  /// It aims to be true when the viewed buffer matches the data last saved.
  /// If it is uncertain or difficult to track it will return false.
  pub fn saved(&self) -> bool {
    self.saved_i == Some(self.viewed_i)
  }
  /// Mark the currently viewed buffer state as saved
  ///
  /// If `dont_snapshot` is set this instead behaves as
  /// `set_unsaved()`, as we cannot be sure a snapshot will exist
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
  pub fn current(&self) -> &T {
    &self.snapshots[self.viewed_i].1
  }
  /// Get a mutable state to make new changes
  ///
  /// - Takes a string describing what is causing this new snapshot. (Should
  ///   generally be the full command, if not be as clear as possible.)
  /// - If currently viewing history, will create a revert snapshot at end of
  ///   history.
  /// - Unless self.dont_snapshot, will create a new snapshot tagged with the
  ///   given cause for modification.
  /// - Returns mutable access to the snapshot at the end of history.
  pub fn current_mut(&mut self,
    modification_cause: String,
  ) -> &mut T {
    self.snapshot(modification_cause);
    &mut self.snapshots[self.viewed_i].1
  }

  fn internal_create_snapshot(&mut self, label: String) {
    // Push the current index to end of history with label
    // (reverts if in history, snapshots if at end of history)
    self.snapshots.push((label, self.snapshots[self.viewed_i].1.create_snapshot()));
    // Move to end of history
    self.viewed_i = self.snapshots.len() - 1;
  }
  /// Manually add a snapshot
  ///
  /// Takes a String as an argument that should describe what causes the
  /// change seen in the created snapshot relative to the preceding snapshot.
  ///
  /// The only case this should be needed is before setting `dont_snapshot` for
  /// a script execution. If `dont_snapshot` isn't set snapshots are created
  /// automatically whenever [`Self.current_mut`] is executed.
  pub fn snapshot(&mut self,
    modification_cause: String,
  ) {
    // If we are in the past, create a revert snapshot
    // This is needed even if snapshots are disabled, to not change history
    if self.viewed_i < self.snapshots.len() - 1 {
      self.internal_create_snapshot(format!(
        "Reverted the {} actions undone to as one operation.",
        self.snapshots.len().saturating_sub(self.viewed_i + 1),
      ));
    }
    // If snapshots aren't disabled, create one
    if !self.dont_snapshot {
      self.internal_create_snapshot(modification_cause);
    }
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
    if last_2_iter.next().map(|x| &x.1) == last_2_iter.next().map(|x| &x.1) {
      self.snapshots.pop();
      self.viewed_i = self.snapshots.len() - 1;
    }
  }

  /// Accessor to view the full list of snapshots
  ///
  /// - Entries are in order of creation, the first operation is first in the
  /// list.
  /// - The string beside the snapshot describes what caused the state in the
  ///   snapshot (relative to the preceeding snapshot).
  pub fn snapshots(&self) -> &Vec<(String, T)> {
    &self.snapshots
  }
  /// Shorthand for `.snapshots().len()`
  pub fn len(&self) -> usize {
    self.snapshots.len()
  }
  /// Getter for what index was last saved
  ///
  /// Returns None if no index is believed to be saved.
  ///
  /// Intended to be used to enrich when listing snapshots by marking the one
  /// considered saved.
  pub fn saved_i(&self) -> Option<usize> {
    self.saved_i
  }
  /// Getter for currently viewed snapshot index
  pub fn viewed_i(&self) -> usize {
    self.viewed_i
  }
  /// Setter for currently viewed snapshot index
  ///
  /// Returns the modification cause for the now viewed index.
  ///
  /// Will return error if given index doesn't hold a snapshot (aka. is too
  /// big).
  pub fn set_viewed_i(&mut self, new_i: usize) -> Result<&str> {
    if new_i < self.len() {
      self.viewed_i = new_i;
      Ok(&self.snapshots[self.viewed_i].0)
    }
    else {
      Err(EdError::UndoIndexTooBig{
        index: new_i,
        history_len: self.len(),
        relative_redo_limit: self.len() - self.viewed_i - 1,
      })
    }
  }
}
