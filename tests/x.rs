// Tests for 'x' and 'X' command
// 'x' tests after includes
// 'X' tests thereafter

mod shared;
use shared::fixtures::{
  BasicTest,
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 'x' command
//
// - Takes optional index/selection
//   - If given pastes clipboard contents after given index/selection.1
//   - If not given pastes after state.selection.1
// - Takes printing flags after command
// - Sets unsaved
// - Selection after execution is the pasted lines in their new location

// Test fully defined command
#[test]
fn paste() {
  BasicTest{
    init_buffer: vec!["a","b","d"],
    init_clipboard: vec!["c"],
    command_input: vec!["2x"],
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: false,
    expected_clipboard: vec!["c"],
    expected_selection: (3,3),
    expected_history_tags: vec!["2x"],
  }.run()
}

// Test with default selection
// (Uses '#' to set selection without any print)
#[test]
fn paste_noselection_print() {
  PrintTest{
    init_buffer: vec!["a","b","d"],
    init_clipboard: vec!["c"],
    command_input: vec!["1,2#","xp"],
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: false,
    expected_selection: (3,3),
    expected_clipboard: vec!["c"],
    expected_prints: vec![
      Print{
        text: vec!["c\n".to_string()],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec!["xp"],
  }.run()
}

// Verify behaviour of 'X' command
//
// - Takes optional index/selection
//   - If given pastes clipboard contents before given index/selection.0
//   - If not given pastes before state.selection.0
// - Takes printing flags after command
// - Sets unsaved
// - Selection after execution is the pasted lines in their new location

// Test fully defined command
#[test]
fn paste_before() {
  BasicTest{
    init_buffer: vec!["a","b","d"],
    init_clipboard: vec!["c"],
    command_input: vec!["3X"],
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: false,
    expected_clipboard: vec!["c"],
    expected_selection: (3,3),
    expected_history_tags: vec!["3X"],
  }.run()
}

// Test with default selection
// (Uses '#' to set selection without any print)
#[test]
fn paste_before_noselection_print() {
  PrintTest{
    init_buffer: vec!["a","b","d"],
    init_clipboard: vec!["c"],
    command_input: vec!["3#","Xp"],
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: false,
    expected_selection: (3,3),
    expected_clipboard: vec!["c"],
    expected_prints: vec![
      Print{
        text: vec!["c\n".to_string()],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec!["Xp"],
  }.run()
}
