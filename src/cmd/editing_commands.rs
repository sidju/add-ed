// The more complex commands broken out into functions

use super::*;

pub(super) fn scroll<I: IO>(
  state: &mut Ed<'_, I>,
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

pub(super) fn input<I: IO>(
  state: &mut Ed<'_, I>,
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
  let input = ui.get_input(
    state.see_state(),
    '.',
    #[cfg(feature = "initial_input_data")]
    None,
  )?;
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
    state.buffer.insert(input, index)?;
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

pub(super) fn change<I: IO>(
  state: &mut Ed<'_, I>,
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
      Some(state.buffer.get_selection(sel)?.map(|s| s.1.to_string()).collect())
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

pub(super) fn transfer<I: IO>(
  state: &mut Ed<'_, I>,
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
