/// Defines non-user errors
///
/// Expected to report reaching unreachable code, over-/under-flow, etc., if
/// such is caught. The recommendation when receiving any of these will be:
/// "Save your work, close the editor, and create an issue at
/// https://github.com/sidju/add-ed which describes what you did to trigger
/// this error".
pub enum InternalError {
  /// Undo history is too big to be handled. Save, quit, reopen if you somehow
  /// run an editing session this long... And please tell me how you did it.
  UndoHistoryTooLarge,
}
