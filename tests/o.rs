// Tests for 'o' command

mod shared;
use shared::fixtures::{
  MacroTest,
  MacroErrorTest,
};

use add_ed::{
  EdError,
  macros::{
    Macro,
    NrArguments,
    ModificationMode,
  },
};

// Verify behaviour of 'o' command
//
// - Takes selection set as state selection before macro execution
// - Accepts space separated arguements for the macro after the command character
// - Requires first argument: name of a macro
// - Optional further arguments: list of arguments supplied to the macro for
//   substitution via $<argument nr>
// - Errors if macro execution errors or if nr of given arguments doesn't match
//   the nr of arguments the macro accepts.


fn create_macro_store() -> std::collections::HashMap<&'static str, Macro> {
  let mut store = std::collections::HashMap::new();
  store.insert("double",
    Macro::new("t.")
      .nr_arguments(NrArguments::Exactly(0))
  );
  store.insert("append_word",
    Macro::new(",a\n$1\n.")
      .nr_arguments(NrArguments::Exactly(1))
  );
  store.insert("append_word_expose",
    Macro::new(",a\n$1\n.")
      .nr_arguments(NrArguments::Exactly(1))
      .modification_mode(ModificationMode::Expose)
  );
  store.insert("append_words",
    Macro::new(",a\n$0\n.")
  );
  store.insert("append_words_revert",
    Macro::new(",a\n$0\n.")
      .modification_mode(ModificationMode::Revert)
  );
  store.insert("edit_and_error",
    Macro::new("oappend_word test\n,g/best/p")
  );
  store.insert("recursion",
    Macro::new("orecursion")
      .nr_arguments(NrArguments::Exactly(0))
  );
  store
}

#[test]
fn macro_selection() {
  MacroTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    // We use a standard macro store
    macro_store: create_macro_store(),
    // and specify which macro to test in each test
    macro_invocation: "1o double",
    expected_buffer: vec!["a","a","b","c","d"],
    expected_buffer_saved: false,
    expected_selection: (2,2),
    expected_clipboard: vec![],
    expected_history_tags: vec!["1o double"],
  }.run();
}

#[test]
fn macro_arguments() {
  MacroTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    // We use a standard macro store
    macro_store: create_macro_store(),
    // and specify which macro to test in each test
    macro_invocation: "oappend_word word_to_append",
    expected_buffer: vec!["a","b","word_to_append"],
    expected_buffer_saved: false,
    expected_selection: (3,3),
    expected_clipboard: vec![],
    expected_history_tags: vec!["oappend_word word_to_append"],
  }.run();
}

#[test]
fn macro_wrongnr_arguments() {
  MacroErrorTest{
    init_buffer: vec!["a","b"],
    // We use a standard macro store
    macro_store: create_macro_store(),
    // and specify which macro to test in each test
    macro_invocation: "oappend_word word_to_append unhandled_argument",
    expected_error: EdError::ArgumentsWrongNr{expected: "1".into(), received: 2},
    expected_buffer: vec!["a","b"],
    expected_buffer_saved: true,
    expected_selection: (1,2),
    expected_history_tags: vec![],
  }.run();
}

#[test]
fn macro_allarguments() {
  MacroTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    // We use a standard macro store
    macro_store: create_macro_store(),
    // and specify which macro to test in each test
    macro_invocation: "oappend_words words to append in",
    expected_buffer: vec!["words to append in"],
    expected_buffer_saved: false,
    expected_selection: (1,1),
    expected_clipboard: vec![],
    expected_history_tags: vec!["oappend_words words to append in"],
  }.run();
}

#[test]
fn macro_recursion() {
  MacroErrorTest{
    init_buffer: vec![],
    // We use a standard macro store
    macro_store: create_macro_store(),
    // and specify which macro to test in each test
    macro_invocation: "orecursion",
    expected_error: EdError::InfiniteRecursion,
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_history_tags: vec![],
  }.run();
}


// Tests regarding modification mode and how undo history is created
// Test ModificationMode::Default (tested by other tests, here for clarity)
#[test]
fn modification_default() {
  MacroTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    // We use a standard macro store
    macro_store: create_macro_store(),
    // and specify which macro to test in each test
    macro_invocation: "oappend_words words to append in",
    expected_buffer: vec!["words to append in"],
    expected_buffer_saved: false,
    expected_selection: (1,1),
    expected_clipboard: vec![],
    expected_history_tags: vec!["oappend_words words to append in"],
  }.run();
}

// If an error occurs in a macro it should revert fully and not create snapshots
#[test]
fn error_reverts() {
  MacroErrorTest{
    init_buffer: vec![],
    // We use a standard macro store
    macro_store: create_macro_store(),
    // and specify which macro to test in each test
    macro_invocation: "oedit_and_error",
    expected_error: EdError::RegexNoMatch("best".into()),
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_history_tags: vec![],
  }.run();
}

// Test ModificationMode::Revert
#[test]
fn modification_revert() {
  MacroTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    // We use a standard macro store
    macro_store: create_macro_store(),
    // and specify which macro to test in each test
    macro_invocation: "oappend_words_revert words to append in",
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,1), // Left as-is on successful execution
    expected_clipboard: vec![],
    expected_history_tags: vec![],
  }.run();
}

// Test ModificationMode::Expose
#[test]
fn modification_expose() {
  MacroTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    // We use a standard macro store
    macro_store: create_macro_store(),
    // and specify which macro to test in each test
    macro_invocation: "oappend_word_expose word",
    expected_buffer: vec!["word"],
    expected_buffer_saved: false,
    expected_selection: (1,1),
    expected_clipboard: vec![],
    expected_history_tags: vec![",a"],
  }.run();
}
