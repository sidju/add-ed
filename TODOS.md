# Todos:
- Consider types in traits.
  Maybe let UI hand in String for input?
- Shell interaction. ('!' as usual)
- Inject context environment variables into shell interaction. File, lines, etc.
- Expand 'r' and 'w' to allow reading from/writing to stdout/in
- Add | command for passing selection through shell command?
  Or do that if ! is given a selection, like vim does?
- Support % as whole buffer? (redundant with ',')
- Have some variant of a ed.hup file
  (try to write buffer to it before panicking)
- 'j' should put the replaced lines in the clipboard
  (look over clipboard interactions in ed docs)

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
