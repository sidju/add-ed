//! Current error handling hack. Const strings for error messages.

// Relevant non-error consts
pub const HELP_TEXT: &str = 
"Application commands:
  q: Quit the editor. Warns on unsaved changes.
  h: Print last occured error
  H: Toggle printing errors vs just noting there was an error
  help: Print this help text
  =: Print current selection
  #: Do nothing (start of comment)
File commands:
  f: Print current filepath; or set filepath if one is given
  e: Open given filepath for editing. If none given use current filepath. Warns on unsaved changes.
  r: Append contents from given filepath to current selection. If none given use current filepath.
  w: Write contents of buffer to given filepath. If none given use current filepath.
  W: Append contents of buffer to given filepath. If none given use current filepath.
Print commands:
  Most commands below take flags p (print), n (numbered print), l (escaped print).
  (no command): Normal print by default. Also takes flags.
  z: Scroll (and print) given number of lines down from end of selection. 
  Z: Scroll (and print) given number of lines up from start of selection. 
Editing commands:
  a: Append lines entered after the command to selection. Stop line entry with lone '.' on a line.
  i: Same as 'a' but places lines before selection.
  c: Same as 'a' except it also cuts the selection into clipboard.
  d: Cut the selection into clipboard.
  y: Copy the selection into clipboard.
  x/X: Append/prepend clipboard contents to selection.
  m: Move the selection to index given after command.
  t: Copy the selection to index given after command.
  j: Join the selection into one line. (only removes newlines)
Regex commands:
  s: Uses the first character as a separator between a regex matching pattern and a replacement string.
     If no arguments are given it re-uses the arguments given last execution.
  g: Uses the first character as a separator between a regex matching pattern and any number of commands.
     If the line doesn't end with the separator it takes input until the separator is given alone on a line.
  G: Same as 'g' but only takes a pattern. The commands to run are requested for each matching line.
     Input is terminated by the separator alone on a line, just as 'g' if command line isn't separator terminated.
  v/V: Same as their 'g' counterparts except they invert the pattern."
;

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
pub const ABORTED: &str = "Aborted. To close application use 'q'.";

// Terminal interaction errors
// No carriage returns, since only used through panic messages.
pub const TERMINAL_READ: &str = "Failed to read from terminal.";
pub const TERMINAL_WRITE: &str = "Failed to write to terminal.";
pub const DISABLE_RAWMODE: &str = "Failed to clear raw mode. Either restart terminal or run 'reset'. Good luck!";

