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
  let steps = match arg_iter.next() {
    None => 1,
    // Go to point in history TODO
    //Some('*') => {},
    // Undo/redo (negative is redo
    _ => {
      arguments
        .parse::<isize>()
        .map_err(|_| EdError::UndoStepsNotInt(arguments.to_owned()))
      ?
    },
  };
  if steps == 0 { return Err(EdError::NoOp); }
  let new_pos = state.history.undo( steps )?;
  ui.print_message(&format!(
    "{} {} operation(s) to right after {}",
    if steps < 1 { "Redid" } else { "Undid" },
    if steps < 1 { -steps } else { steps },
    new_pos,
  ))?;
  Ok(())
}
