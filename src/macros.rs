use std::borrow::Cow;

use crate::{Result, EdError};

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

/// Trait over different ways to get macros by name
///
/// The intent is to allow for different methods of storing macros without
/// requiring loading them in at editor startup. For example reading macros from
/// filepaths based on their names, or perhaps from environment variables.
///
/// A ready implementation exists for HashMap, if you prefer to load in at
/// startup for infallible macro getting during execution. A very good option if
/// if you embedd your macro declarations in your editor's main config file.
pub trait MacroGetter {
  fn get_macro(&self, name: &str) -> Result<Option<&Macro>>;
}

impl MacroGetter for std::collections::HashMap<&str, Macro> {
  fn get_macro(&self, name: &str) -> Result<Option<&Macro>> {
    Ok(self.get(name))
  }
}

/// Parse the macro and its arguments into a command string
///
/// Will error if the Macro expects another number of arguments than the given.
/// Will not error on malformed Macro, instead ignoring non-inserting '$'
/// instances and insert nothing for non existing arguments.
pub fn apply_arguments<
  S: std::ops::Deref<Target = str>,
>(
  mac: &Macro,
  args: &[S],
) -> Result<String> {
  // Verify arguments
  if let Some(x) = mac.arguments {
    if args.len() != x { return Err(EdError::ArgumentsWrongNr{
      expected: format!("{}", x).into(),
      received: args.len(),
    }); }
  }
  // Iterate over every character in the macro to find "$<char>", replace with the
  // matching argument (or $ in the case of $$)
  // We construct a small state machine
  let mut active_dollar_index = None;
  let mut output = String::new();
  let mut partial_number = String::new();
  for (i,c) in mac.input.char_indices() {
    match (c, active_dollar_index) {
      // A first dollar sign, start of substitution sequence
      ('$', None) => {
        active_dollar_index = Some(i);
      },
      // Means we got "$$", which should be "$" in output
      ('$', Some(j)) if j+1 == i => {
        output.push('$');
        active_dollar_index = None;
      },
      // We are receiving the digits for the integer, so we aggregate the digits
      // until we reach the end of them and parse them.
      (x, Some(_)) if x.is_ascii_digit() => {
        partial_number.push(x);
      },
      // Means we received a bad/empty escape, a dollar sign followed by
      // something else than another dollar sign or an integer.
      // Handled by not substituting, just forwarding the input we got.
      (x, Some(j)) if j+1 == i => {
        output.push('$');
        output.push(x);
        active_dollar_index = None;
      },
      // Means we have reached end of a chain of at least one digit and should
      // substitute in the corresponding argument
      // (implicit `if j > i+1 && !x.is_ascii_digit()` due to if clauses in
      // above matches)
      (x, Some(_)) => {
        // Safe to unwrap as we give it at least one digit and only ascii digits
        let index = partial_number.parse::<usize>().unwrap();
        partial_number.clear();
        if index == 0 {
          // Insert all the arguments space separated
          for (i, arg) in args.iter().enumerate() {
            if i != 0 { output.push(' '); } // Add spaces only between args
            output.push_str(&arg);
          }
        }
        else {
          // Insert the argument (default to none if not given)
          output.push_str(args
            .get(index-1)
            .map(|x|->&str {&x})
            .unwrap_or("")
          );
        }
        // If we are now on another dollar we note that, else put in the
        // current character into output
        match x {
          '$' => {
            active_dollar_index = Some(i);
          },
          x => {
            active_dollar_index = None;
            output.push(x);
          },
        }
      },
      // The normal case, just write in the char into the output
      (x, None) => {
        output.push(x);
      },
    }
  }
  // After looping, check that all variables were handled
  if let Some(_) = active_dollar_index {
    if !partial_number.is_empty() {
      // Parse out a substitution that clearly was at the end of the macro
      // (basically a copy of end of chain handling above)
      let index = partial_number.parse::<usize>().unwrap();
      partial_number.clear();
      if index == 0 {
        // Insert all the arguments space separated
        for (i, arg) in args.iter().enumerate() {
          if i != 0 { output.push(' '); } // Add spaces only between args
          output.push_str(&arg);
        }
      }
      else {
        // Insert the argument (default to none if not given)
        output.push_str(args
          .get(index-1)
          .map(|x|->&str {&x})
          .unwrap_or("")
        );
      }
    }
    else {
      // Insert lonely '$' that was clearly at the end of the macro
      output.push('$');
    }
  }
  // Finally, return the constructed output
  Ok(output)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn argument_substitution() {
    let mac = Macro::new("$1 world. Test$2", 2);
    let args = ["Hello", "ing"];
    let output = apply_arguments(&mac, &args).unwrap();
    assert_eq!(
      &output,
      "Hello world. Testing",
      "$<integer> should be replaced with argument at index <integer> - 1."
    );
  }

  #[test]
  fn dollar_escaping() {
    let mac = Macro::new("$$1 and $1.", 1);
    let args = ["one dollar"];
    let output = apply_arguments(&mac, &args).unwrap();
    assert_eq!(
      &output,
      "$1 and one dollar.",
      "$$ in macro should be replaced with $, to enable escaping $ characters."
    );
  }

  #[test]
  fn ignore_bad_escape() {
    let mac = Macro::new("$a $", 1);
    let args = ["shouldn't appear"];
    let output = apply_arguments(&mac, &args).unwrap();
    assert_eq!(
      &output,
      "$a $",
      "Invalid argument references ($ not followed by integer) should be left as is."
    );
  }

  #[test]
  fn all_arguments() {
    let mac = Macro::without_arg_validation("hi $0");
    let args = ["alice,","bob,","carol"];
    let output = apply_arguments(&mac, &args).unwrap();
    assert_eq!(
      &output,
      "hi alice, bob, carol",
      "$0 should be replaced with all arguments (space separated)."
    );
  }
}
