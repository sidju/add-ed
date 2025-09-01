use crate::{
  io::IO,
  ui::UILock,
  buffer::iters::LinesIter,
};
use super::Result;

#[derive(Debug, PartialEq)]
pub enum FakeIOError {
  ChildExitError,
  NotFound,
  Overwrite,
}
impl std::fmt::Display for FakeIOError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use FakeIOError as FIE;
    match self {
      FIE::ChildExitError => write!(f,"Child process returned error after running."),
      FIE::NotFound => write!(f,"Could not open file. Not found or invalid path."),
      FIE::Overwrite => write!(f,"Will not overwrite existing file.")
    }
  }
}
impl std::error::Error for FakeIOError {}
impl crate::error::IOErrorTrait for FakeIOError {}

use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ShellCommand {
  pub command: String,
  pub input: String,
}

/// An [`IO`] implementation intended to simulate filesystem and shell
/// interactions for testing.
#[derive(Clone)]
pub struct FakeIO {
  pub fake_fs: HashMap<String, String>,
  pub fake_shell: HashMap<ShellCommand, String>,
}

impl IO for FakeIO {
  /// Returns [`FakeIOError::ChildExitError`] if command is not represented by a
  /// [`ShellCommand`] with empty input in `fake_shell`.
  fn run_command(&mut self,
    _ui: &mut UILock,
    command: String,
  ) -> Result<()> {
    if self.fake_shell.contains_key(
      &ShellCommand{command, input: String::new()}
    ) {
      Ok(())
    } else {
      // sh is child and returns error on command not found
      Err(FakeIOError::ChildExitError.into())
    }
  }
  /// Returns [`FakeIOError::ChildExitError`] if command is not represented by a
  /// [`ShellCommand`] with empty input in `fake_shell`.
  fn run_read_command(&mut self,
    _ui: &mut UILock,
    command: String,
  ) -> Result<String> {
    match self.fake_shell.get(
      &ShellCommand{command, input: String::new()}
    ) {
      Some(x) => Ok(x.to_owned()),
      // sh is child and returns error on command not found
      None => Err(FakeIOError::ChildExitError.into()),
    }
  }
  /// Returns [`FakeIOError::ChildExitError`] if command is not represented by a
  /// [`ShellCommand`] with the given input in `fake_shell`.
  fn run_write_command(&mut self,
    _ui: &mut UILock,
    command: String,
    input: LinesIter,
  ) -> Result<usize> {
    let input = input.fold(String::new(), |mut s, x| {s.push_str(x); s});
    let inputlen = input.len();
    match self.fake_shell.get(
      &ShellCommand{command, input}
    ) {
      Some(_) => Ok(inputlen),
      // sh is child and returns error on command not found
      None => Err(FakeIOError::ChildExitError.into()),
    }
  }
  /// Returns [`FakeIOError::ChildExitError`] if command is not represented by a
  /// [`ShellCommand`] with the given input in `fake_shell`.
  fn run_transform_command(&mut self,
    _ui: &mut UILock,
    command: String,
    input: LinesIter,
  ) -> Result<String> {
    let input = input.fold(String::new(), |mut s, x| {s.push_str(x); s});
    match self.fake_shell.get(
      &ShellCommand{command, input}
    ) {
      Some(x) => Ok(x.to_owned()),
      // sh is child and returns error on command not found
      None => Err(FakeIOError::ChildExitError.into()),
    }
  }
  fn write_file(&mut self,
    path: &str,
    append: bool,
    overwrite: bool,
    data: LinesIter,
  ) -> Result<usize> {
    let base_data = match self.fake_fs.get(path) {
      Some(_) if overwrite => { return Err(FakeIOError::Overwrite.into()); },
      Some(x) if append => x.clone(),
      _ => String::new(),
    };
    let data = data.fold(base_data, |mut s, x|{s.push_str(x); s});
    let datalen = data.len();
    self.fake_fs.insert(path.to_owned(), data);
    Ok(datalen)
  }
  /// Returns [`FakeIOError::NotFound`] if `fake_fs` doesn't have an entry with
  /// the given path as key.
  fn read_file(&mut self,
    path: &str,
    must_exist: bool,
  ) -> Result<String> {
    match self.fake_fs.get(path) {
      Some(x) => Ok(x.to_owned()),
      None => if must_exist {
        Err(FakeIOError::NotFound.into())
      } else {
        Ok(String::new())
      },
    }
  }
}
