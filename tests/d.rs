// Test behaviour of 'd'

mod shared;
use shared::fixtures::{
  BasicTest,
  ErrorTest,
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 'd' command
//
// - Takes optional selection
//   - If given deletes lines in selection
//   - If not given deletes lines in state.selection
// - Selection after:
//   - Selects nearest line following the deleted lines
//   - If no line after deleted lines, selects nearest preceeding
//   - If no lines left in buffer after delete selects (1,0)
// - Lines deleted are moved to the clipboard
// - Sets unsaved if selection is valid/it is executed

// No selection, no buffer
#[test]
fn delete_noselection_nobuffer() {
  ErrorTest{
    init_buffer: vec![],
    command_input: vec!["d"],
    expected_error: add_ed::error_consts::SELECTION_EMPTY,
  }.run();
}

// No selection, default selection from fixture means deleting whole buffer
#[test]
fn delete_noselection() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec!["d"],
    expected_buffer: vec![],
    expected_buffer_saved: false,
    expected_selection: (1,0),
    expected_clipboard: vec!["a","b","c"],
  }.run();
}

// No selection, deletes whole buffer and tries to print
// (This should be used to improve when using an error testing fixture, we
// should preferably error before deleting the buffer instead of after.)
#[test]
fn delete_noselection_allofbuffer_print() {
  ErrorTest{
    init_buffer: vec!["a","b","c"],
    command_input: vec!["dp"],
    expected_error: add_ed::error_consts::PRINT_AFTER_WIPE,
  }.run();
}

// No selection, modified selection by pre-command gives partial delete
#[test]
fn delete_noselection_middleofbuffer_numbered_literal() {
  PrintTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec!["2","dnl"],
    expected_buffer: vec!["a","c"],
    expected_buffer_saved: false,
    expected_selection: (2,2),
    expected_clipboard: vec!["b"],
    expected_prints: vec![
      Print{
        text: vec!["b\n".to_string()],
        n: false,
        l: false,
      },
      Print{
        text: vec!["c\n".to_string()],
        n: true,
        l: true,
      },
    ],
  }.run();
}

// Explicit selection deleting start of buffer
#[test]
fn delete_startofbuffer_print() {
  PrintTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec!["1,2dp"],
    expected_buffer: vec!["c"],
    expected_buffer_saved: false,
    expected_selection: (1,1),
    expected_clipboard: vec!["a","b"],
    expected_prints: vec![
      Print{
        text: vec!["c\n".to_string()],
        n: false,
        l: false,
      },
    ],
  }.run();
}

// Explicit selection deleting end of buffer
#[test]
fn delete_endofbuffer() {
  BasicTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec!["2,3d"],
    expected_buffer: vec!["a"],
    expected_buffer_saved: false,
    expected_selection: (1,1),
    expected_clipboard: vec!["b","c"],
  }.run();
}
