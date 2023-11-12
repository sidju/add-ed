# Add-ED

## The standard editor, now as a library

Some time ago I decided to write an ED clone with syntax highlighting,
[hired](https://github.com/sidju/hired). On my way to that I found that the
macro commands (`g`, `v`, `G`, `V`, `:`) pretty much require that you write an
ED library for them. So I did, and here it is.

## Differences from GNU Ed

I have taken some liberties in the commands I have found less than ergonomic, so
especially advanced commands have some differences (both syntax and behaviour)
from GNU Ed. This strives to be a list of these changes.

- `g` and related commands take command list in input mode (with the regex
  separator as terminator instead of '.').
  (Also, the 'I' suffix for case insensitive matching isn't implemented)
- `#` accepts a selection and will set state.selection to it without printing
  anything. This is added to be able to set selection without printing.

## Core concepts

### The selection:

The original ED keeps track of the last line you interacted with and defaults to
working on that for most commands. I felt this deserved expanding, so add-ed
instead tracks the last span of lines you interacted with. My hope is that this
is more intuitive. (To avoid unpleasantries such as partial saving some commands
default to the whole buffer instead, ie. `w` .)

### Flexible APIs

The library has been designed with clear traits to enable changing out most
components easily. For example it should be somewhat easy to create a SSH/SFTP
IO implementation for remote editing, a GUI frontend implementing the UI
trait or a MacroGetter that loads macros from any given filepath.

## New features compared to Ed

- `A` and `I` commands, which first `a`/`i` and then `j` the preceding/following
  line. Perfect for commenting out a single line or adding a forgotten ';'.
- `C` command, acts as `c` but hands out the selection's previous value to the
  UI's input method. This enables you to edit the selection instead of replacing
  it.

  (Availability depends on UI and the `initial_input_data` feature.)
- `P` command, toggles the default for the given print flags.
- `:` command, runs the macro with the name given as argument (whitespace
  trimmed). Macro execution behaves like 'g' execution. 'q' or error returns
  early.
- `|` command, pipes selection through given shell command (like `<selection>!`
  does in vim). It is distinct from `!` to enable defaulting to current
  selection when no explicit selection is given.

## Feature flags:

### local_io:

Include and expose a simple local fs and shell `IO` implementation.

### initial_input_data:

Add 'C' command. This modifies the UI trait.

### serde:

`#[derive(serde::Serialize, serde::Deserialize)]` on `Macro` struct.

Other structs can get derived on upon request, but `Macro` (and its members) was
the only one that seemed relevant.

## Attributions:

This project is essentially built upon the regex crate, as regex is the heart of Ed.

## Contributing:

There are two main contributions welcomed as of now.

1. Adding tests. Though core uses for command are tested, more behaviours should
   be validated. If you have the time, add test cases that validate that:
   - Commands behave as [COMMANDS.md](COMMANDS.md) states.
   - Commands behave same as in GNU Ed, unless otherwise documented in the
     *Differences from GNU Ed* section above.
2. Checking off ToDo:s. Look into the [TODOS.md](TODOS.md) file, pick a task to
   do and go wild. You can add a PR to prevent others from working on the same
   task (mark it as WIP until relevant to review).
3. Create issues for undocumented or unexpected behaviour.
4. Request support for your use-cases. Create a feature request issue for what
   you need. (Look into [POTENTIAL_FEATURES.md](POTENTIAL_FEATURES.md) to see if
   a similar feature is prepared and if so note it in the feature request.)
