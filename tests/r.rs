use std::collections::HashMap;
use add_ed::{
  buffer::{
    Buffer,
  },
  ui::DummyUI,
  Ed,
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
      (ShellCommand{
        command:"echo hi".to_owned(),
        input: String::new()},
      "hi\n".to_owned()),
    ]),
  };
  let buffer = Buffer::new();
  (io, buffer)
}
// Append data from given path to buffer
// Should add data, select new data and not modify state.file
#[test]
fn append_in_given_path() {
  // Create the testing structs
  let (mut io, mut buffer) = prepare_read_test();

  {
    let mut ui = DummyUI{
      input: vec![
        "e".to_owned(),
        "r read_file\n".to_owned(),
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
      (3,5),
      "Selection after r wasn't set to newly read data."
    );
    assert_eq!(
      &ed.see_state().file,
      &"state_file",
      "State.file was modified by calling r."
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["file\n", "data\n", "data\n", "in\n", "file\n"],
    "Data wasn't correctly appended to buffer."
  );
}

// Append data from given state.file to buffer
// Should add data and select new data
#[test]
fn append_in_state_file() {
  // Create the testing structs
  let (mut io, mut buffer) = prepare_read_test();

  {
    let mut ui = DummyUI{
      input: vec![
        "e\n".to_owned(),
        "r\n".to_owned(),
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
      (3,4),
      "Selection after r wasn't set to newly read data."
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["file\n", "data\n", "file\n", "data\n"],
    "'r' should append state.file data after buffer, since default selection is all."
  );
}

// Append data from given state.file to specific index
// Should add data to that index and select new data
#[test]
fn append_in_state_file_to_index() {
  // Create the testing structs
  let (mut io, mut buffer) = prepare_read_test();

  {
    let mut ui = DummyUI{
      input: vec![
        "e\n".to_owned(),
        "0,1r\n".to_owned(), // Give both start and end to verify which is used
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
      (2,3),
      "Selection after r wasn't set to newly read data."
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["file\n", "file\n", "data\n", "data\n"],
    "'r' should append state.file data after line 1, aka. selection.1."
  );
}

// Append data from shell command
// Should add data and select new data without change to state.file
#[test]
fn append_in_command() {
  // Create the testing structs
  let (mut io, mut buffer) = prepare_read_test();

  {
    let mut ui = DummyUI{
      input: vec![
        "e\n".to_owned(),
        "r !echo hi\n".to_owned(),
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
      (3,3),
      "Selection after r wasn't set to newly read data."
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["file\n", "data\n", "hi\n"],
    "'r' should append command output after buffer, since default selection is all."
  );
}
