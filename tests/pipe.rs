// Tests for '!' command

use std::collections::HashMap;
mod shared;
use shared::fixtures::{
  IOTest,
};
use shared::fake_io::{
  FakeIO,
  ShellCommand,
};

// Verify behaviour of '|' command
//
// - Takes optional selection
//   - If none given uses default selection
// - Pipes selection through the shell command
//   (stdin -> command -> stdout, stderr prints directly)
// - Takes shell command as only argument
// - Sets unsaved
// - Selection after is set to the piped lines if selection
//   given
// - Doesn't modify state.path

// Function to set up the "filesystem" for these tests
fn test_io() -> FakeIO {
  FakeIO {
    fake_fs: HashMap::from([
      ("text".to_owned(), "file\ndata\nin\nfile\n".to_owned()),
      ("numbers".to_owned(), "4\n5\n2\n1\n".to_owned()),
    ]),
    fake_shell: HashMap::from([
      (
        ShellCommand{
          command:"echo hi".to_owned(),
          input: String::new(),
        },
        "hi\n".to_owned(),
      ),
      (
        ShellCommand{
          command:"sort -n".to_owned(),
          input:"4\n5\n2\n1\n".to_owned(),
        },
        "1\n2\n4\n5\n".to_owned(),
      ),
    ]),
  }
}

// To be moved into pipe.rs
#[test]
fn shell_pipe() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["4","5","2","1"],
    init_io: test_io.clone(),
    init_clipboard: vec!["dummy"],
    init_filepath: "numbers",
    command_input: vec![",|sort -n"],
    expected_buffer: vec!["1","2","4","5"],
    expected_buffer_saved: false,
    expected_selection: (1,4),
    expected_file_changes: vec![],
    expected_clipboard: vec!["4","5","2","1"],
    expected_filepath: "numbers",
  }.run();
}
