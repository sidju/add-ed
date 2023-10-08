# 0.11.0
Nearing a stable release now, so some less clean features are remade in this
release to be easier to maintain in a stable manner forward.

- New command documentation.
- Update the README.md and add more in-repo documentation.
- Add proper macro support under ':', with support for arguments and tests to
  verify its behaviour. This also changed the macro methods on `Ed` struct.
- Make '!' forbid selection, instead require using '|' for filtering text
  through a shell command.
- Cut down on less meaningful variant commands to make it easier to add more
  diverse features without breaking the command language in the future.
  - 'J' has been removed, as it is functionally equivalent or inferior to
    `<selection>!fmt -w 80`.
  - 'L' and 'N' have been combined into a 'P' command, which uses the flags
    themselves to toggle their defaults in a single command. This allows adding
    more flags with toggle-able defaults in the future without needing more
    commands.
  - 'M' and 'T' removed. As you already need to give an index to move/copy to
    you might as well just give that index - 1 to prepend.
- Started testing with insight into undo history, so undo snapshot creation is
  now testable and verified.
- Made some exported structs impossible to create aside from constructors, to
  enable maintaining compability by just keeping the same constructors as the
  member variables change.
- Limit nesting depth when running commands than run commands. (Mainly to
  prevent macros from doing infinite recursion.)

# 0.10.0
- Add a first sketch of undo/redo. Feedback on behaviour is requested in the
  "Undo behaviour" issue (#3) on github
- Rework basically all tests, resulting in many small fixes
  (Old tests kept but not active, a whole new suite written)
- Add IO abstraction for shell and file interactions
  (caused small change in  UI trait)
- Add support for shell escapes to 'r', 'e' and 'w'
- Add '!' command, with expanded functionality compared to ed
  (If given selection it pipes it through the command, replacing the selection)
- Renamed DummyUI to ScriptedUI
- Make more of the Ed state public instead of configured at construction
  (printing flags, cmd_prefix and macros)
- Added back the ability for '#' to take and set a selection, to give a way to
  set selection without any output (no command prints the given selection)

# 0.9.0
- Improve substitute, especially escapes.
- Make ':' default prefix for command input (if handled by UI).
- Classic ui now ignores prefix, since it doesn't handle it correctly.
- Change some selection bugs/unexpected behaviours.
  - 'd' now leaves selection at line preceding deleted lines instead of the 
    following line.
  - 'c' and 'C' left selection as if they had not received input. Now return
    selection over given input if any and over preceding line otherwise.
- Internal refactoring of command parsing.
- Slight improvement on argument parsing for 'z' and 'Z'.
- Write some general buffer tests.
- Clean up the RELEASE_NOTES.md formatting a bit.
- Reduce help text width to 80 columns.
- Add "vecbuffer" as default feature.
- Change Buffer api, expect buffer to handle substitutions in search_replace and
  get_matching. At a minimum \\n and \\\\.
- Add utility substitute module to buffer module, for buffer implementations.
- Add buffer api tests, to verify buffer implementation behaviours.
- Add add_ed integration tests, to avoid breaking our API in the future.
- Add fuzzing support, which caught 2 string slicing errors.
- Correct handling for lone 's' to apply flags from last proper 's' invocation.
- Fix so that 's' return error if no selected line matched the pattern.
- Rename UI function print to print_message, to make its use more clear.
- Remove EdState from print_message, as the UI will normally be called through
  get_command immediately thereafter (any UI updates can wait 'til then).
- Implement the possibility to change default of printing literal and numbered.
  Both via Ed constructor and via 'L' and 'N' commands.

# 0.8.1
- Add proper help text for 'A', 'I' and 'C'.

# 0.8.0
- Add commands 'A' and 'I', which work as their 'a' and 'i' counterparts but
  join with the preceding/following line.
- Add command 'C' behind "initial_input_data" feature flag. Moves selection
  into input buffer, allowing per-character editing of selection.

# 0.7.1 -> 0.7.7
As it looks there will be many updates the coming month with minor fixes as I
gradually clear up bugs using the editor itself. All these updates will be
grouped in under this note.

- Configure the vec-buffer to consider itself saved immediately after opening
  a new file, until first edit.
- Fix some off-by-one errors in vecbuffer coming from the recent change into
  inclusive indices.
- Add a const string for aborted input. It is adjusted for use with ctrl-c
  capture and prints how to quit.
- Add 'z' command and the same backwards under 'Z'. Tried to touch up the help
  text as well.
- Fix off-by-one bug in 'z'.
- Exclude last newline in selection before running regex in vecbuffer. Less
  unexpected consequences from my experience.
- Fix off-by-one bug in 't'.
- Correct a forgotten todo in 's' flag handling.
- Fix off-by-one bug in 'a', 'i' and 'c' handler.

# 0.7.0
Since this is a bit more public now it seems to be time to start with release
notes, so that is the first change for this release. Beyond that there are
some adjustments based on UIs I am trying to write, which cascade into quite
big API changes.

- Create an EdState struct for sharing references to all UI relevant state
  variables.
- Change UI API to use the EdState struct
- Prepare the UI API for command input prefix support. The command to use it
  will come later.
- Greatly widen the required regex version for vecbuffer, to prevent version
  clashes.
