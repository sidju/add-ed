#[cfg(feature = "local_io")]
pub mod local_io;

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
  fn run_command(
    /// UI handle. Created by setting up the UI for passing through
    /// std-in/-out/-err to the child process.
    ui: &mut dyn UIHandle,
    /// Command string from user (with basic substitutions interpreted)
    command: String,
  ) -> Result<(), &'static str>;

  /// Run a read command, collecting stdout to add into buffer
  ///
  /// Stdin and Stderr should be passed through to UI
  fn run_read_command(
    /// UI handle. Created by setting up the UI for passing through
    /// std-in/-err to child process.
    ui: &mut dyn UIHandle,
    /// Command string from user (with basic substitutions interpreted)
    command: String,
  ) -> Result<String, &'static str>;

  /// Run a write command, receiving part of buffer via stdin
  ///
  /// Stdout and Stderr should be passed through to UI
  /// Returns number of bytes written
  fn run_write_command(
    /// UI handle. Created by setting up the UI for passing through std-in/-err
    /// to child process.
    ui: &mut dyn UIHandle,
    /// Command string from user (with basic substitutions interpreted)
    command: String,
    /// Iterator over string slices to send over stdin
    input: impl Iterator<Item = &str>,
  ) -> Result<usize, &'static str>;

  /// Run a transform command, taking part of buffer via stdin and returning it
  /// via stdout.
  ///
  /// Stderr should be passed through to UI
  fn run_transform_command(
    /// UI handle. Created by setting up the UI for passing through
    /// std-in/-out/-err to child process.
    ui: &mut dyn UIHandle,
    /// Command string from user (with basic substitutions interpreted)
    command: String,
    /// Iterator over string slices to send over stdin
    input: impl Iterator<Item = &str>,
  ) -> Result<String, &'static str>;

  /// Normal file write
  /// Returns number of bytes written
  fn write_file(
    /// Path to file as give by user. Not checked beyond shell escape parsing
    path: &str,
    /// Data to write to file
    data: impl Iterator<Item = &str>,
    /// If appending
    append: bool,
  ) -> Result<usize, &'static str>;

  /// Normal file read
  fn read_file(
    /// Path to file as given by user. Not checked beyond shell escape parsing
    path: &str,
  ) -> Result<String, &'static str>;
}
