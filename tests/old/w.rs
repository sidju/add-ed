use std::collections::HashMap;
use add_ed::{
  buffer::{
    Buffer,
  },
  ui::DummyUI,
  Ed,
  error_consts::*,
};

mod mock_io;
use mock_io::{MockIO, ShellCommand};

fn prepare_write_test(
  command_input: Vec<&'static str>,
) -> (MockIO, Buffer, DummyUI) {
  let io = MockIO{
    mock_fs: HashMap::from([
      ("existing_file".to_owned(), "Preexisting\ndata\n".to_owned()),
      ("other_file".to_owned(), "other\ndata\n".to_owned()),
    ]),
    mock_shell: HashMap::from([
      (ShellCommand{
        command: "sudo tee examplefile".to_owned(),
        input: "data\n".to_owned()
      },
      String::new()),
      (ShellCommand{
        command: "sudo tee otherfile".to_owned(),
        input: "write\ndata\n".to_owned()
      },
      String::new()),
    ])
  };
  let mut buffer = Buffer::new();
  buffer.insert(vec!["write\n", "data\n"], 0).unwrap();
  let ui = DummyUI{
    input: command_input.iter()
      .map(|s|s.to_owned().to_owned())
      .collect(),
    print_ui: None,
  };
  (io, buffer, ui)
}

// Write to default path with default selection
// Should default to state.file and whole selection, even when prior selection exists.
#[test]
fn file_write() {
  // Create the testing structs
  let (mut io, mut buffer, mut ui) = prepare_write_test(vec![
    "1", // Create a previous selection that should be disregarded
    "w",
  ]);
  {
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "default_file".to_string(),
      HashMap::new(),
      false,
      false,
    )
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (1,1),
      "Selection incorrectly modified when writing without selection or path."
    );
  }
  assert_eq!(
    io.mock_fs.get("default_file").map(|x|&x[..]),
    Some("write\ndata\n"),
    "Correct data not put into file."
  );
  assert!(
    buffer.saved(),
    "Buffer wasn't flagged as saved after running 'w'"
  );
}

// Append to default path with default selection
// Should default to state.file and whole selection, even when prior selection exists.
#[test]
fn file_append() {
  // Create the testing structs
  let (mut io, mut buffer, mut ui) = prepare_write_test(vec![
    "1", // Create a previous selection that should be disregarded
    "W",
  ]);
  {
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "existing_file".to_string(),
      HashMap::new(),
      false,
      false,
    )
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (1,1),
      "Selection incorrectly modified when writing without selection or path."
    );
  }
  assert_eq!(
    io.mock_fs.get("existing_file").map(|x|&x[..]),
    Some("Preexisting\ndata\nwrite\ndata\n"),
    "Correct data not put into file."
  );
  assert!(
    !buffer.saved(),
    "Buffer was flagged as saved after running 'W'"
  );
}

// Write to specific path with default selection
// Should default to whole selection, despite prior selection,
// and update state.file.
#[test]
fn file_write_to_path() {
  // Create the testing structs
  let (mut io, mut buffer, mut ui) = prepare_write_test(vec![
    "1", // Create a previous selection that should be disregarded
    "w write_file",
  ]);
  {
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "default_file".to_string(),
      HashMap::new(),
      false,
      false,
    )
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (1,1),
      "Selection incorrectly modified when writing without selection or path."
    );
    assert_eq!(
      &ed.see_state().file,
      &"write_file",
      "state.file not updated after writing whole buffer to it."
    );
  }
  assert_eq!(
    io.mock_fs.get("write_file").map(|x|&x[..]),
    Some("write\ndata\n"),
    "Correct data not put into file."
  );
  assert!(
    buffer.saved(),
    "Buffer wasn't flagged as saved after running 'w'"
  );
}

// Append to default path with default selection
// Should default to whole selection, even though selection exists, and update
// state.file but not selection
#[test]
fn file_append_to_path() {
  // Create the testing structs
  let (mut io, mut buffer, mut ui) = prepare_write_test(vec![
    "1", // Create a previous selection that should be disregarded
    "W other_file",
  ]);
  {
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "default_file".to_string(),
      HashMap::new(),
      false,
      false,
    )
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (1,1),
      "Selection incorrectly modified when writing without selection or path."
    );
    assert_eq!(
      &ed.see_state().file,
      &"default_file",
      "state.file changed when its contents aren't exactly all of buffer."
    );
  }
  assert_eq!(
    io.mock_fs.get("other_file").map(|x|&x[..]),
    Some("other\ndata\nwrite\ndata\n"),
    "Correct data not put into file."
  );
  assert!(
    !buffer.saved(),
    "Buffer was flagged as saved after running 'W'"
  );
}

