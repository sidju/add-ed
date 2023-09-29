use super::*;

fn inner_join(
  state: &mut Ed<'_>,
  full_command: &str,
  selection: (usize, usize),
) -> Result<()> {
  let buffer = state.history.current_mut(full_command.into())?;
  // Take out lines
  let mut tail = buffer.split_off(selection.1);
  let data = buffer.split_off(selection.0 - 1);
  // Construct the joined text
  let text = data.iter()
    .fold(String::new(), |mut s, n| {
      s.pop(); // Remove trailing newline (ignored on empty string)
      s.push_str(&n.text[..]);
      s
    })
  ;
  // Insert it into buffer
  buffer.push(Line::new(text).map_err(InternalError::InvalidLineText)?);
  // Add back tail data and save old data into clipboard
  buffer.append(&mut tail);
  state.clipboard = data[..].into();
  Ok(())
}
pub fn join(
  state: &mut Ed<'_>,
  pflags: &mut PrintingFlags,
  full_command: &str,
  selection: Option<Sel<'_>>,
  tail: &str,
) -> Result<()> {
  let selection = interpret_selection(&state, selection, state.selection)?;
  state.history.current().verify_selection(selection)?; // Verify without creating snapshot
  let mut flags = parse_flags(tail, "pnl")?;
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  inner_join(state, full_command, selection)?;
  state.selection = (selection.0, selection.0);
  Ok(())
}
