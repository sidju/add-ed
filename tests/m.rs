// Tests for 'm' and 'M' command
// 'm' tests are after imports
// 'M' tests are thereafter, commented out. See issue #6

mod shared;
use shared::fixtures::{
  BasicTest,
  PrintTest,
  ErrorTest,
};
use shared::mock_ui::Print;

use add_ed::error::EdError;

// Verify behaviour of 'm' command
//
// - Takes optional selection
//   - If given moves lines in given selection
//   - If not given moves lines in state.selection
// - Takes optional index argument
//   - If given moves lines to after index
//   - If not given moves lines to end of buffer
//   - Special, moving to after line 0 moves to start of file
// - Takes printing flags after index argument
// - Sets unsaved
// - Selection after execution is the moved lines in their new location

// Test fully defined command
#[test]
fn mov() {
  BasicTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["3,4m0"],
    expected_buffer: vec!["c","d","a","b"],
    expected_buffer_saved: false,
    expected_clipboard: vec![],
    expected_selection: (1,2),
    expected_history_tags: vec!["3,4m0"],
  }.run()
}

// Test with default selection and default index
// (Uses '#' to set selection without any print)
#[test]
fn mov_noindex_noselection_print() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["2,3#","mp"],
    expected_buffer: vec!["a","d","b","c"],
    expected_buffer_saved: false,
    expected_selection: (3,4),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["b\n".to_string(),"c\n".to_string(),],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec!["mp"],
  }.run()
}

// Test defaults with no buffer
#[test]
fn mov_default_nobuffer() {
  ErrorTest{
    init_buffer: vec![],
    command_input: vec!["m"],
    expected_error: EdError::IndexTooBig{index:1, buffer_len:0},
  }.run()
}

// Verify behaviour of 'M' command
//
// - Takes optional selection
//   - If given moves lines in given selection
//   - If not given moves lines in state.selection
// - Takes optional index argument
//   - If given moves lines to before index
//   - If not given moves lines to beginning of buffer
//   - Special, moving to before line 0 move to beginning of buffer
// - Takes printing flags after index argument
// - Sets unsaved
// - Selection after execution is the moved lines in their new location
//
//// Test fully defined command
//#[test]
//fn mov_before() {
//  BasicTest{
//    init_buffer: vec!["a","b","c","d"],
//    init_clipboard: vec![],
//    command_input: vec!["3,4M2"],
//    expected_buffer: vec!["a","c","d","b"],
//    expected_buffer_saved: false,
//    expected_clipboard: vec![],
//    expected_selection: (2,3),
//  }.run()
//}
//
//// Test with default selection and default index
//// (Uses '#' to set selection without any print)
//#[test]
//fn mov_before_noindex_noselection_print() {
//  PrintTest{
//    init_buffer: vec!["a","b","c","d"],
//    init_clipboard: vec![],
//    command_input: vec!["2,3#","Mp"],
//    expected_buffer: vec!["b","c","a","d"],
//    expected_buffer_saved: false,
//    expected_selection: (1,2),
//    expected_clipboard: vec![],
//    expected_prints: vec![
//      Print{
//        text: vec!["b\n".to_string(),"c\n".to_string(),],
//        n: false,
//        l: false,
//      },
//    ],
//  }.run()
//}
