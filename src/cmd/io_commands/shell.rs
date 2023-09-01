use super::*;

// Basically same as inner_change in editing_commands.rs, for now at least
fn replace_selection(
  state: &mut Ed<'_>,
  mut input: Vec<&str>,
  selection: (usize, usize),
) -> Result<()> {
  // Selection already verified by get_selection call before calling this fn
  let buffer = state.history.current_mut()?;
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
  selection: Option<Sel<'_>>,
  ch: char,
  command: &str,
) -> Result<()> {
  // If ! we default to no selection, if | we default to prior selection
  let sel = if ch == '!' && selection.is_none() {
    None
  }
  else {
    let sel = interpret_selection(&state, selection, state.selection)?;
    state.history.current().verify_selection(sel)?;
    Some(sel)
  };
  let (changed, substituted) = command_substitutions(
    command,
    &state.file,
    &state.prev_shell_command,
  )?;
  state.prev_shell_command = substituted.clone();
  if changed {ui.print_message( &substituted )?;}
  // Depending on selection or not we use run_transform_command or run_command
  match sel {
    // When there is no selection we just run the command, no buffer interaction
    None => {
      state.io.run_command(
        &mut ui.lock_ui(),
        substituted,
      )?;
      ui.print_message(&ch.to_string())?;
    },
    // When there is a selection we pipe that selection through the command and
    // replace it with the output
    Some(s) => {
      let data = state.history.current().get_lines(s)?;
      let transformed = state.io.run_transform_command(
        &mut ui.lock_ui(),
        substituted,
        data,
      )?;
      let lines: Vec<&str> = transformed.split_inclusive('\n').collect();
      let nr_lines = lines.len();
      replace_selection(state, lines, s)?;
      state.selection = if nr_lines != 0 {
        (s.0, s.0 + nr_lines - 1)
      }
      // Same logic as in 'd' and 'c' command
      else {
        (1.max(s.0 - 1), s.0 - 1)
      };
      ui.print_message(&format!(
        "Transformation returned {} bytes through command `{}`",
        transformed.len(),
        &state.prev_shell_command,
      ))?;
    },
  }
  Ok(())
}
