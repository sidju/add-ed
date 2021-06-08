//! Add-Ed is a library implementing the parsing and runtime for Ed in rust.
//!
//! It exports two traits, Buffer and UI, which define the exchangeable parts of the editor.
//!
//! An implementation of the UI trait is needed to support the 'g' command and similar, DummyUI.
//! It is used for macro execution, by taking prepared input from a input list rather than prompting the user.
//!
//! Since the buffer is rather complex a standard Buffer implementation can be build in with the feature "vecbuffer".
//! It is recommended to compare the behaviour of any Buffer implementation to the VecBuffer until Buffer tests are set up.
//!
//! An example of how to use this library is in src/bin/classic.rs

pub mod error_consts;
mod cmd;

pub mod ui;
pub mod buffer;

use ui::UI;
use buffer::Buffer;

/// A small reference struct that gives insight into the editor's state
pub struct EdState<'a> {
  pub selection: &'a Option<(usize, usize)>,
  pub buffer: &'a dyn Buffer,
  pub path: &'a str,
}

/// The state variable used by the editor to track its internal state
pub struct Ed <'a, B: Buffer> {
  // Track the currently selected lines in the buffer
  // This is usually separate from viewed lines in the UI
  selection: Option<(usize, usize)>,
  // A mutable reference to a Buffer implementor
  // The buffer implementor will handle most of the operations and store the data
  buffer: &'a mut B,

  // The path to the currently selected file
  path: String,

  // The previous search_replace's arguments, to support repeating the last
  s_args: Option<(String, String, bool)>,

  // Prefix for command input. Traditionally : but none by default
  cmd_prefix: Option<char>,

  // Wether or not to print errors when they occur (if not, print ? instead of error)
  print_errors: bool,
  // The previous error that occured, since we may not have printed it
  error: Option<&'static str>,
}

impl <'a, B: Buffer> Ed <'a, B> {
  /// Construct a new instance of Ed
  ///
  /// * An empty file string is recommended if no filepath is opened
  /// * Note that you can initialise the buffer with contents before this
  pub fn new(
    buffer: &'a mut B,
    path: String,
  ) -> Result<Self, &'static str> {
    let len = path.len();
    if len != 0 {
      buffer.read_from(&path, None, false)?;
    }
    let tmp = Self {
      // Sane defaults for initial settings
      print_errors: true,
      error: None,
      s_args: None,
      cmd_prefix: None,
      // Trying to set a reasonable default tends to cause trouble
      selection: None,
      // And the given values
      buffer: buffer,
      path: path,
    };
    Ok(tmp)
  }

  /// Run the given command
  ///
  /// Returns true if the command was to quit
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
  ///
  /// The returned error type could be improved, suggestions welcome.
  pub fn run_macro(
    &mut self,
    ui: &mut dyn UI,
  ) -> Result<(), &'static str> {
    // Loop until quit or error
    loop {
      let cmd = match ui.get_command( self.see_state(), self.cmd_prefix ) {
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
            ui.print(self.see_state(), self.error.unwrap())?;
          }
          else {
            ui.print(self.see_state(), "?\n")?;
          }
        },
      }
    }
    Ok(())
  }

  /// Get an immutable reference to part of the editors state
  pub fn see_state(&self) -> EdState {
    EdState{
      selection: &self.selection,
      path: &self.path,
      buffer: self.buffer,
    }
  }
}
