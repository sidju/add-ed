// Test behaviour of 'c'
// TODO: add testing for 'C'

mod shared;
use shared::fixtures::BasicTest;

// Verify behaviour of 'c' command
//
// - Takes optional selection
//   - If given replaces selection with input
//   - If none given replaces state.selection with input
// - Takes input via ui.get_input with '.' as terminator
// - If lines given selection after command is the inserted lines
// - If no lines given set selection like 'd' command:
//   - Tries to select nearest line after deleted selection
//   - If selection was at end of buffer select nearest line before
//   - If the buffer is empty after deletion select (1,0)
// - Always sets unsaved
// - Deleted/replaced lines are placed in clipboard

// Empty buffer, errors always
// TODO: Use error testing fixtures, when errors have been improved
#[test]
#[should_panic]
fn change_nobuffer() {
  BasicTest{
    init_buffer: vec![],
    command_input: vec!["c"],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0)
  }.run();
}
// We don't do any noselection versions of 'c' testing, since default selection
// is invalid for the 'c' command.

// No input, end of buffer. Should delete and select new last line
#[test]
fn change_noinput_endofbuffer() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    command_input: vec!["3c","."],
    expected_buffer: vec!["a","b"],
    expected_buffer_saved: false,
    expected_selection: (2,2)
  }.run();
}

// No input, start of buffer. Should delete and select line following selection
#[test]
fn change_noinput_startofbuffer() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    command_input: vec!["1c","."],
    expected_buffer: vec!["b","c"],
    expected_buffer_saved: false,
    expected_selection: (1,1)
  }.run();
}

// No input, middle of buffer. Should delete and select line following selection
#[test]
fn change_noinput_middleofbuffer() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    command_input: vec!["2c","."],
    expected_buffer: vec!["a","c"],
    expected_buffer_saved: false,
    expected_selection: (2,2)
  }.run();
}

// No input, all of buffer. Should delete and select (1,0)
#[test]
fn change_noinput_allofbuffer() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    command_input: vec![",c","."],
    expected_buffer: vec![],
    expected_buffer_saved: false,
    expected_selection: (1,0)
  }.run();
}

// Fully defined invocation
#[test]
fn change() {
  BasicTest{
    init_buffer: vec!["a","b","d"],
    command_input: vec!["2c","banana","cucumber","."],
    expected_buffer: vec!["a","banana","cucumber","d"],
    expected_buffer_saved: false,
    expected_selection: (2,3)
  }.run();
}
