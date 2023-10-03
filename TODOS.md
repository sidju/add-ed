# Todos:
- Look over when we save to Ed.prev_shell_command, currently before execution.
  (Could be nice to check that it runs successfully first, but for some
  commands (eg. compilation) that could make it unusable...)
- Inject context environment variables into shell interaction.
  (File, selection_start, selection_end, prev_shell_command, if running script)
- Improve classic.rs to support all of ed's command line arguments
- Implement parsing under the trait FromStr instead?
  (using .parse() would look nice, but current way is probably clearer)
- UI.unlock_ui, possibly see if we can require a private phantom/marker
  argument to this, preventing it from being called by any code not in the
  add_ed crate (and thus making it impossible for a library user to
  incorrectly run it from anywhere but UILock::drop)
- Possibly add a private empty variable to LocalIO to enforce using the
  constructor (removing the need to document recommending using the
  constructor).


# Look over command documentation
Currently the help text is very outdated, and it is also getting unreasonably
long. We should probably change out the help text for a minimal summary and
write up a proper manual over commands.

One way to improve this would be to write a proper markdown documentation to
display via 'termimad' (has its own raw-mode viewer). We would probably just
embedd the contents of the .md file in a public constant and let whatever
application uses this library figure out how to present it to the user, but
markdown should be a reasonable option for somewhat rich text which is OK to
print without rendering (as long as we keep it under 80 columns).


# Look over undo/redo
- Add absolute indexing to the 'u' command (`u*0` = go to index 0 in history)
- Possibly eventually add reverse/forward snapshot label search
  (`u?^e?` would search backwards to the last previous 'e' command (just regex))
- Possibly a way to clear the history (probably as a subcommand/argument under
  'U', perhaps better as a distinct command)
- Improve 'U' command, print relative indices of all snapshots.
- Some flags to print history in different ways (the 'U' command).
  - 'a' to print absolute indices for the snapshots
  - 'A' to print the whole history
  - integer to give a specific snapshot to print nearby snapshots to
  - '$' to print snapshots relative to the last existing snapshot


# Look over macros.
Make macros useful by:
- Adding support for arguments
- Adding some per-macro configurations, such as:
  - Abort on error, abort on error except NoMatch and NoOp, or ignore errors
  - Modifies buffer or not (if not we auto undo to before execution and delete
    its potential future, ie. make modifications not have an impact)
  - Snapshot for each command or for the whole macro



# Documentation fixes:
- Look over API documentation again, since refactoring has changed the API.
- Ed, methods should generally be clearer with what error they will return
  under what circumstance.
  (
  Returns any error from command execution vs. returns UI errors that occur
  when getting command vs. returns UI errors that occur when printing any
  error that occured internally.
  Those who write an infallible UI should be able to know what methods may
  no longer return errors.
  )
