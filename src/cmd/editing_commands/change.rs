use super::*;

fn inner_change(
  state: &mut Ed<'_>,
  mut input: Vec<String>,
  selection: (usize, usize),
) -> Result<()> {
  let buffer = state.history.current_mut()?;
  let mut tail = buffer.split_off(selection.1);
  state.clipboard = buffer.split_off(selection.0 - 1)[..].into();
  // Note that drain gives full Strings and Line::new will use them as-is,
  // without re-allocating them (but risking leaving them over allocated).
  for line in input.drain(..) {
    buffer.push(Line::new(line).map_err(InternalError::InvalidLineText)?);
  }
  buffer.append(&mut tail);
  Ok(())
}
pub fn change(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  command: char,
  flags: &str,
) -> Result<()> {
  let sel = interpret_selection(&state, selection, state.selection)?;
  let buffer = state.history.current();
  buffer.verify_selection(sel)?;
  let mut flags = parse_flags(flags, "pnl")?;
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  #[allow(unused_variables)]
  let initial_input_data: Option<Vec<String>> = if command == 'C' {
    #[cfg(feature = "initial_input_data")]
    {
      Some(buffer[sel.0 - 1 .. sel.1].iter()
        .map(|s| (&s.text[..]).to_owned())
        .collect()
      )
    }
    #[cfg(not(feature = "initial_input_data"))]
    {
      return Err(EdError::CommandUndefined(command));
    }
  } else {
    None
  };
  let input = ui.get_input(
    state,
    '.',
    #[cfg(feature = "initial_input_data")]
    initial_input_data,
  )?;
  let inputlen = input.len();
  // If we are about to delete the whole buffer
  if inputlen == 0 && sel.0 == 1 && sel.1 == buffer.len() {
    // Verify that we don't have a print flag set, error if we do
    if pflags.p || pflags.n || pflags.l {
      return Err(EdError::PrintAfterWipe);
    }
  }
  inner_change(state, input, sel)?;
  // Re-declare to allow the old one to drop for mutable call above and still use one after
  let buffer = state.history.current();
  state.selection = {
    // For change behaviour select:
    // - sel.0 to sel.0 + inputlen

    // For deletion behaviour try to select:
    // - nearest following line
    // - if no following line, take the last line in buffer
    // - If buffer empty, fallback to (1,0)

    // The minimum for start is 1, max will switch over to 1 if it would be less
    (1.max(
      // Try to select sel.0
      // (if delete is line after selection, if change is first line of changed)
      // But limit to buffer.len via a min in case we deleted the whole buffer
      sel.0.min(buffer.len())
    ),
      // Try to select sel.0 + inputlen.saturating_sub(1)
      // (if delete is same as sel.0 above, if change is last line of changed
      // But limit to buffer.len via a min in case we deleted the whole buffer
      (sel.0 + inputlen.saturating_sub(1)).min(buffer.len())
    )
  };
  Ok(())
}
