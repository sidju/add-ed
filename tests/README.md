# Test naming:
Tests are named in two parts: a descriptive name for the command tested and 
suffixes describing what specific circumstance is tested. Normal circumstances
are considered to be an explicit selection/index on a buffer with lines in and
input if the command accepts it. (and no print flags)

## Common circumstance suffixes:
- `nobuffer`, the buffer is empty
- `noselection`, the selection is (1,0)
- `noinput`, no input is given to the command
- `print`, print flag is given
- `numbered`, numbered print flag is given
- `literal`, literal print flag is given
