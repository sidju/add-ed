// Tests for '=' command

mod shared;
use shared::fixtures::{
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of '=' command
//
// - Accepts no selection
// - Accepts optional flags, if none given defaults to printing selection
//   - s to print selection
//   - a to print as though all flags had been given
// - Does not change unsaved

// Default case
#[test]
fn equals_default() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["="],
    expected_selection: (1,4),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "(1,4)".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}

// Test with specific flags
#[test]
fn equals_selection() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["=s"],
    expected_selection: (1,4),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "(1,4)".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}

// Test all flags
#[test]
fn equals_all() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["=a"],
    expected_selection: (1,4),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "(1,4)".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}
