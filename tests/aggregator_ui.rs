// Aggregator UI which collects prints into a Vec
// Intended to help verify that editor prints correctly
use add_ed::{
  EdState,
  ui::UI,
  ui::UILock,
};

pub struct Print {
  pub text: Vec<String>,
  pub n: bool,
  pub l: bool,
}

pub struct AggregatorUI {
  pub prints_history: Vec<Print>,
}

impl UI for AggregatorUI {
  fn print_message(
    &mut self,
    data: &str,
  ) -> Result<(), &'static str> {
    self.prints_history.push(
      Print{
        text: vec![data.to_string()],
        n: false,
        l: false,
      }
    );
    Ok(())
  }

  fn print_selection(
    &mut self,
    ed: EdState,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<(), &'static str> {
    self.prints_history.push(
      Print{
        text: ed.buffer.get_selection(selection)?
          .map(|(_, s)|s.to_string())
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
    _ed: EdState,
    _prefix: Option<char>,
  ) -> Result<String, &'static str> {
    panic!("get_command not implemented on aggregator ui")
  }

  fn get_input(
    &mut self,
    _ed: EdState,
    _terminator: char,
    #[cfg(feature = "initial_input_data")]
    _initial_buffer: Option<Vec<String>>
  ) -> Result<Vec<String>, &'static str> {
    panic!("get_input not implemented on aggregator ui")
  }

  fn lock_ui(&mut self) -> UILock<'_> {
    UILock::new(self)
  }
  fn unlock_ui(&mut self){}
}
