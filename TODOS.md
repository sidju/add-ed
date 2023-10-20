# Todos:
- Inject context environment variables into shell interaction.
  (File, selection_start, selection_end, prev_shell_command, if running script)
- Improve classic.rs to support all of ed's command line arguments
- UI.unlock_ui, possibly see if we can require a private phantom/marker
  argument to this, preventing it from being called by any code not in the
  add_ed crate (and thus making it impossible for a library user to
  incorrectly run it from anywhere but UILock::drop)


# Look over undo/redo
- Add absolute indexing to the 'u' command (`u*0` = go to index 0 in history)
- Possibly eventually add reverse/forward snapshot label search
  (`u?^e?` would search backwards to the last previous 'e' command (just regex))
- Possibly a way to clear the history (probably as a subcommand/argument under
  'U', perhaps better as a distinct command)
- Some flags to print history in different ways (the 'U' command).
  - 'a' to print absolute indices for the snapshots
  - 'A' to print the whole history
  - integer to give a specific snapshot to print nearby snapshots to
  - '$' to print snapshots relative to the last existing snapshot


# Look over macros.
Make macros more useful by:
- Adding some per-macro configurations, such as:
  - Abort on error, abort on error except NoMatch and NoOp, or ignore errors
  - Modifies buffer or not (if not we auto undo to before execution and delete
    its potential future, ie. make modifications not have an impact)
  - Snapshot for each command or for the whole macro



# Documentation fixes:
- Look over API documentation again, since refactoring has changed the API.
