use super::*;

mod print;
use print::*;
mod input;
use input::*;

const THEME: &[u8] = include_bytes!("theme.xml");

pub struct CliUI {
  stdout: std::io::Stdout,
  term_size: (u16, u16),

  // Theme variables, because they need to exist for printing
  // Would be nice to get rid of more of these
  theme: syntect::highlighting::Theme,
  syntax: syntect::parsing::SyntaxSet,
}
impl CliUI {
  pub fn new() -> Self {
    // Read our custom theme from const
    let mut theme_reader = std::io::Cursor::new(&THEME[..]);
    let theme = syntect::highlighting::ThemeSet::load_from_reader(&mut theme_reader).unwrap();

    // Get syntax definitions
    // TODO: Look for expansions. Some filetypes not recognised
    let syntax = syntect::parsing::SyntaxSet::load_defaults_newlines();

    Self {
      stdout: std::io::stdout(),
      term_size: crossterm::terminal::size().unwrap_or((80,24)),
      theme: theme,
      syntax: syntax,
    }
  }
}

impl UI for CliUI {
  fn print(
    &mut self,
    data: &str,
  ) -> Result<(), &'static str> {
    print(&mut self.stdout, data);
    Ok(())
  }

  fn get_command(
    &mut self,
    buffer: & dyn Buffer,
  ) -> Result<String, &'static str> {
    get_command(&mut self, buffer)
  }

  fn get_input(
    &mut self,
    buffer: & dyn Buffer,
    terminator: char,
  ) -> Result<Vec<String>, &'static str> {
    get_input(&mut self, buffer, terminator)
  }

  fn print_selection(
    &mut self,
    buffer: & dyn Buffer,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<(), &'static str> {
  }
}
