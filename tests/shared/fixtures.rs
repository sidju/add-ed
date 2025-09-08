#![allow(dead_code)]

use std::collections::HashMap;

use super::{
  inner_fixture,
  fake_io::FakeIO,
  mock_ui::{Print, MockUI},
};
use add_ed::{
  ui::ScriptedUI,
  Ed,
  error::EdError,
  PubLine,
  Clipboard,
  LineText,
  macros::Macro,
};

// A basic fixture
// Sets up state as though reading buffer contents from a file and runs the
// given commands via dummy_ui. (Selection is Ed default, buffer.saved is true)
// Afterwards verifies state against optional expectations on
// buffer contents and selection.
// Panics if any command tries to print, use PrintTest if this isn't desired.
// Terminating '\n' aren't needed nor allowed in any of the Vec<&str> arguments.
pub struct BasicTest {
  pub init_buffer: Vec<&'static str>,
  pub init_clipboard: Vec<&'static str>,
  pub command_input: Vec<&'static str>,
  pub expected_buffer: Vec<&'static str>,
  pub expected_buffer_saved: bool,
  pub expected_selection: (usize, usize),
  pub expected_clipboard: Vec<&'static str>,
  pub expected_history_tags: Vec<&'static str>,
}
impl BasicTest {
  pub fn run(self) {
    inner_fixture(
      self.init_clipboard,
      self.init_buffer,
      true,
      "path",
      None,
      self.command_input,
      Ok(()),
      self.expected_buffer,
      self.expected_buffer_saved,
      self.expected_history_tags,
      self.expected_selection,
      self.expected_clipboard,
      "path",
      vec![], // No prints expected
    )
  }
}

// A macro testing fixture
// Sets up state as though reading buffer contents from a file and runs the
// given macro name via dummy_ui.(Selection is Ed default, buffer.saved is true)
// Afterwards verifies state against expectations on buffer contents, selection
// and history labels.
// Panics if any command tries to print, use PrintTest if this isn't desired.
// Terminating '\n' aren't needed nor allowed in any of the Vec<&str> arguments.
pub struct MacroTest {
  pub init_buffer: Vec<&'static str>,
  pub init_clipboard: Vec<&'static str>,
  pub macro_store: HashMap<&'static str, Macro>,
  pub macro_invocation: &'static str,
  pub expected_buffer: Vec<&'static str>,
  pub expected_buffer_saved: bool,
  pub expected_selection: (usize, usize),
  pub expected_clipboard: Vec<&'static str>,
  pub expected_history_tags: Vec<&'static str>,
}
impl MacroTest {
  pub fn run(self) {
    inner_fixture(
      self.init_clipboard,
      self.init_buffer,
      true,
      "path",
      Some(self.macro_store),
      vec![self.macro_invocation],
      Ok(()),
      self.expected_buffer,
      self.expected_buffer_saved,
      self.expected_history_tags,
      self.expected_selection,
      self.expected_clipboard,
      "path",
      vec![], // No prints expected
    )
  }
}

// A macro error testing fixture
// Sets up state as though reading buffer contents from a file and runs the
// given macro name via dummy_ui.(Selection is Ed default, buffer.saved is true)
// Afterwards verifies returned error and history tags state (no tag should have
// been created if the macro failed in execution).
// Terminating '\n' aren't needed nor allowed in any of the Vec<&str> arguments.
pub struct MacroErrorTest {
  pub init_buffer: Vec<&'static str>,
  pub macro_store: HashMap<&'static str, Macro>,
  pub macro_invocation: &'static str,
  pub expected_error: EdError,
}
impl MacroErrorTest {
  pub fn run(self) {
    let init_buffer_len = self.init_buffer.len();
    inner_fixture(
      vec![],
      self.init_buffer.clone(),
      true,
      "path",
      Some(self.macro_store),
      vec![self.macro_invocation],
      Err(self.expected_error),
      self.init_buffer,
      true,
      vec![],
      (1,init_buffer_len),
      vec![],
      "path",
      vec![], // No prints expected
    )
  }
}

// An error checking fixture
// Sets up state as though reading buffer contents from a file and runs the
// given commands via dummy_ui. (Selection is Ed default, buffer.saved is true)
// Afterwards verifies that state hasn't changed and error matches expectations.
// (Note the caveats in the PartialEq<EdError> implementation.)
// Panics if any command tries to print, use PrintTest if this isn't desired.
// Terminating '\n' aren't needed nor allowed in any of the Vec<&str> arguments.
pub struct ErrorTest {
  pub init_buffer: Vec<&'static str>,
  pub command_input: Vec<&'static str>,
  pub expected_error: EdError,
}
impl ErrorTest {
  pub fn run(self) {
    let init_buffer_len = self.init_buffer.len();
    inner_fixture(
      vec![],
      self.init_buffer.clone(),
      false, // Since some errors need the buffer unsaved to trigger
      "path",
      None,
      self.command_input,
      Err(self.expected_error),
      self.init_buffer,
      false,
      vec![],
      (1,init_buffer_len),
      vec![],
      "path",
      vec![], // No prints expected
    )
  }
}

