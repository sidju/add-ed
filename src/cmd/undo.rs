use super::*;

pub fn undo(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  selection: Option<Sel<'_>>,
  arguments: &str,
) -> Result<()> {
  if selection.is_some() {return Err(EdError::SelectionForbidden); }
  // A undo steps parsing not unlike index parsing would be good later
  // ie. relative AND shorthand for start and end of history
  let mut arg_iter = arguments.chars();
  match arg_iter.next() {
    // Go to point in history TODO
    //Some('*') => {},
    // Undo/redo (negative is redo
    Some('-') => {
      let steps = if arg_iter.next().is_some() {
        arguments[1..]
          .parse::<usize>()
          .map_err(|_| EdError::UndoStepsNotInt(arguments[1..].to_owned()))
        ?
      } else { 1 };
      if steps == 0 { return Err(EdError::NoOp); }
      let new_pos = state.history.set_viewed_i(state.history.viewed_i() + steps)?;
      ui.print_message(&format!(
        "Redid {} operation(s) to right after {}.",
        steps,
        new_pos,
      ))?;
    },
    x => {
      let steps = if x.is_some() {
        arguments
          .parse::<usize>()
          .map_err(|_| EdError::UndoStepsNotInt(arguments.to_owned()))
        ?
      } else { 1 };
      if steps == 0 { return Err(EdError::NoOp); }
      if state.history.viewed_i() < steps {
        return Err(EdError::UndoIndexNegative{relative_undo_limit: state.history.viewed_i()});
      }
      let new_pos = state.history.set_viewed_i(state.history.viewed_i() - steps)?;
      ui.print_message(&format!(
        "Undid {} operation(s) to right after {}.",
        steps,
        new_pos
      ))?;
    },
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
      "{} {} {}",
      if hi == i { '>' } else { ' ' },
      view[hi].0,
      if Some(hi) == saved { "(saved)" } else { "" },
    ));
  }
  ui.print_message(&tmp)?;
  Ok(())
}
