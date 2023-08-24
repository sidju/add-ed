use super::*;

impl std::cmp::PartialEq for EdError {
  fn eq(&self, other: &Self) -> bool {
    use EdError::*;
    match (self, other) {
      (Internal(x),Internal(y)) => x == y,
      (IO(_),IO(_)) => true,
      (UI(_),UI(_)) => true,

      (
        IndexTooBig{index: a, buffer_len: b},
        IndexTooBig{index: c, buffer_len: d},
      ) => {
        a == c && b == d
      },
      (Line0Invalid,Line0Invalid) => true,
      (SelectionEmpty((a,b)),SelectionEmpty((c,d))) => a == c && b == d,
      (SelectionForbidden,SelectionForbidden) => true,

      (UnsavedChanges,UnsavedChanges) => true,
      (NoOp,NoOp) => true,
      (
        UndoStepsInvalid{undo_steps: a, undo_range: b},
        UndoStepsInvalid{undo_steps: c, undo_range: d},
      ) => {
        a == c && b == d
      },
      (DefaultFileInvalid(x),DefaultFileInvalid(y)) => x == y,
      (TagInvalid(x),TagInvalid(y)) => x == y,
      (TagNoMatch(x),TagNoMatch(y)) => x == y,
      (
        RegexInvalid{regex: a, error: b},
        RegexInvalid{regex: c, error: d},
      ) => {
        a == c && b == d
      },
      (RegexNoMatch(x),RegexNoMatch(y)) => x == y,
      (PrintAfterWipe,PrintAfterWipe) => true,

      (DefaultFileUnset,DefaultFileUnset) => true,
      (DefaultShellCommandUnset,DefaultShellCommandUnset) => true,
      (DefaultSArgsUnset,DefaultSArgsUnset) => true,

      (
        IndexSpecialAfterStart{prior_index: a, special_index: b},
        IndexSpecialAfterStart{prior_index: c, special_index: d},
      ) => {
        a == c && b == d
      },
      (IndexNotInt(x),IndexNotInt(y)) => x == y,
      (OffsetNotInt(x),OffsetNotInt(y)) => x == y,
      (
        IndicesUnrelated{prior_index: a, unrelated_index: b},
        IndicesUnrelated{prior_index: c, unrelated_index: d},
      ) => {
        a == c && b == d
      },
      (IndexUnfinished(x),IndexUnfinished(y)) => x == y,

      (CommandUndefined(x),CommandUndefined(y)) => x == y,
      (ArgumentListEscapedEnd(x),ArgumentListEscapedEnd(y)) => x == y,
      (
        ArgumentsWrongNr{expected: a, received: b},
        ArgumentsWrongNr{expected: c, received: d},
      ) => {
        a == c && b == d
      },
      (ScrollNotInt(x),ScrollNotInt(y)) => x == y,
      (UndoStepsNotInt(x),UndoStepsNotInt(y)) => x == y,
      (ReflowNotInt{error: a, text: b},ReflowNotInt{error: c, text: d}) => {
        a == c && b == d
      },
      (MacroUndefined(x),MacroUndefined(y)) => x == y,

      (FlagDuplicate(x),FlagDuplicate(y)) => x == y,
      (FlagUndefined(x),FlagUndefined(y)) => x == y,

      _ => false,
    }
  }
}
