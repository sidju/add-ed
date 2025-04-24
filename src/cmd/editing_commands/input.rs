use super::*;

fn inner_input(
  state: &mut Ed<'_>,
  full_command: &str,
  mut input: Vec<String>,
  index: usize,
) -> Result<()> {
  let buffer = state.history.current_mut(full_command.into());
  let mut tail = buffer.split_off(index);
  for line in input.drain(..) {
    buffer.push(Line::new(line).map_err(InternalError::InvalidLineText)?);
  }
  buffer.append(&mut tail);
  Ok(())
}
enum InlineSide {
  Before,
  After,
}
fn inner_inline_input(
  state: &mut Ed<'_>,
  full_command: &str,
  mut input: Vec<String>,
  line: usize,
  side: InlineSide,
) -> Result<()> {
  let buffer = state.history.current_mut(full_command.into());
  let mut tail = buffer.split_off(line);
  let indexed_line = buffer.split_off(line - 1);
  // We need to verify this here, so we can unwrap from the iterator later
  if input.len() == 0 { return Err(EdError::NoOp); }
  let mut input_iter = input.drain(..);
  // Construct the joined line and insert lines
  // (order based on which side we inline insert on)
  match side {
    InlineSide::Before => {
      // Insert lines from data first, then indexed line joined with last
      let mut joined_line = input_iter.next_back().unwrap();
      joined_line.pop(); // Remove newline that should terminate all lines
      joined_line.push_str(&indexed_line[0].text[..]);
      for line in input_iter {
        buffer.push(Line::new(line).map_err(InternalError::InvalidLineText)?);
      }
      // Send in the line itself
      // Arguably we could use the same tag and matched as from the indexed line
      // but we don't since that would be inconsistent with 'c' and 'C' full_command.
      buffer.push(Line::new(joined_line)
        .map_err(InternalError::InvalidLineText)?
      );
    },
    InlineSide::After => {
      // Insert indexed line joined with first, then lines from data
      let mut joined_line = (&indexed_line[0].text[..]).to_owned();
      joined_line.pop(); // Remove newline that should terminate all lines
      joined_line.push_str(&input_iter.next().unwrap());
      // Arguably we could use the same tag and matched as from the indexed line
      // but we don't since that would be inconsistent with 'c' and 'C' full_command.
      buffer.push(Line::new(joined_line)
        .map_err(InternalError::InvalidLineText)?
      );
      for line in input_iter {
        buffer.push(Line::new(line).map_err(InternalError::InvalidLineText)?);
      }
    },
  }
  buffer.append(&mut tail);
  state.clipboard = (&*indexed_line).into();
  Ok(())
}
pub fn input(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  pflags: &mut PrintingFlags,
  full_command: &str,
  selection: Option<Sel<'_>>,
  command: char,
  flags: &str,
) -> Result<()> {
  let mut flags = parse_flags(flags, "pnl")?;
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();

  let buffer = state.history.current();
  let index = match command {
    'a' | 'A' => {
      let i = interpret_index_from_selection(&state, selection, state.selection, true)?;
      if command == 'a' { buffer.verify_index(i)? } else { buffer.verify_line(i)? }
      i
    },
    // Note that saturating_sub really is needed, since inserting at index 0
    // should be valid and equivalent to inserting at index 1.
    'i' | 'I' => {
      let mut i = interpret_index_from_selection(&state, selection, state.selection, false)?;
      if command == 'i' {
        i = i.saturating_sub(1);
        buffer.verify_index(i)?;
      }
      else { buffer.verify_line(i)? }
      i
    },
    _ => ed_unreachable!()?,
  };
  // Now that we have checked that the command is valid, get input
  // This is done so we don't drop text input, which would be annoying
  let input = ui.get_input(
    state,
    '.',
    #[cfg(feature = "initial_input_data")]
    None,
  )?;
  // Run the actual command and save returned selection to state
  // TODO: replace this post-execution selection prediction with returns from
  // the inner functions.
  state.selection = if !input.is_empty() {
    let start = index + 1; // since buffer.insert puts input after index
    let end = start + input.len() - 1; // Subtract for inclusive select
    // In the case of 'a', 'i' that is all
    // 'A' and 'I' need a join
    match command {
      'A' => {
        inner_inline_input(state, full_command, input, index, InlineSide::After)?;
        // This offsets start and end of sel by -1
        (start - 1, end - 1)
      },
      'I' => {
        inner_inline_input(state, full_command, input, index, InlineSide::Before)?;
        (start - 1,end - 1)
      },
      'a' | 'i' => {
        inner_input(state, full_command, input, index)?;
        (start, end)
      },
      _ => ed_unreachable!()?,
    }
  }
  // If no input is given, keep old selection
  else {
    state.selection
  };
  Ok(())
}
