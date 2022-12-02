// This module currently forwards current process' pty into child command.
// If this approach doesn't work for you, create an issue. I have some ideas for
// solutions that I could put behind a feature if I get some API design help.

use std::process::Child;
use crate::IO;
use crate::UILock;
use crate::error_consts::*;

pub struct LocalIO {
}
impl LocalIO {
  pub fn new() -> Self {
    Self{}
  }
  fn write_internal<'a>(
    path: &str,
    append: bool,
    data: impl Iterator<Item = &'a str>,
  ) -> std::io::Result<usize> {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
      .write(true)
      .append(append)
      .truncate(!append)
      .create(true)
      .open(path)
    ?;
    let mut written = 0;
    for line in data {
      written += line.len();
      file.write_all(line.as_bytes())?;
    }
    file.flush()?;
    Ok(written)
  }
}

impl IO for LocalIO {
  fn run_command(&mut self,
    _ui: &mut UILock,
    command: String,
  ) -> Result<(), &'static str> {
    todo!()
  }

  fn run_read_command(&mut self,
    ui: &mut UILock,
    command: String,
  ) -> Result<String, &'static str> {
    todo!()
  }

  fn run_write_command<'a>(&mut self,
    ui: &mut UILock,
    command: String,
    input: impl Iterator<Item = &'a str>,
  ) -> Result<usize, &'static str> {
    todo!()
  }

  fn run_transform_command<'a>(&mut self,
    ui: &mut UILock,
    command: String,
    input: impl Iterator<Item = &'a str>,
  ) -> Result<String, &'static str> {
    todo!()
  }

  fn write_file<'a>(&mut self,
    path: &str,
    append: bool,
    data: impl Iterator<Item = &'a str>,
  ) -> Result<usize, &'static str> {
    use std::io::ErrorKind;
    Self::write_internal(path, append, data)
      .map_err(|e: std::io::Error| match e.kind() {
        ErrorKind::PermissionDenied => PERMISSION_DENIED,
        ErrorKind::NotFound => NOT_FOUND,
        _ => UNKNOWN,
      })
  }
  fn read_file(&mut self,
    path: &str,
    must_exist: bool,
  ) -> Result<String, &'static str> {
    use std::io::ErrorKind;
    match std::fs::read_to_string(path) {
      Ok(data) => Ok(data),
      Err(e) => match e.kind() {
        ErrorKind::PermissionDenied => Err(PERMISSION_DENIED),
        ErrorKind::NotFound => {
          if must_exist { Err(NOT_FOUND) } else { Ok(String::new()) }
        },
        _ => Err(UNKNOWN),
      },
    }
  }
}
