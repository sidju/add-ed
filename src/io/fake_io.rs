use crate::{
  io::IO,
  ui::UILock,
};
use super::Result;

#[derive(Debug, PartialEq)]
pub enum FakeIOError {
  ChildExitError,
  NotFound,
}
impl std::fmt::Display for FakeIOError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use FakeIOError::*;
    match self {
      ChildExitError => write!(f,"Child process returned error after running."),
      NotFound => write!(f,"Could not open file. Not found or invalid path."),
    }
  }
}
impl std::error::Error for FakeIOError {}
impl crate::error::IOError for FakeIOError {}

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
  fn run_write_command<'a>(&mut self,
    _ui: &mut UILock,
    command: String,
    input: impl Iterator<Item = &'a str>,
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
  fn run_transform_command<'a>(&mut self,
    _ui: &mut UILock,
    command: String,
    input: impl Iterator<Item = &'a str>,
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
  fn write_file<'a>(&mut self,
    path: &str,
    append: bool,
    data: impl Iterator<Item = &'a str>,
  ) -> Result<usize> {
    let base_data = if append {
      match self.fake_fs.get(path) {
        Some(x) => x.clone(),
        None => String::new(),
      }
    } else {
      String::new()
    };
    let data = data.fold(base_data, |mut s, x|{s.push_str(x); s});
    let datalen = data.len();
    self.fake_fs.insert(path.to_owned(), data);
    Ok(datalen)
  }
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
