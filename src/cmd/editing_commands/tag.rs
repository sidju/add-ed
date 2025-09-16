use super::*;

pub fn tag(
  state: &mut Ed<'_>,
  selection: Option<Sel<'_>>,
  command: char,
  tail: &str,
) -> Result<()> {
  let selection = interpret_selection(&state, selection, state.selection)?;
  let buffer = state.history.current();
  buffer.verify_selection(selection)?;
  // we only expect the tag, no flags
  if tail.chars().count() > 1 {
    return Err(EdError::TagInvalid(tail.to_owned()));
  }
  let tag = tail.chars().next();
  match tag {
    Some(ch) => {
      if command == 'k' {
        // Set end first, since selection .0 and .1 may be same index
        buffer[selection.1 - 1].set_tag(Tag::End(ch).into());
        buffer[selection.0 - 1].set_tag(Tag::Start(ch).into());
      } else {
        buffer[selection.1 - 1].set_tag(Tag::Start(ch).into());
      }
    },
    None => {
      buffer[selection.1 - 1].set_tag(Tag::None.into());
      buffer[selection.0 - 1].set_tag(Tag::None.into());
    },
  }
  Ok(())
}
