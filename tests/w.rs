// Tests for 'r' command

use std::collections::HashMap;
mod shared;
use shared::fixtures::{
  IOTest,
};
use shared::fake_io::{
  FakeIO,
  ShellCommand,
};

// Verify behaviour of 'w' command
//
// - Takes optional selection, defaults to whole buffer
// - Takes filepath or shell escape after command, defaults to state.path
//   - Special: if filepath is "q" without space separation it is taken as a
//     flag to quit after saving to current state.path. If not currently saving
//     the whole buffer this returns an unsaved changes error.
// - Writes buffer into file/stdin for shell command
// - Sets saved if whole buffer is written
// - Selection after is not changed unless selection is given, otherwise set to
//   the given selection
// - If whole buffer is written and path isn't a shell command, saves it to
//   state.path

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
fn write_defaults() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["text"],
    init_io: test_io.clone(),
    init_filepath: "text",
    command_input: vec![
      "w",
    ],
    expected_buffer: vec![
      "text",
    ],
    expected_buffer_saved: true,
    expected_selection: (1,1),
    expected_file_changes: vec![("text","text\n")], // buffer overwrites "text"
    expected_filepath: "text",
  }.run();
}

// Fully specified to other path
#[test]
fn write_fully_specified() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["text","data"],
    init_io: test_io.clone(),
    init_filepath: "text",
    command_input: vec![",w new_file"],
    expected_buffer: vec!["text","data"],
    expected_buffer_saved: true,
    expected_selection: (1,2),
    expected_file_changes: vec![("new_file","text\ndata\n")],
    expected_filepath: "new_file",
  }.run();
}

// Verify that writing to shell commands behaves as intended
#[test]
fn write_command() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec![],
    init_io: test_io,
    init_filepath: "numbers",
    command_input: vec!["e","w !sort -n"],
    expected_buffer: vec!["4","5","2","1"],
    expected_buffer_saved: true, // Since 'e' sets saved and 'w' doesn't unset
    expected_selection: (1,4),
    expected_file_changes: vec![],
    expected_filepath: "numbers",
  }.run()
}

// Verify behaviour of 'W' command
//
// - Takes optional selection, defaults to whole buffer
// - Takes filepath or shell escape after command, defaults to state.path
// - Appends buffer to file/writes to stdin for shell command
// - Sets saved if whole buffer is written
// - Selection after is not changed unless selection is given, otherwise set to
//   the given selection
// - Doesn't change state.path

// Test appending write
#[test]
fn write_append_defaults() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["3","6"],
    init_io: test_io,
    init_filepath: "numbers",
    command_input: vec!["W"],
    expected_buffer: vec!["3","6"],
    expected_buffer_saved: true,
    expected_selection: (1,2),
    expected_file_changes: vec![("numbers","4\n5\n2\n1\n3\n6\n")],
    expected_filepath: "numbers",
  }.run()
}

// Test fully defined append
#[test]
fn write_append() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["3","6"],
    init_io: test_io,
    init_filepath: "default",
    command_input: vec!["1W numbers"],
    expected_buffer: vec!["3","6"],
    expected_buffer_saved: true,
    expected_selection: (1,1),
    expected_file_changes: vec![("numbers","4\n5\n2\n1\n3\n")],
    expected_filepath: "default",
  }.run()
}
