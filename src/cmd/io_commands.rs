use super::*;

pub(super) fn filename<I: IO>(
  state: &mut Ed<'_, I>,
  ui: &mut dyn UI,
  path: &str,
) -> Result<(), &'static str> {
  match parse_path(path) {
    None => { // Print current filename
      ui.print_message(
        if state.file.is_empty() { NO_FILE }
        else { &state.file }
      )?;
    }
    Some(x) => { // Set new filename
      match x {
        Path::Command(_) => return Err(INVALID_FILE),
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
) -> Result<(), &'static str> {
  let index =
    if command == 'r' {
      let i = interpret_selection(selection, state.selection, state.buffer)?.1;
      Ok(Some(i))
    }
    else if selection.is_none() {
      Ok(None)
    }
    else { 
      Err(SELECTION_FORBIDDEN)
    }
  ?;
  if !state.buffer.saved() && command == 'e' {
    Err(UNSAVED_CHANGES)
  }
  else {
    let path = parse_path(path).unwrap_or(Path::File(&state.file));
    let unformated_data = match path {
      Path::Command(cmd) => {
        state.io.run_read_command(
          &mut ui.lock_ui(),
          cmd.to_owned(),
        )?
      },
      Path::File(file) => {
        state.io.read_file(file, command == 'E')?
      },
    };
    // We are forced to aggregate to convert into the format insert wants.
    // This has the bonus of telling us how many lines we are inserting.
    let data: Vec<&str> = (&unformated_data).split_inclusive('\n').collect();
    let datalen = data.len();
    match index {
      Some(i) => state.buffer.insert(data, i + 1),
      None => if state.buffer.len() == 0 {
        state.buffer.insert(data, 0)
      } else {
        state.buffer.change(data, (1, state.buffer.len()))
      },
    }?;
    // Handle after-effects
    let index = index.unwrap_or(1);
    state.selection = (index, index + datalen - 1);
    match path {
      // Considering saved after command is odd, and commands cannot be saved
      // into state.file, so no after effects except printout
      Path::Command(cmd) => {
        ui.print_message(&format!(
          "Read {} bytes from command `{}`",
          unformated_data.len(),
          cmd,
        ))?;
      },
      Path::File(file) => {
        ui.print_message(&format!(
          "Read {} bytes from path `{}`",
          unformated_data.len(),
          file,
        ))?;
        // Should only occur if we cleared buffer or it was empty before read
        if state.buffer.len() == datalen {
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
) -> Result<bool, &'static str> {
  // Since 'w' and 'W' should default to the whole buffer rather than previous selection
  // they get some custom code here
  let sel = match selection {
    // If selection given we interpret it
    // (When explicit selection is whole buffer we change it to None to signal that)
    Some(s) => {
      let inter = interpret_selection(Some(s), state.selection, state.buffer)?;
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
  if q && sel.is_some() { return Err(UNSAVED_CHANGES); }
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
      // If given path now contains only the whole buffer, update state.file
      if sel.is_none() { state.file = file.to_string(); }
    },
    Path::Command(cmd) => {
      let written = state.io.run_write_command(
        &mut ui.lock_ui(),
        cmd.to_owned(),
        data,
      )?;
      ui.print_message(&format!(
        "Wrote {} bytes to command `{}`",
        written,
        cmd,
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
) -> Result<(), &'static str> {
  // If ! we default to no selection, if | we default to prior selection
  let sel = if ch == '!' && selection.is_none() {
    None
  }
  else {
    Some(interpret_selection(selection, state.selection, state.buffer)?)
  };
  // Depending on selection or not we use run_filter_command or run_command
  match sel {
    // When there is no selection we just run the command, no buffer interaction
    None => {
      state.io.run_command(
        &mut ui.lock_ui(),
        command.to_owned(),
      )?;
      ui.print_message("!")?;
    },
    // When there is a selection we pipe that selection through the command and
    // replace it with the output
    Some(s) => {
      let data = state.buffer.get_selection(s)?.map(|x| x.1);
      let transformed = state.io.run_transform_command(
        &mut ui.lock_ui(),
        command.to_owned(),
        data,
      )?;
      let lines: Vec<&str> = (&transformed).split_inclusive('\n').collect();
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
        command,
      ))?;
    },
  }
  Ok(())
}