// Write to specific path with specific non-whole selection
// Should change selection and not state.file and overwrite given path
#[test]
fn file_partial_write_to_path() {
  // Create the testing structs
  let (mut io, mut buffer, mut ui) = prepare_write_test(vec![
    "1,2", // Set selection to known value, so we detect change
    "1w partial_file",
  ]);
  {
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "default_file".to_string(),
      HashMap::new(),
      false,
      false,
    )
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (1,1),
      "Selection not correctly updated to specific selection given to 'w'."
    );
    assert_eq!(
      &ed.see_state().file,
      &"default_file",
      "State file changed when not whole buffer was saved."
    );
  }
  assert_eq!(
    io.mock_fs.get("partial_file").map(|x|&x[..]),
    Some("write\n"),
    "Correct data not put into file."
  );
  assert!(
    !buffer.saved(),
    "Buffer was flagged as saved after running 'w' with partial buffer selected."
  );
}

// Append to specific path with specific non-whole selection
// Should change selection and not state.file and add to given path
#[test]
fn file_partial_append_to_path() {
  // Create the testing structs
  let (mut io, mut buffer, mut ui) = prepare_write_test(vec![
    "1,2", // Set selection to known value, so we detect change
    "1W existing_file",
  ]);
  {
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "default_file".to_string(),
      HashMap::new(),
      false,
      false,
    )
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (1,1),
      "Selection not correctly updated to specific selection given to 'W'."
    );
    assert_eq!(
      &ed.see_state().file,
      &"default_file",
      "State file changed when not whole buffer was saved."
    );
  }
  assert_eq!(
    io.mock_fs.get("existing_file").map(|x|&x[..]),
    Some("Preexisting\ndata\nwrite\n"),
    "Correct data not put into file."
  );
  assert!(
    !buffer.saved(),
    "Buffer was flagged as saved after running 'W'"
  );
}

// Write to command with specific non-whole selection
// Should change selection and not state.file and hand in selection to command
#[test]
fn file_partial_write_to_command() {
  // Create the testing structs
  let (mut io, mut buffer, mut ui) = prepare_write_test(vec![
    "1,2", // Set selection to known value, so we detect change
    "2w !sudo tee examplefile",
  ]);
  {
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "default_file".to_string(),
      HashMap::new(),
      false,
      false,
    )
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (2,2),
      "Selection not correctly updated to specific selection given to 'w'."
    );
    assert_eq!(
      &ed.see_state().file,
      &"default_file",
      "State file changed when not whole buffer was written, and to a command."
    );
  }
  assert!(
    !buffer.saved(),
    "Buffer was flagged as saved after running 'w' with partial buffer selected."
  );
}

// Write to command without selection
// Should use default selection,  not change state.file and hand in selection to
// command
#[test]
fn file_write_to_command() {
  // Create the testing structs
  let (mut io, mut buffer, mut ui) = prepare_write_test(vec![
    "1", // Set a selection to detect if w changes it
    "w !sudo tee otherfile",
  ]);
  {
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "default_file".to_string(),
      HashMap::new(),
      false,
      false,
    )
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (1,1),
      "Selection not correctly updated to specific selection given to 'w'."
    );
    assert_eq!(
      &ed.see_state().file,
      &"default_file",
      "State file changed when not whole buffer was written, and to a command."
    );
  }
  assert!(
    !buffer.saved(),
    "Buffer was flagged as saved after running 'w' into a command."
  );
}

// Write to command that doesn't exist
// Should just error with the right message
#[test]
fn file_write_to_invalid_command() {
  // Create the testing structs
  let (mut io, mut buffer, mut ui) = prepare_write_test(vec![
    "1", // Set a selection to detect if w changes it
    "w !not existing",
  ]);
  {
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "default_file".to_string(),
      HashMap::new(),
      false,
      false,
    )
    ;
    let res = ed.run_macro(&mut ui);
    assert_eq!(
      Err(CHILD_EXIT_ERROR),
      res,
      "Running non-existing command didn't return the expected error."
    );
    assert_eq!(
      ed.see_state().selection,
      (1,1),
      "Selection not correctly updated to specific selection given to 'w'."
    );
    assert_eq!(
      &ed.see_state().file,
      &"default_file",
      "State file changed when not whole buffer was written, and to a command."
    );
  }
}
