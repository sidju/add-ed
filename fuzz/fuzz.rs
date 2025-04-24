#![no_main]
use libfuzzer_sys::fuzz_target;
use std::collections::HashMap;

extern crate add_ed;
use add_ed::io::fake_io::FakeIO;

// You can do anything in 4 input lines, so that should be enough
// (and it should help with the out-of-memory problems while fuzzing)
fuzz_target!(|data: &str| {
  let mut ui = add_ed::ui::ScriptedUI {
    input: data.split('\n').map(|x| format!("{}\n", x)).collect(),
    print_ui: None,
  };
  let mut io = FakeIO{fake_fs: HashMap::new(), fake_shell: HashMap::new()};
  let macro_store = HashMap::new();
  let mut ed = add_ed::Ed::new(&mut io, &macro_store);
  loop {
    match ed.get_and_run_command(&mut ui) {
      Ok(true) => break, // Needed to not infinitely ask for input that doesn't exist
      Err(add_ed::EdError::Internal(e)) => panic!("This is a real bug! {e:?}"),
      _ => ()
    }
  }
});
