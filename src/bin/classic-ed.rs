use add_ed::{Ed, Result};
use add_ed::error::UIError;
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
impl add_ed::error::UIErrorTrait for ClassicUIError {}

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
      .map_err(|_| -> UIError { ClassicUIError::TerminalError.into() })?;
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
        return Err(Into::<UIError>::into(ClassicUIError::TerminalError).into());
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
    let selected = ed.history.current().get_lines(selection)?;
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

use clap::Parser;
#[derive(Parser)]
#[command(version)]
struct CliArgs {
  /// Path to file to open or ! followed by command to read output from
  #[arg(default_value_t)] // Default to empty string
  file: String,
}
fn main() {
  let cli = CliArgs::parse();
  // Construct state components
  let mut ui = ClassicUI{};
  let mut io = add_ed::io::LocalIO::new();
  // Construct Ed
  let mut ed = Ed::new(&mut io);
  // Apply any configurations
  // Load in from path if given
  if ! cli.file.is_empty() {
    if let Err(e) = ed.run_command(&mut ui, &format!("e{}", cli.file)) {
      // On failure to open file we print error and quit
      ui.print_message(&e.to_string()).expect("Failed to print error after failing to open file");
      return;
    }
  }
  // Run
  ed.run(&mut ui).expect("Failed to print during execution.");
}
