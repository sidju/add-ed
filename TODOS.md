# Todos:
- Consider adding 'R' command, as 'r' but inserts before selection.
- Inject context environment variables into shell interaction.
  (File, selection_start, selection_end, prev_shell_command, if running script)
- 'j' should put the replaced lines in the clipboard
  (look over clipboard interactions in ed docs)
- Look over how IO interactions error, print ! after error to show handing back
  control into add-ed? And consider if 'r' and 'w' should print '!'.
- Improve classic.rs to support all of ed's command line arguments
- Look over API documentation again, since refactoring has changed the API.

# Testing improvements:
## Unit tests:
- buffer_api test should be split out into multiple tests.

## Integration tests:
- !
- |

## Fuzzing:
- Set up fuzzing again with mock IO, to prevent creating thousands of files.

# vim examples with !:
- `r !git log` to read in prior commit messages
- (explicit selection)`!sort` to sort selected lines
- `.!figlet` to make a line into big ascii art letters
- (explicit selection)`! grep/awk/jq` to filter selection
- `%!python -m json.tool` to send whole buffer though json.tool
  (probably use ed's way to select whole buffer instead)
- `w !sudo tee %` save with sudo (love this!)
- `!git add %`
- `!python %` to run current file (could be `!./%` if chmod +x ?)
- `!mkdir path` for when you can't create a file because it's dir doesn't exist

## In summary:
- `<index>r!<command>` appends stdout to index
  (create `R` counterpart that takes stderr? Flag?)
- `<selection>w!<command>` copies selection to stdin
- `<selection>!<command>` sends selection through command, replacing it with
  what is returned via stdout
- `!<command>` just run command

In all these normal selection handling applies and in command string '%' is
replaced by the path to the current file unless escaped. (No other escapes are
handled)
