// Tests for 'n' and 'N' command
// 'n' tests are immediately after imports
// 'N' tests are after the 'N' tests

mod shared;
use shared::fixtures::{
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 'n' command
//
// - Takes optional selection
//   - If given prints selection
//   - If not given prints state.selection
// - Accepts printing flags
// - Prints numbered unless state.n is set
//   (What numbered means is is left to the UI)
// - Prints literal if state.l is set
// - state.selection is set to printed selection
// - Does not change unsaved

// Normal case, just print some lines numbered
#[test]
fn numbered() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["1,4n"],
    expected_selection: (1,4),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "a\n".to_string(),
          "b\n".to_string(),
          "c\n".to_string(),
          "d\n".to_string(),
        ],
        n: true,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}

// Test flag handling and using default selection
#[test]
fn numbered_literal_noselection() {
  PrintTest{
    init_buffer: vec!["a","\tb","$c","d"],
    init_clipboard: vec![],
    command_input: vec!["nl"],
    expected_selection: (1,4),
    expected_buffer: vec!["a","\tb","$c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "a\n".to_string(),
          "\tb\n".to_string(),
          "$c\n".to_string(),
          "d\n".to_string(),
        ],
        n: true,
        l: true,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}

// Verify behaviour of 'N' command
//
// - Takes no selection
// - Does not modify selection
// - Does not modify saved
// - Toggles the state.n bool, which sets if to print numbered by default

