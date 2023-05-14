/// Defines all possible parsing errors
pub enum ParsingError {
  // Index parsing errors
  /// Special index character found after start of index
  SpecialIndex(String),
  /// Given index couldn't be parsed as a number
  IndexNotInt(String),
  /// Offset part of index couldn't be parsed as a number
  OffsetNotInt(String),
  /// Multiple indices with unclear relation (for example `'x2`)
  UnrelatedIndices(String),
  /// Unfinished index, a special index without its arguments
  UnfinishedIndex(String),

  // Command and argument parsing errors
  /// The given command doesn't exist
  UndefinedCommand(char),
  /// Argument list ended with `\`
  EscapedArgumentListEnd(String),
  /// Wrong number of arguments, (expected, received)
  WrongNrArguments((usize, usize)),
  /// `z` command received a non numeric number of lines to scroll
  ScrollNotInt(String),
  /// `u` or `U` command couldn't interpret nr of steps to undo/redo as integer
  UndoRedoNotInt(String),
  /// `J` command received a non numeric number of columns to reflow within
  ReflowNotInt(String),
  /// The macro invoked wasn't found
  UndefinedMacro(String),

  // Flag parsing errors
  /// Same flag appears more than once
  DuplicateFlag(char),
  /// Unexpected flag was received
  UndefinedFlag(char),
}
