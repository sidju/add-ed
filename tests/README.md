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

# Shared testing function:
There is a function that most testing fixtures calls named `inner_fixture`. This
is in part to reduce code duplication, but also so that we effectively verify
that commands don't have more side effects than they should. The `inner_fixture`
function always checks all the pre and post conditions, so even if a test
doesn't mention (for example) the filepath it will be verified using static pre
and post conditions defined in the fixture wrapping `inner_fixture`.
