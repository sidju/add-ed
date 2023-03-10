// Tests for 'k' and 'K' command
// 'k' tests are immediately after imports
// 'K' tests are after the 'k' tests

mod shared;
use shared::fixtures::{
  BasicTest,
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 'k' command
//
// - Takes optional line
//   - If given tags line with the char argument
//   - If selection tags the first line in selection
//   - If none given tags first line in state.selection
// - Takes an optional single char after command as argument
//   - If given tags line with that char
//   - If none given clears previous tags from the line
// - Selection after command is unchanged
// - Clipboard isn't modified
// - Buffer isn't set as unsaved

// Normal use-case, tag a line
#[test]
fn mark() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec![
      "2,3kp", // Mark line with p
    ],
    expected_buffer: vec!["a","b","c"],
    expected_buffer_saved: true,
    expected_selection: (1,3),
    expected_clipboard: vec![],
    expected_filepath: "path",
  }.run()
}

// Tag a line and then print what line is tagged by that char
#[test]
fn mark_print_adress() {
  PrintTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec![
      "2,3kp", // Mark line with p
      "'p=", // Print index of line tagged by p
    ],
    expected_buffer: vec!["a","b","c"],
    expected_buffer_saved: true,
    expected_selection: (2,2),
    expected_clipboard: vec![],
    expected_filepath: "path",
    expected_prints: vec![
      Print{
        text: vec!["(2,2)".to_string()],
        n: false,
        l: false,
      },
    ],
  }.run()
}

// Verify behaviour of 'K' command
//
// Same as 'k' command, except select last line in selections

// Normal use-case, tag a line
#[test]
fn mark_last() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec![
      "1,2Kp", // Mark line with p
    ],
    expected_buffer: vec!["a","b","c"],
    expected_buffer_saved: true,
    expected_selection: (1,3),
    expected_clipboard: vec![],
    expected_filepath: "path",
  }.run()
}

// Tag a line and then print what line is tagged by that char
#[test]
fn mark_last_print_adress() {
  PrintTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec![
      "1,2Kp", // Mark line with p
      "'p=", // Print index of line tagged by p
    ],
    expected_buffer: vec!["a","b","c"],
    expected_buffer_saved: true,
    expected_selection: (2,2),
    expected_clipboard: vec![],
    expected_filepath: "path",
    expected_prints: vec![
      Print{
        text: vec!["(2,2)".to_string()],
        n: false,
        l: false,
      },
    ],
  }.run()
}
