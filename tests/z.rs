// Tests for 'z' and 'Z' commands
// 'z' tests after includes
// 'Z' tests thereafter

mod shared;
use shared::fixtures::{
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 'z' command
//
// - Takes optional index/selection
//   - If given prints lines following index/selection.1
//   - If not given prints lines following state.selection.1
// - Accepts an integer argument of how many lines to print
// - Accepts printing flags
// - state.selection is set to printed selection
// - Does not change unsaved

// Normal case, just print some lines
#[test]
fn scroll() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["1,1z2"],
    expected_selection: (2,3),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "b\n".to_string(),
          "c\n".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}

// Test flag handling and using default selection
#[test]
fn scroll_literal_numbered_noselection_endofbuffer() {
  PrintTest{
    init_buffer: vec!["a","\tb","$c","d"],
    init_clipboard: vec![],
    command_input: vec!["zln"],
    expected_selection: (4,4),
    expected_buffer: vec!["a","\tb","$c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "d\n".to_string(),
        ],
        n: true,
        l: true,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}
// Verify behaviour of 'Z' command
//
// - Takes optional index/selection
//   - If given prints lines preceeding index/selection.0
//   - If not given prints lines preceeding state.selection.0
// - Accepts an integer argument of how many lines to print
// - Accepts printing flags
// - state.selection is set to printed selection
// - Does not change unsaved

// Normal case, just print some lines
#[test]
fn scroll_backwards() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["4,4Z2"],
    expected_selection: (2,3),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "b\n".to_string(),
          "c\n".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}

// Test flag handling and using default selection
#[test]
fn scroll_backwards_literal_numbered_noselection_startofbuffer() {
  PrintTest{
    init_buffer: vec!["a","\tb","$c","d"],
    init_clipboard: vec![],
    command_input: vec!["Zln"],
    expected_selection: (1,1),
    expected_buffer: vec!["a","\tb","$c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "a\n".to_string(),
        ],
        n: true,
        l: true,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}
