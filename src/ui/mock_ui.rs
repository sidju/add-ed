use crate::{
  Ed,
  ui::UI,
  ui::UILock,
};
use super::Result;

/// Struct describing a print invocation.
#[derive(Debug, PartialEq)]
pub struct Print {
  /// The lines printed. Each string is a separate line.
  pub text: Vec<String>,
  /// If true, would have printed with line numbers.
  pub n: bool,
  /// If true, would have printed with literal escapes.
  pub l: bool,
}

/// A mock UI that logs all prints and panics when asked for input.
pub struct MockUI {
  pub prints_history: Vec<Print>,
}

impl UI for MockUI {
  fn print_message(
    &mut self,
    data: &str,
  ) -> Result<()> {
    self.prints_history.push(
      Print{
        text: vec![data.to_owned()],
        n: false,
        l: false,
      }
    );
    Ok(())
  }

  fn print_selection(
    &mut self,
    ed: &Ed,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<()> {
    self.prints_history.push(
      Print{
        text: ed.history.current().get_lines(selection)?
          .map(|s| s.to_string())
          .collect()
        ,
        n: numbered,
        l: literal,
      }
    );
    Ok(())
  }

  fn get_command(
    &mut self,
    _ed: &Ed,
    _prefix: Option<char>,
  ) -> Result<String> {
    panic!("get_command not implemented on mock ui")
  }

  fn get_input(
    &mut self,
    _ed: &Ed,
    _terminator: char,
    #[cfg(feature = "initial_input_data")]
    _initial_buffer: Option<Vec<String>>
  ) -> Result<Vec<String>> {
    panic!("get_input not implemented on mock ui")
  }

  fn lock_ui(&mut self) -> UILock<'_> {
    UILock::new(self)
  }
  fn unlock_ui(&mut self){}
}
