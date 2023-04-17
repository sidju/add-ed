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

// Verify behaviour of '!' command
//
// - Takes optional selection
//   - If given, pipes selection through the shell command
//     (stdin -> command -> stdout, stderr prints directly)
//   - If not given, just runs the shell command
//     (stdin, stdout, stderr passed in)
// - Takes shell command after '!'
// - Sets unsaved if selection is given, else not
// - Selection after is
//   - Unmodified if no selection given
//   - Set to the piped lines if selection given
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

// No selection, just run command
#[test]
fn shell_escape() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["text"],
    init_io: test_io.clone(),
    init_clipboard: vec!["dummy"],
    init_filepath: "text",
    command_input: vec![
      "!echo hi",
    ],
    expected_buffer: vec![
      "text",
    ],
    expected_buffer_saved: true,
    expected_selection: (1,1),
    expected_file_changes: vec![], // No changes to the fs
    expected_clipboard: vec!["dummy"],
    expected_filepath: "text",
  }.run();
}

// Fully specified prepend
#[test]
fn shell_pipe() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["4","5","2","1"],
    init_io: test_io.clone(),
    init_clipboard: vec!["dummy"],
    init_filepath: "numbers",
    command_input: vec![",!sort -n"],
    expected_buffer: vec!["1","2","4","5"],
    expected_buffer_saved: false,
    expected_selection: (1,4),
    expected_file_changes: vec![],
    expected_clipboard: vec!["4","5","2","1"],
    expected_filepath: "numbers",
  }.run();
}
