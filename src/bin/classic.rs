use std::collections::HashMap;

use add_ed::EdState;
use add_ed::ui::{UI, UILock};
use add_ed::error_consts::*;

/// A simple UI based on the original ED editor
struct ClassicUI{}
impl UI for ClassicUI {
    fn print_message(
    &mut self,
    s: &str
  ) -> Result<(), &'static str> {
    println!("{}", s);
    Ok(())
  }
  fn get_command(
    &mut self,
    _ed: EdState,
    _prefix: Option<char>,
  ) -> Result<String, &'static str> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
      .map_err(|_| TERMINAL_READ)?;
    Ok(input)
  }
  fn get_input(
    &mut self,
    _ed: EdState,
    terminator: char,
    #[cfg(feature = "initial_input_data")]
    initial_buffer: Option<Vec<String>>, // error if Some
  ) -> Result<Vec<String>, &'static str> {
    #[cfg(feature = "initial_input_data")]
    {
      // If an initial buffer is given that is invalid
      if initial_buffer.is_some() { return Err(add_ed::error_consts::UNSUPPORTED_INITIAL_DATA); }
    }
    let mut input = Vec::new();
    let stdin = std::io::stdin();
    let terminator = format!("{}\n", terminator);
    loop {
      let mut buf = String::new();
      let res = stdin.read_line(&mut buf);
      if res.is_err() {
        return Err(TERMINAL_READ);
      }
      if buf == terminator { return Ok(input); }
      else { input.push(buf); }
    }
  }
  fn print_selection(
    &mut self,
    ed: EdState,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<(), &'static str> {
    let selected = ed.buffer
      .get_selection(selection)?
      .map(|(_, t)| t);
    let mut line_nr = selection.0;
    for line in selected {
      if numbered {
        print!("{}: ", line_nr);
        line_nr += 1;
      }
      for ch in line.chars() {
        match ch {
          '\n' => {
            if literal { println!("$") } else { println!() }
          },
          '$' => {
            if literal { print!("\\$") } else { print!("$") }
          },
          c => print!("{}", c),
        }
      }
    }
    Ok(())
  }
  // Requires no additional code for locking and unlocking
  fn lock_ui(&mut self) -> UILock<'_> {
    UILock::new(self)
  }
  fn unlock_ui(&mut self) {}
}

fn main() {
  // Here one should add command line argument parsing, to get the filename
  let path = "".to_string();
  let mut ui = ClassicUI{};
  let mut buffer = add_ed::buffer::VecBuffer::new();
  let mut io = add_ed::io::LocalIO::new();
  // Read in the file given and instantiate the editor
  let mut ed = add_ed::Ed::new(&mut buffer, &mut io, path, HashMap::new(), false, false)
    .expect("Failed to open file.")
  ;
  // Run the editor with the created UI
  ed.run(&mut ui).unwrap();
}
