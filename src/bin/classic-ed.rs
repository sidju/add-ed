use add_ed::{Ed, Result};
use add_ed::error::{EdError, UIError};
use add_ed::ui::{UI, UILock};
/// Error type for a [`ClassicUI`]
#[derive(Debug)]
enum ClassicUIError {
  TerminalError,
  #[cfg(feature = "initial_input_data")]
  InitialData,
}
impl std::fmt::Display for ClassicUIError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use ClassicUIError::*;
    match self {
      TerminalError => write!(f, "Failed to read from terminal. This is bad, save if you can."),
      #[cfg(feature = "initial_input_data")]
      InitialData => write!(f, "UI received initial data when taking input, this isn't supported."),
    }
  }
}
impl std::error::Error for ClassicUIError {}
impl add_ed::error::UIError for ClassicUIError {}

/// A simple UI based on the original ED editor
struct ClassicUI{}
impl UI for ClassicUI {
    fn print_message(
    &mut self,
    s: &str
  ) -> Result<()> {
    println!("{}", s);
    Ok(())
  }
  fn get_command(
    &mut self,
    _ed: &Ed,
    _prefix: Option<char>,
  ) -> Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
      .map_err(|_| -> Box<dyn UIError> {ClassicUIError::TerminalError.into()})?;
    Ok(input)
  }
  fn get_input(
    &mut self,
    _ed: &Ed,
    terminator: char,
    #[cfg(feature = "initial_input_data")]
    initial_buffer: Option<Vec<String>>, // error if Some
  ) -> Result<Vec<String>> {
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
        return Err(EdError::UI(Box::new(ClassicUIError::TerminalError)))?;
      }
      if buf == terminator { return Ok(input); }
      else { input.push(buf); }
    }
  }
  fn print_selection(
    &mut self,
    ed: &Ed,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<()> {
    let selected = ed.buffer.get_selection(selection)?;
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
  // Here is where command line argument parsing should go
  let path = "".to_string();
  // Construct state components
  let mut ui = ClassicUI{};
  let mut io = add_ed::io::LocalIO::new();
  // Construct Ed
  let mut ed = Ed::new(&mut io, path);
  // Apply any configurations
  ed.cmd_prefix = None;
  // Run
  ed.run(&mut ui).unwrap();
}
