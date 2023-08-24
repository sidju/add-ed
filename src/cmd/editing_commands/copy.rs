use super::*;

fn inner_copy(
  state: &mut Ed<'_>,
  selection: (usize, usize),
) -> Result<()> {
  let buffer = state.history.current();
  state.clipboard = buffer[selection.0 - 1 .. selection.1].into();
  Ok(())
}
pub fn copy(
  state: &mut Ed<'_>,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  tail: &str,
) -> Result<()> {
  let sel = interpret_selection(&state, selection, state.selection)?;
  state.history.current().verify_selection(sel)?;
  let mut flags = parse_flags(tail, "pnl")?;
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  inner_copy(state, sel)?;
  state.selection = sel;
  Ok(())
}
