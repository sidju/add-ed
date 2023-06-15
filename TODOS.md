# Todos:
- Consider adding 'R' command, as 'r' but inserts before selection.
- Inject context environment variables into shell interaction.
  (File, selection_start, selection_end, prev_shell_command, if running script)
- Improve classic.rs to support all of ed's command line arguments
- Look over API documentation again, since refactoring has changed the API.
- Implement parsing under the trait FromStr instead?
- Consider if Buffer should even be an object anymore, since History exists.

# Testing improvements:
## Unit tests:
- buffer_api test should be split out into multiple tests.
