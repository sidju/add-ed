// Tests for 'l' and 'L' command
// 'l' tests are immediately after imports
// 'L' tests are after the 'l' tests

mod shared;
use shared::fixtures::{
  PrintTest,
};
use shared::mock_ui::Print;
use shared::dummy_io::DummyIO;
use add_ed::{
  buffer::Buffer,
  ui::ScriptedUI,
  Ed,
};
use std::collections::HashMap;

// Verify behaviour of 'l' command
//
// - Takes optional selection
//   - If given prints selection
//   - If not given prints state.selection
// - Accepts printing flags
// - Prints literally unless state.literal is set
//   (What literal printing is is left to the UI)
// - Prints numbered if state.numbered is set
// - state.selection is set to printed selection
// - Does not change unsaved

// Normal case, just print some lines literally
#[test]
fn literal() {
  PrintTest{
    init_buffer: vec!["a","\tb","$c","d"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["1,4l"],
    expected_selection: (1,4),
    expected_buffer: vec!["a","\tb","$c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_filepath: "path",
    expected_prints: vec![
      Print{
        text: vec![
          "a\n".to_string(),
          "\tb\n".to_string(),
          "$c\n".to_string(),
          "d\n".to_string(),
        ],
        n: false,
        l: true,
      }
    ],
  }.run()
}

// Test flag handling and using default selection
#[test]
fn literal_numbered_noselection() {
  PrintTest{
    init_buffer: vec!["a","\tb","$c","d"],
    init_clipboard: vec![],
    init_filepath: "path",
    command_input: vec!["ln"],
    expected_selection: (1,4),
    expected_buffer: vec!["a","\tb","$c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_filepath: "path",
    expected_prints: vec![
      Print{
        text: vec![
          "a\n".to_string(),
          "\tb\n".to_string(),
          "$c\n".to_string(),
          "d\n".to_string(),
        ],
        n: true,
        l: true,
      }
    ],
  }.run()
}

// Verify behaviour of 'L' command
//
// - Takes no selection
// - Does not modify selection
// - Does not modify saved
// - Toggles the state.literal bool, which sets if to print literal by default

// Verify toggling of literal by viewing state before and after
#[test]
fn literal_toggle_on() {
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();
  buffer.set_saved();
  let mut ui = ScriptedUI{
    print_ui: None,
    input: vec![
      "L",
    ].iter().map(|x|{
      let mut s = x.to_string();
      s.push('\n');
      s
    }).collect(),
  };
  // Construct editor state and run
  let mut ed = Ed::new(
    &mut buffer,
    &mut io,
    "path".to_owned(),
    HashMap::new(),
  );
  ed.run_macro(&mut ui).expect("Error running test");
  assert_eq!(ed.l, true);
  assert!(buffer.is_empty());
}
#[test]
fn literal_toggle_off() {
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();
  buffer.set_saved();
  let mut ui = ScriptedUI{
    print_ui: None,
    input: vec![
      "L",
    ].iter().map(|x|{
      let mut s = x.to_string();
      s.push('\n');
      s
    }).collect(),
  };
  // Construct editor state and run
  let mut ed = Ed::new(
    &mut buffer,
    &mut io,
    "path".to_owned(),
    HashMap::new(),
  );
  ed.l = true;
  ed.run_macro(&mut ui).expect("Error running test");
  assert_eq!(ed.l, false);
  assert!(buffer.is_empty());
}
