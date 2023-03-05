// Tests for 'i' and 'I' command
// 'i' tests are immediately after imports
// 'I' tests are after the 'i' tests

mod shared;
use shared::fixtures::{
  BasicTest,
  ErrorTest,
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 'ai' command
//
// - Takes optional index
//   - If given, adds input before line at index
//   - If none, same using state.selection.0 as index
//   - Special: 0 is valid index to insert to, inserts before line 1
//   - Note: default state.selection should be (1,buffer.len()), thus (1,0) for
//     an empty buffer
// - Takes input via ui.get_input with '.' as terminator
// - Selection after command is the inserted lines
//   (If no lines doesn't change selection)
// - Sets unsaved if input given (else no change)

// No selection on empty buffer and no input
// (Due to post selection and no input tests default selection)
#[test]
fn insert_noselection_noinput_nobuffer() {
  BasicTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["i","."],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
    expected_filepath: "path",
  }.run();
}

// Empty buffer and no input
// (Should behave identically as noselection above)
#[test]
fn insert_noinput_nobuffer() {
  BasicTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["0i","."],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
    expected_filepath: "path",
  }.run();
}

// No selection on empty buffer with input
#[test]
fn insert_noselection_nobuffer_print() {
  PrintTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["ip","1","2","."],
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
    expected_filepath: "path",
  }.run();
}

// Empty buffer with input
// (Should behave identically as noselection above)
#[test]
fn insert_nobuffer() {
  BasicTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["1i","1","2","."],
    expected_buffer: vec!["1","2"],
    expected_buffer_saved: false,
    expected_selection: (1,2),
    expected_clipboard: vec![],
    expected_filepath: "path",
  }.run();
}

// No selection, no input
// (Due to post selection and no input tests default selection)
#[test]
fn insert_noselection_noinput_numbered() {
  PrintTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["in","."],
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
    expected_filepath: "path",
  }.run();
}

// No input
// (Should behave identically as noselection above)
#[test]
fn insert_noinput() {
  BasicTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["2i","."],
    expected_buffer: vec!["a","b"],
    expected_buffer_saved: true,
    expected_selection: (1,2),
    expected_clipboard: vec![],
    expected_filepath: "path",
  }.run();
}

// No selection
#[test]
fn insert_noselection() {
  PrintTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["il","c","d","."],
    expected_buffer: vec!["c","d","a","b"],
    expected_buffer_saved: false,
    expected_selection: (1,2),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["c\n".to_string(), "d\n".to_owned(),],
        n: false,
        l: true,
      },
    ],
    expected_filepath: "path",
  }.run();
}

// Fully defined invocation
// (Should behave identically as noselection above)
#[test]
fn insert() {
  BasicTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["1i","c","d","."],
    expected_buffer: vec!["c","d","a","b"],
    expected_buffer_saved: false,
    expected_selection: (1,2),
    expected_clipboard: vec![],
    expected_filepath: "path",
  }.run();
}

// Verify behaviour of 'I' command
//
// - Takes optional index
//   - If given, inserts inline to index
//   - If none, inserts inline to state.selection.0
//   - (Inline appending to index 0 is not valid, error not a line)
//   - Note: Not valid for any index when buffer is empty
// - Takes input via ui.get_input with '.' as terminator
// - Selection after command is the inserted lines AND the now modified index
//   (If no lines doesn't change selection)
// - Sets unsaved if input given (else no change)
// - Clipboard after execution is last input line and line at index before join

// No selection, no buffer, no input required (returns from execution before)
#[test]
fn inline_insert_nobuffer() {
  ErrorTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["I"],
    expected_error: add_ed::error_consts::INDEX_TOO_BIG,
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
    expected_filepath: "path",
  }.run();
}

// No selection, no input
// (Due to post selection and no input tests default selection)
#[test]
fn inline_insert_noselection_noinput() {
  PrintTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["Ip","."],
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
    expected_filepath: "path",
  }.run();
}

// No input
// (Should behave identically as noselection above)
#[test]
fn inline_insert_noinput_literal() {
  PrintTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["2Il","."],
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
    expected_filepath: "path",
  }.run();
}

// No selection
#[test]
fn inline_insert_noselection() {
  BasicTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["I","lake","saun","."],
    expected_buffer: vec!["lake", "sauna","b"],
    expected_buffer_saved: false,
    expected_selection: (1,2),
    expected_clipboard: vec!["a"],
    expected_filepath: "path",
  }.run();
}

// Fully defined invocation
// (Should behave identically as noselection above)
#[test]
fn inline_insert() {
  PrintTest{
    init_buffer: vec!["a","b"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["1In","lake","saun","."],
    expected_buffer: vec!["lake","sauna","b"],
    expected_buffer_saved: false,
    expected_selection: (1,2),
    expected_clipboard: vec!["a"],
    expected_prints: vec![
      Print{
        text: vec!["lake\n".to_string(),"sauna\n".to_string()],
        n: true,
        l: false,
      }
    ],
    expected_filepath: "path",
  }.run();
}