// A test fixture what allows and verifies prints
// Sets up state as though reading buffer contents from a file and runs the
// given commands via dummy_ui. (Selection is Ed default, buffer.saved is true)
// Afterwards verifies state against optional expectations on
// buffer contents, prints and selection.
// Terminating '\n' aren't needed nor allowed in any of the Vec<&str> arguments.
pub struct PrintTest {
  pub init_buffer: Vec<&'static str>,
  pub init_clipboard: Vec<&'static str>,
  pub command_input: Vec<&'static str>,
  pub expected_buffer: Vec<&'static str>,
  pub expected_buffer_saved: bool,
  pub expected_selection: (usize, usize),
  pub expected_clipboard: Vec<&'static str>,
  pub expected_prints: Vec<Print>,
  pub expected_history_tags: Vec<&'static str>,
}
impl PrintTest {
  pub fn run(self) {
    inner_fixture(
      self.init_clipboard,
      self.init_buffer,
      true,
      "path",
      None,
      self.command_input,
      Ok(()),
      self.expected_buffer,
      self.expected_buffer_saved,
      self.expected_history_tags,
      self.expected_selection,
      self.expected_clipboard,
      "path",
      self.expected_prints,
    )
  }
}

// A test fixture for verifying filename changes, essentially just for 'f'
pub struct PathTest {
  pub init_filepath: &'static str,
  pub command_input: Vec<&'static str>,
  pub expected_filepath: &'static str,
  pub expected_history_tags: Vec<&'static str>,
}
impl PathTest {
  pub fn run(self) {
    inner_fixture(
      vec![],
      vec![],
      true,
      self.init_filepath,
      None,
      self.command_input,
      Ok(()),
      vec![],
      true,
      self.expected_history_tags,
      (1,0),
      vec![],
      self.expected_filepath,
      vec![],
    )
  }
}

// A test fixture that simulates and verifies IO interactions
// Sets up state as though reading buffer contents from a file and runs the
// given commands via dummy_ui. (Selection is Ed default, buffer.saved is true)
// Afterwards verifies state against optional expectations on
// buffer contents, fake filesystem state and selection.
// Note that expected file changes is a Vec<(filename, new_contents)>.
// Terminating '\n' aren't needed nor allowed in any of the Vec<&str> arguments.
pub struct IOTest {
  pub init_buffer: Vec<&'static str>,
  pub init_clipboard: Vec<&'static str>,
  pub init_io: FakeIO,
  pub init_filepath: &'static str,
  pub command_input: Vec<&'static str>,
  pub expected_buffer: Vec<&'static str>,
  pub expected_buffer_saved: bool,
  pub expected_clipboard: Vec<&'static str>,
  pub expected_selection: (usize, usize),
  pub expected_file_changes: Vec<(&'static str, &'static str)>,
  pub expected_filepath: &'static str,
}
impl IOTest {
  pub fn run(mut self) {
    // Create and init ed state
    let macros = HashMap::<String, Macro>::new();
    let mut ed = Ed::new(
      &mut self.init_io,
      &macros,
    );
    ed.file = self.init_filepath.to_owned();
    let init_clipboard = self.init_clipboard.iter().fold(Clipboard::new(), |mut c, x| {
      c.push(PubLine{
        tag: '\0',
        text: LineText::new(format!("{}\n", x)).unwrap(),
      });
      c
    });
    ed.clipboard = init_clipboard;
    let init_buffer = self.init_buffer.iter().fold(Clipboard::new(), |mut c, x| {
      c.push(PubLine{
        tag: '\0',
        text: LineText::new(format!("{}\n", x)).unwrap(),
      });
      c
    });
    ed.history.current_mut("initial load".into()).append(&mut (&init_buffer).try_into().unwrap());
    ed.history.set_saved();
    // Create scripted UI (with mock UI, which tracks print invocations)
    let mut inner_ui = MockUI{ prints_history: Vec::new() };
    let mut ui = ScriptedUI{
      print_ui: Some(&mut inner_ui),
      // For each element convert to String & add newline, collect into VecDeque
      input: self.command_input.iter().map(|x|{
        let mut s = x.to_string();
        s.push('\n');
        s
      }).collect(),
    };

    // Run test
    ed.selection = (1,ed.history.current().len());
    loop {
      if ed.get_and_run_command(&mut ui).expect("Error running test.") { break; }
    }

    // Verify state after test execution
    assert_eq!(
      ed.selection,
      self.expected_selection,
      "Selection after test (left) didn't match expectations (right)."
    );
    assert_eq!(
      ed.file,
      self.expected_filepath,
      "Filepath after test (left) didn't match expectations (right)."
    );
    assert_eq!(
      ed.history.saved(),
      self.expected_buffer_saved,
      "Buffer.saved() (left) after test didn't match expectations (right)."
    );
    assert_eq!(
      ed.history.current().iter()
        .map(|l| l.text.trim_end_matches('\n'))
        .collect::<Vec<&str>>()
      ,
      self.expected_buffer,
      "Buffer contents (left) after test didn't match expectations (right)."
    );
    assert_eq!(
      ed.clipboard[..].iter()
        .map(|l| l.text.trim_end_matches('\n'))
        .collect::<Vec<&str>>()
      ,
      self.expected_clipboard,
      "Cliboard contents (left) after test didn't match expectations (right)."
    );
    let mut expected_post_state = self.init_io.clone();
    for (file, new_contents) in self.expected_file_changes {
      expected_post_state.fake_fs.insert(file.to_owned(), new_contents.to_owned());
    }
    assert_eq!(
      &self.init_io.fake_fs,
      &expected_post_state.fake_fs,
      "Filesystem state after test (left) didn't match expectations (right)."
    );
  }
}
