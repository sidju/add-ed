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

/// Small enum describing argument nr constraints
///
/// (We use serde's default, externally tagged)
#[derive(Debug)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature="serde", serde(rename_all="lowercase"))]
pub enum NrArguments {
  Any,
  None,
  Exactly(usize),
  Between{incl_min: usize, incl_max: usize},
}

/// A struct representing a runnable macro
///
/// It is intended to add more/change the variables, but the constructor and any
/// thereafter applied modification should produces instances with the same
/// behaviour through any changes.
///
/// If the `serde` feature is enabled, serialization will produce the most
/// backwards compatible representation while still ensuring the same behaviour.
/// Deserialization should produce identically behaving macros when valid for
/// the version of `add-ed` being used, if newer features are used in the macro
/// than the deserializing version of `add-ed` has access to an unknown field
/// error will be raised.
#[derive(Debug)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature="serde", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Macro {
  /// Input to simulate
  ///
  /// Should be a string of newline separated commands. Execution is equivalent
  /// to if this input was given on STDIN while the editor is running.
  pub input: Cow<'static, str>,
  /// The number of arguments the macro accepts
  ///
  /// `Any` performs no validation, `Exactly` verifies that it is exactly that
  /// nr of arguments given, and if `None` is set no argument substitution is 
  /// run on the macro (which means '$'s don't need to be doubled in the macro).
  pub nr_arguments: NrArguments,
  // TODO, enable this later
  // /// How the macro execution interacts with undo/redo snapshotting
  // snapshotting_mode: MacroSnapshottingMode,
}
impl Macro {
  /// Construct a macro
  ///
  /// Creates a macro with the given text as command input and all other options
  /// default. Use the builder pattern operators below or modify the public
  /// member variables to configure the rest.
  pub fn new<T: Into<Cow<'static, str>>>(
    input: T,
  ) -> Self {
    Self{
      input: input.into(),
      nr_arguments: NrArguments::Any,
    }
  }
  /// Configure required nr of arguments for the macro
  pub fn nr_arguments(mut self, nr: NrArguments) -> Self {
    self.nr_arguments = nr;
    self
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
  use NrArguments as NA;
  match mac.nr_arguments {
    NA::None => {
      if !args.is_empty() { return Err(EdError::ArgumentsWrongNr{
        expected: "absolutely no".into(),
        received: args.len(),
      }); }
      // For this case we skip arguments substitution completely
      return Ok(mac.input.to_string());
    },
    NA::Exactly(x) => {
      if args.len() != x { return Err(EdError::ArgumentsWrongNr{
        expected: format!("{}",x).into(),
        received: args.len(),
      }); }
    },
    NA::Between{incl_min, incl_max} => {
      if args.len() > incl_max || args.len() < incl_min {
        return Err(EdError::ArgumentsWrongNr{
          expected: format!("between {} and {}", incl_min, incl_max).into(),
          received: args.len(),
        });
      }
    },
    NA::Any => {},
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
    let mac = Macro::new("$1 world. Test$2")
      .nr_arguments(NrArguments::Exactly(2))
    ;
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
    let mac = Macro::new("$$1 and $1.");
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
    let mac = Macro::new("$a $");
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
    let mac = Macro::new("hi $0");
    let args = ["alice,","bob,","carol"];
    let output = apply_arguments(&mac, &args).unwrap();
    assert_eq!(
      &output,
      "hi alice, bob, carol",
      "$0 should be replaced with all arguments (space separated)."
    );
  }

  // Verify that no substitution is done if arguments none specified
  #[test]
  fn no_arguments() {
    let mac = Macro::new("test $$ test")
      .nr_arguments(NrArguments::None)
    ;
    let args: &[&str] = &[];
    let output = apply_arguments(&mac, &args).unwrap();
    assert_eq!(
      &output,
      "test $$ test",
      "When no arguments are allowed no substitution should be done."
    );
  }
}
