use std::collections::HashMap;
use super::dummy_io::DummyIO;
use add_ed::{
  buffer::Buffer,
  ui::ScriptedUI,
  Ed,
};

// A basic fixture
// Sets up state as though reading buffer contents from a file and runs the
// given commands via dummy_ui. (Selection is Ed default, buffer.saved is true)
// Afterwards verifies state against optional expectations on
// buffer contents and selection.
// Terminating '\n' aren't needed nor allowed in any of the Vec<&str> arguments.
pub struct BasicTest {
  pub init_buffer: Vec<&'static str>,
  pub init_clipboard: Vec<&'static str>,
  pub command_input: Vec<&'static str>,
  pub expected_buffer: Vec<&'static str>,
  pub expected_buffer_saved: bool,
  pub expected_selection: (usize, usize),
  pub expected_clipboard: Vec<&'static str>,
}
impl BasicTest {
  pub fn run(self) {
    // Instantiate dummy IO
    let mut io = DummyIO::new();
    // Create and init buffer
    let mut buffer = Buffer::new();
    let init_clipboard: Vec<String> = self.init_clipboard.iter().map(|x| {
      let mut s = x.to_string();
      s.push('\n');
      s
    }).collect();
    let init_buffer: Vec<String> = self.init_buffer.iter().map(|x| {
      let mut s = x.to_string();
      s.push('\n');
      s
    }).collect();
    if !self.init_clipboard.is_empty() {
      buffer.insert(init_clipboard, 0).unwrap();
      buffer.cut((1,buffer.len())).unwrap();
    }
    buffer.insert(init_buffer, 0).unwrap();
    buffer.set_saved();
    // Create scripted UI (with no printing UI, errors on print invocations)
    let mut ui = ScriptedUI{
      print_ui: None,
      // For each element convert to String & add newline, collect into VecDeque
      input: self.command_input.iter().map(|x|{
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
        "default_file".to_owned(),
        HashMap::new(),
        false,
        false,
      );
      ed.run_macro(&mut ui).expect("Error running test.");

      // Before dropping editor, read selection if expectation
      assert_eq!(
        ed.see_state().selection,
        self.expected_selection,
        "Selection after test (left) didn't match expectations (right)."
      );
    }
    assert_eq!(
      buffer.saved(),
      self.expected_buffer_saved,
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
      self.expected_buffer,
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
      self.expected_clipboard,
      "Cliboard contents (left) after test didn't match expectations (right)."
    );
  }
}
