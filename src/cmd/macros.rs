use super::*;

pub fn run_macro(
  state: &mut Ed<'_>,
  ui: &mut dyn UI,
  recursion_depth: usize,
  selection: Option<Sel<'_>>,
  clean_command: &str,
  clean: &str,
) -> Result<()> {
  let given_selection = if selection.is_some() {
    let s = interpret_selection(&state, selection, state.selection)?;
    state.history.current().verify_selection(s)?;
    Some(s)
  }
  else {
    None
  };
  // Sloppy argument parsing into list, improve by using same logic as 'g' and 's'
  let mut args = clean.trim_start().split(' ');
  let macro_name = args.next().unwrap_or("");
  let args: Vec<&str> = args.collect();
  match state.macro_getter.get_macro(macro_name)? {
    Some(m) => {
      // (This data isn't needed in every branch, but doesn't hurt to save)
      let orig_dont_snapshot = state.history.dont_snapshot;
      let orig_selection = state.selection;

      // Prepare to obey modification mode
      match m.modification_mode {
        // If revert is set we must create a snapshot we discard to revert
        ModificationMode::Revert => {
          state.history.force_create_snapshot(format!(
            "revert_snapshot: {clean_command}"
          ));
          state.history.dont_snapshot = true;
        }
        // For expose we don't need to do anything, just run the commands
        ModificationMode::Expose => {},
        // For squash/default create a normal snapshot and set dont_snapshot
        ModificationMode::Default | ModificationMode::Squash => {
          state.history.snapshot(clean_command.into());
          state.history.dont_snapshot = true;
        },
      }

      if let Some(selection) = given_selection {
        state.selection = selection;
      }


      // TODO: change so this conforms to error handling
      let res = state.private_run_macro(ui, m, &args, recursion_depth+1);

      // Re-set snapshotting after
      state.history.dont_snapshot = orig_dont_snapshot;

      // Apply ModificationMode
      match m.modification_mode {
        ModificationMode::Default => {
          // If orig_dont_snapshot this isn't our job, this is nested
          // macro execution and the top-level macro is responsible for
          // handling this
          if !orig_dont_snapshot {
            // On error, abort and try to not leave a trace
            if res.is_err() {
              state.history.delete_present()?;
              state.selection = orig_selection;
            } else {
              state.history.dedup_present()?;
            }
          }
        },
        // Delete present to revert changes without creating history
        ModificationMode::Revert => {
          state.history.delete_present()?;
          if res.is_err() {
            state.selection = orig_selection;
          }
          else if let Some(selection) = given_selection {
            state.selection = selection;
          }
        },
        // Squashing occurs by default
        ModificationMode::Squash => {
          // See Default branch for reasoning
          if res.is_err() && !orig_dont_snapshot {
            state.history.delete_present()?;
            state.selection = orig_selection;
          }
        },
        // Expose is done by not setting don't snapshot
        ModificationMode::Expose => (),
      }

      res
    },
    None => Err(EdError::MacroUndefined(macro_name.to_owned())),
  }?;
  Ok(())
}
