// Tests for 'm' and 'M' command
// 'm' tests immediately after imports
// 'M' tests after the 'm' tests

mod shared;
use shared::fixtures::{
  BasicTest,
};

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
