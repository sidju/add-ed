use super::*;

impl std::fmt::Display for EdError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use EdError::*;
    match self {
      Internal(_) => write!(f,
        "Internal error! Save and quit, and please report at `https://github.com/sidju/add-ed/issues`"
      ),
      IO(e) => write!(f,
        "IO error: {}",
        e,
      ),
      UI(e) => write!(f,
        "UI error: {}",
        e,
      ),

      IndexTooBig{index, buffer_len} => write!(f, 
        "Given index ({}) overshoots the length of the buffer ({}).",
        index,
        buffer_len,
      ),
      Line0Invalid => write!(f,
        "Index 0 was given when a line was needed, there is no line at index 0."
      ),
      SelectionEmpty(s) => write!(f,
        "Selection given ({},{}) is empty or inverted.",
        s.0, s.1,
      ),
      SelectionForbidden => write!(f,
        "That command doesn't allow any selection."
      ),

      UnsavedChanges => write!(f,
        "Unsaved changes! Capitalise command to ignore.",
      ),
      NoOp => write!(f,
        "That combination of command and arguments doesn't do anything.",
      ),
      UndoStepsInvalid{undo_steps, undo_range} => write!(f,
        "Undo(/Redo) exceeds history at {} steps. Valid undo span is from {} to {}.",
        undo_steps,
        undo_range.start, undo_range.end,
      ),
      DefaultFileInvalid(_path) => write!(f,
        "Cannot set default file to a shell command.",
      ),
      TagInvalid(args) => write!(f,
        "Invalid tag given: {}. Tags should be a single character.",
        args,
      ),
      TagNoMatch(t) => write!(f,
        "Could not find any line matching the tag `{}`",
        t,
      ),
      RegexInvalid{regex, error} => write!(f,
        "Regex `{}` invalid! {}",
        regex,
        error,
      ),
      RegexNoMatch(regex) => write!(f,
        "No matches found for regex `{}`.",
        regex,
      ),
      PrintAfterWipe => write!(f,
        "Would print after deleting whole buffer, refusing to run command.",
      ),

      DefaultFileUnset => write!(f,
        "Couldn't read default file as it hasn't been set. Set by opening file or with the `f` command",
      ),
      DefaultShellCommandUnset => write!(f,
        "Couldn't read default shell command as it hasn't been set. Run a fully defined shell command first.",
      ),
      DefaultSArgsUnset => write!(f,
        "Couldn't read default `s` arguments as they haven't been set. Run `s` with arguments first.",
      ),

      _ => write!(f, "todo"),
    }
  }
}
