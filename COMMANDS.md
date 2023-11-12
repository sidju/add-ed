# Command notation

Short representations to keep this documentation concise.

(Note that the descriptions of what a shorthand should be replaced by will use
the previously presented shorthands.)

- `(x)` An optional `x`.
- `x|y` `x` or `y`, and not both.
- `[abc]` Any combination of `a`, `b` or `c`, but no duplicates.
- `.` An index. Can be any of
  - `.` Interpreted as the start of the currently viewed selection in most cases
    but as the end of the currently viewed selection when given to an appending
    command (or as end of a selection).
  - `<positive integer>` Interpreted as index of a line.
  - `$` Interpreted as index of the last line, or 0 there are no lines.
  - `'<char>` Interpreted as index of first line tagged with
    the given character.
  - `/<pattern>/` Interpreted as index of nearest following
    line matching the given regex pattern.
  - `?<pattern>?` same as above but nearest preceeding.
  - `(<any index>)+(<positive integer>`) Interpreted as the other index plus
    the positive integer. If no other index given treated as `.`. If no integer
    given treated as `1`.
  - `(<any index>)-(<positive integer>)` Same as above but minus.
  - `<nothing>` Is generally equivalent to `.` if an index is accepted.
    (Exceptions exist, as noted by the commands below.)
- `.,.` A selection. Can be any of
  - `.` Any lone index, which selects only that line. Errors if no line exists
    at the index.
  - `.,.` Two indices separated by a comma, which selects both those lines and
    all lines between. Errors if any of the selected lines doesn't exist.
    (Empty indices are interpreted as index `1` and `$` respectively.)
  - `.;.` Two indices separated by a semicolon, which selects both those lines
    and all lines between them. Erors if any of the selected lines doesn't
    exist.
    (Empty indices are interpreted as index `1` and `.` respectively.)
  - `<nothing>` Interpreted as the currently viewed selection. Use the `=`
    command to print the currently viewed selection.
- `/` A separator. Can be any character (except newline), but for each command
  invocation you must use the same separator. Traditionally `/` or `_`.


# Printing flags:

All the printing commands and most other commands accept *printing flags*. These
are `[pnl]`.
- `p` prints the selection after the command.
- `n` prints the selection after the command with line numbers (or without, if
  the `N` default is on).
- `l` prints the selection after the command with `$` before newlines, `--->`
  instead of tabs and `$$` instead of `$`. (Or not, if the `L` default is on).


# Printing commands

Commands to print buffer contents.

- `<nothing>` Prints as many lines after the currently selected as you have
  selected. (Intended so you can print the first 20 lines and press enter to do
  so again.)
