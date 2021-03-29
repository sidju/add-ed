//! The base of the editor module
//! This will declare the required traits and export functions for
//! running this editor as a library

pub mod error_consts;
pub mod cmd;

pub mod ui;
pub mod buffer;

use ui::UI;
use buffer::Buffer;

/// The state variable used by the editor to track its internal state
pub struct Ed <'a, B: Buffer> {
  /// Track the currently selected lines in the buffer
  /// This is usually separate from viewed lines in the UI
  selection: Option<(usize, usize)>,
  /// A mutable reference to a Buffer implementor
  /// The buffer implementor will handle most of the operations and store the data
  buffer: &'a mut B,

  /// The path to the currently selected file
  path: String,

  /// Wether or not to print errors when they occur (if not, print ? instead of error)
  print_errors: bool,
  /// The previous error that occured, since we may not have printed it
  error: Option<&'static str>,
}

impl <'a, B: Buffer> Ed <'a, B> {
  /// Construct a new instance of Ed
  /// An empty file string is recommended if no filepath is opened
  /// Note that you can initialise the buffer with contents before this
  pub fn new(
    buffer: &'a mut B,
    path: String,
  ) -> Result<Self, &'static str> {
    let len = path.len();
    let tmp = Self {
      // Sane defaults for initial settings
      print_errors: true,
      error: None,
      selection: None,
      // And the given values
      buffer: buffer,
      path: path,
    };
    if len != 0 {
      tmp.buffer.read_from(&tmp.path, None, false)?;
    }
    Ok(tmp)
  }

  /// Run the given command
  /// Returns true if the command was to quit
  /// The error is inherited from UI
  pub fn run_command(
    &mut self,
    ui: &mut dyn UI,
    command: &str,
  ) -> Result<bool, &'static str> {
    // Just hand execution into the cmd module
    match cmd::run(self, ui, command) {
      // If error, note it in state
      Err(e) => {
        self.error = Some(e);
        Err(e)
      },
      x => x,
    }
  }

  /// Run given instance of Ed until it receives a command to quit or errors
  /// The returned error type could be improved, suggestions welcome.
  pub fn run_macro(
    &mut self,
    ui: &mut dyn UI,
  ) -> Result<(), &'static str> {
    // Loop until quit or error
    loop {
      let cmd = match ui.get_command( self.buffer ) {
        Err(e) => { self.error = Some(e); return Err(e) },
        Ok(x) => x,
      };
      if self.run_command(ui, &cmd)? {
        break;
      }
    }
    Ok(())
  }
  pub fn run(
    &mut self,
    ui: &mut dyn UI,
  ) -> Result<(), &'static str> {
    loop {
      match self.run_macro(ui) {
        Ok(()) => break,
        Err(_) => {
          if self.print_errors {
            ui.print(self.error.unwrap())?;
          }
          else {
            ui.print("?\n")?;
          }
        },
      }
    }
    Ok(())
  }
}
