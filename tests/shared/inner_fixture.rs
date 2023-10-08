#![allow(dead_code)]

use std::collections::HashMap;

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
  LineText,
  macros::Macro,
};

pub fn inner_fixture(
  init_clipboard: Vec<&str>,
  init_buffer: Vec<&str>,
  init_buffer_saved: bool,
  init_filepath: &str,
  init_macros: Option<HashMap<&str, Macro>>,
  command_input: Vec<&str>,
  expected_result: Result<(),EdError>,
  expected_buffer: Vec<&str>,
  expected_buffer_saved: bool,
  expected_history_tags: Vec<&str>,
  expected_selection: (usize, usize),
  expected_clipboard: Vec<&str>,
  expected_filepath: &str,
  expected_prints: Vec<Print>,
) {
  // Instantiate dummy IO
  let mut io = DummyIO::new();
  // Apply given or default to no macros
  let macros = init_macros.unwrap_or(HashMap::new());
  // Create ed state and init ed.buffer
  let mut ed = Ed::new(
    &mut io,
    &macros,
  );
  ed.file = init_filepath.to_owned();
  let init_clipboard = init_clipboard.iter().fold(Clipboard::new(), |mut c, x| {
    c.push(PubLine{
      tag: '\0',
      text: LineText::new(format!("{}\n", x)).unwrap(),
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
      text: LineText::new(format!("{}\n", x)).unwrap(),
    });
    c
  });
  ed.history.current_mut("initial load".into()).append(&mut (&init_buffer).try_into().unwrap());
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
  let res = loop {
    match ed.get_and_run_command(&mut ui) {
      Ok(true) => break Ok(()),
      Ok(false) => (),
      Err(e) => break Err(e),
    }
  };
  assert_eq!(
    res,
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
    ed.history.snapshots()[2..].iter()
      .map(|(tag, _)| &tag[..])
      .collect::<Vec<&str>>()
    ,
    expected_history_tags,
    "The snapshot tags after test (left) didn't match expectations (right)."
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
