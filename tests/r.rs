// Tests for 'r' command

use add_ed::EdError;
use std::collections::HashMap;
mod shared;
use shared::fixtures::{
  IOTest,
  ErrorTest,
};
use shared::fake_io::{
  FakeIO,
  ShellCommand,
};

// Verify behaviour of 'r' command
//
// - Takes optional index, defaults to state.selection.1
// - Takes filepath or shell escape after command, defaults to state.path
// - Inserts contents of file/output of shell command after selection
// - Sets unsaved
// - Selection after is all of the newly read data
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

// No selection and no path, verify defaults
// (As starting selection is all of buffer we should append the data from
// state.file)
#[test]
fn read_defaults() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["text"],
    init_io: test_io.clone(),
    init_clipboard: vec!["dummy"],
    init_filepath: "text",
    command_input: vec![
      "r",
    ],
    expected_buffer: vec![
      "text",
      "file",
      "data",
      "in",
      "file",
    ],
    expected_buffer_saved: false,
    expected_selection: (2,5),
    expected_file_changes: vec![], // No changes to the fs
    expected_clipboard: vec!["dummy"],
    expected_filepath: "text",
  }.run();
}

// Test that a relative index resolves relative to end of selection
#[test]
fn read_relative() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["text","data","in","buffer"],
    init_io: test_io.clone(),
    init_clipboard: vec!["dummy"],
    init_filepath: "text",
    command_input: vec!["1,2#","+r",],
    expected_buffer: vec![
      "text",
      "data",
      "in",
      "file",
      "data",
      "in",
      "file",
      "buffer",
    ],
    expected_buffer_saved: false,
    expected_selection: (4,7),
    expected_file_changes: vec![], // No changes to the fs
    expected_clipboard: vec!["dummy"],
    expected_filepath: "text",
  }.run();
}

// Fully specified prepend
#[test]
fn read_prepend() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["numbers"],
    init_io: test_io.clone(),
    init_clipboard: vec!["dummy"],
    init_filepath: "",
    command_input: vec!["0r numbers"],
    expected_buffer: vec![
      "4",
      "5",
      "2",
      "1",
      "numbers",
    ],
    expected_buffer_saved: false,
    expected_selection: (1,4),
    expected_file_changes: vec![],
    expected_clipboard: vec!["dummy"],
    expected_filepath: "",
  }.run();
}

// Fully specified prepend with errant space before command
#[test]
fn read_prepend_but_with_space() {
  ErrorTest{
    init_buffer: vec!["numbers"],
    command_input: vec!["0 r numbers"],
    expected_error: EdError::CommandUndefined(' '),
  }.run();
}
