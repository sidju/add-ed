use super::*;

impl std::fmt::Display for EdError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    use EdError::*;
    match self {
      Internal(e) => write!(f,
        "Internal error! Save and quit, and please report at {}\n\n debug data: {:?}",
        "`https://github.com/sidju/add-ed/issues`",
        e,
      ),
      IO(e) => write!(f,
        "IO error: {}",
        e.inner,
      ),
      UI(e) => write!(f,
        "UI error: {}",
        e.inner,
      ),

      InfiniteRecursion => write!(f,
        "Execution recursion hit recursion limit, no changes made."
      ),

      IndexTooBig{index, buffer_len} => {
        if *buffer_len == 0 {
          write!(f,
            "Buffer is empty, so no valid selections or lines can exist.",
          )
        } else {
          write!(f,
            "Given index ({}) overshoots the length of the buffer ({}).",
            index,
            buffer_len,
          )
        }
      },
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
      UndoIndexNegative{relative_undo_limit} => write!(f,
        "Tried to undo to before undo history.\nHighest valid nr of undo steps is {}.",
        relative_undo_limit,
      ),
      UndoIndexTooBig{index, history_len, relative_redo_limit} => write!(f,
        "Tried to redo beyond existing snapshots.\nHigest valid nr of redo steps is {}.\n(Given index: {}, Highest valid index: {})",
        relative_redo_limit,
        index,
        history_len - 1,
      ),
      CommandEscapeForbidden(_path) => write!(f,
        "Command doesn't accept shell escapes. Use \\! if path begins with !.",
      ),
      TagInvalid(args) => write!(f,
        "Invalid tag given: {}. Tags should be a single character.",
        args,
      ),
      TagNoMatch(t) => write!(f,
        "Could not find any line matching the tag `{}`.",
        t,
      ),
      RegexInvalid{regex, error} => write!(f,
        "Regex `{}` invalid! {}.",
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
        "Couldn't read default file as it hasn't been set. Set by opening file or with the `f` command.",
      ),
      DefaultShellCommandUnset => write!(f,
        "Couldn't read default shell command as it hasn't been set. Run a fully defined shell command first.",
      ),
      DefaultSArgsUnset => write!(f,
        "Couldn't read default `s` arguments as they haven't been set. Run `s` with arguments first.",
      ),

      IndexSpecialAfterStart{prior_index, special_index} => write!(f,
        "Special index character `{}` found after index `{}`.",
        special_index,
        prior_index,
      ),
      IndexNotInt(text) => write!(f,
        "Failed to parse given index string `{}` as a number.",
        text,
      ),
      OffsetNotInt(text) => write!(f,
        "Failed to parse given index offset `{}` as a number.",
        text,
      ),
      IndicesUnrelated{prior_index, unrelated_index} => write!(f,
        "Received non-chainable index `{}` immediately after index `{}`.",
        unrelated_index,
        prior_index,
      ),
      IndexUnfinished(text) => write!(f,
        "Special index unfinished: `{}`.",
        text,
      ),

      CommandUndefined(cmd) => write!(f,
        "Unknown command `{}`.",
        cmd,
      ),
      ArgumentListEscapedEnd(arguments) => write!(f,
        "Argument list's end was escaped: `{}`.",
        arguments,
      ),
      ArgumentsWrongNr{expected, received} => write!(f,
        "Wrong nr of arguments given. Expected {}, received {}.",
        expected,
        received,
      ),
      ScrollNotInt(text) => write!(f,
        "Failed to parse nr of lines to scroll `{}` as a number.",
        text,
      ),
      UndoStepsNotInt(text) => write!(f,
        "Failed to parse nr of changes to undo/redo `{}` as a number.",
        text,
      ),
      ReflowNotInt{error: e, text: t} => write!(f,
        "Failed to parse nr of columns to reflow within `{}` as a number: {}",
        t,
        e,
      ),
      MacroUndefined(macro_name) => write!(f,
        "Given macro `{}` is not defined.",
        macro_name,
      ),

      FlagDuplicate(flag) => write!(f,
        "Flag `{}` was given more than once.",
        flag,
      ),
      FlagUndefined(flag) => write!(f,
        "Given flag `{}` isn't valid for that command.",
        flag,
      ),
    }
  }
}
