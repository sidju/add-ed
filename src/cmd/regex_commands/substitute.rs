use super::*;

// Helper to perform regex substitutions
//
// Cuts out the selection, performs substitution and returns the index of the
// selection's end after the substitution (since selection's length may change)
fn inner_substitute(
  history: &mut crate::History<crate::Buffer>,
  clipboard: &mut Clipboard,
  command: &str, // Only because history needs it
  selection: (usize, usize),
  pattern: &str,
  substitute: &str,
  global: bool,
) -> Result<usize> {
  use regex::RegexBuilder;
  let regex = RegexBuilder::new(pattern)
    .multi_line(true)
    .build()
    .map_err(|e| EdError::regex_error(e, pattern))
  ?;
  // Get a buffer view to verify selection and look for a match
  let buffer_view = history.current();
  buffer_view.verify_selection(selection)?;
  let mut agg = String::new();
  for line in &buffer_view[selection.0 - 1 .. selection.1] {
    agg.push_str(&line.text);
  }
  if !regex.is_match(&agg) {
    // Since we haven't modified any state we can safely return here
    return Err(EdError::RegexNoMatch(pattern.to_owned()));
  }

  // If there was a match we can get a mutable access to the buffer
  // (creating an undo snapshot) and make the actual change.
  let buffer = history.current_mut(command.into());
  // Cut up the buffer into relevant pieces
  let mut tail = buffer.split_off(selection.1);
  let before = buffer.split_off(selection.0 - 1);
  // The before state should be saved in clipboard for all editing operations
  *clipboard = (&*before).into();

  // interpret escape sequences, then perform substitution
  // We use data from buffer_view, since it cannot have changed
  // (because we hold &mut Ed)
  let replace = substitute_escape(substitute);
  let after = if global {
    regex.replace_all(&agg, replace).to_string()
  } else {
    regex.replace(&agg, replace).to_string()
  };
  // Split on newlines and put into the buffer
  // The lines iterator doesn't care about if there is a last newline,
  // so that handles that edgecase.
  for line in after.lines() {
    buffer.push(
      Line::new( format!("{}\n", line) )
        .map_err(InternalError::InvalidLineText)?
    )
  }

  // The buffer length at this stage is the new end of the selection
  let end = buffer.len();
  // Put tail back on and return end
  buffer.append(&mut tail);
  Ok(end)
}

pub fn substitute(
  state: &mut Ed<'_>,
  pflags: &mut PrintingFlags,
  command: &str,
  selection: Option<Sel<'_>>,
  tail: &str,
) -> Result<()> {
  let selection = interpret_selection(&state, selection, state.selection)?;
  // Clip newline from tail if any
  let tail = tail.trim_end_matches('\n');
  // switch based on if tail was given or not
  if tail.is_empty() {
    // This means we use the arguments stored in state.s_args
    match &state.prev_s {
      None => return Err(EdError::DefaultSArgsUnset),
      Some(s) => {
        pflags.p = s.p;
        pflags.n = s.n;
        pflags.l = s.l;
        let end = inner_substitute(
          &mut state.history,
          &mut state.clipboard,
          command,
          selection,
          &s.pattern,
          &s.substitute,
          s.global,
        )?;
        // If we have deleted the whole selection we start sel at end,
        // in order to select line before the deleted lines. (min(end))
        // If end is smaller than 1 we have deleted to start of
        // buffer, then we use (1,0). (max(1))
        state.selection = (selection.0.min(end).max(1), end);
      }
    }
  }
  else {
    let expressions = parse_expressions(tail)?;
    if expressions.len() != 3 {
      return Err( EdError::ArgumentsWrongNr{expected: "none or 3", received: expressions.len()} );
    }
    let mut flags = parse_flags(&(expressions[2]), "gpnl")?;
    let g = flags.remove(&'g').unwrap();
    pflags.p = flags.remove(&'p').unwrap();
    pflags.n = flags.remove(&'n').unwrap();
    pflags.l = flags.remove(&'l').unwrap();
    let end = inner_substitute(
      &mut state.history,
      &mut state.clipboard,
      command,
      selection,
      &expressions[0],
      &expressions[1],
      g,
    )?;
    // If we have deleted the whole selection we start sel at end,
    // in order to select line before the deleted lines. (min(end))
    // If end is smaller than 1 we have deleted whole buffer,
    // then we use (1,0). (max(1))
    state.selection = (selection.0.min(end).max(1), end);
    // If that was valid we save all the arguments to support lone 's'
    state.prev_s = Some(Substitution{
      pattern: expressions[0].to_string(),
      substitute: expressions[1].to_string(),
      global: g,
      p: pflags.p,
      n: pflags.n,
      l: pflags.l,
    });
  }
  Ok(())
}
