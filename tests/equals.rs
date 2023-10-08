// Tests for '=' command

mod shared;
use shared::fixtures::{
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of '=' command
//
// - Takes optional selection
//   - If given prints selection as "({sel.0}, {sel.1})"
//   - If not given does same with state.selection
// - state.selection is set to selection
// - Does not change unsaved

// Default case
#[test]
fn equals_noselection() {
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

// Test with specific selection
#[test]
fn equals() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["1,$-1="],
    expected_selection: (1,3),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "(1,3)".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}
