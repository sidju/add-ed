#![no_main]
use libfuzzer_sys::fuzz_target;
use std::collections::VecDeque;
use std::collections::HashMap;

extern crate add_ed;
use add_ed::io::fake_io::FakeIO;

fuzz_target!(|data: VecDeque<String>| {
  let mut ui = add_ed::ui::ScriptedUI {
    input: data,
    print_ui: None,
  };
  let mut io = FakeIO{fake_fs: HashMap::new(), fake_shell: HashMap::new()};
  let mut buffer = add_ed::buffer::Buffer::new();
  let mut ed = add_ed::Ed::new(&mut buffer, &mut io, "".to_string());
  let _ = ed.run_macro(&mut ui);
});