- `(.,.)[pnl]` Print given selection.
  (`p` is used to distinct the invocation from `<nothing>` when not giving an
  explicit selection, it doesn't affect the printing.)
- `(.,.)z(<positive integer>)[pnl]` Prints the given number of lines following
  the given selection with the given printing configuration.


# Basic editing commands

Simple commands to edit the text in the editing buffer.

- `(.)a[pnl]` Append text after given line. Enters input mode terminated by '.'.
  After running the inserted text is selected.
- `(.)i[pnl]` Insert text before given line. Otherwise same behaviour as `a`.
- `(.,.)d[pnl]` Cut the selected lines into (editor internal) clipboard. Selects
  the nearest following line if any, otherwise the nearest preceeding. If
  deleting all of the buffer there is no selection after running, wherefore
  doing so with print flags will error.
- `(.,.)y[pnl]` Copy the selected lines into (editor internal) clipboard.
  Selects the given selection.
- `(.)x[pnl]` Paste the contents of the (editor internal) clipboard after given
  index. Selects the pasted lines.
- `(.)X[pnl]` Same as `x` except pastes before the given index.
- `(.,.)j[pnl]` Joins the selected lines into a single line (simply removes the
  newline characters, everything else is kept). Selects the resulting line.


# Combined editing commands

Commands that kind of combine two basic editing commands.

- `(.,.)c[pnl]` Change out the selected lines. Enters input mode terminated by
  '.'. Equivalent to `.,.d` followed by `i`. Selects the inserted text if any
  given, if none given behaves like `d`.
- `(.,.)C[pnl]` *ONLY IF `initial_input_data` FEATURE IS ENABLED!!!*
  Behaves just like `c` except the selected lines are put into the input field,
  allowing you to edit them directly.
- `(.,.)m(.)[pnl]` Move selected lines directly to given index. If no index
  given it moves to the end of the buffer by default. Kind of equivalent to
  `.,.d` followed by `x.`, except it doesn't affect the (editor internal)
  clipboard. Selects the moved lines in their new location.
- `(.,.)t(.)[pnl]` Copy selected lines directly to given index. If no index
  given it copies to the end of the buffer by default. Kind of equivalent to
  `.,.y` followed by `x.`, except it doesn't affect the (editor internal)
  clipboard. Selects the copied lines in their new location.


# File and shell commands

Commands to read and write to the surrounding system, both directly to/from
files and using shell commands.

Note that most of these commands accept a shell command (prefixed by `!`). If
such is given it will be run as a child process and read from (stdout)/written
to (stdin) in place of the file path otherwise accepted. The last shell command
run is saved (no matter its success or failure) and can be re-run if no command
is given. (For commands taking a path or a command you still need to give a `!`
to indicate to run a command.)

- `e(<path>|!<shell command>)` Replace buffer contents with data read from
  given path/command. If no path/command given uses the default path. Sets the
  default path to given path if path given, leaves default path unchanged
  otherwise. Selects all lines in the buffer after reading in.
  If the buffer contains unsaved edits aborts with error, capitalize `e` to `E`
  to override the warning.
- `(.)r(<path>|!<shell command>)` Read in data from give path/command and
  insert it after the given index. If no index given defaults to inserting after
  current selection. If no path or shell command given uses default path.
  Selects the added lines after running.
- `wq` Save the whole buffer to the default path and quit the editor. Errors if
  it is given a selection other that the whole buffer.
- `(.,.)w(<path>|!<shell command>)` Write selected lines to given
  path/command. If no selection given writes the whole buffer. If no path given
  writes to default path. If selection was explicitly given selects that,
  otherwise leaves selection unchanged. If selection was not given and a path
  was given that path is set as default path.
- `(.,.)W(<path>)` Append the selected lines to the given path. If no path
  given appends to default path. If no selection given appends whole buffer.
  Selects the appended lines after running.
- `(.,.)|(<shell command>)` Transform selection via given shell command. If no
  selection is given the default selection is used. The original line data is
  placed in the clipboard and the new lines are selected.
- `!(<shell command>)` Run the given shell command interactively.


# Batch editing commands

More advanced commands to apply the same or similar changes many times.

- `(.,.)s(/<regex>/<substitution>/[gpnl])` Replaces text within selection that
  matches the regex with the substitution. If the `g` flag is given replaces all
  occurences of the regex, if not only the first is replaced. Selects the
  selection, whatever size it ends up being after replacing.
- `(.,.)g/<regex>/<command>(/)` Runs commands on all lines matching the regex.
  If the last separator is given the commands are run immediately, if not it
  enters input mode terminated by the separator. The matching line is selected
  (using default selection, the commands will run them on the matched line) and
  run in the order given. Doesn't set selection, but the commands run through
  it do.
- `(.,.)v/<regex>/<command>(/)` Inverse of `g`. Runs given commands on lines
  that **don't** match the given regex.
- `(.,.)G/<regex>/` Interactive version of `g`. For each matching line prints it
   and enters input mode terminated by the separator. The given commands are run
   on that line, same as `g`.
- `(.,.)V/<regex>/` Inverse of `G`. Does the same for lines that don't match the
   given regex.
- `(.,.):<macro-name>(<space separated arguments>)` Set selection to given
  selection (if any) and run given macro. Same as `g` it doesn't set selection,
  but the commands in the macro will probably do so.

  If the macro has specified number of arguments any other number will cause an
  error. If and only if the allowed number of arguments is specified to none the
  substitution routine won't be run, which means that '$' characters don't need
  to be escaped (by adding another '$').


# Status commands

For printing information about and changing editor state.

- `help` Print a short list of commands.
- `Help` Print this documentation.
- `q` Quits the editor. If the buffer contains unsaved edits aborts with error.
  Capitalize 'q' to 'Q' to override and quit anyways.
- `h` Print last previous error.
- `H` Toggle between printing the error or only `?` when an error occurs.
- `(.,.)=` Prints selection. If none given prints the current selection.
- `(.,.)#(<anything>)` If no selection is given it does nothing, to enable
  inlining comments in scripts. If a selection is given that selection is set
  without printing (this is the only way to do this, as even no command prints).
- `f(<path>)` If no path given prints the default path, otherwise sets the given
  path as default path.
