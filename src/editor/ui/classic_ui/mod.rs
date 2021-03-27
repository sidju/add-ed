//! The most basic UI implementation.
//! It aims to behave like the original 'ed' cli.

use std::io;

use crate::error_consts::*;
use super::*;

pub struct ClassicUI {
}
impl UI for ClassicUI {
  fn print(
    &mut self,
    s: &str
  ) -> Result<(), &'static str> {
    print!("{}", s);
    Ok(())
  }
  fn get_command(
    &mut self,
    _: & dyn Buffer,
  ) -> Result<String, &'static str> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
      .map_err(|_| TERMINAL_READ)?;
    Ok(input)
  }
  fn get_input(
    &mut self,
    _: & dyn Buffer,
    terminator: char
  ) -> Result<Vec<String>, &'static str> {
    let mut input = Vec::new();
    let stdin = io::stdin();
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
    buffer: & dyn Buffer,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<(), &'static str> {
    let selected = buffer.get_selection(selection)?;
    let mut line_nr = selection.0;
    for line in selected {
      if numbered {
        line_nr += 1;
        print!("{}: ", line_nr);
      }
      for ch in line.chars() {
        match ch {
          '\n' => {
              if literal { print!("$/n") } else { print!("\n") }
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
}
