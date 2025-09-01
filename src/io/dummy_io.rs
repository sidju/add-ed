use crate::{
  io::IO,
  ui::UILock,
  buffer::iters::LinesIter,
};

use super::Result;

/// An [`IO`] implementation for when no IO should occur. Intended for non IO
/// tests.
pub struct DummyIO {}
impl DummyIO {
  pub fn new() -> Self { Self{} }
}

impl IO for DummyIO {
  fn run_command(&mut self,
    _ui: &mut UILock,
    _command: String,
  ) -> Result<()> {
    unimplemented!()
  }
  fn run_read_command(&mut self,
    _ui: &mut UILock,
    _command: String,
  ) -> Result<String> {
    unimplemented!()
  }
  fn run_write_command(&mut self,
    _ui: &mut UILock,
    _command: String,
    _input: LinesIter,
  ) -> Result<usize> {
    unimplemented!()
  }
  fn run_transform_command(&mut self,
    _ui: &mut UILock,
    _command: String,
    _input: LinesIter,
  ) -> Result<String> {
    unimplemented!()
  }
  fn write_file(&mut self,
    _path: &str,
    _append: bool,
    _overwrite: bool,
    _data: LinesIter,
  ) -> Result<usize> {
    unimplemented!()
  }
  fn read_file(&mut self,
    _path: &str,
    _must_exist: bool,
  ) -> Result<String> {
    unimplemented!()
  }
}
