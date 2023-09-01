# What I'm currently doing:
- Fixing up usage documentation for Buffer and Clipboard, so that it is clear
  how to look into and modify them in an ergonomic and safe way. Note the
  not compiling example code in src/buffer/mod.rs which tries to describe
  ergonomic usage (and currently mostly shows its absence). To make that build
  we need to make PubLine and Line be constructable from both &T and T where
  applicable, so loads of From<...> plumbing.


# Todos:
- Add a way to print the available undo/redo span ('U'?)
  (We probably want snapshot annotations to indicate what command caused a state
  to make this meaningful)
  (If so, check over EdError::UndoStepsNotInt which refers to 'U' as redo)
- Add undo/redo utilities:
  - Absolute indexing `u*20` goes to the 20:th snapshot
  - A way to clear the history (to clear memory usage)
  - Make it so the history deduplication is run in more cases (to save on RAM)?
    (Could be confusing, probably better to abort more using `EdError::NoOp`)
- Look over when we save to Ed.prev_shell_command, currently before execution.
  (Could be nice to check that it runs successfully first, but for some
  commands (eg. compilation) that could make it unusable...)
- Consider adding 'R' command, as 'r' but inserts before selection.
- Inject context environment variables into shell interaction.
  (File, selection_start, selection_end, prev_shell_command, if running script)
- Improve classic.rs to support all of ed's command line arguments
- Look over API documentation again, since refactoring has changed the API.
- Implement parsing under the trait FromStr instead?


# Look over command documentation
Currently the help text is very outdated, and it is also getting unreasonably
long. We should probably change out the help text for a minimal summary and
write up a proper manual over commands.


# Look over macros.
We probably wish to support more languages than a string of ed commands, so
we should add some way to signify how to run the macro.

Or maybe we double down against that and say that any fancy code should run
via the shell interaction features, making command lists "completely
sufficient" as long as we add support for macro arguments in command lists.

We should also consider error handling in scripts, as they could reasonably want
to use `s`, which currently errors if there is no match. The easiest fix is to
continue execution in the case of NoOp and NoMatch errors in macros, as such
would generally be non-fatal. This would best be combined with a small Macro
struct that has some configurations, such as wether or not to abort on errors
and also if the macro should be able to modify the buffer (if set to no we can
run the macro and delete the snapshot created for it, so even if it left the
buffer in an unclean state we are unaffected (also saves a big Eq check)).

Even with some global configurations in scripts we would probably want some way
to ignore errors in a specific command while reacting to others, and the
opposite as well. Perhaps something with `H` (Hide) or some braces (to contain
the error).



# Documentation fixes:
- Document more in the History struct, as there isn't more details/links on
  github.
- The Snapshot trait from history.rs should be public, if we wish to make it
  re-usable, and should document the solution with Rc and Cell for dedup and
  making fields mutable without snapshot creation.
- History set_saved documentation has bad references (Self not working?).
- History.current_mut should document why it might error. (Same for snapshot)
- Ed, history field documentation should refer to undo/redo states, not past,
  present, future.
- Ed, clipboard field should refer to PubLine by reference.
- Ed, selection should note that invalid selection not causing crashes is
  verified by fuzzing (and we should check that that's true as well).
- Ed::new() should document the defaults of public fields.
- Ed, macros should probably be flagged as especially experimental.
- Ed, methods should generally be clearer with what error they will return
  under what circumstance.
  (
  Returns any error from command execution vs. returns UI errors that occur
  when getting command vs. returns UI errors that occur when printing any
  error that occured internally.
  Those who write an infallible UI should be able to know what methods may
  no longer return errors.
  )
- In UI docs, possibly mention that you can combine ScriptedUI with MockUI
  to script input and capture output.
- UILock, elaborate why this is a thing. Link to UI.lock_ui().
- UI.unlock_ui, possibly see if we can require a private phantom/marker
  argument to this, preventing it from being called by any code not in the
  add_ed crate (and thus making it impossible for a library user to
  incorrectly run it from anywhere but UILock::drop)
- DummyIO, document that all invocations return Ok(()) without doing
  anything.
- Possibly elaborate in FakeIOError when the different errors may occur.
  (Child exit error is given when command not found, not found is given
  when a file is not found)
- On that topic, LocalIOError::ChildReturnedError should note that most
  shells will return an error when the given command wasn't found.
  Perhaps we should even consider adding that to the Display impl?
- Possibly add a private empty variable to LocalIO to enforce using the
  constructor (removing the need to document recommending using the
  constructor).
- IOError wrapper needs a bit of documentation, mention the downcast
  helper and From implementations.
- UIError wrapper needs a bit of documentation, mention the downcast
  helper and From implementations.
- IOErrorTrait should explain why it exists and how to use it.
- UIErrorTrait should explain why it exists and how to use it.
- EdError should note that it doesn't check their equality on the IO
  and UI variants own documentation.
- UIError and IOError should give a downcast example.
- EdError IndexTooBig should either note that it is returned on empty
  buffer or we should have a separate error for empty buffer.
