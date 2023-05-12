// Tests for 'u' and 'U' command
// 'u' tests are after imports
// 'U' tests are thereafter

mod shared;
use shared::fixtures::{
  BasicTest,
};

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
  BasicTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec!["dummy"],
    command_input: vec!["1d","3d","u2"],
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec!["d"], // Because "3d" sets it and 'u' leaves it
    expected_selection: (2,2),
  }.run()
}

// Test defaults
#[test]
fn undo_default() {
  BasicTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec!["dummy"],
    command_input: vec!["1d","3d","u"],
    expected_buffer: vec!["b","c","d"],
    expected_buffer_saved: false,
    expected_clipboard: vec!["d"], // Because 'd' sets it and 'u' leaves it
    expected_selection: (2,2),
  }.run()
}

// Verify behaviour of 'U' command
//
// - Doesn't allow selection or index
// - Accepts a signed numeric argument of how many steps to redo
//   - If none given defaults to 1
// - Changes the state of the buffer the given number of modifying commands
//   forward in history, negative number moves backwards.
//   - If it isn't possible to move the given number of steps gives error,
//     INVALID_UNDO_STEPS.
// - Sets unsaved unconditionally for now.
//   (Tracking of which undo step was saved could be added)
// - Doesn't modify selection.
//   (Setting selection as if after the redone command later maybe? If so
//   pairing selection with buffer state could be good, or validation.
//   Current code may leave state.selection in an invalid state.
//   Optimum would be to store selection with the buffer state, but that would
//   require a _big_ refactor...)

// Test fully defined
#[test]
fn redo() {
  BasicTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec!["dummy"],
    command_input: vec!["1d","d","u2","U2"],
    expected_buffer: vec!["c","d"],
    expected_buffer_saved: false,
    expected_clipboard: vec!["b"], // Because 'd' sets it and 'u'/'U' leaves it
    expected_selection: (1,1),
  }.run()
}

// Test defaults
#[test]
fn redo_default() {
  BasicTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec!["dummy"],
    command_input: vec!["1d","3d","u","U"],
    expected_buffer: vec!["b","c"],
    expected_buffer_saved: false,
    expected_clipboard: vec!["d"], // Because 'd' sets it and 'u' leaves it
    expected_selection: (2,2),
  }.run()
}
