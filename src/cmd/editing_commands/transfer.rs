use super::*;

// Note that we must be careful to break the Rc connection between the tag and
// matched fields on the lines. (Otherwise those lines will be considered the
// same for tagging and 'g' matching)
enum TransferType {
  Copy,
  Move,
}
fn inner_transfer(
  state: &mut Ed<'_>,
  full_command: &str,
  selection: (usize, usize),
  index: usize,
  mode: TransferType,
) -> Result<(usize, usize)> {
  let buffer = state.history.current(); // Immutable buffer to verify indices against
  buffer.verify_selection(selection)?;
  buffer.verify_index(index)?;
  // Moving into the selection is an error we need to check for specifically
  if let TransferType::Move = mode {
    if index >= selection.0 && index < selection.1 {
      return Err(EdError::NoOp).into();
    }
  }
  let buffer = state.history.current_mut(full_command.into())?;
  match mode {
    // The simple one, just iterate over selection into a tmp vec, then add it
    // after given index.
    TransferType::Copy => {
      // We make sure to not duplicate the tag or matched Rc:s when copying by
      // using a temporary clipboard, which breaks those references.
      let tmp: Clipboard = buffer[selection.0 - 1 .. selection.1].into();
      let mut tail = buffer.split_off(index);
      let start_ind = buffer.len() + 1; // +1 excludes current last line
      buffer.append(&mut (&tmp).into());
      let end_ind = buffer.len();
      buffer.append(&mut tail);
      Ok((start_ind, end_ind))
    },
    // We need to act differently based on if we move forward or backward, but
    // at least we don't have to worry about accidentally duplicating Rc:s.
    TransferType::Move => {
      // Moving backwards
      if index < selection.0 {
        let mut tail = buffer.split_off(selection.1);
        let mut data = buffer.split_off(selection.0 - 1);
        let mut middle = buffer.split_off(index);
        let start_ind = buffer.len() + 1;
        buffer.append(&mut data);
        let end_ind = buffer.len();
        buffer.append(&mut middle);
        buffer.append(&mut tail);
        Ok((start_ind, end_ind))
      }
      // Moving forwards
      else if index >= selection.1 {
        let mut tail = buffer.split_off(index);
        let mut middle = buffer.split_off(selection.1);
        let mut data = buffer.split_off(selection.0 - 1);
        buffer.append(&mut middle);
        let start_ind = buffer.len() + 1;
        buffer.append(&mut data);
        let end_ind = buffer.len();
        buffer.append(&mut tail);
        Ok((start_ind, end_ind))
      }
      // We check for this already, to prevent wasted snapshots
      else {
        ed_unreachable!()
      }
    },
  }
}
pub fn transfer(
  state: &mut Ed<'_>,
  pflags: &mut PrintingFlags,
  full_command: &str,
  selection: Option<Sel<'_>>,
  command: char,
  tail: &str,
) -> Result<()> {
  let selection = interpret_selection(&state, selection, state.selection)?;
  // Parse the target index, then the flags if any
  let (ind_end, ind) = parse_index(tail)?;
  let index = interpret_index(
    &state,
    ind.unwrap_or_else(||{
      if command == 'M' || command == 'T' { Ind::Literal(1) }
      else { Ind::BufferLen }
    }),
    state.selection.1,
  )?;
  let mut flags = parse_flags(&tail[ind_end..], "pnl")?;
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  // Run the command, returning the selection after
  state.selection = match command {
    'm' => inner_transfer(state, full_command, selection, index, TransferType::Move)?,
    'M' => inner_transfer(state, full_command, selection, index.saturating_sub(1), TransferType::Move)?,
    't' => inner_transfer(state, full_command, selection, index, TransferType::Copy)?,
    'T' => inner_transfer(state, full_command, selection, index.saturating_sub(1), TransferType::Copy)?,
    _ => return ed_unreachable!(),
  };
  Ok(())
}
