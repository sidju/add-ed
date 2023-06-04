// Tests for default command

mod shared;
use shared::fixtures::{
  PrintTest,
  ErrorTest,
};
use shared::mock_ui::Print;
use add_ed::error::EdError;

// Verify behaviour of default command
//
// - Takes optional selection
//   - If given prints selection
//   - If not given prints line after state.selection.1
// - Prints literally if state.l is set
// - Prints numbered if state.n is set
// - state.selection is set to printed selection
// - Does not change unsaved

// Normal case, just print next line
#[test]
fn default() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["1,4"],
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
        n: false,
        l: false,
      }
    ],
  }.run()
}

// Test default selection
#[test]
fn default_noselection() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec![""],
    expected_selection: (4,4),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "d\n".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
  }.run()
}

// Test default with no buffer
#[test]
fn default_noselection_nobuffer() {
  ErrorTest{
    init_buffer: vec![],
    command_input: vec![""],
    expected_error: EdError::SelectionEmpty((1,0)),
  }.run()
}
