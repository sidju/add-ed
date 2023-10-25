#![no_main]
use libfuzzer_sys::fuzz_target;
use std::collections::HashMap;

extern crate add_ed;
use add_ed::io::fake_io::FakeIO;

// You can do anything in 4 input lines, so that should be enough
// (and it should help with the out-of-memory problems while fuzzing)
fuzz_target!(|data: [String; 4]| {
  let mut ui = add_ed::ui::ScriptedUI {
    input: data.into(),
    print_ui: None,
  };
  let mut io = FakeIO{fake_fs: HashMap::new(), fake_shell: HashMap::new()};
  let macro_store = HashMap::new();
  let mut ed = add_ed::Ed::new(&mut io, &macro_store);
  loop {
    if let Ok(true) = ed.get_and_run_command(&mut ui) { break; }
  }
});
