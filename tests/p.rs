// Tests for 'p' command

mod shared;
use shared::fixtures::{
  PrintTest,
};
use shared::mock_ui::Print;
use shared::dummy_io::DummyIO;

use add_ed::{
  ui::ScriptedUI,
  macros::Macro,
  Ed,
};

// Verify behaviour of 'p' command
//
// - Takes optional selection
//   - If given prints selection
//   - If not given prints state.selection
// - Accepts printing flags
// - Prints literally if state.l is set
// - Prints numbered if state.n is set
// - state.selection is set to printed selection
// - Does not change unsaved

// Normal case, just print some lines
#[test]
fn print() {
  PrintTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["1,4p"],
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
        n: false,
        l: false,
      }
    ],
    expected_history_tags: vec![],
  }.run()
}

// Test flag handling and using default selection
#[test]
fn print_literal_numbered_noselection() {
  PrintTest{
    init_buffer: vec!["a","\tb","$c","d"],
    init_clipboard: vec![],
    command_input: vec!["pln"],
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

// Verify toggling of numbered by knowing initial state and verifying after
#[test]
fn toggle_numbered_on() {
  let mut io = DummyIO::new();
  let macros = std::collections::HashMap::<String,Macro>::new();
  let mut ui = ScriptedUI{
    print_ui: None,
    input: vec![
      "Pn",
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
  ed.history.set_saved();
  loop {
   if ed.get_and_run_command(&mut ui).expect("Error running test") { break; }
  }
  assert_eq!(ed.n, true);
  assert!(ed.history.current().is_empty());
}
#[test]
fn toggle_numbered_off() {
  let mut io = DummyIO::new();
  let macros = std::collections::HashMap::<String,Macro>::new();
  let mut ui = ScriptedUI{
    print_ui: None,
    input: vec![
      "Pn",
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
  ed.history.set_saved();
  ed.n = true;
  loop {
   if ed.get_and_run_command(&mut ui).expect("Error running test") { break; }
  }
  assert_eq!(ed.n, false);
  assert!(ed.history.current().is_empty());
}
// Verify toggling of literal by knowing state before and verifying after
#[test]
fn toggle_literal_on() {
  let mut io = DummyIO::new();
  let macros = std::collections::HashMap::<String,Macro>::new();
  let mut ui = ScriptedUI{
    print_ui: None,
    input: vec![
      "Pl",
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
fn toggle_literal_off() {
  let mut io = DummyIO::new();
  let macros = std::collections::HashMap::<String,Macro>::new();
  let mut ui = ScriptedUI{
    print_ui: None,
    input: vec![
      "Pl",
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
