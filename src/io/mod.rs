//! Defines IO Trait, LocalIO (if enabled) and some testing implementations.
//!
//! Used to abstract filesystem and shell interactions.

type Result<T> = core::result::Result<T, crate::error::IOError>;

use crate::UILock;
use crate::LinesIter;

pub mod fake_io;
pub mod dummy_io;

#[cfg(feature = "local_io")]
pub mod local_io;
#[cfg(feature = "local_io")]
pub use local_io::LocalIO;

/// Trait that abstracts file interactions and running shell commands
///
/// Intended to allow modifying how and where system interactions occur.
/// Example cases for replacing this:
/// - Dummy IO to prevent filesystem modifications while testing.
/// - SSH forwarding to save to remote system and run commands remotely.
/// - Restricted IO to forbid command running and restrict file paths.
pub trait IO {
  /// Run a lone command (unrelated from the buffer)
  ///
  /// Stdin, Stdout and Stderr passed through to UI
  fn run_command(&mut self,
    // UI handle. Created by setting up the UI for passing through
    // std-in/-out/-err to the child process.
    ui: &mut UILock,
    // Command string from user (with basic substitutions interpreted)
    command: String,
  ) -> Result<()>;

  /// Run a read command, collecting stdout to add into buffer
  ///
  /// Stdin and Stderr should be passed through to UI
  ///
  /// The returned string will be split into lines and added into the buffer.
  /// All line endings are converted into '\n' when adding into the buffer.
  fn run_read_command(&mut self,
    // UI handle. Created by setting up the UI for passing through
    // std-in/-err to child process.
    ui: &mut UILock,
    // Command string from user (with basic substitutions interpreted)
    command: String,
  ) -> Result<String>;

  /// Run a write command, receiving part of buffer via stdin
  ///
  /// Stdout and Stderr should be passed through to UI
  /// Returns number of bytes written
  ///
  /// The LinesIter contains string slices over '\n' terminated lines. If you
  /// with to use "\r\n" line endings in the command input this should be
  /// handled in the IO implementation.
  fn run_write_command(&mut self,
    // UI handle. Created by setting up the UI for passing through std-in/-err
    // to child process.
    ui: &mut UILock,
    // Command string from user (with basic substitutions interpreted)
    command: String,
    // Iterator over string slices to send over stdin
    input: LinesIter,
  ) -> Result<usize>;

  /// Run a transform command, taking part of buffer via stdin and returning it
  /// via stdout.
  ///
  /// Stderr should be passed through to UI
  ///
  /// The LinesIter contains string slices over '\n' terminated lines. If you
  /// with to use "\r\n" line endings in the command input this should be
  /// handled in the IO implementation.
  ///
  /// The returned string will be split into lines and added into the buffer.
  /// All line endings are converted into '\n' when adding into the buffer.
  fn run_transform_command(&mut self,
    // UI handle. Created by setting up the UI for passing through
    // std-in/-out/-err to child process.
    ui: &mut UILock,
    // Command string from user (with basic substitutions interpreted)
    command: String,
    // Iterator over string slices to send over stdin
    input: LinesIter,
  ) -> Result<String>;

  /// Normal file write
  ///
  /// Returns number of bytes written
  ///
  /// The LinesIter contains string slices over '\n' terminated lines. If you
  /// with to write "\r\n" line endings into the file this should be handled in
  /// the IO implementation.
  fn write_file(&mut self,
    // Path to file as give by user. Not checked beyond shell escape parsing
    path: &str,
    // If appending
    append: bool,
    // If not set won't overwrite an existing file
    overwrite: bool,
    // Data to write to file
    data: LinesIter,
  ) -> Result<usize>;

  /// Normal file read
  ///
  /// The returned string will be split into lines and added into the buffer.
  /// All line endings are converted into '\n' when adding into the buffer.
  fn read_file(&mut self,
    // Path to file as given by user. Not checked beyond shell escape parsing
    path: &str,
    // If true the method should error if no file is found at path
    must_exist: bool,
  ) -> Result<String>;
}
