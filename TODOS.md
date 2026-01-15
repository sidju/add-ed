# Todos:

- Inject context environment variables into shell interaction.
  (File, selection_start, selection_end, prev_shell_command, if running script)
- Improve classic.rs to support all of ed's command line arguments
- Implement missing features from GNU Ed.
  - List more missing features in [README.md](README.md) (look into GNU Ed
    manual and compare to add-ed).
  - Implement missing features and remove them from listing in README.md.
    - 'g' command should accept an argument for case insensitive matching.
- Possibly eventually add reverse/forward snapshot label search
  (`u?^e?` would search backwards to the last previous 'e' command (just regex))


# Look over macros.

- Write proper tests for the local_macrostore (similar to local_io)


# Documentation fixes:

- Look over API documentation again, since refactoring has changed the API.


# Expose macro-recording data in the ed state

- Probably in some Vec<String>, to make nested macro recordings easy


