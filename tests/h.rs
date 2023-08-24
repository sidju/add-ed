// Test 'h', 'H' and "help" command
// (In that order)

mod shared;
use shared::fixtures::PrintTest;
use shared::mock_ui::*;

use shared::dummy_io::DummyIO;
use add_ed::ui::ScriptedUI;
use add_ed::Ed;
use add_ed::error::EdError;
use add_ed::messages::HELP_TEXT;

// We have some tests without fixtures in here, as we shouldn't panic on error
// and care about state.print_errors unlike all other fixtures.

// Verify behaviour of 'h' command
//
// - Takes no index.
// - Takes no flags or arguments.
// - Doesn't modify state.selection.
// - Doesn't set/unset unsaved.
// - If an error has occured during current session:
//   - Prints that error.
//   - Else prints that no error has occured (not an error).

// Test 'h' when no error
#[test]
fn help_noerror() {
  PrintTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    command_input: vec!["h"],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["No errors recorded.".to_string()],
        n: false,
        l: false,
      }
    ],
  }.run();
}

// Test 'h' when there is an error
#[test]
fn help() {
  let mut io = DummyIO::new();
  let mut inner_ui = MockUI{ prints_history: Vec::new() };
  let mut ui = ScriptedUI{
    print_ui: Some(&mut inner_ui),
    input: vec![
      ",n",
      "h",
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
  assert_eq!(ed.run_macro(&mut ui), Err(EdError::IndexTooBig{index:1,buffer_len:0}));
  ed.run_macro(&mut ui).expect("Error running test");
  assert!(ed.history.current().is_empty());
  assert_eq!(
    vec![
      Print{
        text: vec![EdError::IndexTooBig{index:1,buffer_len:0}.to_string(),],
        n: false,
        l: false,
      },
    ],
    inner_ui.prints_history,
  );
}

#[test]
fn help_toggle() {
  let mut io = DummyIO::new();
  let mut ui = ScriptedUI{
    print_ui: None,
    input: vec![
      "H",
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
  assert_eq!(ed.print_errors, true);
  ed.run_macro(&mut ui).expect("Error running test");
  assert_eq!(ed.print_errors, false);
  assert!(ed.history.current().is_empty());
}

#[test]
fn help_text() {
  PrintTest{
    init_buffer: vec![],
    init_clipboard: vec![],
    command_input: vec!["help"],
    expected_buffer: vec![],
    expected_buffer_saved: true,
    expected_selection: (1,0),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: [HELP_TEXT].iter().map(|s| s.to_string()).collect(),
        n: false,
        l: false,
      }
    ],
  }.run();
}
