# Add-ED
## The standard editor, now as a library
Some time ago I decided to write an ED clone with syntax highlighting. On my way to that goal I needed to write an implementation of ED's command parsing and execution to wrap in that syntax highlighting. And so, here it is.

## Early APIs
The current API design is only based on my use-case. If you are interested in using it and need some change to make it work better you are more than welcome to suggest changes.

## Core concepts:
### The selection:
The original 'ed' keeps track of one line that you recently interacted with and defaults to working on that if no lines are given with a command. This is an extension of that logic, making it a span of lines. I find that this is more intuitive.
(To avoid unpleasantries some commands don't default to the selection, such as 'w'. If you want to modify the selection behavior for any command create an issue, I may well have missed one.)

### Flexible APIs:
The modules have been set up with clear traits to enable changing out the components easily. For example it should be somewhat trivial to code a SSH+sed Buffer implementation for remote editing or a GUI frontend conforming to the UI trait.

## Commands:
This list is not fully updated. Hired now supports nearly all Ed commands, only missing 'z', 'u' and '!'.
 
### Lone commands:
Commands that take no input or selection.
- q: Quit. Returns error if you have unsaved changes.
- Q: Force Quit. Ignores unsaved changes.
- h: Print Error. Print the last occured error.
- H: Toggle Error Printing. Toggle printing errors as they occur.

### File commands:
- f: If no filepath is given, prints filepath. Else sets the filepath to given path.
- e: Closes current file and opens the given path. Returns error if you have unsaved changes.
- E: Closes current file and opens the given path, ignoring unsaved changes.
- r: Append the data from given path to given selection.

- w: Write the given selection (default whole buffer) to given path (default filepath). Warning, silently overwrites the file.
- W: Append the given selection (default whole buffer) to given path (default filepath).

### Print commands:
- p: Print the given selection.
- n: Print the given selection with numbered lines.
- l: Print the given selection with character escaping. 

### Basic editing commands:
- a: Append. Append lines given after the command to given selection. Stop entry with only '.' on a line.
- i: Insert. Insert lines given after the command before given selection. Stop entry with only '.' on a line.
- c: Change. Replace given selection with lines given after the command. Stop entry with only '.' on a line.
- d: Delete. Delete the given selection.

### Advanced editing commands:
- m: Move. Move given selection to given index.
- t: Transfer. Copy given selection to given index.
- j: Join. Append together given selection into one line.

### Regex commands:
- s: Substitute. Regex replace, just like 'sed'.
- g: Global command. Command list given as arguments or text input runs on all matching lines.
- v: inVerted global. Command list given as arguments or text input runs on all non-matching lines.
- G: interactive Global. Prints matching lines and takes command list for each line as text input.
- V: interactive inVerted global. Prints non-matching lines and takes commands for each as text input.

### Special cases:
- no command: Takes the given selection and sets it to the default selection.
## Attributions:
This project has of course greatly benefited from all the crates it depends on. Especially I'd like to thank regex and syntect for helping me through my, to various degrees badly though out, issues.

Then I have also gotten a hand up from 'bat', which I also consider an excellent companion to this application, from their handling of 16-color terminals. My theme is currently copied from their repo and will probably always be based on theirs.
