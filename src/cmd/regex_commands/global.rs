use super::*;

// Helper for all the "do X on matching lines" commands
//
// Sets up the relevant recursion depth in the matched Vec on every line in the
// selection. Also ensures no data exists at a higher recursion depth, as that
// would be outdated.
//
// As get_matching already needs to default to false where new lines have an
// empty matched Vec, we leave an index empty instead of false where possible
// (should save a little memory).
fn mark_matching(
  state: &mut Ed<'_>,
  selection: (usize, usize),
  pattern: &str,
  inverse: bool,
  recursion_depth: usize,
) -> Result<()> {
  use regex::RegexBuilder;
  let buffer = state.history.current();
  buffer.verify_selection(selection)?;
  let regex = RegexBuilder::new(pattern)
    .multi_line(true)
    .build()
    .map_err(|e| EdError::regex_error(e, pattern))
  ?;
  let mut match_found = false;
  for (i, line) in buffer.iter().enumerate() {
    let mut matched_vec = line.matched.borrow_mut();
    // To ensure that there are no outdated values for this recursion depth in the
    // vec we shrink to one less than the size that would hold current depth.
    matched_vec.truncate(recursion_depth);
    // If we are actually in the selection we check if lines match
    if i >= selection.0 -1 && i < selection.1 {
      // When we actually match we enlarge the vec to the needed length, adding
      // false to fill out, to enable writing true onto current index.
      if regex.is_match(&(line.text)) ^ inverse {
        matched_vec.resize(recursion_depth, false);
        matched_vec.push(true);
        match_found = true;
      }
    }
  }
  if !match_found {
    Err(EdError::RegexNoMatch(pattern.to_owned()))
  } else {
    Ok(())
  }
}
// Helper for all the "do X on matching lines" commands
//
// Gets and wipes matched lines at a given recursion depth.
fn get_marked(
  state: &mut Ed<'_>,
  recursion_depth: usize,
) -> Option<usize> {
  let buffer = state.history.current();
  for (index,line) in buffer.iter().enumerate() {
    let mut matched_vec = line.matched.borrow_mut();
    match matched_vec.get(recursion_depth) {
      Some(x) => {
        let tmp = *x;
        // Shrinking vec to not include current depth to remove, cleans up and
        // thus unsets matched for this line+recursion_depth combination. 
        matched_vec.truncate(recursion_depth);
        if tmp { return Some(index + 1); }
      },
      None => (),
    }
  }
  None
}

pub fn global(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  selection: Option<Sel<'_>>,
  command: char,
  tail: &str,
  recursion_depth: usize,
) -> Result<()> {
  let selection = interpret_selection(&state, selection, state.selection)?;
  // Since this command takes input we need to check inputs early
  let mut expressions = parse_expressions(tail)?;
  if expressions.len() < 2 {
    return Err( EdError::ArgumentsWrongNr{expected: "2 or more".into(), received: expressions.len()} );
  }
  // We first try to mark all matching lines, to tell if there is any issue
  mark_matching(state, selection, &expressions[0], command == 'v', recursion_depth + 1)?;
  // Then we get the script to run against them, if not already given
  // First grab commands given on command line
  let mut commands: Vec<String> = expressions.split_off(1).iter().map(|s| s.to_string()).collect();
  // If the last command in that list is not empty it means the list was not terminated,
  // so we take more from input
  if commands.last().map(|s| s.trim()) != Some("") {
    // expressions.len() would be 0 if no command, so safe to unwrap
    let mut input = ui.get_input(
      state,
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
  while let Some(index) = get_marked(state, recursion_depth + 1) {
    // Use dummy UI to recurse while supporting text input
    let mut scripted = ScriptedUI{
      input: commands.iter().cloned().collect(),
      print_ui: Some(ui),
    };
    state.selection = (index, index);
    loop {
      if state.private_get_and_run_command(&mut scripted, recursion_depth + 1)? {
        break;
      }
    }
  }
  Ok(())
}

pub fn global_interactive(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  selection: Option<Sel<'_>>,
  command: char,
  tail: &str,
  recursion_depth: usize,
) -> Result<()> {
  let selection = interpret_selection(&state, selection, state.selection)?;
  // Since this command takes input we need to check inputs early
  let expressions = parse_expressions(tail)?;
  if expressions.len() != 2 {
    return Err( EdError::ArgumentsWrongNr{expected: "2".into(), received: expressions.len()} );
  }
  if !expressions[1].is_empty() && expressions[1] != "\n" {
    return Err(EdError::FlagUndefined(expressions[1].chars().next().unwrap()));
  }

  // Mark first, to check if the expression is valid
  mark_matching(state, selection, &expressions[0], command == 'V', recursion_depth + 1)?;
  // With all data gathered we fetch and iterate over the lines
  while let Some(index) = get_marked(state, recursion_depth + 1) {
    // Print the line, so the user knows what they are changing
    ui.print_selection(state, (index, index), state.n, state.l)?;
    // Get input and create dummy-ui with it
    // expressions.len() == 2 implies that a separator was given
    let input = ui.get_input(
      state,
      tail.chars().next().unwrap(),
      #[cfg(feature = "initial_input_data")]
      None,
    )?;
    let mut scripted = ScriptedUI{
      input: input.into(),
      print_ui: Some(ui),
    };
    state.selection = (index, index);
    loop {
      if state.private_get_and_run_command(&mut scripted, recursion_depth + 1)? {
        break;
      }
    }
  }
  Ok(())
}
