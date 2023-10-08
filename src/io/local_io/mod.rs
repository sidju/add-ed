// This module currently forwards current process' pty into child command.
// If this approach doesn't work for you, create an issue. I have some ideas for
// solutions that I could put behind a feature if I get some API design help.

use std::process::{
  Command,
  Stdio,
};
use crate::IO;
use super::LinesIter;
use crate::UILock;
use super::Result;


mod error;
pub use error::LocalIOError;

#[cfg(all(feature = "test_local_io", test))]
mod test;

fn spawn_transfer<'a, I, O>(
  i: I,
  mut o: O,
) -> std::thread::JoinHandle<usize> where
  I: Iterator<Item = &'a str>,
  O: std::io::Write + std::marker::Send + 'static,
{
  let aggregated_input = i.fold(String::new(),|mut s, a| {s.push_str(a); s});
  std::thread::spawn(move || {
    let inputlen = aggregated_input.len();
    o.write_all(aggregated_input.as_bytes()).expect("Pipe forwarding failed.");
    inputlen
  })
}

/// Filesystem and process tree local [`IO`] implementation.
#[non_exhaustive]
pub struct LocalIO {
}
impl LocalIO {
  /// Construct LocalIO instance
  ///
  /// Currently there are no internal members in [`LocalIO`], but for stability
  /// into the future I'd still recommend using this method in case a need for
  /// state is found.
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
impl Default for LocalIO {
  fn default() -> Self {
    Self::new()
  }
}

impl IO for LocalIO {
  fn run_command(&mut self,
    _ui: &mut UILock,
    command: String,
  ) -> Result<()> {
    let shell = std::env::var("SHELL").unwrap_or("sh".to_owned());
    // Create and run child process, passing through all io
    let res = Command::new(shell)
      .arg("-c")
      .arg(command)
      .spawn() // When spawn io defaults to inherited
      .map_err(LocalIOError::ChildCreationFailed)?
      .wait()
      .map_err(LocalIOError::ChildFailedToStart)?
    ;
    if !(res.success()) {
      return Err(LocalIOError::child_return_res(res.code()).into());
    }
    Ok(())
  }

  fn run_read_command(&mut self,
    _ui: &mut UILock,
    command: String,
  ) -> Result<String> {
    let shell = std::env::var("SHELL").unwrap_or("sh".to_owned());
    // Create child process
    let child = Command::new(shell)
      .arg("-c")
      .arg(command)
      .stdout(Stdio::piped())
      .spawn()
      .map_err(LocalIOError::ChildCreationFailed)?
    ;
    // Blocks until child has finished running
    let res = child.wait_with_output()
      .map_err(LocalIOError::ChildFailedToStart)
    ?;
    if !(res.status.success()) {
      return Err(LocalIOError::child_return_res(res.status.code()).into());
    }
    let output = String::from_utf8(res.stdout)
      .map_err(LocalIOError::BadUtf8)?
    ;
    Ok(output)
  }

  fn run_write_command(&mut self,
    _ui: &mut UILock,
    command: String,
    input: LinesIter,
  ) -> Result<usize> {
    let shell = std::env::var("SHELL").unwrap_or("sh".to_owned());
    // Create child process
    let mut child = Command::new(shell)
      .arg("-c")
      .arg(command)
      .stdin(Stdio::piped())
      .spawn()
      .map_err(LocalIOError::ChildCreationFailed)?
    ;
    let i = spawn_transfer(
      input,
      child.stdin.take().unwrap(),
    );
    // Blocks until child has finished running
    let res = child.wait()
      .map_err(LocalIOError::ChildFailedToStart)
    ;
    // Wait for the other child thread before triggering early returns with ?
    let transfer_res = i.join().map_err(|_|LocalIOError::ChildPipingError)?;
    let res = res?;
    if !(res.success()) {
      return Err(LocalIOError::child_return_res(res.code()).into());
    }
    Ok(transfer_res)
  }

  fn run_transform_command(&mut self,
    _ui: &mut UILock,
    command: String,
    input: LinesIter,
  ) -> Result<String> {
    let shell = std::env::var("SHELL").unwrap_or("sh".to_owned());
    // Create child process
    let mut child = Command::new(shell)
      .arg("-c")
      .arg(command)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn()
      .map_err(LocalIOError::ChildCreationFailed)?
    ;
    let i = spawn_transfer(
      input,
      child.stdin.take().unwrap(),
    );
    // Blocks until child has finished running
    let res = child.wait_with_output()
      .map_err(LocalIOError::ChildFailedToStart)
    ;
    // Wait for the other child thread before triggering early returns with ?
    let _transfer_res = i.join().map_err(|_|LocalIOError::ChildPipingError)?;
    let res = res?;
    if !(res.status.success()) {
      return Err(LocalIOError::child_return_res(res.status.code()).into());
    }
    let output = String::from_utf8_lossy(&res.stdout).into_owned();
    Ok(output)
  }

  fn write_file(&mut self,
    path: &str,
    append: bool,
    data: LinesIter,
  ) -> Result<usize> {
    Self::write_internal(path, append, data)
      .map_err(|e| LocalIOError::file_error(path, e).into())
  }
  fn read_file(&mut self,
    path: &str,
    must_exist: bool,
  ) -> Result<String> {
    match std::fs::read_to_string(path)
      .map_err(|e| LocalIOError::file_error(path, e))
    {
      Ok(data) => Ok(data),
      Err(e) => match e {
        LocalIOError::FileNotFound{..} => {
          if must_exist { Err(e.into()) } else { Ok(String::new()) }
        },
        _ => Err(e.into()),
      },
    }
  }
}
