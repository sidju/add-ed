use super::*;

pub fn cut(
  state: &mut Ed<'_>,
  pflags: &mut PrintingFlags,
  full_command: &str,
  selection: Option<Sel<'_>>,
  tail: &str,
) -> Result<()> {
  let sel = interpret_selection(&state, selection, state.selection)?;
  let buffer = state.history.current();
  buffer.verify_selection(sel)?;
  // Since selection after execution can be 0 it isn't allowed to auto print after
  // Get the flags
  let mut flags = parse_flags(tail, "pnl")?;
  // Set the global print pflags (safe to unwrap since parse_flags never removes a key)
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  // If we are about to delete whole buffer
  if sel.0 == 1 && sel.1 == buffer.len() {
    // And we are to print after execution, error
    if pflags.p || pflags.n || pflags.l {
      return Err(EdError::PrintAfterWipe);
    }
  }
  let buffer = state.history.current_mut(full_command.into());
  let mut tail = buffer.split_off(sel.1);
  let data = buffer.split_off(sel.0 - 1);
  buffer.append(&mut tail);
  state.clipboard = data[..].into();
  // Try to figure out a selection after the deletion
  state.selection = {
    // For deletion behaviour try to select:
    // - nearest following line
    // - if no following line, take the last line in buffer
    // - If buffer empty, fallback to (1,0)

    // Minimum for start is 1, max will return 1 if it would be less
    (1.max(
      // Try to select sel.0, after delete it is line after selection
      // Limit to buffer.len via min in case we deleted whole buffer
      sel.0.min(buffer.len())
    ),
      // Same as above but without the minimum of one, giving 0 if
      // buffer is empty
      sel.0.min(buffer.len())
    )
  };
  Ok(())
}
