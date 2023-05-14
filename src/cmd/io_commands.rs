use super::*;

pub(super) fn filename<I: IO>(
  state: &mut Ed<'_, I>,
  ui: &mut dyn UI,
  path: &str,
) -> Result<()> {
  match parse_path(path) {
    None => { // Print current filename
      ui.print_message(
        if state.file.is_empty() { NO_FILE }
        else { &state.file }
      )?;
    }
    Some(x) => { // Set new filename
      match x {
        Path::Command(_) => return Err(EdError::InvalidDefaultFile),
        Path::File(file) => { state.file = file.to_owned(); },
      }
    }
  }
  Ok(())
}

pub(super) fn read_from_file<I: IO>(
  state: &mut Ed<'_, I>,
  ui: &mut dyn UI,
  selection: Option<Sel<'_>>,
  command: char,
  path: &str,
) -> Result<()> {
  let index =
    if command == 'r' {
      let i = interpret_selection(selection, state.selection, &state.buffer)?.1;
      verify_index(&state.buffer, i)?;
      Ok(Some(i))
    }
    else if selection.is_none() {
      Ok(None)
    }
    else { 
      Err(EdError::SelectionForbidden)
    }
  ?;
  if !state.buffer.saved() && command == 'e' {
    Err(EdError::UnsavedChanges)
  }
  else {
    let path = parse_path(path).unwrap_or(Path::File(&state.file));
    let unformated_data = match path {
      Path::Command(cmd) => {
        let (changed, substituted) = command_substitutions(
          cmd,
          &state.file,
          &state.prev_shell_command,
        )?;
        state.prev_shell_command = substituted.clone();
        if changed {ui.print_message( &substituted )?;}
        state.io.run_read_command(
          &mut ui.lock_ui(),
          substituted,
        )?
      },
      Path::File(file) => {
        state.io.read_file(file, command == 'E')?
      },
    };
    // We are forced to aggregate to convert into the format insert wants.
    // This has the bonus of telling us how many lines we are inserting.
    let data: Vec<&str> = unformated_data.split_inclusive('\n').collect();
    let datalen = data.len();
    match index {
      Some(i) => state.buffer.insert(data, i),
      None => state.buffer.replace_buffer(data),
    }?;
    // Handle after-effects
    let index = index.unwrap_or(0) + 1;
    state.selection = (index, index + datalen - 1);
    match path {
      // Considering saved after command is odd, and commands cannot be saved
      // into state.file, only aftereffect is state.prev_shell_command
      Path::Command(_cmd) => {
        ui.print_message(&format!(
          "Read {} bytes from command `{}`",
          unformated_data.len(),
          &state.prev_shell_command,
        ))?;
      },
      Path::File(file) => {
        ui.print_message(&format!(
          "Read {} bytes from path `{}`",
          unformated_data.len(),
          file,
        ))?;
        // Should only occur if we cleared buffer or it was empty before read.
        // Rule of least surprise means 'r' shouldn't do this even then, since
        // it normally won't.
        if state.buffer.len() == datalen && command != 'r' {
          state.file = file.to_owned();
          state.buffer.set_saved();
        }
      },
    }
    Ok(())
  }
}

pub(super) fn write_to_file<I: IO>(
  state: &mut Ed<'_, I>,
  ui: &mut dyn UI,
  selection: Option<Sel<'_>>,
  command: char,
  path: &str,
) -> Result<bool> {
  // Since 'w' and 'W' should default to the whole buffer rather than previous selection
  // they get some custom code here
  let sel = match selection {
    // If selection given we interpret it
    // (When explicit selection is whole buffer we change it to None to signal that)
    Some(s) => {
      let inter = interpret_selection(Some(s), state.selection, &state.buffer)?;
      if inter == (1, state.buffer.len()) {
        None
      } else {
        Some(inter)
      }
    },
    // If no selection defaults to selecting the whole buffer
    None => None,
  };

  // If not wq, parse path
  let (q, path) = if path != "q" {
    (false, parse_path(path).unwrap_or(Path::File(&state.file)))
  }
  // If wq, use current file path
  else {
    (true, (Path::File(&state.file)))
  };
  // If the 'q' flag is set the whole buffer must be selected
  if q && sel.is_some() { return Err(EdError::UnsavedChanges); }
  // Read out data from buffer
  let data = state.buffer.get_selection(
    sel.unwrap_or((1, state.buffer.len()))
  )?
    .map(|x| x.1)
  ;
  // Write into command or file, print nr of bytes written
  match path {
    Path::File(file) => {
      let append = command == 'W';
      let written = state.io.write_file(
        file,
        append,
        data,
      )?;
      ui.print_message(&format!(
        "Wrote {} bytes to path `{}`",
        written,
        file,
      ))?;
      // Since path isn't allowed to be a command, do check in here
      // If path now contains only whole buffer, set saved and update state.file.
      // Rule of least surprise means 'W' shouldn't do so even then, since it
      // normally won't
      if sel.is_none() && command != 'W' {
        state.file = file.to_string();
        state.buffer.set_saved();
      }
    },
    Path::Command(cmd) => {
      let (changed, substituted) = command_substitutions(
        cmd,
        &state.file,
        &state.prev_shell_command,
      )?;
      state.prev_shell_command = substituted.clone();
      if changed {ui.print_message( &substituted )?;}
      let written = state.io.run_write_command(
        &mut ui.lock_ui(),
        substituted,
        data,
      )?;
      ui.print_message(&format!(
        "Wrote {} bytes to command `{}`",
        written,
        &state.prev_shell_command,
      ))?;
    },
  }
  // If selection was given, save that selection
  match sel {
    None => (),
    Some(s) => {
      state.selection = (s.0, s.1);
    },
  }
  Ok(q)
}

pub fn run_command<I: IO>(
  state: &mut Ed<'_, I>,
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
    Some(interpret_selection(selection, state.selection, &state.buffer)?)
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
      let data = state.buffer.get_selection(s)?.map(|x| x.1);
      let transformed = state.io.run_transform_command(
        &mut ui.lock_ui(),
        substituted,
        data,
      )?;
      let lines: Vec<&str> = transformed.split_inclusive('\n').collect();
      let nr_lines = lines.len();
      state.buffer.change(lines, s)?;
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
