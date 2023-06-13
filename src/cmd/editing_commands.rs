// The more complex commands broken out into functions

use super::*;

pub(super) fn scroll(
  state: &mut Ed<'_>,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  command: char,
  clean: &str,
  default_scroll_length: usize,
) -> Result<()> {
  // Depending on forward or backward we use start or end of selection as starting point
  let sel = interpret_selection(selection, state.selection, &state.buffer)?;
  let index = if command == 'z' {
    sel.1
  } else {
    sel.0
  };
  verify_index(&state.buffer, index)?;
  // Parse the arguments to see how many lines to scroll
  let nr_end = clean.find( | c: char | !c.is_numeric() ).unwrap_or(clean.len());
  let nr = if nr_end == 0 {
    default_scroll_length
  } else {
    let nr = clean[.. nr_end].parse::<usize>()
      .map_err(|_|EdError::ScrollNotInt(clean[..nr_end].to_owned()))
    ?;
    // Scrolling 0 lines is invalid, return error
    if nr == 0 { return Err(EdError::NoOp); }
    nr
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
    let start = 1.max(index.saturating_sub(nr));
    let end = 1.max(index.saturating_sub(1));
    (start, end)
    // Old version
    //(index.saturating_sub(1 + nr), index.saturating_sub(1))
  };
  // Verify selection before applying. Probably only fails if buffer is empty.
  verify_selection(&state.buffer, new_sel)?;
  // If all is well we set it and trust the p,n,l flag catcher to print for us
  state.selection = new_sel;
  Ok(())
}

pub(super) fn input(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  command: char,
  flags: &str,
) -> Result<()> {
  let sel = interpret_selection(selection, state.selection, &state.buffer)?;
  let mut flags = parse_flags(flags, "pnl")?;
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  match command {
    'a' => verify_index(&state.buffer, sel.1)?,
    'i' => verify_index(&state.buffer, sel.0.saturating_sub(1))?,
    'A' => verify_line(&state.buffer, sel.1)?,
    'I' => verify_line(&state.buffer, sel.0)?,
    _ => { panic!("Unreachable code reached"); }
  }
  // Now that we have checked that the command is valid, get input
  // This is done so we don't drop text input, that would be annoying
  let input = ui.get_input(
    state,
    '.',
    #[cfg(feature = "initial_input_data")]
    None,
  )?;
  // Run the actual command and save returned selection to state
  state.selection = if !input.is_empty() {
    let index = match command {
      'a' | 'A' => sel.1,
      'i' => sel.0.saturating_sub(1),
      'I' => sel.0,
      _ => unreachable!(),
    };
    let start = index + 1; // since buffer.insert puts input after index
    let end = start + input.len() - 1; // Subtract for inclusive select
    // In the case of 'a', 'i' that is all
    // 'A' and 'I' need a join
    match command {
      'A' => {
        state.buffer.inline_append(input, index)?;
        // This offsets start and end of sel by -1
        (start - 1, end - 1)
      },
      'I' => {
        state.buffer.inline_insert(input, index)?;
        (start - 1,end - 1)
      },
      'a' | 'i' => {
        state.buffer.insert(input, index)?;
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

pub(super) fn change(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  command: char,
  flags: &str,
) -> Result<()> {
  let sel = interpret_selection(selection, state.selection, &state.buffer)?;
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
      return Err(EdError::CommandUndefined(command));
    }
  } else {
    verify_selection(&state.buffer, sel)?;
    None
  };
  let input = ui.get_input(
    state,
    '.',
    #[cfg(feature = "initial_input_data")]
    initial_input_data,
  )?;
  let inputlen = input.len();
  // If we are about to delete the whole buffer
  if inputlen == 0 && sel.0 == 1 && sel.1 == state.buffer.len() {
    // Verify that we don't have a print flag set, error if we do
    if pflags.p || pflags.n || pflags.l {
      return Err(EdError::PrintAfterWipe);
    }
  }
  state.buffer.change(input, sel)?;
  state.selection = {
    // For change behaviour select:
    // - sel.0 to sel.0 + inputlen

    // For deletion behaviour try to select:
    // - nearest following line
    // - if no following line, take the last line in buffer
    // - If buffer empty, fallback to (1,0)

    // The minimum for start is 1, max will switch over to 1 if it would be less
    (1.max(
      // Try to select sel.0
      // (if delete is line after selection, if change is first line of changed)
      // But limit to buffer.len via a min in case we deleted the whole buffer
      sel.0.min(state.buffer.len())
    ),
      // Try to select sel.0 + inputlen.saturating_sub(1)
      // (if delete is same as sel.0 above, if change is last line of changed
      // But limit to buffer.len via a min in case we deleted the whole buffer
      (sel.0 + inputlen.saturating_sub(1)).min(state.buffer.len())
    )
  };
  Ok(())
}

pub(super) fn transfer(
  state: &mut Ed<'_>,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  command: char,
  tail: &str,
) -> Result<()> {
  // Parse the target index, then the flags if any
  let (ind_end, ind) = parse_index(tail)?;
  let index = interpret_index(
    ind.unwrap_or_else(||{
      if command == 'M' || command == 'T' { Ind::Literal(1) }
      else { Ind::BufferLen }
    }),
    &state.buffer,
    state.selection.1,
  )?;
  let mut flags = parse_flags(&tail[ind_end..], "pnl")?;
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  // Calculate the selection
  let selection = interpret_selection(selection, state.selection, &state.buffer)?;
  verify_selection(&state.buffer, selection)?;
  // Beware, is actually 1 less than move size due to inclusive bounds
  let move_size = selection.1 - selection.0;
  match command {
    'm' => state.buffer.mov(selection, index)?,
    'M' => state.buffer.mov(selection, index.saturating_sub(1))?,
    't' => state.buffer.mov_copy(selection,index)?,
    'T' => state.buffer.mov_copy(selection,index.saturating_sub(1))?,
    _ => unreachable!(),
  };
  // Note that we subtract/add one to index to exclude index itself
  state.selection = match command {
    // If moving forward we must detract moved lines from resulting selection
    'm' if selection.1 < index => (index - move_size, index),
    'M' if selection.1 < index.saturating_sub(1) => (index - 1 - move_size, index - 1),
    // Otherwise the general case should work
    'M' | 'T' => ( index, index + move_size),
    'm' | 't' => (index + 1, index + move_size + 1),
    _ => unreachable!(),
  };
  Ok(())
}
