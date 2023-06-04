use crate::{
  EdState,
  ui::{UI,UILock},
};

use super::Result;

/// Dummy UI for testing
/// Panics on any UI interaction but supports no-op locking and unlocking
pub struct DummyUI {
}
impl UI for DummyUI {
  fn print_message(&mut self,
    _data: &str,
  ) -> Result<()> {
    unimplemented!()
  }
  fn get_command(&mut self,
    _ed: EdState,
    _prefix: Option<char>,
  ) -> Result<String> {
    unimplemented!()
  }
  fn get_input(&mut self,
    _ed: EdState,
    _terminator: char,
    #[cfg(feature = "initial_input_data")]
    _initial_buffer: Option<Vec<String>>,
  ) -> Result<Vec<String>> {
    unimplemented!()
  }
  fn print_selection(&mut self,
    _ed: EdState,
    _selection: (usize, usize),
    _numbered: bool,
    _literal: bool,
  ) -> Result<()> {
    unimplemented!()
  }
  fn lock_ui(&mut self) -> UILock<'_> {
    UILock::new(self)
  }
  fn unlock_ui(&mut self) {
  }
}
