//! This module defines the UI trait
//!
//! It is used to allow exchanging the UI of hired and to insert a dummy UI for script input.

use super::EdState;

mod lock;
pub use lock::UILock;

mod scripted_ui;
pub use scripted_ui::ScriptedUI;

#[cfg(any(feature = "testing", fuzzing, test))]
pub mod mock_ui;
#[cfg(any(feature = "testing", fuzzing, test))]
pub mod dummy_ui;

/// The UI trait used to abstract all common UI operations
pub trait UI {
  /// A basic print for output of commands
  fn print_message(&mut self,
    data: &str,
  ) -> Result<(), &'static str>;

  /// Get a command for parsing and execution
  ///
  /// * EdState passed in for interactive viewing and status printouts. Ignore if unused.
  /// * Prefix is printed at start of the line if given. Ignore if unsuited for your UI.
  /// * Must return a single line to be parsed, trimming optional
  fn get_command(&mut self,
    ed: EdState,
    prefix: Option<char>,
  ) -> Result<String, &'static str>;

  /// Get input lines until given character is entered alone on a line
  ///
  /// * EdState passed in for interactive viewing and status printouts. Ignore if unused.
  /// * Must return a vector newline terminated strings and not return the terminating line
  fn get_input(&mut self,
    ed: EdState,
    terminator: char,
    #[cfg(feature = "initial_input_data")]
    initial_buffer: Option<Vec<String>>,
  ) -> Result<Vec<String>, &'static str>;

  /// Print the given selection with the given options
  ///
  /// Depending on UI this may mean changing viewport settings and moving to given selection.
  /// * EdState passed in for path based highlighting and status printouts. Ignore if unused.
  /// * Separate selection passed in since the selection to print isn't saved to state
  ///   until after printing.
  fn print_selection(&mut self,
    ed: EdState,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<(), &'static str>;

  /// Prepare UI before handing down stdin/out/err to child process
  ///
  /// The returned UIHandle should hold a mutable reference to its parent UI.
  /// Using that reference the UIHandle calls unlock_ui() when being dropped.
  fn lock_ui(&mut self) -> UILock<'_>;

  /// Resume UI after lock_ui has been called
  ///
  /// This method shouldn't be called except by UIHandle's Drop implementation.
  fn unlock_ui(&mut self);
}
