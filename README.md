# Add-ED
## The standard editor, now as a library
Some time ago I decided to write an ED clone with syntax highlighting, (hired)[https://github.com/sidju/hired].
On my way to that I found that the macro commands ('g', 'v', 'G', 'V') pretty much require that you write an ED library for them.
So I did, and here it is.

## Early APIs
Currently it is based on both my experiences with hired and the hired repo specifically (a manual fork, so it has the whole history).
This may well mean the API is ill suited for your use. If that is the case I'd be happy to make some changes to make it more general.

## Core concepts
### The selection:
The original ED keeps track of the last line you interacted with and defaults to working on that for most commands.
I felt this deserved expanding, so add-ed instead tracks the last span of lines you interacted with.
My hope is that this is more intuitive.
(To avoid unpleasantries such as partial saving some commands default to the whole buffer instead. Such as 'w', the save command.)

### Flexible APIs
The library has been designed with clear traits to enable changing out most components easily.
For example it should be somewhat easy to create a SSH+sed Buffer implementation for remote editing,
or a GUI frontend implementing the UI trait.

## New features compared to Ed
- 'A' and 'I' commands, which act as 'a'/'i' and then 'j' the preceding/following line.
  Perfect for commenting out a single line or adding a forgotten ;.
- 'C' command, acts as 'c' but hands out previous value to the Ui's input method.
  This enables you to edit the selection instead of replacing it (depends on Ui).
- 'N' and 'L' commands, toggle the default print conf. Line numbers and escapes respectively.
- 'K' command tags the last line in selection (instead of first as 'k'). (selection adjustment)
- ':' command, runs the macro with the name given as argument (whitespace trimmed).
  Macro execution behaves like 'g' execution. 'q' or error returns early.

## Feature flags:
### vecbuffer:
Include and expose a simple in-memory buffer implementation
### initial_input_data:
Add 'C' command. This modifies the UI trait.

## Attributions:
This project is essentially built upon the regex crate, as regex is the heart of Ed.
