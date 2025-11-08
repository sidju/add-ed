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

- Some flags to print history in different ways (the 'U' command).
  - 'a' to print absolute indices for the snapshots
  - 'A' to print the whole history
  - integer to give a specific snapshot to print nearby snapshots to
  - '$' to print snapshots relative to the last existing snapshot
- Possibly eventually add reverse/forward snapshot label search
  (`u?^e?` would search backwards to the last previous 'e' command (just regex))
- Possibly a way to clear the history (probably as a subcommand/argument under
  'U', perhaps better as a distinct command)


# Look over macros.

- Provide a default macro-store that live-loads from a config dir in addition to
  reading from config


# Documentation fixes:

- Look over API documentation again, since refactoring has changed the API.


# Expose macro-recording data in the ed state

- Probably in some Vec<String>, to make nested macro recordings easy


# Minor refactors

- Really consider if we really should be using usize for all the parsing, it
  would be smart to either really check against under-/overflow and properly
  error on that or use something like i128 and check that it is within usize.

- Check that all Error variants are covered in PartialEq
  (I'm quite confident only one of the HistoryIndex variants are...)
