// Tests for 'f' command

mod shared;
use shared::fixtures::{
  BasicTest,
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 'f' command
// - Takes no index
// - Takes optional filepath
//   - If path given, sets state.filepath to that path
//     (Interprets shell escape as path)
//   - If no path given prints current state.filepath

// Default behaviour, print current
#[test]
fn filename_default() {
  PrintTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec![
      "f",
    ],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
    expected_filepath: "path",
    expected_prints: vec![
      Print{
        text: vec!["path".to_owned()],
        n: false,
        l: false,
      },
    ],
  }.run();
}

// With given path, set new state.path
#[test]
fn filename() {
  BasicTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    init_filepath: "old",
    command_input: vec!["f new"],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
    expected_filepath: "new",
  }.run();
}

// With unseparated given path, set new state.path
#[test]
fn filename_without_space() {
  BasicTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    init_filepath: "old",
    command_input: vec!["fnew"],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
    expected_filepath: "new",
  }.run();
}
