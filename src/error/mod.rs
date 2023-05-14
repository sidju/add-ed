//! Holds Error type for the crate

use std::error::Error;

pub type Result<T> = std::result::Result<T, EdError>;

mod internal;
pub use internal::*;


/// Defines all possible parsing errors
/// Defines all errors that occur based on state
///
/// Some errors seem like they are obvious at parsing, but they can also arise
/// unexpectedly based on state so they are placed here.
pub enum ExecutionError {
}
/// A basic enum Error implementation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdError{
  /// Catches internal errors, usually from something OS related.
  Internal(InternalError),
  /// A holder for errors from the IO implementation
  IO(Box<dyn Error>),
  /// A holder for errors from the UI implementation
  UI(Box<dyn Error>),

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
  /// Tried to get default shell command but it isn't yet set
  DefaultShellCommandUnset,
  /// Tried to get default `s` arguments, but it isn't yet set
  DefaultSArgsUnset,

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
  WrongNrArguments((&'static str, usize)),
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
