use add_ed::{
  EdState,
  ui::{UI,UILock},
};

/// Dummy UI for testing
/// Panics on any UI interaction but supports no-op locking and unlocking
pub struct DummyUI {
}
impl UI for DummyUI {
  fn print_message(&mut self,
    _data: &str,
  ) -> Result<(), &'static str> {
    unimplemented!()
  }
  fn get_command(&mut self,
    _ed: EdState,
    _prefix: Option<char>,
  ) -> Result<String, &'static str> {
    unimplemented!()
  }
  fn get_input(&mut self,
    _ed: EdState,
    _terminator: char,
    #[cfg(feature = "initial_input_data")]
    _initial_buffer: Option<Vec<String>>,
  ) -> Result<Vec<String>, &'static str> {
    unimplemented!()
  }
  fn print_selection(&mut self,
    _ed: EdState,
    _selection: (usize, usize),
    _numbered: bool,
    _literal: bool,
  ) -> Result<(), &'static str> {
    unimplemented!()
  }
  fn lock_ui(&mut self) -> UILock<'_> {
    UILock::new(self)
  }
  fn unlock_ui(&mut self) {
  }
}
