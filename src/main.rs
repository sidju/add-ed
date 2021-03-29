mod editor;

pub use editor::error_consts;

fn main() {
  // UI and Buffer created separate from the editor to
  // better support for strange initialisation requirements

  // Create a UI for the editor
  let mut ui = editor::ui::ClassicUI{};

  // Instantiate a buffer implementation for it
  let mut buffer = editor::buffer::VecBuffer::new();

  // Then construct the editor
  let mut ed = editor::Ed::new(&mut buffer, "".to_string()).expect("Failed to open file");

  // And finally run it
  ed.run(&mut ui).unwrap();
}
