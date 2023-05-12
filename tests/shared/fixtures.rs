#![allow(dead_code)]

use super::{
  inner_fixture,
  fake_io::FakeIO,
  mock_ui::{Print, MockUI},
};
use add_ed::{
  ui::ScriptedUI,
  Ed,
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
}
impl BasicTest {
  pub fn run(self) {
    inner_fixture(
      self.init_clipboard,
      self.init_buffer,
      true,
      "path",
      self.command_input,
      Ok(()),
      self.expected_buffer,
      self.expected_buffer_saved,
      self.expected_selection,
      self.expected_clipboard,
      "path",
      vec![], // No prints expected
    )
  }
}

// An error checking fixture
// Sets up state as though reading buffer contents from a file and runs the
// given commands via dummy_ui. (Selection is Ed default, buffer.saved is true)
// Afterwards verifies that state hasn't changed and error matches expectations.
// Panics if any command tries to print, use PrintTest if this isn't desired.
// Terminating '\n' aren't needed nor allowed in any of the Vec<&str> arguments.
pub struct ErrorTest {
  pub init_buffer: Vec<&'static str>,
  pub command_input: Vec<&'static str>,
  pub expected_error: &'static str,
}
impl ErrorTest {
  pub fn run(self) {
    let init_buffer_len = self.init_buffer.len();
    inner_fixture(
      vec![],
      self.init_buffer.clone(),
      false, // Since some errors need the buffer unsaved to trigger
      "path",
      self.command_input,
      Err(self.expected_error),
      self.init_buffer,
      false,
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
}
impl PrintTest {
  pub fn run(self) {
    inner_fixture(
      self.init_clipboard,
      self.init_buffer,
      true,
      "path",
      self.command_input,
      Ok(()),
      self.expected_buffer,
      self.expected_buffer_saved,
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
}
impl PathTest {
  pub fn run(self) {
    inner_fixture(
      vec![],
      vec![],
      true,
      self.init_filepath,
      self.command_input,
      Ok(()),
      vec![],
      true,
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
    let mut ed = Ed::new(
      &mut self.init_io,
      self.init_filepath.to_owned(),
    );
    let init_clipboard: Vec<String> = self.init_clipboard.iter()
      .map(|x| x.to_string())
      .collect()
    ;
    let init_buffer: Vec<String> = self.init_buffer.iter().map(|x| {
      let mut s = x.to_string();
      s.push('\n');
      s
    }).collect();
    ed.buffer.insert(init_clipboard, 0).unwrap();
    ed.buffer.cut((1,ed.buffer.len())).unwrap();
    ed.buffer.insert(init_buffer, 0).unwrap();
    ed.buffer.set_saved();
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
    ed.selection = (1,ed.buffer.len());
    ed.run_macro(&mut ui).expect("Error running test.");

    // Verify state after test execution
    assert_eq!(
      ed.see_state().selection,
      self.expected_selection,
      "Selection after test (left) didn't match expectations (right)."
    );
    assert_eq!(
      ed.see_state().file,
      self.expected_filepath,
      "Filepath after test (left) didn't match expectations (right)."
    );
    assert_eq!(
    ed.buffer.saved(),
      self.expected_buffer_saved,
      "Buffer.saved() (left) after test didn't match expectations (right)."
    );
    assert_eq!(
      if ed.buffer.len() != 0 {
        ed.buffer.get_selection((1,ed.buffer.len()))
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
    let end_of_buf = ed.buffer.len();
    ed.buffer.paste(end_of_buf).unwrap();
    if end_of_buf != 0 { ed.buffer.cut((1,end_of_buf)).unwrap(); }
    assert_eq!(
      if ed.buffer.len() != 0 {
        ed.buffer.get_selection((1,ed.buffer.len()))
          .unwrap()
          .map(|(_,s)| s.trim_end_matches('\n'))
          .collect::<Vec<&str>>()
      } else {
        vec![]
      },
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
