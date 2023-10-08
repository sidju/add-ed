// Tests for 'u' and 'U' command
// 'u' tests are after imports
// 'U' tests are thereafter

mod shared;
use shared::fixtures::{
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 'u' command
//
// - Doesn't allow selection or index
// - Accepts a signed numeric argument of how many steps to undo
//   - If none given defaults to 1
// - Changes the state of the buffer the given number of modifying commands back
//   in history, negative number moves forward.
//   - If it isn't possible to move the given number of steps prints error,
//     INVALID_UNDO_STEPS.
// - Sets saved / unsaved after if that undo step is saved / unsaved
// - Currently doesn't modify selection.
//   (Later it may be good to set selection to the selection _acted upon_ in the
//   last undone step. But to do that pairing selection with buffer state could
//   be good, or validation. Current code may leave state.selection in an
//   invalid state. Optimum would be to store selection with the buffer state,
//   but that would require a _big_ refactor...)
// Test fully defined
#[test]
fn undo() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec!["dummy"],
    command_input: vec!["1d","3d","u2"],
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec!["d"], // Because "3d" sets it and 'u' leaves it
    expected_selection: (2,2),
    expected_prints: vec![
      Print{
        text: vec!["Undid 2 operation(s) to right after initial load.".to_owned()],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec!["1d","3d"],
  }.run()
}

// Test defaults
#[test]
fn undo_default() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec!["dummy"],
    command_input: vec!["1d","3d","u"],
    expected_buffer: vec!["b","c","d"],
    expected_buffer_saved: false,
    expected_clipboard: vec!["d"], // Because 'd' sets it and 'u' leaves it
    expected_selection: (2,2),
    expected_prints: vec![
      Print{
        text: vec!["Undid 1 operation(s) to right after 1d.".to_owned()],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec!["1d","3d"],
  }.run()
}

// Test defaults
#[test]
fn modification_after_undo() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec!["dummy"],
    command_input: vec!["1d","u","3d"],
    expected_buffer: vec!["a","b","d"],
    expected_buffer_saved: false,
    expected_clipboard: vec!["c"], // Due to "3d"
    expected_selection: (3,3),
    expected_prints: vec![
      Print{
        text: vec!["Undid 1 operation(s) to right after initial load.".to_owned()],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec!["1d","u1","3d"],
  }.run()
}

// Verify that line tags exist on their lines throughout history
#[test]
fn undo_tag_move() {
  PrintTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec![
      "2m", // move line 2 to end of buffer
      "2kp", // mark new line 2, previously line 3
      "'p=", // print index of marked line (should be 2)
      "u", // undo the move (move is the only snapshot creating command here)
      "'p=", // print index of marked line _before it was marked_ (should be 3)
    ],
    expected_buffer: vec!["a","b","c"],
    expected_buffer_saved: true,
    expected_selection: (3,3),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["(2,2)".to_owned()],
        n: false,
        l: false,
      },
      Print{
        text: vec!["Undid 1 operation(s) to right after initial load.".to_owned()],
        n: false,
        l: false,
      },
      Print{
        text: vec!["(3,3)".to_owned()],
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec!["2m"],
  }.run()
}
