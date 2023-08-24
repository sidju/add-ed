use super::*;

pub fn tag(
  state: &mut Ed<'_>,
  selection: Option<Sel<'_>>,
  command: char,
  tail: &str,
) -> Result<()> {
  let selection = interpret_selection(&state, selection, state.selection)?;
  let index = if command == 'k' { selection.0 } else { selection.1 };
  let buffer = state.history.current();
  buffer.verify_line(index)?;
  // we only expect the tag, no flags
  if tail.chars().count() > 1 {
    return Err(EdError::TagInvalid(tail.to_owned()));
  }
  buffer[index - 1].tag.set(tail.chars().next().unwrap_or('\0').into());
  Ok(())
}
