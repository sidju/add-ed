# Basic editing commands:
- `a/i` Insert lines after/before index.
- `d` Cut lines into clipboard.
- `y` Copy lines into clipboard.
- `x/X` Paste clipboard after/before index.
- `j` Join selection into one line.

# Combined editing commands:
- `A/I` As `a`/`i` but join first/last line with indexed line.
- `c` Replace selection with input. Like `d` and `i`.
- `C` As `c` but with selection as initial input.
- `m` Move selection to index. Like `d` and `x`.
- `t` Copy selection to index. Like `y` and `x`.

# File and shell commands:
- `e` Open given file.
- `r` Read from given file to given index.
- `w` Write to given file.
- `W` Append to given file.
- `|` Pipe data through given command.
- `!` Run given shell command.

# Batch editing commands:
- `s` Search and replace
- `g/v` Run commands on matching/not-matching lines.
- `G/V` Interactively run commands on matching/not-matching lines.
- `:` Run macro.

# Status commands:
- `help` Print this help section.
- `Help` Print commands documentation.
- `q` Quit the editor, warns on unsaved changes.
- `Q` Quit ignoring unsaved changes.
- `h` Print last occured error.
- `H` Toggle printing error or `?` on error.
- `=` Print current selection.
- `#` Do nothing (start of comment)
- `f` Print default file, or replace if one given.
