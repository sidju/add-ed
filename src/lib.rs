//! Add-Ed is a library implementing the parsing and runtime for Ed in rust.
//!
//! It exports two traits, Buffer and UI, which define the exchangeable parts of the editor.
//!
//! An implementation of the UI trait is needed to support the 'g' command and similar, DummyUI.
//! It is used for macro execution, by taking prepared input from a input list rather than prompting the user.
//!
//! Since the buffer is rather complex a standard Buffer implementation is included in the feature "vecbuffer".
//! It is recommended to compare the behaviour of any Buffer implementation to the VecBuffer in addition to the api tests.
//!
//! An example of how to use this library is in src/bin/classic.rs

use std::collections::HashMap;

pub mod error_consts;
mod cmd;

pub mod ui;
pub mod buffer;
pub mod io;

use ui::{UI, UILock};
use buffer::Buffer;
use io::IO;

/// A small reference struct that gives insight into the editor's state
pub struct EdState<'a> {
  pub selection: (usize, usize),
  pub buffer: &'a Buffer,
  pub file: &'a str,
}

/// A ready parsed 's' invocation
struct Substitution {
  pattern: String,
  substitute: String,
  global: bool,
  p: bool,
  n: bool,
  l: bool,
}

/// The state variable used by the editor to track its internal state
pub struct Ed <'a, I: IO> {
  // Track the currently selected lines in the buffer
  // This is usually separate from viewed lines in the UI
  selection: (usize, usize),
  // A mutable reference to a Buffer implementor
  // The buffer implementor will handle most of the operations and store the data
  buffer: &'a mut Buffer,
  // The path to the currently selected file
  file: String,
  // The fully processed command last given by the user
  // (Saved before successful run, so may be invalid)
  prev_shell_command: String,
  // A mutable reference to an IO implementor
  // It will handle file interactions and command execution
  io: &'a mut I,
  // The previous search_replace's arguments, to support repeating the last
  prev_s: Option<Substitution>,
  // Flag to prevent auto-creating undo-points when running macros or the like
  dont_snapshot: bool,

  // Prefix for command input. Traditionally ':' so that by default
  cmd_prefix: Option<char>,
  // Default states for printing flags
  // Allows to print numbered or literal by default
  n: bool,
  l: bool,
  // Map of macro name to macro script
  macros: HashMap<String, String>,
  // Wether or not to print errors when they occur (if not, print ? instead of error)
  print_errors: bool,
  // The previous error that occured, since we may not have printed it
  error: Option<&'static str>,
}

impl <'a, I: IO> Ed <'a, I> {
  /// Construct a new instance of Ed
  ///
  /// * An empty file string is recommended if no filepath is opened
  /// * Note that you can initialise the buffer with contents before this.
  /// * macros behave like scripts given to the 'g' command
  ///   an example is "a\n\n.\n" which appends an empty line
  pub fn new(
    buffer: &'a mut Buffer,
    io: &'a mut I,
    file: String,
    macros: HashMap<String, String>,
    n: bool,
    l: bool,
  ) -> Result<Self, &'static str> {
    let selection = (1,0); // Empty, but that is handled in cmd module
    let tmp = Self {
      // Sane defaults for initial settings
      print_errors: true,
      error: None,
      prev_s: None,
      cmd_prefix: Some(':'),
      selection,
      dont_snapshot: false,
      prev_shell_command: String::new(),
      // And the given values
      buffer,
      io,
      file,
      n,
      l,
      macros,
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
        Err(e) => {
          if self.print_errors {
            ui.print_message(e)?;
          }
          else {
            ui.print_message("?\n")?;
          }
        },
      }
    }
    Ok(())
  }

  /// Get an immutable reference to part of the editors state
  pub fn see_state(&self) -> EdState {
    EdState{
      selection: self.selection,
      file: &self.file,
      buffer: self.buffer,
    }
  }
}
