# 7.0
Since this is a bit more public now it seems to be time to start with release notes, so that is the
first change for this release. Beyond that there are some adjustments based on UIs I am trying to write,
which cascade into quite big API changes.

* Create an EdState struct for sharing references to all UI relevant state variables.
* Change UI API to use the EdState struct
* Prepare the UI API for command input prefix support. The command to use it will come later.
* Greatly widen the required regex version for vecbuffer, to prevent version clashes.
