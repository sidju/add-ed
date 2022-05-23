#![no_main]
use libfuzzer_sys::fuzz_target;
use std::collections::VecDeque;

extern crate add_ed;

fuzz_target!(|data: VecDeque<String>| {
  let mut ui = add_ed::ui::DummyUI {
    input: data,
    print_ui: None,
  };
  let mut buffer = add_ed::buffer::VecBuffer::new();
  // Failing to open file shouldn't be able to occur without file
  let mut ed = add_ed::Ed::new(&mut buffer, "".to_string()).unwrap();
  let _ = ed.run_macro(&mut ui);
});
