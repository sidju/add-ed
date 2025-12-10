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
// - Accepts a history index of where in history to move to
//   - If none given defaults to one step into history
//   - If a lone number is given move that number of steps into history
//   - . and $ are shortcuts to current position and latest snapshot
//   - *<index> takes you to an absolute history index
//   - +/-[<steps>] shifts preceeding index further back/forward respectively
//     (If no preceeding index . is assumed. If no steps 1 is assumed.)
// - Changes the state of the buffer the given number of modifying commands back
//   in history, negative number moves forward.
//   - If it isn't possible to move the given number of steps prints error,
//     INVALID_UNDO_STEPS.
// - Sets saved / unsaved after if that undo step is saved / unsaved
// - As a temporary improvement we make the selection point within the buffer
//   with as little modification as possible.
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
      "'p#",
      "=", // print index of marked line (should be 2)
      "u", // undo the move (move is the only snapshot creating command here)
      "'p#",
      "=", // print index of marked line _before it was marked_ (should be 3)
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

// Try out absolute undo indexing
#[test]
fn undo_absolute_index() {
  PrintTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec![
      "2t", // copy line 2 to end of buffer
      "2d", // delete the original
      "u*2", // undo the delete by absolute index (2 first from fixture (0 indexed))
    ],
    expected_buffer: vec!["a","b","c","b"],
    expected_buffer_saved: false,
    expected_selection: (2,2),
    expected_clipboard: vec!["b"],
    expected_prints: vec![
      Print{
        text: vec!["Undid 1 operation(s) to right after 2t.".to_owned()],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec!["2t", "2d"],
  }.run()
}

// Try out the last index literal
#[test]
fn redo_all() {
  PrintTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec![
      "2t", // copy line 2 to end of buffer
      "2d", // delete the original
      "u*2", // undo the delete by absolute index (2 first from fixture (0 indexed))
      "u$", // redo everything
    ],
    expected_buffer: vec!["a","c","b"],
    expected_buffer_saved: false,
    expected_selection: (2,2),
    expected_clipboard: vec!["b"],
    expected_prints: vec![
      Print{
        text: vec!["Undid 1 operation(s) to right after 2t.".to_owned()],
        n: false,
        l: false,
      },
      Print{
        text: vec!["Redid 1 operation(s) to right after 2d.".to_owned()],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec!["2t", "2d"],
  }.run()
}

// Try out the add/subtract undo index
#[test]
fn undo_math() {
  PrintTest{
    init_buffer: vec!["a","b","c"],
    init_clipboard: vec![],
    command_input: vec![
      "2t", // copy line 2 to end of buffer
      "2d", // delete the original
      "u.-1+3-3", // undo the delete by explicit relative
    ],
    expected_buffer: vec!["a","b","c","b"],
    expected_buffer_saved: false,
    expected_selection: (2,2),
    expected_clipboard: vec!["b"],
    expected_prints: vec![
      Print{
        text: vec!["Undid 1 operation(s) to right after 2t.".to_owned()],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec!["2t", "2d"],
  }.run()
}


// Verify behaviour of 'U' command
//
// - Prints undo snapshots
// - Doesn't allow selection or index
// - Accepts a history index, same as for 'u'
// - Accepts flags after the argument
//   - 'a' prints absolute indices (instead of relative to current)
//   - 'A' prints all undo snapshots instead of the nearest surrounding

// Normal case
#[test]
fn undo_list() {
  PrintTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    command_input: vec![
      "U",
    ],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![" 1: Before reading in a file (empty)\n>0: initial load (saved)\n".to_owned()],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec![],
  }.run()
}

// Normal case
#[test]
fn undo_list_absolute() {
  PrintTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    command_input: vec![
      "Ua",
    ],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![" *0: Before reading in a file (empty)\n>*1: initial load (saved)\n".to_owned()],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec![],
  }.run()
}

// Normal case
#[test]
fn undo_list_everything_absolute() {
  PrintTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    command_input: vec![
      "i",
      "a",
      ".",
      ".s_._$0\n$0_g",
      ".s",
      ".s",
      ".s",
      ".s",
      ".s",
      ".s",
      ".s",
      ".d",
      ".d",
      ".d",
      ".d",
      ",j",
      "UAa",
    ],
    expected_buffer: vec!["aaaaa"],
    expected_buffer_saved: false,
    expected_selection: (1,1),
    expected_clipboard: vec!["a", "a", "a", "a", "a"],
    expected_prints: vec![
      Print{
        text: vec![concat!(
          " *0: Before reading in a file (empty)\n",
          " *1: initial load (saved)\n",
          " *2: i\n",
          " *3: .s_._$0\n$0_g\n",
          " *4: .s\n",
          " *5: .s\n",
          " *6: .s\n",
          " *7: .s\n",
          " *8: .s\n",
          " *9: .s\n",
          " *10: .s\n",
          " *11: .d\n",
          " *12: .d\n",
          " *13: .d\n",
          " *14: .d\n",
          ">*15: ,j\n",
        ).to_owned()],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec![
      "i",
      ".s_._$0\n$0_g",
      ".s",
      ".s",
      ".s",
      ".s",
      ".s",
      ".s",
      ".s",
      ".d",
      ".d",
      ".d",
      ".d",
      ",j",
    ],
  }.run()
}
