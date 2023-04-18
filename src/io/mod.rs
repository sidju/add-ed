use super::ui::UILock;

#[cfg(any(feature = "testing", fuzzing, test))]
pub mod fake_io;
#[cfg(any(feature = "testing", fuzzing, test))]
pub mod dummy_io;

#[cfg(feature = "local_io")]
pub mod local_io;
#[cfg(feature = "local_io")]
pub use local_io::LocalIO;

#[cfg(all(feature = "test_local_io", test))]
mod test;

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
  ) -> Result<(), &'static str>;

  /// Run a read command, collecting stdout to add into buffer
  ///
  /// Stdin and Stderr should be passed through to UI
  fn run_read_command(&mut self,
    // UI handle. Created by setting up the UI for passing through
    // std-in/-err to child process.
    ui: &mut UILock,
    // Command string from user (with basic substitutions interpreted)
    command: String,
  ) -> Result<String, &'static str>;

  /// Run a write command, receiving part of buffer via stdin
  ///
  /// Stdout and Stderr should be passed through to UI
  /// Returns number of bytes written
  fn run_write_command<'a>(&mut self,
    // UI handle. Created by setting up the UI for passing through std-in/-err
    // to child process.
    ui: &mut UILock,
    // Command string from user (with basic substitutions interpreted)
    command: String,
    // Iterator over string slices to send over stdin
    input: impl Iterator<Item = &'a str>,
  ) -> Result<usize, &'static str>;

  /// Run a transform command, taking part of buffer via stdin and returning it
  /// via stdout.
  ///
  /// Stderr should be passed through to UI
  fn run_transform_command<'a>(&mut self,
    // UI handle. Created by setting up the UI for passing through
    // std-in/-out/-err to child process.
    ui: &mut UILock,
    // Command string from user (with basic substitutions interpreted)
    command: String,
    // Iterator over string slices to send over stdin
    input: impl Iterator<Item = &'a str>,
  ) -> Result<String, &'static str>;

  /// Normal file write
  /// Returns number of bytes written
  fn write_file<'a>(&mut self,
    // Path to file as give by user. Not checked beyond shell escape parsing
    path: &str,
    // If appending
    append: bool,
    // Data to write to file
    data: impl Iterator<Item = &'a str>,
  ) -> Result<usize, &'static str>;

  /// Normal file read
  fn read_file(&mut self,
    // Path to file as given by user. Not checked beyond shell escape parsing
    path: &str,
    // If true the method should error if no file is found at path
    must_exist: bool,
  ) -> Result<String, &'static str>;
}
