// Tests for 'y' command

mod shared;
use shared::fixtures::{
  BasicTest,
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 'y' command
//
// - Takes optional selection
//   - If given copies lines in selection to clipboard
//   - If not given copies state.selection
// - Takes printing flags after command
// - Selection after execution is the copied lines

// Test fully defined command
#[test]
fn copy() {
  BasicTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec!["previous"],
    command_input: vec!["2y"],
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec!["b"],
    expected_selection: (2,2),
  }.run()
}

// Test with default selection
// (Uses '#' to set selection without any print)
#[test]
fn copy_noselection_print() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec!["previous"],
    command_input: vec!["1,2#","yp"],
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_selection: (1,2),
    expected_clipboard: vec!["a","b"],
    expected_prints: vec![
      Print{
        text: vec!["a\n".to_string(),"b\n".to_string()],
        n: false,
        l: false,
      },
    ],
  }.run()
}
