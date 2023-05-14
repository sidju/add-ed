/// Defines all errors that occur based on state
///
/// Some errors seem like they are obvious at parsing, but they can also arise
/// unexpectedly based on state so they are placed here.
pub enum ExecutionError {
  // Selection/index interpretation/validation errors
  /// Given index exceeds size of buffer
  IndexTooBig,
  /// Index 0 isn't a valid line
  InvalidLine0,
  /// Selection empty or inverted
  SelectionEmpty,
  /// Given command doesn't allow any selection
  SelectionForbidden,

  // Command+argument+flag interpretation errors
  /// Unsaved changes when about to non-forcibly drop/delete buffer
  UnsavedChanges,
  /// An argument was given that makes its command do nothing
  NoOpArgument,
  /// Given number of undo steps exceeds existing history
  InvalidUndoSteps,
  /// Tried to set a shell escape as default file
  InvalidDefaultFile,
  /// `k` or `K` command received an invalid character to tag with
  InvalidTag(char),
  /// Any regex operation received an invalid regex or substitution
  InvalidRegex,
  /// Flags asked to print after the whole buffer was deleted
  PrintAfterWipe,
  /// Given regex found no match
  NoRegexMatch,
  /// Given tag found no match
  NoTagMatch,

  // Errors related to unset state variables
  /// Tried to get default file but it isn't yet set
  DefaultFileUnset,
  /// Tried to get default shell command but it isn't yet set
  DefaultShellCommandUnset,
  /// Tried to get default `s` arguments, but it isn't yet set
  DefaultSArgsUnset,
  /// Tried to get last error but it isn't yet set
  LastErrorUnset,
}
