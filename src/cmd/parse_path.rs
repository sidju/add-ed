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
  else {
    if trimmed.chars().next() == Some('!') {
      Some(Path::Command(&trimmed[1..])) // Cut away the exclamation mark
    }
    else {
      Some(Path::File(trimmed))
    }
  }
}
