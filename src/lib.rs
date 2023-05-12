//! Add-Ed is a library implementing the parsing, IO and runtime for Ed in rust.
//!
//! Behaviour is initially based off of [GNU Ed] with modifications to improve
//! on usability. See the [readme] and [release notes] for details.
//!
//! This library exports two traits, [`IO`](io::IO) and [`UI`](ui::UI), which
//! define the exchangeable parts of the editor. If you enable the `local_io`
//! feature there is a ready implementation of the IO, but you will need to
//! bring your own UI if you wish to do any user interaction. If you don't wish
//! to do any user interaction, [`ScriptedUI`](ui::ScriptedUI) should be quite
//! easy to use.
//!
//! Minimal scripted usage example:
//! ```
//! use add_ed::{
//!   ui::ScriptedUI,
//!   io::LocalIO,
//!   Ed,
//! };
//!
//! # fn main() -> Result<(),&'static str> {
//! // Construct all the components
//! let mut ui = ScriptedUI{ input: vec!["e\n".to_string()].into(), print_ui: None, };
//! let mut io = LocalIO::new();
//! // Construct and run ed
//! let mut ed = Ed::new(&mut io, "Cargo.toml".to_string());
//! ed.run(&mut ui)?;
//! # Ok(()) }
//! ```
//!
//!
//! A full example of how to use this library is in src/bin/classic-ed.rs
//!
//! [GNU Ed]: https://www.gnu.org/software/ed/manual/ed_manual.html
//! [readme]: https://github.com/sidju/add-ed/blob/main/README.md
//! [release notes]: https://github.com/sidju/add-ed/blob/main/RELEASE_NOTES.md

use std::collections::HashMap;

pub mod error_consts;
mod cmd;

pub mod ui;
use ui::{UI, UILock};
pub mod io;
use io::IO;

pub mod buffer;
use buffer::Buffer;

/// A small reference struct that gives insight into the editor's state
///
/// Primarily built to hand out data needed by [`UI`] implementations. For other
/// uses consider direct interaction with the [`Ed`] instance.
pub struct EdState<'a> {
  pub selection: (usize, usize),
  pub buffer: &'a Buffer,
  pub file: &'a str,
}

/// A ready parsed 's' invocation, including command and printing flags
pub struct Substitution {
  pub pattern: String,
  pub substitute: String,
  pub global: bool,
  pub p: bool,
  pub n: bool,
  pub l: bool,
}

/// The state variable used to track the editor's internal state.
///
/// It is designed to support mutation and analysis by library users, but be
/// careful: modifying this state wrong will cause user facing errors.
pub struct Ed <'a, I: IO> {
  /// The buffer holds Ed's text data.
  ///
  /// It also abstracts basically all Ed editing operations, though it is
  /// often recommended to use Ed::run_command instead so state is updated
  /// correctly.
  pub buffer: Buffer,
  /// Tracks the currently selected lines in the buffer.
  ///
  /// Inclusive 1-indexed start and end bounds over selected lines. Selected
  /// lines aren't required to exist, but it is recommended for user comfort.
  /// Empty selection should only occur when the buffer is empty, and in that
  /// case exactly (1,0). Invalid selections cause errors, not crashes.
  pub selection: (usize, usize),
  /// Currently used IO implementor
  ///
  /// It will be used to handle file interactions and command execution as
  /// required during command execution
  pub io: &'a mut I,
  /// The path to the currently selected file.
  pub file: String,

  /// Shell command last given by the user
  ///
  /// Fully processed, meaning all substitutions of % to current file and 
  /// similar should already have occured before saving here.
  /// (Currently saved before successful run, so may be invalid).
  pub prev_shell_command: String,
  /// The previous `s` commands arguments, to support repeating last `s` command
  /// when no arguments are given to `s`.
  pub prev_s: Option<Substitution>,

  /// Configuration of prefix before command input.
  ///
  /// Traditionally ':' so set to that by default.
  pub cmd_prefix: Option<char>,
  /// Set default to print numbered lines.
  ///
  /// If set `n` printing flag behaviour inverts and disables line numbers.
  pub n: bool,
  /// Set default to print literal lines. (A type of escaped print.)
  ///
  /// If set `l` printing flag behaviour inverts and disables literal printing.
  pub l: bool,
  /// Wether or not to print errors when they occur.
  ///
  /// If not true Ed prints ? on error, expecting use of `h` command to get
  /// the error.
  pub print_errors: bool,
  /// The previous error that occured.
  ///
  /// Is printed by `h` command.
  pub error: Option<&'static str>,
  /// Configuration field for macros.
  ///
  /// A map from macro name to string of newline separated commands.
  pub macros: HashMap<String, String>,
}

impl <'a, I: IO> Ed <'a, I> {
  /// Construct a new instance of Ed
  ///
  /// * An empty file string is recommended if no filepath is opened
  pub fn new(
    io: &'a mut I,
    file: String,
  ) -> Self {
    let selection = (1,0);
    Self {
      // Init internal state
      selection,
      buffer: Buffer::new(),
      prev_s: None,
      prev_shell_command: String::new(),
      // Sane defaults for externally visible variables
      error: None,
      print_errors: true,
      n: false,
      l: false,
      cmd_prefix: Some(':'),
      macros: HashMap::new(),
      // And the given values
      io,
      file,
    }
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

  /// Get a single command from the UI and run it
  ///
  /// Returns true if the command was to quit, false otherwise.
  /// Returns error if any occurs
  pub fn get_and_run_command(
    &mut self,
    ui: &mut dyn UI,
  ) -> Result<bool, &'static str> {
    // Define a temporary closure, since try blocks aren't stabilized
    let mut clos = || {
      let cmd = ui.get_command( self.see_state(), self.cmd_prefix )?;
      self.run_command(ui, &cmd)
    };
    // Run it, save any error, and forward result
    match clos() {
      Err(e) => {
        self.error = Some(e);
        Err(e)
      },
      x => x,
    }
  }

  /// Run given instance of Ed until it receives a command to quit or errors
  ///
  /// Returns Ok(()) when quit by command (or end of macro input)
  pub fn run_macro(
    &mut self,
    ui: &mut dyn UI,
  ) -> Result<(), &'static str> {
    // Loop over it, handling errors, until quit received
    loop {
      if self.get_and_run_command(ui)? { break; }
    }
    Ok(())
  }

  /// Run until quit by command
  ///
  /// Prints ? or error message as errors occur (depending on `print_errors`).
  /// Returns error if error occurs when printing.
  pub fn run(
    &mut self,
    ui: &mut dyn UI,
  ) -> Result<(), &'static str> {
    // Loop getting and running command, handling errors, until quit received
    loop {
      match self.get_and_run_command(ui) {
        Ok(true) => break,
        Ok(false) => (),
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

  /// Get an immutable reference to some internal parts of the editors state
  ///
  /// Creates the data struct used by [`UI`] implementations. For other uses
  /// consider direct access to the [`Ed`] instance.
  pub fn see_state(&self) -> EdState {
    EdState{
      selection: self.selection,
      file: &self.file,
      buffer: &self.buffer,
    }
  }
}
