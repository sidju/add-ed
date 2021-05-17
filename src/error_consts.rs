// Relevant non-error consts
pub const HELP_TEXT: &str = 
"Application commands:
  q: Quit the application
  Q: Quit the application regardless of unsaved changes
  h: Print last occured error
  H: Toggle printing errors as they occur
  ?: Print this help text
File commands:
  f: If filepath is given, sets active file to that. Else prints current filepath.
  e: Closes current file and opens given path.
  E: Closes current file and opens given path regardless of unsaved changes
  r: Append data from given path to selection.
  w: Write the selection to given filepath. Default selection is whole file and default path active file.
  W: Same as 'w' but appends to given filepath instead of overwriting.
Print commands:
  p: Print the selection.
  n: Print the selection with line numbers.
  l: Print the selection with escapes on some invisible characters.
Basic editing commands:
  a: Append lines entered after the command to selection. Stop line entry with only '.' on a line.
  i: Insert. Same as 'a' but places before selection.
  c: Change. Replace selection with lines entered after the command. Stop line ently with only '.' on a line.
  d: Delete. Deletes the selection.
Advanced editing commands:
  m: Move selection to index given after command.
  t: Transfer (copy) selection to index given after command.
  j: Join selected lines into one line.
Regex commands:
  s: Substitute selection with regex replace very similar to 'sed'.
Special cases:
  No command: Takes the given selection (if any) and sets current selection to that.
";
// Pre-command parsing errors
pub const INDEX_PARSE: &str = "Could not parse given index.";
pub const NO_COMMAND: &str = "No valid command given.";
pub const NO_SELECTION: &str = "No prior selection exists.";
pub const NEGATIVE_INDEX: &str = "Resulting index is negative.";

// Command handling errors
pub const UNDEFINED_COMMAND: &str = "Command not defined.";
pub const SELECTION_FORBIDDEN: &str = "That command doesn't take a selection.";
pub const UNSAVED_CHANGES: &str = "Unsaved changes. Force with the capitalised version of your command or save with 'w'.";
pub const NO_ERROR: &str = "No errors recorded.";
pub const NO_FILE: &str = "No file set.";

// Post-command parsing errors
pub const EXPRESSION_TOO_SHORT: &str = "Expression too short or not closed.";
pub const NO_PRIOR_S: &str = "'s' has not been run before, so no default exists.";
pub const UNDEFINED_FLAG: &str = "Unknown flag entered.";
pub const DUPLICATE_FLAG: &str = "A flag was entered twice.";

// Buffer command errors
pub const BUFFER_NOT_IMPLEMENTED: &str = "Feature not implemented in buffer.";
pub const INDEX_TOO_BIG: &str = "Selection overshoots buffer length.";
pub const SELECTION_EMPTY: &str = "Selection empty or inverted.";
pub const MOVE_INTO_SELF: &str = "Cannot move selection into itself.";
pub const INVALID_TAG: &str = "Invalid line tag entered.";
pub const INVALID_REGEX: &str = "Invalid regex entered.";
pub const NO_MATCH: &str = "No line matched requirements.";

// File interaction errors
pub const PERMISSION_DENIED: &str = "Could not open file. Permission denied.";
pub const NOT_FOUND: &str = "Could not open file. Not found or invalid path.";
pub const UNKNOWN: &str = "Unknown error while reading file.";

// UI errors
pub const NO_INPUT: &str = "Failed to get input."; // Probably only used by DummyUI, which has limited input.

// Terminal interaction errors
// No carriage returns, since only used through panic messages.
pub const TERMINAL_READ: &str = "Failed to read from terminal.";
pub const TERMINAL_WRITE: &str = "Failed to write to terminal.";
pub const DISABLE_RAWMODE: &str = "Failed to clear raw mode. Either restart terminal or run 'reset'. Good luck!";

