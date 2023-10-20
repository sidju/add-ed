//! Holds Error type for the crate

use std::rc::Rc;
use std::borrow::Cow;

pub type Result<T> = std::result::Result<T, EdError>;

#[macro_use]
mod internal;
pub use internal::*;

// Define structs and traits for UI and IO errors
/// A trait to mark fulfilling the requirements put upon UI error types.
pub trait UIErrorTrait: std::error::Error + as_any::AsAny + 'static {}
/// A wrapping struct for any UI's error type
///
/// To use the wrapper implement [`UIErrorTrait`] on the error type to wrap and
/// use `.into()` to convert it into this UIError wrapper.
#[derive(Clone, Debug)]
pub struct UIError {
  pub inner: Rc<dyn UIErrorTrait>,
}
impl UIError {
  /// Helper for downcasting into the internal error type
  ///
  /// Due to how finnicky this is to get right, with coercing the Rc<T> into &T
  /// before downcasting, I very much recommend using this helper.
  pub fn downcast_ref<T: UIErrorTrait>(&self) -> Option<&T> {
    use as_any::Downcast;
    (&*self.inner).downcast_ref::<T>()
  }
}
/// A trait to mark fulfilling the requirements put upon IO error types.
pub trait IOErrorTrait: std::error::Error + as_any::AsAny + 'static {}
/// A wrapper type for any IO implementation's error type
///
/// To use the wrapper implement [`IOErrorTrait`] on the error type to wrap. The
/// return types on the [`crate::IO`] trait's methods will give automatic
/// conversion via the `?` operator in a lot of cases, but in some cases it is
/// likely still needed to call `.into()` to convert.
#[derive(Clone, Debug)]
pub struct IOError {
  pub inner: Rc<dyn IOErrorTrait>,
}
impl IOError {
  /// Helper for downcasting into the internal error type
  ///
  /// Due to how finnicky this is to get right, with coercing the Rc<T> into &T
  /// before downcasting, I very much recommend using this helper.
  pub fn downcast_ref<T: IOErrorTrait>(&self) -> Option<&T> {
    use as_any::Downcast;
    (&*self.inner).downcast_ref::<T>()
  }
}

// Large trait implementations in their own files
mod display;
mod partialeq;

/// A basic enum Error implementation
///
/// Whilst it does implement `PartialEq`, partial is key in that. Since UI and
/// IO errors aren't sure to be comparable they are assumed to be equal, so 
/// library users can easily identify UI resp. IO errors and downcast them for
/// the proper comparison for the abstracted type.
#[derive(Debug, Clone)]
pub enum EdError {
  /// Internal error, usually from something OS related.
  Internal(InternalError),
  /// A holder for errors from the IO implementation.
  ///
  /// WARNING: internal equality of the held IO error will not be checked. You
  /// will need to downcast and verify this yourself if relevant. (See helper on
  /// [`IOError`].)
  IO(IOError),
  /// A holder for errors from the UI implementation.
  ///
  /// WARNING: internal equality of the held UI error will not be checked. You
  /// will need to downcast and verify this yourself if relevant. (See helper on
  /// [`UIError`].)
  UI(UIError),

  /// Execution recursed more times than [`Ed.recursion_limit`], indicating
  /// infinite recursion.
  ///
  /// Contains no details until someone writes stack unwinding for it.
  InfiniteRecursion,

  // Selection/index interpretation/validation errors
  /// Given index exceeds size of buffer.
  ///
  /// (Always given if buffer is empty)
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
  /// Tried to undo beyond start of history.
  UndoIndexNegative{relative_undo_limit: usize},
  /// Tried to redo past end of history.
  UndoIndexTooBig{index: usize, history_len: usize, relative_redo_limit: usize},
  /// Tried to given shell escape where a file path is required.
  /// Holds given path string.
  CommandEscapeForbidden(String),
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
  ArgumentsWrongNr{expected: Cow<'static, str>, received: usize},
  /// `z` command received a non numeric number of lines to scroll.
  /// Holds given argument.
  ScrollNotInt(String),
  /// `u` or `U` command couldn't interpret nr of steps to undo/redo as integer.
  /// Holds given argument.
  UndoStepsNotInt(String),
  /// `J` command received a non numeric number of columns to reflow within.
  /// Holds given argument.
  ReflowNotInt{error: String, text: String},
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

impl From<UIError> for EdError {
  fn from(e: UIError) -> Self {
    Self::UI(e)
  }
}
impl<E: UIErrorTrait> From<E> for UIError {
  fn from(e: E) -> Self {
    Self{ inner: Rc::new(e) }
  }
}
// Causes conflicting trait bounds error for now. Instead use
// Into::<UIError>::into(error).into() to convert via UIError.
//impl<E: UIErrorTrait> From<E> for EdError {
//  fn from(e: E) -> Self {
//    Self::UI(e.into())
//  }
//}
impl From<IOError> for EdError {
  fn from(e: IOError) -> Self {
    Self::IO(e)
  }
}
impl<E: IOErrorTrait> From<E> for IOError {
  fn from(e: E) -> Self {
    Self{ inner: Rc::new(e) }
  }
}
// Causes conflicting trait bounds error for now. Instead use
// Into::<IOError>::into(error).into() to convert via IOError.
//impl<E: IOErrorTrait> From<E> for EdError {
// fn from(e: E) -> Self {
//   Self::IO(e.into())
// }
//}
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
