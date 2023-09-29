//#![deny(missing_docs)]

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
//!   EdError,
//! };
//!
//! # fn main() -> Result<(), EdError> {
//! // Construct all the components
//! let mut ui = ScriptedUI{
//!   input: vec![format!("e {}\n", "Cargo.toml")].into(),
//!   print_ui: None,
//! };
//! let mut io = LocalIO::new();
//! // Construct and run ed
//! let mut ed = Ed::new(&mut io);
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

pub mod messages;

#[macro_use]
pub mod error;
pub use error::{
  Result,
  EdError,
};
use error::InternalError;

mod cmd;

pub mod ui;
use ui::{UI, UILock};
pub mod io;
use io::IO;

mod history;
pub use history::History;

pub use buffer::iters::*;
mod buffer;
pub use buffer::{
  LineText,
  Line,
  Buffer,
  PubLine,
  Clipboard,
};

/// A ready parsed 's' invocation, including command and printing flags
pub struct Substitution {
  /// Regex pattern to match against
  pub pattern: String,
  /// Substitution template to replace it with
  pub substitute: String,
  /// Set true to apply to all occurences (instead of only the first)
  pub global: bool,
  /// Flag to print after execution
  pub p: bool,
  /// Flag to print with line numbers after execution
  pub n: bool,
  /// Flag to print with literal escapes after execution
  pub l: bool,
}

/// The state variable used to track the editor's internal state.
///
/// It is designed to support mutation and analysis by library users, but be
/// careful: modifying this state wrong will cause user facing errors.
pub struct Ed <'a> {
  /// Holds the past, present and sometimes future states of the editing buffer
  ///
  /// See [`History`] documentation for how to use.
  pub history: History<Buffer>,
  /// The current clipboard contents
  ///
  /// Uses a special [`Buffer`] analogue over [`PubLine`], since some of the
  /// internal data in Line could cause unexpected behavior if pasted as is.
  pub clipboard: Clipboard,
  /// Tracks the currently selected lines in the buffer.
  ///
  /// Inclusive 1-indexed start and end bounds over selected lines. Selected
  /// lines aren't required to exist, but it is recommended for user comfort.
  /// Empty selection should only occur when the buffer is empty, and in that
  /// case exactly (1,0). Invalid selections cause errors, not crashes (verified
  /// by fuzzing).
  pub selection: (usize, usize),
  /// Currently used IO implementor
  ///
  /// It will be used to handle file interactions and command execution as
  /// required during command execution
  pub io: &'a mut dyn IO,
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
  ///
  /// UI errors occuring outside of Ed should also be written to this variable,
  /// so `h` prints the latest error that occured in the whole application.
  pub error: Option<EdError>,
  /// EXPERIMENTAL: Configuration field for macros.
  ///
  /// A map from macro name to string of newline separated commands. A change of
  /// format from a string of newline separated commands to a wrapping struct is
  /// planned.
  pub macros: HashMap<String, String>,
  /// Set how many recursions should be allowed.
  ///
  /// One recursion is counted as one macro or 'g'/'v'/'G'/'V' invocation. Under
  /// 2 is likely to interfere with basic use, 4 will require that macros don't
  /// call into eachother, 16 is unlikely to abort needlessly.
  pub recursion_limit: usize,
}

impl <'a, > Ed <'a> {
  /// Construct a new instance of Ed
  ///
  /// Defaults are as follow:
  /// - `file`: empty string
  /// - `clipboard`: empty clipboard
  /// - `error`: `None`
  /// - `print_errors`: `true`
  /// - `n`: `false`,
  /// - `l`: `false`,
  /// - `cmd_prefix`: `Some(':')`
  /// - `macros`: empty hashmap
  /// - `recursion_limit`: `16`
  pub fn new(
    io: &'a mut dyn IO,
  ) -> Self {
    let selection = (1,0);
    Self {
      // Init internal state
      selection,
      history: History::new(),
      prev_s: None,
      prev_shell_command: String::new(),
      // Sane defaults for externally visible variables
      file: String::new(),
      clipboard: Clipboard::new(),
      error: None,
      print_errors: true,
      n: false,
      l: false,
      cmd_prefix: Some(':'),
      macros: HashMap::new(),
      recursion_limit: 16,
      // And the given values
      io,
    }
  }

  /// Run the given command
  ///
  /// Returns true if the command was to quit
  pub fn run_command(
    &mut self,
    ui: &mut dyn UI,
    command: &str,
  ) -> Result<bool> {
    self.private_run_command(ui, command, 0)
  }
  // Exists to handle nesting depth, for nested 'g' invocations, without
  // exposing that argument to the public interface (since it will always be 0
  // when called from the public API).
  fn private_run_command(
    &mut self,
    ui: &mut dyn UI,
    command: &str,
    recursion_depth: usize,
  ) -> Result<bool> {
    // Just hand execution into the cmd module
    match cmd::run(self, ui, command, recursion_depth) {
      // If error, note it in state
      Err(e) => {
        self.error = Some(e.clone());
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
  ) -> Result<bool> {
    self.private_get_and_run_command(ui, 0)
  }
  // Exists to handle nesting depth, for nested 'g' invocations, without
  // exposing that argument to the public interface (since it will always be 0
  // when called from the public API).
  fn private_get_and_run_command(
    &mut self,
    ui: &mut dyn UI,
    recursion_depth: usize,
  ) -> Result<bool> {
    // Define a temporary closure to catch UI errors, needed since try blocks
    // aren't stabilized
    let mut clos = || {
      let cmd = ui.get_command(self, self.cmd_prefix)?;
      self.private_run_command(ui, &cmd, recursion_depth)
    };
    // Run it, save any error, and forward result
    match clos() {
      Err(e) => {
        self.error = Some(e.clone());
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
  ) -> Result<()> {
    self.private_run_macro(ui, 0)
  }
  // Exists to handle nesting depth, for nested 'g' invocations, without
  // exposing that argument to the public interface (since it will always be 0
  // when called from the public API).
  fn private_run_macro(
    &mut self,
    ui: &mut dyn UI,
    recursion_depth: usize,
  ) -> Result<()> {
    // Loop over it, handling errors, until quit received
    loop {
      if self.private_get_and_run_command(ui, recursion_depth)? { break; }
    }
    Ok(())
  }

  /// Run until quit by command
  ///
  /// Prints ? or error message as errors occur (depending on `print_errors`).
  /// Returns error only if error occurs when printing an error.
  pub fn run(
    &mut self,
    ui: &mut dyn UI,
  ) -> Result<()> {
    // Loop getting and running command, handling errors, until quit received
    loop {
      match self.get_and_run_command(ui) {
        Ok(true) => break,
        Ok(false) => (),
        Err(e) => {
          if self.print_errors {
            ui.print_message(&e.to_string())?;
          }
          else {
            ui.print_message("?\n")?;
          }
        },
      }
    }
    Ok(())
  }
}
