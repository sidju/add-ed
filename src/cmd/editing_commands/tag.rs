use super::*;

pub fn tag(
  state: &mut Ed<'_>,
  selection: Option<Sel<'_>>,
  command: char,
  tail: &str,
) -> Result<()> {
  let selection = interpret_selection(state, selection, state.selection)?;
  let buffer = state.history.current();
  buffer.verify_selection(selection)?;
  // we only expect the tag, no flags
  if tail.chars().count() > 1 {
    return Err(EdError::TagInvalid(tail.to_owned()));
  }
  let ch = tail.chars().next().unwrap_or('\0');
  if command == 'k' {
    // Set end first, since selection .0 and .1 may be same index
    buffer[selection.1 - 1].set_tag(ch.into());
    buffer[selection.0 - 1].set_tag(ch.into());
  } else {
    buffer[selection.1 - 1].set_tag(ch.into());
  }
  Ok(())
}
