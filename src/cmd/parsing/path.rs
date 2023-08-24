use crate::error::*;

pub enum Path<'a> {
  File(&'a str),
  Command(&'a str),
}

// Returns filepath or command if any given, else none
pub fn parse_path(input: &str)
  -> Option<Path<'_>>
{
  let trimmed = input.trim_start();
  if trimmed.is_empty() {
    None
  }
  else if let Some(stripped) = trimmed.strip_prefix('!') {
    Some(Path::Command(stripped))
  }
  else {
    Some(Path::File(trimmed))
  }
}

pub fn command_substitutions(
  command: &str,
  state_file: &str,
  prev_command: &str,
) -> Result<(bool, String)> {
  // In command we replace ! with previous command and % with state.file.
  // To not clash with other escape processing we only handle \% and \!,
  // for every other case we print the escaping \ and the escaped char.
  let mut escaped = false;
  let mut modified = false;
  let mut output = String::new();
  for ch in command.chars() {
    match ch {
      '\\' => if escaped {
        output.push_str("\\\\");
        escaped = false;
      } else {
        escaped = true;
      },
      '%' => if escaped {
        output.push(ch);
        escaped = false;
      } else {
        modified = true;
        if state_file.is_empty() { return Err(EdError::DefaultFileUnset); }
        output.push_str(state_file);
      },
      '!' => if escaped {
        output.push(ch);
        escaped = false;
      } else {
        modified = true;
        if prev_command.is_empty() {
          return Err(EdError::DefaultShellCommandUnset);
        }
        output.push_str(prev_command);
      },
      _ => {
        if escaped {
          output.push('\\');
          escaped = false;
        }
        output.push(ch);
      },
    }
  }
  Ok((modified, output))
}
#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_command_escapes() {
    assert_eq!(
      &command_substitutions(
        "%",
        "state.file",
        "prev_command",
      ).unwrap().1,
      "state.file",
      "command_substitutions didn't replace % with data from state_file."
    );
    assert_eq!(
      &command_substitutions(
        "\\%",
        "state.file",
        "prev_command",
      ).unwrap().1,
      "%",
      "command_substitutions didn't respect escape on %."
    );
    assert_eq!(
      &command_substitutions(
        "!",
        "state.file",
        "prev_command",
      ).unwrap().1,
      "prev_command",
      "command_substitutions didn't replace ! with data from prev_command."
    );
    assert_eq!(
      &command_substitutions(
        "\\!",
        "state.file",
        "prev_command",
      ).unwrap().1,
      "!",
      "command_substitutions didn't respect escape on !."
    );
    assert_eq!(
      &command_substitutions(
        "\\\\",
        "state.file",
        "prev_command",
      ).unwrap().1,
      "\\\\",
      "command_substitution handled escape on \\, it should be passed through."
    );
  }
}
