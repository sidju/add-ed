use super::*;

fn inner_reflow(
  state: &mut Ed<'_>,
  selection: (usize, usize),
  width: usize,
) -> Result<usize> {
  state.history.current().verify_selection(selection)?;
  let buffer = state.history.current_mut()?;
  // Get the selected data
  let mut tail = buffer.split_off(selection.1);
  let data = buffer.split_off(selection.0 - 1);
  // Convert it into a vec of chars (replacing newlines with spaces)
  let mut chars = Vec::new();
  for line in &data {
    for ch in line.text.chars() {
      chars.push(match ch{
        '\n' => ' ',
        c => c,
      });
    }
  }
  // Move the original data into clipboard
  state.clipboard = data[..].into();
  // Remove the trailing newline, which is now a pointless space
  chars.pop();
  // Replace the space nearest before the selected width with newline
  let mut w = 0; // Characted width of current line
  let mut latest_space = None;
  for i in 0..chars.len() {
    // Increment width for each
    w += 1;
    // Check if a space, if so note it as latest space
    if chars[i] == ' ' { latest_space = Some(i); }
    // Check if at width, if so replace latest space and update width
    if w > width {
      if let Some(s) = latest_space {
        chars[s] = '\n';
        w = i - s;
        latest_space = None;
      }
    }
  }
  // Convert the char vec into lines in the buffer
  for line in chars.split(|c| c == &'\n') {
    buffer.push(Line::new(line.iter().collect::<String>(), '\0'));
  }
  // Note end of reflowed lines
  let end = buffer.len();
  // Add back tail and return
  buffer.append(&mut tail);
  Ok(end)
}
pub fn reflow(
  state: &mut Ed<'_>,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  tail: &str,
) -> Result<()> {
  let selection = interpret_selection(&state, selection, state.selection)?;
  // Find the end of numeric argument, and parse nr of columns to reflow within
  let nr_end = tail.find( |c: char| !c.is_numeric() ).unwrap_or(tail.len());
  let width = if nr_end == 0 {
    80
  } else {
    tail[.. nr_end] .parse::<usize>()
      .map_err(|e| EdError::ReflowNotInt{error: e.to_string(), text: tail[..nr_end].into()})
    ?
  };
  let mut flags = parse_flags(&tail[nr_end ..], "pnl")?;
  pflags.p = flags.remove(&'p').unwrap();
  pflags.n = flags.remove(&'n').unwrap();
  pflags.l = flags.remove(&'l').unwrap();
  let end = inner_reflow(state, selection, width)?;
  // selection.0 has some extra logic to handle that effective deletion may
  // occur when reflowing a selection without any words. See 'd' command logic.
  state.selection = (selection.0.min(end).max(1), end);
  Ok(())
}
