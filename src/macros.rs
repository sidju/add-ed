use std::borrow::Cow;

use crate::Result;

// TODO, enable this later
///// How to handle undo/redo snapshotting during macro execution
//pub enum MacroSnapshottingMode {
//  /// The default mode, same behaviour as the 'g' command
//  ///
//  /// Will squash modifications into the invocation itself *AND* remove that
//  /// snapshot if it isn't changed from the previous.
//  Default,
//  /// Any modifications to the buffer are rollbacked after execution
//  RevertMutation,
//  /// Any modifications are shown as caused by the macro invocation
//  SquashModifications,
//  /// Any modifications are shown as caused by the modifying command in the
//  /// macro
//  ExposeModifications,
//}

/// A struct representing a runnable macro
///
/// It is intended to add more/change the variables, but the constructors should
/// produce instances with the same behaviour through any change.
#[non_exhaustive]
pub struct Macro {
//  /// Description / Help text for the macro
//  ///
//  /// Should describe what it does and any arguments it accepts or requires.
//  help_text: Cow<'static, str>,
  /// Input to simulate
  ///
  /// Should be a string of newline separated commands. Execution is equivalent
  /// to if this input was given on STDIN while the editor is running.
  pub input: Cow<'static, str>,
  /// The number of arguments the macro accepts
  ///
  /// None means there is no specific number, disabling validation of correct nr
  /// of given arguments before execution.
  pub arguments: Option<usize>,
  // TODO, enable this later
  // /// How the macro execution interacts with undo/redo snapshotting
  // snapshotting_mode: MacroSnapshottingMode,
}
impl Macro {
  /// Construct a macro
  ///
  /// Creates a macro with the given text as command input and the given nr of
  /// allowed arguments
  pub fn new<T: Into<Cow<'static, str>>>(
    input: T,
    arguments: usize,
  ) -> Self {
    Self{
      input: input.into(),
      arguments: Some(arguments),
    }
  }
  /// Construct a macro that takes any number of commands
  ///
  /// Creates a macro with the given text as command input and disables
  /// validation of nr of arguments to the macro. This is intended for macros
  /// using the "all arguments" substitutor instead of numbered argument
  /// substitutors.
  pub fn without_arg_validation<T: Into<Cow<'static, str>>>(
    input: T,
  ) -> Self {
    Self{
      input: input.into(),
      arguments: None,
    }
  }
}

pub fn apply_arguments(
  mac: Macro,
  args: &[&str],
) -> Result<String> {
  if let Some(x) = mac.arguments {
    if args.len() != x { return Err(EdError::ArgumentsWrongNr{
      expected: format!("exactly {}", x).into(),
      received: usize,
    }); }
  }

  // Iterate over every character in the macro to find "$<char>", replace with the
  // matching argument (or $ in the case of $$)
}
