# Todos:
- Add a way to print the available undo/redo span ('U'?)
  (If so, check over EdError::UndoStepsNotInt which refers to 'U' as redo)
- Look over when we save to Ed.prev_shell_command, currently before execution.
  (Could be nice to check that it runs successfully first, but for some
  commands (eg. compilation) that could make it unusable...)
- Consider adding 'R' command, as 'r' but inserts before selection.
- Inject context environment variables into shell interaction.
  (File, selection_start, selection_end, prev_shell_command, if running script)
- Improve classic.rs to support all of ed's command line arguments
- Look over API documentation again, since refactoring has changed the API.
- Implement parsing under the trait FromStr instead?


# Look over macros.
We probably wish to support more languages than a string of ed commands, so
we should add some way to signify how to run the macro.

Or maybe we double down against that and say that any fancy code should run
via the shell interaction features, making command lists "completely
sufficient"...




# Documentation fixes:
- Clipboard and Buffer should have examples for how you should and shouldn't
  modify them.
- Line should clearly document that it can be constructed via PubLine
- PubLine shouldn't refer to positioning in the file, as it may be reordered
  when the documentation is rendered.
- PubLine has some bad references in the field documentations
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
