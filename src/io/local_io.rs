// This module currently forwards current process' pty into child command.
// If this approach doesn't work for you, create an issue. I have some ideas for
// solutions that I could put behind a feature if I get some API design help.

pub struct LocalIO {
}
impl LocalIO {
  fn write_internal(
    path: &str,
    append: bool,
    data: impl Iterator<Item = &str>,
  ) -> std::io::Result<()> {
    use std::io::Write;
    let file = std::fs::OpenOptions::new()
      .write(true)
      .append(append)
      .truncate(!append)
      .create(true)
      .open(path)
    ?;
    for line in data {
      file.write_all(line.as_bytes())?;
    }
    file.flush()?;
    Ok(())
  }
}

impl IO for LocalIO {
  fn run_command(
    _ui: &mut dyn UIHandle,
    command: String,
  ) -> Result<(), &'static str> {
    todo!()
  }

  fn run_read_command(
  ) -> Result<String, &'static str> {
    todo!()
  }

  fn run_write_command(
  ) -> Result<(), &'static str> {
    todo!()
  }

  fn run_transform_command(
  ) -> Result<(), &'static str> {
    todo!()
  }

  fn write_file(
    path: &str,
    append: bool,
    data: impl Iterator<Item = &str>,
  ) -> Result<(), &'static str> {
    write_internal(path, append, data)
      .map_err(|e: std::io::Error| match e.kind() {
        ErrorKind::PermissionDenied => PERMISSION_DENIED,
        ErrorKind::NotFound => NOT_FOUND,
        _ => UNKNOWN,
      })
  }

  fn read_file(
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
    })
  }
}
