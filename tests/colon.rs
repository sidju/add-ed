// Tests for ':' command

mod shared;
use shared::fixtures::{
  MacroTest,
  MacroErrorTest,
};

use add_ed::{
  EdError,
  macros::Macro,
};

// Verify behaviour of ':' command
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
  store.insert("double", Macro::new("t.", 0));
  store.insert("append_word", Macro::new(",a\n$1\n.",1));
  store.insert("append_words", Macro::without_arg_validation(",a\n$0\n."));
  store.insert("recursion", Macro::new(":recursion", 0));
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
    macro_invocation: "1:double",
    expected_buffer: vec!["a","a","b","c","d"],
    expected_buffer_saved: false,
    expected_selection: (2,2),
    expected_clipboard: vec![],
    expected_history_tags: vec!["1:double"],
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
    macro_invocation: ":append_word word_to_append",
    expected_buffer: vec!["a","b","word_to_append"],
    expected_buffer_saved: false,
    expected_selection: (3,3),
    expected_clipboard: vec![],
    expected_history_tags: vec![":append_word word_to_append"],
  }.run();
}

#[test]
fn macro_wrongnr_arguments() {
  MacroErrorTest{
    init_buffer: vec!["a","b"],
    // We use a standard macro store
    macro_store: create_macro_store(),
    // and specify which macro to test in each test
    macro_invocation: ":append_word word_to_append unhandled_argument",
    expected_error: EdError::ArgumentsWrongNr{expected: "1".into(), received: 2},
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
    macro_invocation: ":append_words words to append in",
    expected_buffer: vec!["words to append in"],
    expected_buffer_saved: false,
    expected_selection: (1,1),
    expected_clipboard: vec![],
    expected_history_tags: vec![":append_words words to append in"],
  }.run();
}

#[test]
fn macro_recursion() {
  MacroErrorTest{
    init_buffer: vec![],
    // We use a standard macro store
    macro_store: create_macro_store(),
    // and specify which macro to test in each test
    macro_invocation: ":recursion",
    expected_error: EdError::InfiniteRecursion,
  }.run();
}
