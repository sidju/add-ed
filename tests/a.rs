// Tests for 'a' and 'A' command
// 'a' tests are immediately after imports
// 'A' tests are after the 'a' tests

mod shared;
use shared::fixtures::BasicTest;

// Verify behaviour of 'a' command
//
// - Takes optional selection
//   - If given, appends to selection.1
//   - If none, appends to state.selection.1
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
    command_input: vec!["a","."],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0)
  }.run();
}

// Empty buffer and no input
// (Should behave identically as noselection above)
#[test]
fn append_noinput_nobuffer() {
  BasicTest{
    init_buffer: vec![],
    command_input: vec!["0a","."],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0)
  }.run();
}

// No selection on empty buffer with input
#[test]
fn append_noselection_nobuffer() {
  BasicTest{
    init_buffer: vec![],
    command_input: vec!["a","1","2","."],
    expected_buffer: vec!["1","2"],
    expected_buffer_saved: false,
    expected_selection: (1,2)
  }.run();
}

// Empty buffer with input
// (Should behave identically as noselection above)
#[test]
fn append_nobuffer() {
  BasicTest{
    init_buffer: vec![],
    command_input: vec!["0a","1","2","."],
    expected_buffer: vec!["1","2"],
    expected_buffer_saved: false,
    expected_selection: (1,2)
  }.run();
}

// No selection, no input
// (Due to post selection and no input tests default selection)
#[test]
fn append_noselection_noinput() {
  BasicTest{
    init_buffer: vec!["a","b"],
    command_input: vec!["a","."],
    expected_buffer: vec!["a","b"],
    expected_buffer_saved: true,
    expected_selection: (1,2)
  }.run();
}

// No input
// (Should behave identically as noselection above)
#[test]
fn append_noinput() {
  BasicTest{
    init_buffer: vec!["a","b"],
    command_input: vec!["2a","."],
    expected_buffer: vec!["a","b"],
    expected_buffer_saved: true,
    expected_selection: (1,2)
  }.run();
}

// No selection
#[test]
fn append_noselection() {
  BasicTest{
    init_buffer: vec!["a","b"],
    command_input: vec!["a","c","d","."],
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: false,
    expected_selection: (3,4)
  }.run();
}

// Fully defined invocation
// (Should behave identically as noselection above)
#[test]
fn append() {
  BasicTest{
    init_buffer: vec!["a","b"],
    command_input: vec!["1,2a","c","d","."],
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: false,
    expected_selection: (3,4)
  }.run();
}

// Verify behaviour of 'A' command
//
// - Takes optional selection
//   - If given, appends inline to selection.1
//   - If none, appends inline to state.selection.1
//   - (Inline appending to index 0 is not valid, error not a line)
//   - Note: Not valid for any index when buffer is empty
// - Takes input via ui.get_input with '.' as terminator
// - Selection after command is the inserted lines AND the now modified index
//   (If no lines doesn't change selection)
// - Sets unsaved if input given (else no change)

// No selection, no buffer, no input required (returns error before)
// TODO: Use error testing fixture, when errors have been improved
#[test]
#[should_panic]
fn inline_append_empty_buffer() {
  BasicTest{
    init_buffer: vec![],
    command_input: vec!["A"],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0)
  }.run();
}

// No selection, no input
// (Due to post selection and no input tests default selection)
#[test]
fn inline_append_noselection_noinput() {
  BasicTest{
    init_buffer: vec!["a","b"],
    command_input: vec!["A","."],
    expected_buffer: vec!["a","b"],
    expected_buffer_saved: true,
    expected_selection: (1,2)
  }.run();
}

// No input
// (Should behave identically as noselection above)
#[test]
fn inline_append_noinput() {
  BasicTest{
    init_buffer: vec!["a","b"],
    command_input: vec!["2A","."],
    expected_buffer: vec!["a","b"],
    expected_buffer_saved: true,
    expected_selection: (1,2)
  }.run();
}

// No selection
#[test]
fn inline_append_noselection() {
  BasicTest{
    init_buffer: vec!["a","b"],
    command_input: vec!["A","anana","cucumber","."],
    expected_buffer: vec!["a","banana","cucumber"],
    expected_buffer_saved: false,
    expected_selection: (2,3)
  }.run();
}

// Fully defined invocation
// (Should behave identically as noselection above)
#[test]
fn inline_append() {
  BasicTest{
    init_buffer: vec!["a","b"],
    command_input: vec!["1,2A","anana","cucumber","."],
    expected_buffer: vec!["a","banana","cucumber"],
    expected_buffer_saved: false,
    expected_selection: (2,3)
  }.run();
}
