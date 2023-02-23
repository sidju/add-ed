use super::*;

pub(super) fn substitute<I: IO>(
  state: &mut Ed<'_, I>,
  pflags: &mut PrintingFlags,
  selection: Option<Sel<'_>>,
  tail: &str,
) -> Result<(), &'static str> {
  let selection = interpret_selection(selection, state.selection, state.buffer)?;
  // Clip newline from tail if any
  let tail = tail.trim_end_matches('\n');
  // switch based on if tail was given or not
  if tail.is_empty() {
    // This means we use the arguments stored in state.s_args
    match &state.prev_s {
      None => return Err(NO_PRIOR_S),
      Some(s) => {
        pflags.p = s.p;
        pflags.n = s.n;
        pflags.l = s.l;
        let end = state.buffer.search_replace((&s.pattern, &s.substitute), selection, s.global)?;
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
    if expressions.len() != 3 { return Err(EXPRESSION_TOO_SHORT); }
    let mut flags = parse_flags(&(expressions[2]), "gpnl")?;
    let g = flags.remove(&'g').unwrap();
    pflags.p = flags.remove(&'p').unwrap();
    pflags.n = flags.remove(&'n').unwrap();
    pflags.l = flags.remove(&'l').unwrap();
    let end = state.buffer.search_replace((&expressions[0], &expressions[1]), selection, g)?;
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

pub(super) fn global<I: IO>(
  state: &mut Ed<'_, I>,
  ui: &mut dyn UI,
  selection: Option<Sel<'_>>,
  command: char,
  tail: &str,
) -> Result<(), &'static str> {
  let selection = interpret_selection(selection, state.selection, state.buffer)?;
  // Since this command may take input we need to check just as carefully as with a, i, c
  verify_selection(state.buffer, selection)?;
  let mut expressions = parse_expressions(tail)?;
  if expressions.len() < 2 { return Err(EXPRESSION_TOO_SHORT); }
  // We first try to mark all matching lines, to tell if there is any issue
  state.buffer.mark_matching(&expressions[0], selection, command == 'v')?;
  // Then we get the script to run against them, if not already given
  // First grab commands given on command line
  let mut commands: Vec<String> = expressions.split_off(1).iter().map(|s| s.to_string()).collect();
  // If the last command in that list is not empty it means the list was not terminated,
  // so we take more from input
  if commands.last().map(|s| s.trim()) != Some("") {
    // expressions.len() would be 0 if no command, so safe to unwrap
    let mut input = ui.get_input(
      state.see_state(),
      tail.chars().next().unwrap(),
      #[cfg(feature = "initial_input_data")]
      None,
    )?;
    commands.append(&mut input);
  }
  else {
    // If the last command was empty we should pop it, since since it will
    // otherwise cause an unexpected print
    commands.pop();
  }
  // If no other command given, default to print
  if commands.is_empty() {
    commands.push("p\n".to_string())
  }
  // After command collection we get the matching lines to run them at and do so
  while let Some(index) = state.buffer.get_marked()? {
    // Use dummy UI to recurse while supporting text input
    let mut scripted = ScriptedUI{
      input: commands.iter().cloned().collect(),
      print_ui: Some(ui),
    };
    state.selection = (index, index);
    state.run_macro(&mut scripted)?;
  }
  Ok(())
}

pub(super) fn global_interactive<I: IO>(
  state: &mut Ed<'_, I>,
  ui: &mut dyn UI,
  selection: Option<Sel<'_>>,
  command: char,
  tail: &str,
) -> Result<(), &'static str> {
  let selection = interpret_selection(selection, state.selection, state.buffer)?;
  // Since this command takes input we need to check just as carefully as with a, i, c
  verify_selection(state.buffer, selection)?;
  let expressions = parse_expressions(tail)?;
  if expressions.len() != 2 { return Err(EXPRESSION_TOO_SHORT); }
  if !expressions[1].is_empty() && expressions[1] != "\n" { return Err(UNDEFINED_FLAG); }
  // Mark first, to check if the expression is valid
  state.buffer.mark_matching(&expressions[0], selection, command == 'V')?;
  // With all data gathered we fetch and iterate over the lines
  while let Some(index) = state.buffer.get_marked()? {
    // Print the line, so the user knows what they are changing
    ui.print_selection(state.see_state(), (index, index), state.n, state.l)?;
    // Get input and create dummy-ui with it
    // expressions.len() == 2 implies that a separator was given
    let input = ui.get_input(
      state.see_state(),
      tail.chars().next().unwrap(),
      #[cfg(feature = "initial_input_data")]
      None,
    )?;
    let mut scripted = ScriptedUI{
      input: input.into(),
      print_ui: Some(ui),
    };
    state.selection = (index, index);
    state.run_macro(&mut scripted)?;
  }
  Ok(())
}
