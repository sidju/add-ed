use super::*;

pub fn scroll(
  state: &mut Ed<'_>,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  command: char,
  tail: &str,
  default_scroll_length: usize,
) -> Result<()> {
  // Depending on forward or backward we use start or end of selection as starting point
  let sel = interpret_selection(&state, selection, state.selection)?;
  let index = if command == 'z' {
    sel.1
  } else {
    sel.0
  };
  let buffer = state.history.current(); // As we only need an immutable buffer we use the same one
  buffer.verify_index(index)?;
  // Parse the arguments to see how many lines to scroll
  let nr_end = tail.find( | c: char | !c.is_numeric() ).unwrap_or(tail.len());
  let nr = if nr_end == 0 {
    default_scroll_length
  } else {
    let nr = tail[.. nr_end].parse::<usize>()
      .map_err(|_|EdError::ScrollNotInt(tail[..nr_end].to_owned()))
    ?;
    // Scrolling 0 lines is invalid, return error
    if nr == 0 { return Err(EdError::NoOp); }
    nr
  };
  // Check what isn't numeric for flags
  let mut flags = parse_flags(&tail[nr_end ..], "pnl")?;
  pflags.p = true; // This command should print, so p always true
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  // The real purpose is to update the selection, do that
  let new_sel = if command == 'z' {
    // Gracefully handle overrunning bufferlen
    let start = buffer.len().min(index + 1);
    let end = buffer.len().min(index + nr);
    (start, end)
  } else {
    // Gracefully handle going under 0
    let start = 1.max(index.saturating_sub(nr));
    let end = 1.max(index.saturating_sub(1));
    (start, end)
    // Old version
    //(index.saturating_sub(1 + nr), index.saturating_sub(1))
  };
  // Verify selection before applying. Probably only fails if buffer is empty.
  buffer.verify_selection(new_sel)?;
  // If all is well we set it and trust the p,n,l flag catcher to print for us
  state.selection = new_sel;
  Ok(())
}
