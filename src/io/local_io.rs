// This module currently forwards current process' pty into child command.
// If this approach doesn't work for you, create an issue. I have some ideas for
// solutions that I could put behind a feature if I get some API design help.

use std::process::{
  Command,
  Stdio,
};
use crate::IO;
use crate::UILock;
use crate::error_consts::*;

fn spawn_transfer<'a, O>(
  i: Vec<String>,
  mut o: O,
) -> std::thread::JoinHandle<()> where
  O: std::io::Write + std::marker::Send + 'static,
{
  std::thread::spawn(move || {
    use std::io::{Read, Write, copy};
    for line in i {
      o.write_all(line.as_bytes()).expect("Pipe forwarding failed.");
    }
  })
}
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
    let shell = std::env::var("SHELL").unwrap_or("sh".to_owned());
    // Create and run child process, passing through all io
    let _child_result = Command::new(shell)
      .arg("-c")
      .arg(command)
      .spawn() // When spawn io defaults to inherited
      .map_err(|_| "Failed to spawn child process.")?
      .wait()
      .map_err(|_| "Child process failed to start.")?
    ;
    Ok(())
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
    let shell = std::env::var("SHELL").unwrap_or("sh".to_owned());
    // Create child process
    let mut child = Command::new(shell)
      .arg("-c")
      .arg(command)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .map_err(|_| CHILD_CREATION_FAILED)?
    ;
    let input_vec: Vec<String> = input.map(|s| s.to_owned()).collect();
    let i = spawn_transfer(
      input_vec,
      child.stdin.take().unwrap(),
    );
    // Blocks until child has finished running
    let res = child.wait_with_output()
      .map_err(|_| CHILD_FAILED_TO_START)?
    ;
    if !(res.status.success()) {
      return Err(CHILD_EXIT_ERROR)
    }
    let output = String::from_utf8_lossy(&res.stdout).into_owned();
    Ok(output)
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
