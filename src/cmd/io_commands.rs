use super::*;

pub(super) fn filename<I: IO>(
  state: &mut Ed<'_, I>,
  ui: &mut dyn UI,
  path: &str,
) -> Result<(), &'static str> {
  match parse_path(path) {
    None => { // Print current filename
      ui.print_message(
        if state.path.is_empty() { NO_FILE }
        else { &state.path }
      )?;
    }
    Some(x) => { // Set new filename
      state.path = x.to_string();
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
    let path = parse_path(path).unwrap_or(&state.path);
    let unformated_data = state.io.read_file(path, command == 'E')?;
    // We are forced to aggregate to know how many lines we are handing in.
    // Redefining Buffer::insert to return number of lines inserted would fix.
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
    state.buffer.set_saved();
    if command != 'r' {
      state.path = path.to_string();
    }
    let index = index.unwrap_or(1);
    state.selection = (index, index + datalen - 1);
    // Print bytes read from what path
    ui.print_message(&format!(
      "Read {} bytes from {}",
      unformated_data.len(),
      &state.path,
    ))?;
    Ok(())
  }
}
pub(super) fn write_to_file<I: IO>(
  state: &mut Ed<'_, I>,
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
    (false, parse_path(path).unwrap_or(&state.path))
  }
  // If wq, use current file path
  else {
    (true, &state.path[..])
  };
  // If the 'q' flag is set the whole buffer must be selected
  if q && sel.is_some() { return Err(UNSAVED_CHANGES); }
  // Read out data from buffer
  let data = state.buffer.get_selection(
    sel.unwrap_or((1, state.buffer.len()))
  )?
    .map(|x| x.1)
  ;
  // Write it into the file (append if 'W')
  let append = command == 'W';
  state.io.write_file(
    path,
    append,
    data,
  )?;
  // If given path now contains only the whole buffer, update state.file
  // If selection was given, save that selection
  match sel {
    None => {
      if !append { state.path = path.to_string(); }
    },
    Some(s) => {
      state.selection = (s.0, s.1);
    },
  }
  Ok(q)
}
