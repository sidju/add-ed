// Tests for 'a' and 'A' command
// 'a' tests are immediately after imports
// 'A' tests are after the 'a' tests

mod shared;
use shared::fixtures::{
  BasicTest,
  ErrorTest,
  PrintTest,
};
use shared::mock_ui::Print;

use add_ed::error::EdError;

// Verify behaviour of 'a' command
//
// - Takes optional index
//   - If given, adds input after line at index
//   - If none, same using state.selection.1 as index
//   - Special: 0 is valid index to append to, inserts before line 1
//   - Note: default state.selection should be (1,buffer.len()), thus (1,0) for
//     an empty buffer
// - Takes input via ui.get_input with '.' as terminator
// - Selection after command is the inserted lines
//   (If no lines doesn't change selection)
// - Sets unsaved if input given (else no change)

// No selection on empty buffer and no input
// (Due to post selection and no input tests default selection)
#[test]
fn append_noselection_noinput_nobuffer() {
  BasicTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    command_input: vec!["a","."],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
    expected_history_tags: vec![],
  }.run();
}

// Empty buffer and no input
// (Should behave identically as noselection above)
#[test]
fn append_noinput_nobuffer() {
  BasicTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    command_input: vec!["0a","."],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
    expected_history_tags: vec![],
  }.run();
}

// No selection on empty buffer with input
#[test]
fn append_noselection_nobuffer_print() {
  PrintTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    command_input: vec!["ap","1","2","."],
    expected_buffer: vec!["1","2"],
    expected_buffer_saved: false,
    expected_selection: (1,2),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["1\n".to_string(),"2\n".to_string(),],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec!["ap"],
  }.run();
}

// Empty buffer with input
// (Should behave identically as noselection above)
#[test]
fn append_nobuffer() {
  BasicTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    command_input: vec!["0a","1","2","."],
    expected_buffer: vec!["1","2"],
    expected_buffer_saved: false,
    expected_selection: (1,2),
    expected_clipboard: vec![],
    expected_history_tags: vec!["0a"],
  }.run();
}

// No selection, no input
// (Due to post selection and no input tests default selection)
#[test]
fn append_noselection_noinput_numbered() {
  PrintTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    command_input: vec!["an","."],
    expected_buffer: vec!["a","b"],
    expected_buffer_saved: true,
    expected_selection: (1,2),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["a\n".to_string(),"b\n".to_string()],
        n: true,
        l: false,
      },
    ],
    expected_history_tags: vec![],
  }.run();
}

// No input
// (Should behave identically as noselection above)
#[test]
fn append_noinput() {
  BasicTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    command_input: vec!["2a","."],
    expected_buffer: vec!["a","b"],
    expected_buffer_saved: true,
    expected_selection: (1,2),
    expected_clipboard: vec![],
    expected_history_tags: vec![],
  }.run();
}

// No selection
#[test]
fn append_noselection() {
  PrintTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    command_input: vec!["al","c","d","."],
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: false,
    expected_selection: (3,4),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["c\n".to_string(), "d\n".to_owned(),],
        n: false,
        l: true,
      },
    ],
    expected_history_tags: vec!["al"],
  }.run();
}

// Fully defined invocation
// (Should behave identically as noselection above)
#[test]
fn append() {
  BasicTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    command_input: vec!["2a","c","d","."],
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: false,
    expected_selection: (3,4),
    expected_clipboard: vec![],
    expected_history_tags: vec!["2a"],
  }.run();
}


// Append a few lines after selection
#[test]
fn append_relative() {
  BasicTest{
    init_buffer: vec!["a","b","c","e"],
    init_clipboard: vec![],
    command_input: vec!["1,2#","+a","d","."],
    expected_buffer: vec!["a","b","c","d","e"],
    expected_buffer_saved: false,
    expected_selection: (4,4),
    expected_clipboard: vec![],
    expected_history_tags: vec!["+a"],
  }.run();
}

// Verify behaviour of 'A' command
//
// - Takes optional index
//   - If given, appends inline to index
//   - If none, appends inline to state.selection.1
//   - (Inline appending to index 0 is not valid, error not a line)
//   - Note: Not valid for any index when buffer is empty
// - Takes input via ui.get_input with '.' as terminator
// - Selection after command is the inserted lines AND the now modified index
//   (If no lines doesn't change selection)
// - Sets unsaved if input given (else no change)
// - Clipboard after execution is the line at index and first input line.

// No selection, no buffer, no input required (returns from execution before)
#[test]
fn inline_append_nobuffer() {
  ErrorTest{
    init_buffer: vec![],
    command_input: vec!["A"],
    expected_error: EdError::Line0Invalid,
  }.run();
}

// No selection, no input
// (Due to post selection and no input tests default selection)
#[test]
fn inline_append_noselection_noinput() {
  PrintTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    command_input: vec!["Ap","."],
    expected_buffer: vec!["a","b"],
    expected_buffer_saved: true,
    expected_selection: (1,2),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["a\n".to_string(),"b\n".to_string()],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run();
}

// No input
// (Should behave identically as noselection above)
#[test]
fn inline_append_noinput_literal() {
  PrintTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    command_input: vec!["2Al","."],
    expected_buffer: vec!["a","b"],
    expected_buffer_saved: true,
    expected_selection: (1,2),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["a\n".to_string(),"b\n".to_string()],
        n: false,
        l: true,
      }
    ],
    expected_history_tags: vec![],
  }.run();
}

// No selection
#[test]
fn inline_append_noselection() {
  BasicTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    command_input: vec!["A","anana","cucumber","."],
    expected_buffer: vec!["a","banana","cucumber"],
    expected_buffer_saved: false,
    expected_selection: (2,3),
    expected_clipboard: vec!["b"],
    expected_history_tags: vec!["A"],
  }.run();
}

// Fully defined invocation
// (Should behave identically as noselection above)
#[test]
fn inline_append() {
  PrintTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    command_input: vec!["2An","anana","cucumber","."],
    expected_buffer: vec!["a","banana","cucumber"],
    expected_buffer_saved: false,
    expected_selection: (2,3),
    expected_clipboard: vec!["b"],
    expected_prints: vec![
      Print{
        text: vec!["banana\n".to_string(),"cucumber\n".to_string()],
        n: true,
        l: false,
      }
    ],
    expected_history_tags: vec!["2An"],
  }.run();
}
