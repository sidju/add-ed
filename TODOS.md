# Todos:

- Inject context environment variables into shell interaction.
  (File, selection_start, selection_end, prev_shell_command, if running script)
- Improve classic.rs to support all of ed's command line arguments
- Implement missing features from GNU Ed.
  - List more missing features in [README.md](README.md) (look into GNU Ed
    manual and compare to add-ed).
  - Implement missing features and remove them from listing in README.md.
    - 'g' command should accept an argument for case insensitive matching.


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
- Add tests over modification mode
- Provide a default macro-store that live-loads from a config dir in addition to
  reading from config


# Documentation fixes:

- Look over API documentation again, since refactoring has changed the API.
