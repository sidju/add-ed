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

fn prepare_read_test(
) -> (MockIO, Buffer) {
  let io = MockIO{
    mock_fs: HashMap::from([
      ("state_file".to_owned(), "file\ndata\n".to_owned()),
      ("read_file".to_owned(), "data\nin\nfile\n".to_owned()),
    ]),
    mock_shell: HashMap::from([
      (ShellCommand{command: "echo hi".to_owned(), input: String::new()},
      "hi\n".to_owned()),
    ]),
  };
  let buffer = Buffer::new();
  (io, buffer)
}

// Error case for reading
// Since buffer is unsaved we should get an unsaved error before wiping buffer.
#[test]
fn read_when_unsaved() {
  let (mut io, mut buffer) = prepare_read_test();
  {
    let mut ui = DummyUI{
      input: vec![
        "i\n".to_owned(),
        "1\n".to_owned(),
        ".\n".to_owned(),
        "e\n".to_owned(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "state_file".to_owned(),
      HashMap::new(),
      false,
      false,
    );
    let res = ed.run_macro(&mut ui);
    assert_eq!(
      Err(UNSAVED_CHANGES),
      res,
      "Didn't return UNSAVED_CHANGES when opening new file with unsaved changes."
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n"],
    "Buffer was modified when opening new file with unsaved changes."
  );
}

// Force open to ignore unsaved
// Should read from state_file and select the read data
#[test]
fn force_read_when_unsaved() {
  let (mut io, mut buffer) = prepare_read_test();
  {
    let mut ui = DummyUI{
      input: vec![
        "i\n".to_owned(),
        "1\n".to_owned(),
        ".\n".to_owned(),
        "E\n".to_owned(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "state_file".to_owned(),
      HashMap::new(),
      false,
      false,
    );
    ed.run_macro(&mut ui).unwrap();
    assert_eq!(
      ed.see_state().selection,
      (1,2),
      "E didn't set selection correctly."
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["file\n", "data\n"],
    "E didn't read in data correctly."
  );
}

// Base case for reading
// Should read from state.file path, replace buffer with the data and select all
#[test]
fn read_in_state_file() {
  // Create the testing structs
  let (mut io, mut buffer) = prepare_read_test();

  {
    let mut ui = DummyUI{
      input: vec![
        "e".to_owned(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "state_file".to_owned(),
      HashMap::new(),
      false,
      false,
    );
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (1,2),
      "Selection after e wasn't set to newly read data."
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["file\n", "data\n"],
    "Didn't read data from state.file path correctly."
  );
}

// Read from specific path
// Should replace buffer, set selection to all and replace state.file.
#[test]
fn read_in_given_path() {
  // Create the testing structs
  let (mut io, mut buffer) = prepare_read_test();

  {
    let mut ui = DummyUI{
      input: vec![
        "e read_file".to_owned(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "state_file".to_owned(),
      HashMap::new(),
      false,
      false,
    );
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().file,
      "read_file",
      "state.file not updated after opening new file.",
    );
    assert_eq!(
      ed.see_state().selection,
      (1,3),
      "Selection after wasn't set to newly read data after opening new file."
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["data\n", "in\n", "file\n"],
    "Didn't read data from 'read_file' path correctly."
  );
}

// Read from command
// Should replace buffer, set selection to all but leave state.file unchanged
#[test]
fn read_in_command() {
  // Create the testing structs
  let (mut io, mut buffer) = prepare_read_test();

  {
    let mut ui = DummyUI{
      input: vec![
        "e !echo hi".to_owned(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      "state_file".to_owned(),
      HashMap::new(),
      false,
      false,
    );
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().file,
      "state_file",
      "state.file updated after opening command output.",
    );
    assert_eq!(
      ed.see_state().selection,
      (1,1),
      "Selection after wasn't set to newly read data after opening new file."
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["hi\n"],
    "Didn't read data from command correctly."
  );
}
