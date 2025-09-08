use super::*;

// Basically same as inner_change in editing_commands.rs, for now at least
fn replace_selection(
  state: &mut Ed<'_>,
  full_command: &str,
  selection: (usize, usize),
  mut input: Vec<&str>,
) -> Result<()> {
  // Selection already verified by get_selection call before calling this fn
  let buffer = state.history.current_mut(full_command.into());
  let mut tail = buffer.split_off(selection.1);
  state.clipboard = buffer.split_off(selection.0 - 1)[..].into();
  for line in input.drain(..) {
    buffer.push(Line::new(line).map_err(InternalError::InvalidLineText)?);
  }
  buffer.append(&mut tail);
  Ok(())
}
pub fn run_command(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  full_command: &str,
  selection: Option<Sel<'_>>,
  ch: char,
  command: &str,
) -> Result<()> {
  // '!' doesn't allow a selection
  let sel = if ch == '!' {
    if selection.is_some() { return Err(EdError::SelectionForbidden); }
    None
  }
  // '|' parses selection normally
  else {
    let sel = interpret_selection(&state, selection, state.selection)?;
    state.history.current().verify_selection(sel)?;
    Some(sel)
  };
  let substituted = command_substitutions(
    command,
    &state.file,
    &state.prev_shell_command,
  )?;
  state.prev_shell_command = substituted.clone();
  // Depending on selection or not we use run_transform_command or run_command
  match sel {
    // When there is no selection we just run the command, no buffer interaction
    None => {
      state.io.run_command(
        &mut ui.lock_ui(substituted.clone()),
        substituted,
      )?;
    },
    // When there is a selection we pipe that selection through the command and
    // replace it with the output
    Some(s) => {
      let data = state.history.current().get_lines(s)?;
      let mut transformed = state.io.run_transform_command(
        &mut ui.lock_ui(substituted.clone()),
        substituted,
        data,
      )?;
      if !transformed.ends_with('\n') { transformed.push('\n'); }
      let lines: Vec<&str> = transformed.split_inclusive('\n').collect();
      let nr_lines = lines.len();
      replace_selection(state, full_command, s, lines)?;
      state.selection = if nr_lines != 0 {
        (s.0, s.0 + nr_lines - 1)
      }
      // Same logic as in 'd' and 'c' command
      else {
        (1.max(s.0 - 1), s.0 - 1)
      };
      ui.print_message(&format!(
        "Transformation returned {} bytes",
        transformed.len(),
      ))?;
    },
  }
  Ok(())
}
