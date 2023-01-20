// Tests for 'e' and 'E' command
// Only test case testing 'E' is with unsaved buffer

use std::collections::HashMap;
mod shared;
use shared::fixtures::IOTest;
use shared::fake_io::{
  FakeIO,
  ShellCommand,
};

// Verify behaviour of 'e' and 'E' command
//
// - Takes no index
// - Takes filepath or shell escape after command, defaults to state.path
// - Replaces buffer contens with contents of file/output of shell command
// - Sets state.path to given filepath if one is gived, else unchanged
// - Sets saved if a filepath is given
// - Selection after is all of the newly read data, aka. whole buffer
// - Only 'e' command: If unsaved returns unsaved error to prevent losing data

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

// No selection and no path, verify defaults
#[test]
fn edit_defaults() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["text\n"],
    init_clipboard: vec!["dummy\n"],
    init_io: test_io.clone(),
    init_filepath: "text",
    command_input: vec![
      "e",
    ],
    expected_buffer: vec![
      "file\n",
      "data\n",
      "in\n",
      "file\n",
    ],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec!["dummy\n"],
    expected_file_changes: vec![], // No changes to the fs
    expected_filepath: "text",
  }.run();
}
