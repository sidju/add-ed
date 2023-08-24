# Gotcha's:

## The buffer API's indexing is from 1 instead of 0:
This is to match the command languages indexing.
The advantage of this is that one can append to 0 to imitate insert, which some
commands that normally only append neccessitate.

## Selection bounds are _inclusive_:
This means you should use ..= instead of .. in your ranges and that the number
of lines in a selection is sel.1 - sel.0 _+ 1_.
