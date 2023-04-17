// Tests for 'q' and 'Q' command

mod shared;
use shared::fixtures::{
  ErrorTest,
};
use shared::dummy_io::DummyIO;
use shared::dummy_ui::DummyUI;
use add_ed::{
  buffer::Buffer,
  Ed,
};

// Verify behaviour of 'q' and 'Q' command
//
// - Takes no selection
// - If unsaved and 'q' errors on UNSAVED_CHANGES
// - If no error, returns true from command execution to signify being done

// Normal quit when saved
#[test]
fn quit() {
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();
  buffer.set_saved();
  let mut ui = DummyUI{};
  // Construct editor state and run
  let mut ed = Ed::new(
    &mut buffer,
    &mut io,
    "path".to_owned(),
  );
  assert!(ed.run_command(&mut ui, "q\n").expect("Error running test"));
}

// Error test when trying to quit with unsaved changes
#[test]
fn quit_unsaved() {
  ErrorTest{
    init_buffer: vec![],
    command_input: vec!["q"],
    expected_error: add_ed::error_consts::UNSAVED_CHANGES,
  }.run()
}

// Test force shutdown when unsaved
#[test]
fn force_quit_unsaved() {
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();
  buffer.set_unsaved();
  let mut ui = DummyUI{};
  // Construct editor state and run
  let mut ed = Ed::new(
    &mut buffer,
    &mut io,
    "path".to_owned(),
  );
  assert!(ed.run_command(&mut ui, "Q\n").expect("Error running test"));
}
