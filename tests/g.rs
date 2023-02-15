// Test 'g' and 'G' command
// 'g' first, 'G' after

mod shared;
use shared::fixtures::{
  BasicTest,
  PrintTest,
};
use shared::mock_ui::Print;

// Verify behaviour of 'g'
//
// - Takes optional index
//   - If given, marks all matching lines in that selection
//   - If none, same using state.selection
// - Takes a list of arguments separated by the first char following 'g'
//   - First is the regex that lines are marked if matching
//   - Then it takes any number of commands to run on all matching lines.
//   - If the last argument on the line doesn't have the separator after:
//     - Starts taking input with the separator from above as terminator.
//       Each input line is another command to run on all matching lines.
//       (No additional per-line termination required, unlike gnu ed.)
// - If no line matches the regex the command aborts, leaving state unchanged.
// - Selection after command is the selection searched for matches.
// - Doesn't set/unset unsaved, but the commands executed affect as usual.
