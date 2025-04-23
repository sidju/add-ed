// Tests for default command

mod shared;
use shared::fixtures::{
  PrintTest,
  ErrorTest,
};
use shared::mock_ui::Print;
use add_ed::error::EdError;

// Verify behaviour of indexing/selection
//
// - A number should index to that line number.
// - `'<character>` should resolve to the first line tagged by the character.
// - `/<pattern>` should resolve to the nearest following line that matches the
//   pattern (can be closed with `/` to add a command after).
// - `?<pattern>` should resolve to the nearest preceeding line that matches
//   the pattern (can be closed with `?` to add a command after).
// - `$` should resolve to the last line in the buffer, if valid.
// - `.` should resolve to the currently selected line.
// - `+`/`-` should be able to add offsets to any other index before them,
//   if none given a `.` is assumed before


// Normal case, numeric index
#[test]
fn number() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["1"],
    expected_selection: (1,1),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "a\n".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}

// Test default with no buffer
#[test]
fn number_nobuffer() {
  ErrorTest{
    init_buffer: vec![],
    command_input: vec!["1"],
    expected_error: EdError::IndexTooBig{index:1,buffer_len:0},
  }.run()
}

// Last line index
#[test]
fn last_line() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["$"],
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
    expected_history_tags: vec![],
  }.run()
}
// Test last line with no buffer
#[test]
fn last_line_nobuffer() {
  ErrorTest{
    init_buffer: vec![],
    command_input: vec!["$"],
    expected_error: EdError::Line0Invalid,
  }.run()
}

// current index
#[test]
fn current_line() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec![".,.#"],
    expected_selection: (1,4),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![],
    expected_history_tags: vec![],
  }.run()
}

// Forward pattern index
#[test]
fn pattern() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["1#", "/c"],
    expected_selection: (3,3),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "c\n".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}
#[test]
fn pattern_terminated() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["1#", "/c/"],
    expected_selection: (3,3),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "c\n".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}

// Backward pattern index
#[test]
fn revpattern() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["4#", "?c"],
    expected_selection: (3,3),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "c\n".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}
#[test]
fn revpattern_terminated() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["4#", "?c?"],
    expected_selection: (3,3),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "c\n".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}

// Offset index
#[test]
fn offset_default() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    // Default selection is 1,4
    command_input: vec!["+,-"],
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
#[test]
fn offset_fancy() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    // Default selection is 1,4
    command_input: vec!["/b/+,"],
    expected_selection: (3,4),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "c\n".to_string(),
          "d\n".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}
// Tag index index
#[test]
fn tag() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["3kx", "'x"],
    expected_selection: (3,3),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "c\n".to_string(),
        ],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}
// Test last line with no buffer
#[test]
fn tag_invalid() {
  ErrorTest{
    init_buffer: vec!["a","b","c"],
    command_input: vec!["2kx", "'"],
    expected_error: EdError::IndexUnfinished("'".to_string()),
  }.run()
}
