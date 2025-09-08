use super::*;

pub fn filename(
  state: &mut Ed<'_>,
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
        Path::Command(_) => return Err(EdError::CommandEscapeForbidden(path.into())),
        Path::File(file) => { state.file = file.to_owned(); },
      }
    }
  }
  Ok(())
}

fn insert<'a>(
  buffer: &mut Buffer,
  data: impl Iterator<Item = &'a str>,
  index: usize,
) -> Result<usize> {
  // Index should be verified by calling function
  let mut tail = buffer.split_off(index);
  let start = buffer.len();
  for line in data {
    buffer.push(
      Line::new(format!("{}\n", line)).map_err(InternalError::InvalidLineText)?
    );
  }
  let end = buffer.len();
  buffer.append(&mut tail);
  Ok(end - start)
}
fn replace_buffer<'a>(
  buffer: &mut Buffer,
  data: impl Iterator<Item = &'a str>,
) -> Result<usize> {
  buffer.clear();
  for line in data {
    buffer.push(
      Line::new(format!("{}\n", line)).map_err(InternalError::InvalidLineText)?
    );
  }
  Ok(buffer.len())
}
pub fn read_from_file(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  full_command: &str,
  selection: Option<Sel<'_>>,
  command: char,
  path: &str,
) -> Result<()> {
  let index =
    if command == 'r' {
      let i = interpret_index_from_selection(&state, selection, state.selection, true)?;
      state.history.current().verify_index(i)?;
      Ok(Some(i))
    }
    else if selection.is_none() {
      Ok(None)
    }
    else { 
      Err(EdError::SelectionForbidden)
    }
  ?;
  if !state.history.saved() && command == 'e' {
    Err(EdError::UnsavedChanges)
  }
  else {
    let path = parse_path(path).unwrap_or(Path::File(&state.file));
    let unformated_data = match path {
      Path::Command(cmd) => {
        let substituted = command_substitutions(
          cmd,
          &state.file,
          &state.prev_shell_command,
        )?;
        let data = state.io.run_read_command(
          &mut ui.lock_ui(substituted.clone()),
          substituted.clone(),
        )?;
        state.prev_shell_command = substituted;
        data
      },
      Path::File(file) => {
        state.io.read_file(file, command == 'E')?
      },
    };
    let data = unformated_data.lines();
    let datalen = match index {
      Some(i) => insert(state.history.current_mut(full_command.into()), data, i),
      None => replace_buffer(state.history.current_mut(full_command.into()), data),
    }?;
    // Handle after-effects
    let index = index.unwrap_or(0) + 1;
    state.selection = (index, index + datalen - 1);
    match path {
      // Considering saved after command is odd, and commands cannot be saved
      // into state.file, only aftereffect is state.prev_shell_command
      Path::Command(_cmd) => {
        ui.print_message(&format!(
          "Read {} bytes",
          unformated_data.len(),
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
        if state.history.current().len() == datalen && command != 'r' {
          state.file = file.to_owned();
          state.history.set_saved();
        }
      },
    }
    Ok(())
  }
}

pub fn write_to_file(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  selection: Option<Sel<'_>>,
  command: char,
  in_path: &str,
) -> Result<bool> {
  // Since 'w' and 'W' should default to the whole buffer rather than previous selection
  // they get some custom code here
  let sel = match selection {
    // If selection given we interpret it
    // (When explicit selection is whole buffer we change it to None to signal that)
    Some(s) => {
      let inter = interpret_selection(&state, Some(s), state.selection)?;
      if inter == (1, state.history.current().len()) {
        None
      } else {
        Some(inter)
      }
    },
    // If no selection defaults to selecting the whole buffer
    None => None,
  };

  // If not wq, parse path
  let (q, path, overwrite) = if in_path != "q" {
    match parse_path(in_path) {
      Some(p) => (false, p, false),
      None => (false, Path::File(&state.file), true),
    }
  }
  // If wq, use current file path
  else {
    (true, Path::File(&state.file), true)
  };
  // If the 'q' flag is set the whole buffer must be selected
  if q && sel.is_some() { return Err(EdError::UnsavedChanges); }
  // Read out data from buffer (Also verifies selection, to the extent needed)
  let data = state.history.current().get_lines(
    sel.unwrap_or((1, state.history.current().len()))
  )?
  ;
  // Write into command or file, print nr of bytes written
  match path {
    Path::File(file) => {
      let wtype = if command == 'W' {
        WriteType::Append
      } else {
        if overwrite {
          WriteType::Overwrite
        } else {
          WriteType::Create
        }
      };
      let written = state.io.write_file(
        file,
        wtype,
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
        state.history.set_saved();
      }
    },
    Path::Command(cmd) => {
      // 'W' with a command is probably a misstake, error instead
      if command == 'W' {
        return Err(EdError::CommandEscapeForbidden(in_path.to_owned()));
      }
      let substituted = command_substitutions(
        cmd,
        &state.file,
        &state.prev_shell_command,
      )?;
      state.prev_shell_command = substituted.clone();
      let written = state.io.run_write_command(
        &mut ui.lock_ui(substituted.clone()),
        substituted,
        data,
      )?;
      ui.print_message(&format!(
        "Wrote {} bytes",
        written,
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
