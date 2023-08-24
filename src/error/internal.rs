/// Defines non-user errors
///
/// Expected to report reaching unreachable code, over-/under-flow, etc., if
/// such is caught. The recommendation when receiving any of these will be:
/// "Save your work, close the editor, and create an issue at
/// https://github.com/sidju/add-ed which describes what you did to trigger
/// this error".
#[derive(Debug, Clone, PartialEq)]
pub enum InternalError {
  /// Undo history is too big to be handled. Save, quit, reopen if you somehow
  /// run an editing session this long... And please tell me how you did it.
  UndoHistoryTooLarge,
  /// A code path that shouldn't be reachable was reached. We use this error
  /// instead of panic to not drop buffer without letting user save
  UnreachableCode{file: &'static str, line: u32, column: u32},
}

/// A utility macro for panic free coding
///
/// Creates an EdError that details what went wrong where, so you can debug it
/// when it won't drop data.
macro_rules! ed_unreachable {
  () => { Err(EdError::Internal(InternalError::UnreachableCode{
    file: file!(),
    line: line!(),
    column: column!(),
  })) }
}
