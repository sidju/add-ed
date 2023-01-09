use add_ed::{
  io::IO,
  ui::UILock,
};

pub struct DummyIO {}
impl DummyIO {
  pub fn new() -> Self { Self{} }
}

impl IO for DummyIO {
  fn run_command(&mut self,
    _ui: &mut UILock,
    _command: String,
  ) -> Result<(), &'static str> {
    unimplemented!()
  }
  fn run_read_command(&mut self,
    _ui: &mut UILock,
    _command: String,
  ) -> Result<String, &'static str> {
    unimplemented!()
  }
  fn run_write_command<'a>(&mut self,
    _ui: &mut UILock,
    _command: String,
    _input: impl Iterator<Item = &'a str>,
  ) -> Result<usize, &'static str> {
    unimplemented!()
  }
  fn run_transform_command<'a>(&mut self,
    _ui: &mut UILock,
    _command: String,
    _input: impl Iterator<Item = &'a str>,
  ) -> Result<String, &'static str> {
    unimplemented!()
  }
  fn write_file<'a>(&mut self,
    _path: &str,
    _append: bool,
    _data: impl Iterator<Item = &'a str>,
  ) -> Result<usize, &'static str> {
    unimplemented!()
  }
  fn read_file(&mut self,
    _path: &str,
    _must_exist: bool,
  ) -> Result<String, &'static str> {
    unimplemented!()
  }
}
