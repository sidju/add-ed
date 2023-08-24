// Tests for 'n' and 'N' command
// 'n' tests are immediately after imports
// 'N' tests are after the 'N' tests

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

// Verify behaviour of 'n' command
//
// - Takes optional selection
//   - If given prints selection
//   - If not given prints state.selection
// - Accepts printing flags
// - Prints numbered unless state.n is set
//   (What numbered means is is left to the UI)
// - Prints literal if state.l is set
// - state.selection is set to printed selection
// - Does not change unsaved

// Normal case, just print some lines numbered
#[test]
fn numbered() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["1,4n"],
    expected_selection: (1,4),
    expected_buffer: vec!["a","b","c","d"],
    expected_buffer_saved: true,
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "a\n".to_string(),
          "b\n".to_string(),
          "c\n".to_string(),
          "d\n".to_string(),
        ],
        n: true,
        l: false,
      }
    ],
  }.run()
}

// Test flag handling and using default selection
#[test]
fn numbered_literal_noselection() {
  PrintTest{
    init_buffer: vec!["a","\tb","$c","d"],
    init_clipboard: vec![],
    command_input: vec!["nl"],
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
  }.run()
}

// Verify behaviour of 'N' command
//
// - Takes no selection
// - Does not modify selection
// - Does not modify saved
// - Toggles the state.n bool, which sets if to print numbered by default

// Verify toggling of numbered by knowing initial state and verifying after
#[test]
fn numbered_toggle_on() {
  let mut io = DummyIO::new();
  let mut ui = ScriptedUI{
    print_ui: None,
    input: vec![
      "N",
    ].iter().map(|x|{
      let mut s = x.to_string();
      s.push('\n');
      s
    }).collect(),
  };
  // Construct editor state and run
  let mut ed = Ed::new(
    &mut io,
  );
  ed.history.set_saved();
  ed.run_macro(&mut ui).expect("Error running test");
  assert_eq!(ed.n, true);
  assert!(ed.history.current().is_empty());
}
#[test]
fn numbered_toggle_off() {
  let mut io = DummyIO::new();
  let mut ui = ScriptedUI{
    print_ui: None,
    input: vec![
      "N",
    ].iter().map(|x|{
      let mut s = x.to_string();
      s.push('\n');
      s
    }).collect(),
  };
  // Construct editor state and run
  let mut ed = Ed::new(
    &mut io,
  );
  ed.history.set_saved();
  ed.n = true;
  ed.run_macro(&mut ui).expect("Error running test");
  assert_eq!(ed.n, false);
  assert!(ed.history.current().is_empty());
}
