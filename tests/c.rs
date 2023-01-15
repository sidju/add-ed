// Test behaviour of 'c'
// TODO: add testing for 'C'

mod shared;
use shared::fixtures::{
  BasicTest,
  PrintTest,
};
use shared::mock_ui::Print;

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
//   - If the buffer is empty after deletion select (1,0),
// - Always sets unsaved
// - Deleted/replaced lines are placed in clipboard

// Empty buffer, errors always
// TODO: Use error testing fixtures, when errors have been improved
#[test]
#[should_panic]
fn change_nobuffer() {
  BasicTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    command_input: vec!["c"],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
  }.run();
}
// We don't do any noselection versions of 'c' testing, since default selection
// is invalid for the 'c' command.

// No input, end of buffer. Should delete and select new last line
#[test]
fn change_noinput_endofbuffer_print() {
  PrintTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec!["3cp","."],
    expected_buffer: vec!["a","b"],
    expected_buffer_saved: false,
    expected_selection: (2,2),
    expected_clipboard: vec!["c"],
    expected_prints: vec![
      Print{
        text: vec!["b\n".to_string()],
        n: false,
        l: false,
      },
    ],
  }.run();
}

// No input, start of buffer. Should delete and select line following selection
#[test]
fn change_noinput_startofbuffer_numbered() {
  PrintTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec!["1cn","."],
    expected_buffer: vec!["b","c"],
    expected_buffer_saved: false,
    expected_selection: (1,1),
    expected_clipboard: vec!["a"],
    expected_prints: vec![
      Print{
        text: vec!["b\n".to_string()],
        n: true,
        l: false,
      },
    ],
  }.run();
}

// No input, middle of buffer. Should delete and select line following selection
#[test]
fn change_noinput_middleofbuffer_literal() {
  PrintTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec!["2cl","."],
    expected_buffer: vec!["a","c"],
    expected_buffer_saved: false,
    expected_selection: (2,2),
    expected_clipboard: vec!["b"],
    expected_prints: vec![
      Print{
        text: vec!["c\n".to_string()],
        n: false,
        l: true,
      },
    ],
  }.run();
}

// No input, all of buffer. Should delete and select (1,0),
#[test]
fn change_noinput_allofbuffer() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec![",c","."],
    expected_buffer: vec![],
    expected_buffer_saved: false,
    expected_selection: (1,0),
    expected_clipboard: vec!["a","b","c"],
  }.run();
}

// Fully defined invocation
#[test]
fn change() {
  BasicTest{
    init_buffer: vec!["a","b","d"],
    init_clipboard: vec![],
    command_input: vec!["2c","banana","cucumber","."],
    expected_buffer: vec!["a","banana","cucumber","d"],
    expected_buffer_saved: false,
    expected_selection: (2,3),
    expected_clipboard: vec!["b"],
  }.run();
}
