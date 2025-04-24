use super::*;

fn inner_paste(
  state: &mut Ed<'_>,
  full_command: &str,
  index: usize,
) -> Result<usize> {
  state.history.current().verify_index(index)?;
  let buffer = state.history.current_mut(full_command.into());
  let mut tail = buffer.split_off(index);
  buffer.append(&mut (&state.clipboard).into() );
  buffer.append(&mut tail);
  Ok(state.clipboard.len())
}
pub fn paste(
  state: &mut Ed<'_>,
  pflags: &mut PrintingFlags,
  full_command: &str,
  selection: Option<Sel<'_>>,
  command: char,
  tail: &str,
) -> Result<()> {
  let mut flags = parse_flags(tail, "pnl")?;
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  // Append or prepend based on command
  let mut index = interpret_index_from_selection(&state, selection, state.selection, command == 'x')?;
  if command == 'X' { index = index.saturating_sub(1); }
  let length = inner_paste(state, full_command, index)?;
  if length != 0 {
    state.selection = (index + 1, index + length);
  }
  Ok(())
}
