// The more complex commands broken out into functions

use super::*;

pub(super) fn filename<B: Buffer>(
  state: &mut Ed<'_, B>,
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

pub(super) fn read_from_file<B: Buffer>(
  state: &mut Ed<'_, B>,
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
    let datalen = state.buffer.read_from(path, index, command == 'E')?;
    if command != 'r' {
      state.path = path.to_string();
    }
    let index = index.unwrap_or(1);
    state.selection = (index, index + datalen - 1);
    Ok(())
  }
}
pub(super) fn write_to_file<B: Buffer>(
  state: &mut Ed<'_, B>,
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
  // Write it into the file (append if 'W')
  let append = command == 'W';
  state.buffer.write_to(sel, path, append)?;
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

pub(super) fn scroll<B: Buffer>(
  state: &mut Ed<'_, B>,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  command: char,
  clean: &str,
  default_scroll_length: usize,
) -> Result<(), &'static str> {
  // Depending on forward or backward we use start or end of selection as starting point
  let sel = interpret_selection(selection, state.selection, state.buffer)?;
  let index = if command == 'z' {
    sel.1
  } else {
    sel.0
  };
  verify_index(state.buffer, index)?;
  // Parse the arguments to see how many lines to scroll
  let nr_end = clean.find( | c: char | !c.is_numeric() ).unwrap_or(clean.len());
  let nr = if nr_end == 0 {
    default_scroll_length
  } else {
    clean[.. nr_end].parse::<usize>().map_err(|_| INTEGER_PARSE)?
  };
  // Check what isn't numeric for flags
  let mut flags = parse_flags(&clean[nr_end ..], "pnl")?;
  pflags.p = true; // This command should print, so p always true
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  // The real purpose is to update the selection, do that
  let new_sel = if command == 'z' {
    // Gracefully handle overrunning bufferlen
    let start = state.buffer.len().min(index + 1);
    let end = state.buffer.len().min(index + nr);
    (start, end)
  } else {
    // Gracefully handle going under 0
    // (If we end up under 1 that is handled by print logic below)
    (index.saturating_sub(1 + nr), index.saturating_sub(1))
  };
  // Verify selection before applying. Probably only fails if buffer is empty.
  verify_selection(state.buffer, new_sel)?;
  // If all is well we set it and trust the p,n,l flag catcher to print for us
  state.selection = new_sel;
  Ok(())
}

pub(super) fn input<B: Buffer>(
  state: &mut Ed<'_, B>,
  ui: &mut dyn UI,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  command: char,
  flags: &str,
) -> Result<(), &'static str> {
  let sel = interpret_selection(selection, state.selection, state.buffer)?;
  let mut flags = parse_flags(flags, "pnl")?;
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  match command {
    'a' => verify_index(state.buffer, sel.1)?,
    'i' => verify_index(state.buffer, sel.0.saturating_sub(1))?,
    'A' => verify_line(state.buffer, sel.1)?,
    'I' => verify_line(state.buffer, sel.0)?,
    _ => { panic!("Unreachable code reached"); }
  }
  // Now that we have checked that the command is valid, get input
  // This is done so we don't drop text input, that would be annoying
  let tmp = ui.get_input(
    state.see_state(),
    '.',
    #[cfg(feature = "initial_input_data")]
    None,
  )?;
  // Input conversion, to follow buffer api
  let mut input = tmp.iter().map(|string| &string[..]);
  // Run the actual command and save returned selection to state
  state.selection = if input.len() != 0 {
    let index = if command == 'a' || command == 'A' {
      sel.1
    }
    else {
      sel.0.saturating_sub(1)
    };
    let start = index + 1; // since buffer.insert puts input after index
    let end = start + input.len() - 1; // Subtract for inclusive select
    state.buffer.insert(&mut input, index)?;
    // In the case of 'a', 'i' that is all
    // 'A' and 'I' need a join
    match command {
      // For 'A' we join the first input line with its preceeding, and thus reduce selection with 1
      'A' => {
        // That the next line exists is checked by check_line on sel.1 above
        state.buffer.join((index, index + 1))?;
        // This offsets start and end of sel by -1
        (start.saturating_sub(1), end.saturating_sub(1))
      },
      // For 'I' we do the same with the last input line and the following line, requiring no selection change
      'I' => {
        // That next line exists is checked by check_line on sel.0 above
        state.buffer.join((end, end + 1))?;
        (start,end)
      },
      // 'a' and 'i' need only pass out start and end
      'a' | 'i' => {
        (start, end)
      },
      _ => { panic!("Unreachable code reached"); }
    }
  }
  // If no input is given, keep old selection
  else {
    state.selection
  };
  Ok(())
}

pub(super) fn change<B: Buffer>(
  state: &mut Ed<'_, B>,
  ui: &mut dyn UI,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  command: char,
  flags: &str,
) -> Result<(), &'static str> {
  let sel = interpret_selection(selection, state.selection, state.buffer)?;
  let mut flags = parse_flags(flags, "pnl")?;
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  #[allow(unused_variables)]
  let initial_input_data: Option<Vec<String>> = if command == 'C' {
    #[cfg(feature = "initial_input_data")]
    {
      Some(state.buffer.get_selection(sel)?.map(|s| s.to_string()).collect())
    }
    #[cfg(not(feature = "initial_input_data"))]
    {
      return Err(UNDEFINED_COMMAND);
    }
  } else {
    verify_selection(state.buffer, sel)?;
    None
  };
  let input = ui.get_input(
    state.see_state(),
    '.',
    #[cfg(feature = "initial_input_data")]
    initial_input_data,
  )?;
  let inputlen = input.len();
  let mut input = input.iter().map(|string| &string[..]);
  state.buffer.change(&mut input, sel)?;
  state.selection = if inputlen != 0 {
    (sel.0, sel.0 + inputlen - 1)
  }
  else {
    // If we just deleted all then sel.0 == 1 then
    // this resolves to (1,0)
    // Otherwise selects nearest line before selection
    (1.max(sel.0 - 1), sel.0 - 1)
  };
  Ok(())
}

pub(super) fn transfer<B: Buffer>(
  state: &mut Ed<'_, B>,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  command: char,
  tail: &str,
) -> Result<(), &'static str> {
  // Parse the target index, then the flags if any
  let (ind_end, ind) = parse_index(tail)?;
  let index = interpret_index(
    ind.unwrap_or(Ind::BufferLen),
    state.buffer,
    state.selection.1,
  )?;
  let mut flags = parse_flags(&tail[ind_end..], "pnl")?;
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  // Calculate the selection
  let selection = interpret_selection(selection, state.selection, state.buffer)?;
  // Beware, is actually 1 less than move size due to inclusive bounds
  let move_size = selection.1 - selection.0;
  if command == 'm' {
    state.buffer.mov(selection, index)?;
  } else {
    state.buffer.mov_copy(selection,index)?;
  };
  // Note that we subtract/add one to index to exclude index itself
  state.selection = if command == 'm' && selection.1 < index {
    // If moving forward detract moved lines from resulting selection
    (index - move_size, index)
  } else {
    (index + 1, index + move_size + 1)
  };
  Ok(())
}
