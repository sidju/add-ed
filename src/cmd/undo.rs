use super::*;

pub fn undo(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  selection: Option<Sel<'_>>,
  arguments: &str,
) -> Result<()> {
  // Verify/parse input
  if selection.is_some() {return Err(EdError::SelectionForbidden); }
  let (i, hist_ind) = parse_history_index(arguments)?;
  parse_flags(&arguments[i..], "")?;
  // Realize the input
  let hist_ind = hist_ind.map_or_else(
    || Ok(state.history.viewed_i().saturating_sub(1)),
    |ind| interpret_history_index(
      state,
      ind,
      state.history.viewed_i(),
    )
  )?;
  let change = hist_ind as i64 - state.history.viewed_i() as i64;
  let new_pos = state.history.set_viewed_i(hist_ind)?;
  match change {
    0 => { return Err(EdError::NoOp); },
    x if x > 0 => {
      ui.print_message(&format!(
        "Redid {} operation(s) to right after {}.",
        x,
        new_pos,
      ))?
    },
    x => {
      ui.print_message(&format!(
        "Undid {} operation(s) to right after {}.",
        -x,
        new_pos,
      ))?
    },
  };
  // As a bonus, check that our selection isn't out of buffer
  let buf_len = state.history.current().len();
  if state.selection.1 > buf_len && buf_len != 0 {
    state.selection.1 = buf_len;
    if state.selection.0 > state.selection.1 {
      state.selection.0 = state.selection.1;
    }
  }
  Ok(())
}

pub fn manage_history(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  selection: Option<Sel<'_>>,
  tail: &str,
) -> Result<()> {
  if selection.is_some() {return Err(EdError::SelectionForbidden); }
  // Some custom flags (or maybe arguments) should probably be added later
  let mut _flags = parse_flags(tail, "")?;
  // To enable undoing to an absolute "index" from start of editing the
  // History struct must allow both accessing current index and the labels for
  // all indices. Due to the data structure this will also require allowing
  // access to the snapshotted states and effectively expose internal design.

  // Figure out the history index slice for the nearest 10 snapshots
  let i = state.history.viewed_i();
  let view = state.history.snapshots();
  // If in the first five snapshots we want the first 10
  let history_indices = if i < 10 {
    // Use .min(view.len()) to limit within valid slicing
    0 .. 10.min(view.len())
  }
  // If in the last five snapshots we want the last 10
  else if view.len().saturating_sub(10) <= i {
    // Use saturating sub to avoid underflow
    view.len().saturating_sub(10) .. view.len()
  }
  // Otherwise we want the 5 preceding, current and 4 following snapshots
  // (Since none of the preceeding were true we can safely slice this)
  else {
    i - 5 .. i + 4
  };

  // Print it nicely
  let saved = state.history.saved_i();
  let mut tmp = String::new();
  for hi in history_indices {
    tmp.push_str(&format!(
      "{} {}{}\n",
      if hi == i { '>' } else { ' ' },
      view[hi].0,
      if Some(hi) == saved { " (saved)" } else { "" },
    ));
  }
  ui.print_message(&tmp)?;
  Ok(())
}
