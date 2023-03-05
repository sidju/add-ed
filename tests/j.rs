// Tests for 'j' and 'J' command
// 'j' tests are immediately after imports
// 'J tests are after the 'j' tests

mod shared;
use shared::fixtures::{
  BasicTest,
  ErrorTest,
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 'j' command
//
// - Takes optional selection
//   - If given joins selection into one line (remove newlines)
//   - If none given does the same on state.selection
//   - Special: If selection is one line returns error, cannot join single line.
// - Selection after execution is the resulting line after joining.
// - Clipboard after execution is the original selection before joining.

// Normal use-case, join two lines
#[test]
fn join() {
  BasicTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["2,3j"],
    expected_buffer: vec!["a","bc","d"],
    expected_buffer_saved: false,
    expected_clipboard: vec!["b","c"],
    expected_selection: (2,2),
    expected_filepath: "path",
  }.run()
}
