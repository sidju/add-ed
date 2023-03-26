// Tests for '#' command

mod shared;
use shared::fixtures::{
  BasicTest,
};

// Verify behaviour of '#' command
//
// - Takes optional selection.
//   - If given updates state.selection to given selection
//   - If not given does nothing.
// - Accepts any text after the command char
// - Does nothing

// Test that it does nothing at all without selection
#[test]
fn octothorpe_comment() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec!["#h"],
    expected_buffer: vec!["a","b","c"],
    expected_buffer_saved: true,
    expected_selection: (1,3),
    expected_clipboard: vec![],
  }.run()
}

// Test that it updates selection without any other effect with selection
#[test]
fn octothorpe_update_selection() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec!["1,2#ignored text"],
    expected_buffer: vec!["a","b","c"],
    expected_buffer_saved: true,
    expected_selection: (1,2),
    expected_clipboard: vec![],
  }.run()
}
