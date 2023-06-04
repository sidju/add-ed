//! Holds Error type for the crate

pub type Result<T> = std::result::Result<T, EdError>;

mod internal;
pub use internal::*;

// Define traits for UI and IO errors
pub trait UIError: std::error::Error + as_any::AsAny + 'static {}
pub trait IOError: std::error::Error + as_any::AsAny + 'static {}

// Large trait implementations in their own files
mod display;
mod partialeq;

/// A basic enum Error implementation
///
/// Whilst it does implement `PartialEq`, partial is key in that. Since UI and
/// IO errors aren't sure to be comparable they are assumed to be equal, so 
/// library users can easily identify UI resp. IO errors and downcast them for
/// the proper comparison for the abstracted type.
#[derive(Debug)]
pub enum EdError {
  /// Internal error, usually from something OS related.
  Internal(InternalError),
  /// A holder for errors from the IO implementation.
  IO(Box<dyn IOError>),
  /// A holder for errors from the UI implementation.
  UI(Box<dyn UIError>),

  // Selection/index interpretation/validation errors
  /// Given index exceeds size of buffer.
  IndexTooBig{index: usize, buffer_len: usize},
  /// Index 0 isn't a valid line.
  Line0Invalid,
  /// Selection empty or inverted.
  /// Holds the interpreted bad selection.
  SelectionEmpty((usize, usize)),
  /// Given command doesn't allow any selection and one was given.
  SelectionForbidden,

  // Command+argument+flag interpretation errors
  /// Unsaved changes when about to non-forcibly drop/delete buffer.
  UnsavedChanges,
  /// Selection and arguments were given that makes its command do nothing.
  NoOp,
  /// Given number of undo steps exceeds existing history.
  UndoStepsInvalid{undo_steps: isize, undo_range: std::ops::Range<isize>},
  /// Tried to set a shell escape as default file.
  /// Holds given path string.
  DefaultFileInvalid(String),
  /// `k` or `K` command received an invalid character to tag with.
  /// Holds given argument string.
  TagInvalid(String),
  /// Given tag found no match.
  /// Holds the used tag.
  TagNoMatch(char),
  /// Any regex operation received an invalid regex or substitution.
  RegexInvalid{regex: String, error: regex::Error},
  /// Given regex found no match.
  /// Holds the used regex.
  RegexNoMatch(String),
  /// Flags asked to print after the whole buffer was deleted.
  PrintAfterWipe,

  // Errors related to unset state variables
  /// Tried to get default shell command, but it isn't yet set
  DefaultFileUnset,
  /// Tried to get default shell command, but it isn't yet set
  DefaultShellCommandUnset,
  /// Tried to get default `s` arguments, but it isn't yet set
  DefaultSArgsUnset,

  // Index parsing errors
  /// Special index character found after start of index.
  IndexSpecialAfterStart{prior_index: String, special_index: char},
  /// Given index couldn't be parsed as a number.
  /// Holds its text.
  IndexNotInt(String),
  /// Offset part of index couldn't be parsed as a number.
  /// Holds its text.
  OffsetNotInt(String),
  /// Multiple indices with unclear relation (for example `'x2`)
  IndicesUnrelated{prior_index: String, unrelated_index: String},
  /// Unfinished index, a special index without its arguments.
  /// Holds its text.
  IndexUnfinished(String),

  // Command and argument parsing errors
  /// The given command doesn't exist.
  /// Holds given command char.
  CommandUndefined(char),
  /// Argument list ended with `\`.
  /// Holds whole argument list.
  ArgumentListEscapedEnd(String),
  /// Wrong number of argument.
  ArgumentsWrongNr{expected: &'static str, received: usize},
  /// `z` command received a non numeric number of lines to scroll.
  /// Holds given argument.
  ScrollNotInt(String),
  /// `u` or `U` command couldn't interpret nr of steps to undo/redo as integer.
  /// Holds given argument.
  UndoStepsNotInt(String),
  /// `J` command received a non numeric number of columns to reflow within.
  /// Holds given argument.
  ReflowNotInt(String),
  /// The macro invoked wasn't found.
  /// Holds given macro name.
  MacroUndefined(String),

  // Flag parsing errors
  /// Same flag appears more than once.
  /// Holds duplicated flag.
  FlagDuplicate(char),
  /// Unexpected flag was received.
  /// Holds undefined flag.
  FlagUndefined(char),
}

impl std::error::Error for EdError {}

impl From<Box<dyn UIError>> for EdError {
  fn from(e: Box<dyn UIError>) -> Self {
    Self::UI(e)
  }
}
impl<E: UIError> From<E> for Box<dyn UIError> {
  fn from(e: E) -> Self {
    Box::new(e)
  }
}
impl From<Box<dyn IOError>> for EdError {
  fn from(e: Box<dyn IOError>) -> Self {
    Self::IO(e)
  }
}
impl<E: IOError> From<E> for Box<dyn IOError> {
  fn from(e: E) -> Self {
    Box::new(e)
  }
}
impl From<InternalError> for EdError {
  fn from(e: InternalError) -> Self {
    Self::Internal(e)
  }
}

impl EdError {
  pub fn regex_error<S: Into<String>>(error: regex::Error, regex: S) -> Self {
    Self::RegexInvalid{regex: regex.into(), error: error}
  }
}
