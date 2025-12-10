use super::*;

pub fn status(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  selection: Option<Sel<'_>>,
  clean: &str,
) -> Result<()> {
  // Verify/parse input
  if selection.is_some() { return Err(EdError::SelectionForbidden); }
  let mut flags = parse_flags(clean, "as")?;
  let mut message = String::new();
  // Print selection
  if
    flags.remove(&'s').unwrap() ||
    flags.remove(&'a').unwrap() ||
    clean.is_empty()
  {
    message.push_str(&format!("({},{})", state.selection.0, state.selection.1));
  }
  ui.print_message(&message)?;
  Ok(())
}
