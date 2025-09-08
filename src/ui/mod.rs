//! Defines UI trait and some testing implementations
//!
//! It is used to allow exchanging the UI of hired and to insert a dummy UI for
//! script input.

use crate::{
  Ed,
  Result,
};

mod lock;
pub use lock::UILock;

mod scripted_ui;
pub use scripted_ui::ScriptedUI;

pub mod mock_ui;
pub mod dummy_ui;

/// The UI trait used to abstract all common UI operations
pub trait UI {
  /// A basic print for errors and other information messages
  ///
  /// Note that this must be able to print more than one message during the
  /// execution of a command, since some commands may call it several times.
  fn print_message(&mut self,
    data: &str,
  ) -> Result<()>;

  /// Print a listing of the commands with short descriptions
  ///
  /// Default implementation uses `self.print_message()` to print the const
  /// string exported at `add_ed::messages::COMMANDS_LISTING`.
  fn print_commands(&mut self) -> Result<()> {
    self.print_message(crate::messages::COMMAND_LIST)
  }

  /// Print commands documentation
  ///
  /// Print usage documentation for the commands. You can use the std::concat
  /// macro to add your own documentation to the commands documentation string
  /// at `add_ed::messages::COMMANDS_DOCUMENTATION`.
  fn print_command_documentation(&mut self) -> Result<()>;

  /// Get a command for parsing and execution
  ///
  /// * Ed passed in for interactive viewing and status printouts. Ignore if unused.
  /// * Prefix is printed at start of the line if given. Ignore if unsuited for your UI.
  /// * Must return a single line to be parsed, trimming optional
  fn get_command(&mut self,
    ed: &Ed,
    prefix: Option<char>,
  ) -> Result<String>;

  /// Get input lines until given character is entered alone on a line
  ///
  /// * Ed passed in for interactive viewing and status printouts. Ignore if unused.
  /// * Must return a vector newline terminated strings and not return the terminating line
  fn get_input(&mut self,
    ed: &Ed,
    terminator: char,
    #[cfg(feature = "initial_input_data")]
    initial_buffer: Option<Vec<String>>,
  ) -> Result<Vec<String>>;

  /// Print the given selection with the given options
  ///
  /// Depending on UI this may mean changing viewport settings and moving to given selection.
  /// * Ed passed in for path based highlighting and status printouts. Ignore if unused.
  /// * Separate selection passed in since the selection to print isn't saved to state
  ///   until after printing.
  fn print_selection(&mut self,
    ed: &Ed,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<()>;

  /// Prepare UI before handing down stdin/out/err to a child process
  ///
  /// The returned UIHandle should hold a mutable reference to its parent UI.
  /// Using that reference the UIHandle calls unlock_ui() when being dropped.
  /// * title name for lock reason handed in for clarifying print (such as
  ///   "returned from {}")
  fn lock_ui(&mut self,
    child_title: String,
  ) -> UILock<'_>;

  /// Resume UI after lock_ui has been called
  ///
  /// This method shouldn't be called except by UIHandle's Drop implementation.
  fn unlock_ui(&mut self);
}
