# 0.7.1 -> 0.7.6
As it looks there will be many updates the coming month with minor fixes as I gradually clear up bugs
using the editor itself. All these updates will be grouped in under this note

* Configure the vec-buffer to consider itself saved immediately after opening a new file, until first edit.
* Fix some off-by-one errors in vecbuffer coming from the recent change into inclusive indices.
* Add a const string for aborted input. It is adjusted for use with ctrl-c capture and prints how to quit.
* Add 'z' command and the same backwards under 'Z'. Tried to touch up the help text as well.
* Fix off-by-one bug in 'z'.
* Exclude last newline in selection before running regex in vecbuffer. Less unexpected consequences from my experience.
* Fix off-by-one bug in 't'.
* Correct a forgotten todo in 's' flag handling.
* Fix off-by-one bug in 'a', 'i' and 'c' handler.

# 0.7.0
Since this is a bit more public now it seems to be time to start with release notes, so that is the
first change for this release. Beyond that there are some adjustments based on UIs I am trying to write,
which cascade into quite big API changes.

* Create an EdState struct for sharing references to all UI relevant state variables.
* Change UI API to use the EdState struct
* Prepare the UI API for command input prefix support. The command to use it will come later.
* Greatly widen the required regex version for vecbuffer, to prevent version clashes.
