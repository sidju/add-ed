// Tests for 'l' and 'L' command
// 'l' tests are immediately after imports
// 'L' tests are after the 'l' tests

mod shared;
use shared::fixtures::{
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 'l' command
//
// - Takes optional selection
//   - If given prints selection
//   - If not given prints state.selection
// - Accepts printing flags
// - Prints literally unless state.l is set
//   (What literal printing is is left to the UI)
// - Prints numbered if state.n is set
// - state.selection is set to printed selection
// - Does not change unsaved

// Normal case, just print some lines literally
#[test]
fn literal() {
  PrintTest{
    init_buffer: vec!["a","\tb","$c","d"],
    init_clipboard: vec![],
    command_input: vec!["1,4l"],
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
        n: false,
        l: true,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}

// Test flag handling and using default selection
#[test]
fn literal_numbered_noselection() {
  PrintTest{
    init_buffer: vec!["a","\tb","$c","d"],
    init_clipboard: vec![],
    command_input: vec!["ln"],
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

// Verify behaviour of 'L' command
//
// - Takes no selection
// - Does not modify selection
// - Does not modify saved
// - Toggles the state.l bool, which sets if to print literal by default

