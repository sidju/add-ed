# Plugin support, `@`:
Using a specific Macro trait and a `HashMap<&str, &mut dyn Macro>`.

Intended to be compiled in with the editor, but if someone could do something
clean and *stable* with dynamic library loading I'd be open to it.

Not implemented yet since shell interaction and macros can mostly emulate the
behaviour without running external code within the editor process. But since
plugins could have significant performance advantages (as they don't need to
pipe all the data out and in again, breaking all the Rc links preventing
duplication of line data) this is intended to be implemented eventually.

# Multi-buffer support (as part of UI), `b`:
Allow opening a file as a different buffer. The arguments for this command would
be handed into a specific feature enabled `UI` method, allowing the UI to create
or switch to another instance of `add_ed::Ed`.

For some examples of how this could work look into how `qed`, `sam` and `vim`
handle multiple open files. This feature should fit all those approaches.

Not implemented yet as I (sidju) don't have any interest in this feature. If you
want to use this feature, tell me and give me two weeks.

# Command failover into UI or other object:
A more flexible way to extent functionality than plugins under `@`. Every time a
command isn't recognized (more or less) the partially parsed command data would
be handed into a "`CommandExtender`" or similar, which can then hold
implementations for any command not defined in add-ed itself.

Not implemented since I (sidju) don't need it, this might be a bit complex to
implement. Further it might be better to keep non-`add-ed` commands more
clearly separate. If you want to use this feature please provide an example
implementation and settle in for some discussion, so the feature is implemented
in a clean way that fits your use-case.

# More variant commands:
There are loads of potential variations on existing commands. I (sidju) have
decided to be restrictive on what I add initially, so that any further built-in
commands can be decided upon through discussion with users later.

Though some variants were added and have implementations ready in the codebase
none of them are enabled, as they were found to not be that useful. Any further
commands will need to be significant distincts (not just prepend to an index
instead of appending, but using start of selection instead of end of selection
is sufficiently distinct if relevant).
