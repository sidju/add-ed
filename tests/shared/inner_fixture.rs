#![allow(dead_code)]

use std::collections::HashMap;
use super::{
  dummy_io::DummyIO,
  mock_ui::{Print, MockUI},
};
use add_ed::{
  buffer::Buffer,
  ui::ScriptedUI,
  Ed,
};

pub fn inner_fixture(
  init_clipboard: Vec<&str>,
  init_buffer: Vec<&str>,
  init_filepath: &str,
  command_input: Vec<&str>,
  expected_result: Result<(),&str>,
  expected_buffer: Vec<&str>,
  expected_buffer_saved: bool,
  expected_selection: (usize, usize),
  expected_clipboard: Vec<&str>,
  expected_filepath: &str,
  expected_prints: Vec<Print>,
) {
  // Instantiate dummy IO
  let mut io = DummyIO::new();
  // Create and init buffer
  let mut buffer = Buffer::new();
  let init_clipboard: Vec<String> = init_clipboard.iter().map(|x| {
    let mut s = x.to_string();
    s.push('\n');
    s
  }).collect();
  let init_buffer: Vec<String> = init_buffer.iter().map(|x| {
    let mut s = x.to_string();
    s.push('\n');
    s
  }).collect();
  if !init_clipboard.is_empty() {
    buffer.insert(init_clipboard, 0).unwrap();
    buffer.cut((1,buffer.len())).unwrap();
  }
  buffer.insert(init_buffer, 0).unwrap();
  buffer.set_saved();
  // Create scripted UI (with mock UI, which tracks print invocations)
  let mut inner_ui = MockUI{ prints_history: Vec::new() };
  let mut ui = ScriptedUI{
    print_ui: Some(&mut inner_ui),
    // For each element convert to String & add newline, collect into VecDeque
    input: command_input.iter().map(|x|{
      let mut s = x.to_string();
      s.push('\n');
      s
    }).collect(),
  };

  // Instantiate editor and run test
  {
    let mut ed = Ed::new(
      &mut buffer,
      &mut io,
      init_filepath.to_owned(),
      HashMap::new(),
      false,
      false,
    );
    assert_eq!(
      ed.run_macro(&mut ui),
      expected_result,
      "Result from running test (left) didn't match expectations (right)."
    );

    // Before dropping editor, verify selection and filepath
    assert_eq!(
      ed.see_state().selection,
      expected_selection,
      "Selection after test (left) didn't match expectations (right)."
    );
    assert_eq!(
      ed.see_state().file,
      expected_filepath,
      "state.filepath after test (left) didn't match expectations (right)."
    );
  }
  assert_eq!(
    buffer.saved(),
    expected_buffer_saved,
    "Buffer.saved() (left) after test didn't match expectations (right)."
  );
  assert_eq!(
    if buffer.len() != 0 {
      buffer.get_selection((1,buffer.len()))
        .unwrap()
        .map(|(_,s)| s.trim_end_matches('\n'))
        .collect::<Vec<&str>>()
    } else {
      vec![]
    },
    expected_buffer,
    "Buffer contents (left) after test didn't match expectations (right)."
  );
  // Switch out buffer contents to clipboard contents
  let end_of_buf = buffer.len();
  buffer.paste(end_of_buf).unwrap();
  if end_of_buf != 0 { buffer.cut((1,end_of_buf)).unwrap(); }
  assert_eq!(
    if buffer.len() != 0 {
      buffer.get_selection((1,buffer.len()))
        .unwrap()
        .map(|(_,s)| s.trim_end_matches('\n'))
        .collect::<Vec<&str>>()
    } else {
      vec![]
    },
    expected_clipboard,
    "Clipboard contents (left) after test didn't match expectations (right)."
  );
  assert_eq!(
    inner_ui.prints_history,
    expected_prints,
    "The history of prints (left) from the test didn't match expectations (right)."
  );
}
