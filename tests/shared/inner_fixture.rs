#![allow(dead_code)]

use super::{
  dummy_io::DummyIO,
  mock_ui::{Print, MockUI},
};
use add_ed::{
  ui::ScriptedUI,
  error::EdError,
  Ed,
  Clipboard,
  PubLine,
};
use std::rc::Rc;

pub fn inner_fixture(
  init_clipboard: Vec<&str>,
  init_buffer: Vec<&str>,
  init_buffer_saved: bool,
  init_filepath: &str,
  command_input: Vec<&str>,
  expected_result: Result<(),EdError>,
  expected_buffer: Vec<&str>,
  expected_buffer_saved: bool,
  expected_selection: (usize, usize),
  expected_clipboard: Vec<&str>,
  expected_filepath: &str,
  expected_prints: Vec<Print>,
) {
  // Instantiate dummy IO
  let mut io = DummyIO::new();
  // Create ed state and init ed.buffer
  let mut ed = Ed::new(
    &mut io,
    init_filepath.to_owned(),
  );
  let init_clipboard = init_clipboard.iter().fold(Clipboard::new(), |mut c, x| {
    c.push(PubLine{
      tag: '\0',
      text: Rc::new(format!("{}\n", x)),
    });
    c
  });
  if !init_clipboard.is_empty() {
    ed.clipboard = init_clipboard;
  }
  // We construct a Clipboard, since we cannot construct Buffer directly and it
  // can be easily converted into Buffer when needed.
  let init_buffer = init_buffer.iter().fold(Clipboard::new(), |mut c, x| {
    c.push(PubLine{
      tag: '\0',
      text: Rc::new(format!("{}\n", x)),
    });
    c
  });
  ed.history.current_mut().unwrap().append(&mut (&init_buffer).into());
  if init_buffer_saved { ed.history.set_saved(); }
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

  // Set correct default selection and run test
  ed.selection = (1,ed.history.current().len());
  assert_eq!(
    ed.run_macro(&mut ui),
    expected_result,
    "Result from running test (left) didn't match expectations (right)."
  );

  // Verify state after execution
  assert_eq!(
    ed.history.current()[..].iter()
      .map(|l| l.text.trim_end_matches('\n'))
      .collect::<Vec<&str>>()
    ,
    expected_buffer,
    "Buffer contents after test (left) didn't match expectations (right)."
  );
  assert_eq!(
    ed.history.saved(),
    expected_buffer_saved,
    "Buffer.saved() after test (left) didn't match expectations (right)."
  );
  assert_eq!(
    ed.selection,
    expected_selection,
    "Selection after test (left) didn't match expectations (right)."
  );
  assert_eq!(
    ed.file,
    expected_filepath,
    "state.filepath after test (left) didn't match expectations (right)."
  );
  assert_eq!(
    ed.clipboard[..].iter()
      .map(|l| l.text.trim_end_matches('\n'))
      .collect::<Vec<&str>>()
    ,
    expected_clipboard,
    "Clipboard contents after test (left) didn't match expectations (right)."
  );
  assert_eq!(
    inner_ui.prints_history,
    expected_prints,
    "The history of prints (left) from the test didn't match expectations (right)."
  );
}
