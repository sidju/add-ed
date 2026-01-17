# Basic editing commands:

- `d` Cut lines into clipboard.
- `y` Copy lines into clipboard.
- `x/X` Paste clipboard after/before index.
- `j` Join selection into one line.

# Input mode commands:

- `a/i` Insert lines after/before index.

# Combined editing commands:

- `c` Replace selection with input. Like `d` and `i`.
- `C` As `c` but with selection as initial input.
- `m` Move selection to index. Like `d` and `x`.
- `t` Copy selection to index. Like `y` and `x`.
- `A/I` Insert lines after/before index, joining with indexed line.

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
- `o` Run macro.

# Status commands:

- `help` Print this help section.
- `Help` Print commands documentation.
- `q` Quit the editor, warns on unsaved changes.
- `Q` Quit ignoring unsaved changes.
- `h` Print last occured error.
- `H` Toggle printing error or `?` on error.
- `=` Print editor status (defaults to selection, use `a` for full status).
- `#` Do nothing (start of comment)
- `f` Print default file, or replace if one given.
- `k/K` Tag lines with character for reference.
- `P` Toggle default printing behavior.

# History and undo commands:

- `u` Undo/redo operations.
- `U` Manage and view history.

# Printing commands:

- `<nothing>` Print lines after current selection.
- `p/n/l` Print selection with various formatting.
- `z/Z` Scroll forward/backward with line count.
