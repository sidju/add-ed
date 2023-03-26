// Tests for 'm' command

mod shared;
use shared::fixtures::{
  BasicTest,
  PrintTest,
};
use shared::mock_ui::Print;

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
  }.run()
}
