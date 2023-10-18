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
  ui::ScriptedUI,
  Ed,
};

// Verify behaviour of 'l' command
//
// - Takes optional selection
//   - If given prints selection
//   - If not given prints state.selection
// - Accepts printing flags
// - Prints literally unless state.l is set
//   (What literal printing is is left to the UI)
// - Prints numbered if state.n is set
// - state.selection is set to printed selection
// - Does not change unsaved

// Normal case, just print some lines literally
#[test]
fn literal() {
  PrintTest{
    init_buffer: vec!["a","\tb","$c","d"],
    init_clipboard: vec![],
    command_input: vec!["1,4l"],
    expected_selection: (1,4),
    expected_buffer: vec!["a","\tb","$c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
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
    expected_history_tags: vec![],
  }.run()
}

// Test flag handling and using default selection
#[test]
fn literal_numbered_noselection() {
  PrintTest{
    init_buffer: vec!["a","\tb","$c","d"],
    init_clipboard: vec![],
    command_input: vec!["ln"],
    expected_selection: (1,4),
    expected_buffer: vec!["a","\tb","$c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
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
    expected_history_tags: vec![],
  }.run()
}

// Verify behaviour of 'L' command
//
// - Takes no selection
// - Does not modify selection
// - Does not modify saved
// - Toggles the state.l bool, which sets if to print literal by default

// Verify toggling of literal by knowing state before and verifying after
#[test]
fn literal_toggle_on() {
  let mut io = DummyIO::new();
  let macros = std::collections::HashMap::new();
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
    &mut io,
    &macros,
  );
  loop {
    if ed.get_and_run_command(&mut ui).expect("Error running test") { break; }
  }
  assert_eq!(ed.l, true);
  assert!(ed.history.current().is_empty());
}
#[test]
fn literal_toggle_off() {
  let mut io = DummyIO::new();
  let macros = std::collections::HashMap::new();
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
    &mut io,
    &macros,
  );
  ed.l = true;
  loop {
    if ed.get_and_run_command(&mut ui).expect("Error running test") { break; }
  }
  assert_eq!(ed.l, false);
  assert!(ed.history.current().is_empty());
}
