// Tests for 'e' and 'E' command
// Only test for 'E' is with unsaved buffer, last in file

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
// (Since there are no changes 'e' should reread from disk, for example to get a
// updated version of a file after git pull or similar)
#[test]
fn edit_defaults() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["text"],
    init_clipboard: vec!["dummy"],
    init_io: test_io.clone(),
    init_filepath: "text",
    command_input: vec![
      "e",
    ],
    expected_buffer: vec![
      "file",
      "data",
      "in",
      "file",
    ],
    expected_buffer_saved: true,
    expected_selection: (1,4),
    expected_clipboard: vec!["dummy"],
    expected_file_changes: vec![], // No changes to the fs
    expected_filepath: "text",
  }.run();
}

// Give path, verify state changes
#[test]
fn edit_path() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["text"],
    init_clipboard: vec!["dummy"],
    init_io: test_io.clone(),
    init_filepath: "text",
    command_input: vec![
      "e numbers",
    ],
    expected_buffer: vec![
      "4",
      "5",
      "2",
      "1",
    ],
    expected_buffer_saved: true,
    expected_selection: (1,4),
    expected_clipboard: vec!["dummy"],
    expected_file_changes: vec![], // No changes to the fs
    expected_filepath: "numbers",
  }.run();
}

// Give path that doesn't exist yet, verify state changes
#[test]
fn edit_new_file() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["text"],
    init_clipboard: vec!["dummy"],
    init_io: test_io.clone(),
    init_filepath: "text",
    command_input: vec![
      "e new_file",
    ],
    expected_buffer: vec![
    ],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec!["dummy"],
    expected_file_changes: vec![], // No changes to the fs
    expected_filepath: "new_file",
  }.run();
}

// Give selection, should panic
#[test]
fn edit_selection() {
  ErrorTest{
    init_buffer: vec!["text"],
    init_clipboard: vec!["dummy"],
    init_filepath: "text",
    command_input: vec![
      ",e",
    ],
    expected_error: add_ed::error_consts::SELECTION_FORBIDDEN,
    // Everything should be unchanged, since we should error without changes
    expected_buffer: vec!["text"],
    expected_buffer_saved: true,
    expected_selection: (1,1),
    expected_clipboard: vec!["dummy"],
    expected_filepath: "text",
  }.run();
}

// With edits, should error early so we don't need an IO
#[test]
fn edit_unsaved() {
  ErrorTest{
    init_buffer: vec!["text"],
    init_clipboard: vec!["dummy"],
    init_filepath: "text",
    command_input: vec![
      "i",
      "line",
      ".",
      "e",
    ],
    expected_error: add_ed::error_consts::UNSAVED_CHANGES,
    // Expectations shouldn't be affected by the 'e' invocation, since it errors
    expected_buffer: vec!["line","text"],
    expected_buffer_saved: false,
    expected_selection: (1,1),
    expected_clipboard: vec!["dummy"],
    expected_filepath: "text",
  }.run();
}

// With edits, should go through with force
#[test]
fn force_edit_unsaved() {
  let test_io = test_io();
  IOTest{
    init_buffer: vec!["text"],
    init_clipboard: vec!["dummy"],
    init_io: test_io.clone(),
    init_filepath: "text",
    command_input: vec![
      "i",
      "line",
      ".",
      "E",
    ],
    expected_buffer: vec![
      "file",
      "data",
      "in",
      "file",
    ],
    expected_buffer_saved: true,
    expected_selection: (1,4),
    expected_clipboard: vec!["dummy"],
    expected_file_changes: vec![], // No changes to the fs
    expected_filepath: "text",
  }.run();
}
