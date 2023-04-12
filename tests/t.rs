// Test behaviour of 't' and 'T'
// 't' tests are immediately after imports
// 'T' tests are thereafter

mod shared;
use shared::fixtures::{
  BasicTest,
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 't' command
//
// - Takes optional selection
//   - If given copies lines in selection
//   - If none given copies lines in state.selection
// - Takes optional index argument after command
//   - If given copies lines to after given index
//   - If none given copies lines to after end of buffer
//   - Special, index 0 copies to beginning of buffer
// - Accepts printing flags after the index
// - Sets unsaved
// - Selection after execution is the new copied lines

// Test fully defined command
#[test]
fn copy() {
  BasicTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["3,4t0"],
    expected_buffer: vec!["c","d","a","b","c","d"],
    expected_buffer_saved: false,
    expected_clipboard: vec![],
    expected_selection: (1,2),
  }.run()
}

// Test with default selection and default index
// (Uses '#' to set selection without any print)
#[test]
fn copy_noindex_noselection_print() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["2,3#","tp"],
    expected_buffer: vec!["a","b","c","d","b","c"],
    expected_buffer_saved: false,
    expected_selection: (5,6),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["b\n".to_string(),"c\n".to_string(),],
        n: false,
        l: false,
      },
    ],
  }.run()
}

// Verify behaviour of 'T' command
//
// - Takes optional selection
//   - If given copies lines in given selection
//   - If not given copies lines in state.selection
// - Takes optional index argument
//   - If given copies lines to before index
//   - If not given copies lines to beginning of buffer
//   - Special, copying to before line 0 copies to beginning of buffer
// - Takes printing flags after index argument
// - Sets unsaved
// - Selection after execution is the new copied lines

// Test fully defined command
#[test]
fn copy_before() {
  BasicTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["3,4T2"],
    expected_buffer: vec!["a","c","d","b","c","d"],
    expected_buffer_saved: false,
    expected_clipboard: vec![],
    expected_selection: (2,3),
  }.run()
}

// Test with default selection and default index
// (Uses '#' to set selection without any print)
#[test]
fn copy_before_noindex_noselection_print() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["2,3#","Tp"],
    expected_buffer: vec!["b","c","a","b","c","d"],
    expected_buffer_saved: false,
    expected_selection: (1,2),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["b\n".to_string(),"c\n".to_string(),],
        n: false,
        l: false,
      },
    ],
  }.run()
}
