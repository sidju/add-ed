// Test behaviour of 's'

mod shared;
use shared::fixtures::{
  BasicTest,
};

// Verify behaviour of 's' command
//
// - Takes optional selection
//   - If given performs substitution on lines in selection
//   - If not given performs substitution on lines in state.selection
// - Takes no or 3 arguments separated by first character after 's'
//   - First is match pattern
//   - Second is the substitution pattern
//   - Third is command flags, any of "gpnl" (TODO: add 'COUNT' support)
//   - (The separator is escapeable with '\')
// - Selection after is the resulting size of the initial selection after
//   substitution.
// - Clipboard is set to the state of the selection before substitution.
// - Sets unsaved

// Fully specified
#[test]
fn substitute() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec!["d"],
    command_input: vec!["1,3s_b_p_"],
    expected_buffer: vec!["a","p","c"],
    expected_buffer_saved: false,
    expected_selection: (1,3),
    expected_clipboard: vec!["a","b","c"],
  }.run()
}

// Use defaults from prior invocation
#[test]
fn substitute_defaults() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec!["d"],
    command_input: vec![r"s_\w_letter_","2s"],
    expected_buffer: vec!["letter","letter","c"],
    expected_buffer_saved: false,
    expected_selection: (2,2),
    expected_clipboard: vec!["b"],
  }.run()
}

// Test global flag and deleting lines
#[test]
fn substitute_global_deleteline() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec![r",s-\w\n--g"],
    expected_buffer: vec![],
    expected_buffer_saved: false,
    expected_selection: (1,0),
    expected_clipboard: vec!["a","b","c"],
  }.run()
}

// Test multiline pattern matching
#[test]
fn substitute_multiline() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec![",s_a\nb_hello_"],
    expected_buffer: vec!["hello","c"],
    expected_buffer_saved: false,
    expected_selection: (1,2),
    expected_clipboard: vec!["a","b","c"],
  }.run()
}

// Test multiline patterns properly terminates lines with '\n'
#[test]
fn substitute_removenewline() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec![",2s_\n__g"], // Remove all newlines
    expected_buffer: vec!["ab","c"],
    expected_buffer_saved: false,
    expected_selection: (1,1),
    expected_clipboard: vec!["a","b"],
  }.run()
}
