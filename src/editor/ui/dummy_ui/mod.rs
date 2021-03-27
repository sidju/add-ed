//! This module is for supporting scripts.
//! The entire script is put into a vector of strings and treated as input.
//! It optionally takes a mutable UI reference, to support printing when the script requests it.

struct DummyUI {
  index: usize,
  input: Vec<String>,
  print_ui: Option<&mut dyn UI>,
}
impl UI for DummyUI {
  /// Gets the next line of the input
  fn get_command(&mut self,
    _buffer: & dyn Buffer,
  ) -> Result<String, &'static str> {
    if self.index >= self.input.len() {
      Err(NO_INPUT)
    }
    else {
      let ret = self.input[self.index];
      self.index += 1;
      Ok(ret)
    }
  }

  /// Gets lines from input until one matches terminator
  fn get_input(&mut self,
    _buffer: & dyn Buffer,
    terminator: char,
  ) -> Result<Vec<String>, &'static str> {
    let ret = Vec::new();
    // Loop until we run out of data or find the terminator
    loop {
      // First, check that current index is valid
      if self.index >= self.input.len() {
        return Err(NO_INPUT);
      }

      // Then check if we found the terminator
      if
        self.input[self.index].len() == 2 &&
        self.input[self.index].chars().next().unwrap() == terminator
      {
        index += 1;
        return Ok(ret);
      };

      // If nothing else special, it must be input
      ret.push(self.input[self.index]);
      self.index += 1;
    }
  }

  /// Printing is handed to the print_ui if one was given, else ignored
  fn print(&mut self,
    buffer: & dyn Buffer,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<(), &'static str> {
    match self.print_ui {
      Some(ui) => {
        ui.print(buffer, selection, numbered, literal)
      },
      None => Ok(()),
    }
  }
}
