use crate::{
  Ed,
  ui::{UI,UILock},
};

use super::Result;

/// Dummy UI for testing
/// Returns Ok on any operation (with empty data if required) and supports no-op
/// locking and unlocking.
pub struct DummyUI {
}
impl UI for DummyUI {
  fn print_message(&mut self,
    _data: &str,
  ) -> Result<()> {
    Ok(())
  }
  fn get_command(&mut self,
    _ed: &Ed,
    _prefix: Option<char>,
  ) -> Result<String> {
    Ok(String::new())
  }
  fn get_input(&mut self,
    _ed: &Ed,
    _terminator: char,
    #[cfg(feature = "initial_input_data")]
    _initial_buffer: Option<Vec<String>>,
  ) -> Result<Vec<String>> {
    Ok(Vec::new())
  }
  fn print_selection(&mut self,
    _ed: &Ed,
    _selection: (usize, usize),
    _numbered: bool,
    _literal: bool,
  ) -> Result<()> {
    Ok(())
  }
  fn lock_ui(&mut self) -> UILock<'_> {
    UILock::new(self)
  }
  fn unlock_ui(&mut self) {
  }
}
