// Test behaviour of 'd'

mod shared;
use shared::fixtures::BasicTest;

// Verify behaviour of 'd' command
//
// - Takes optional selection
//   - If given deletes lines in selection
//   - If not given deletes lines in state.selection
// - Selection after:
//   - Selects nearest line following the deleted lines
//   - If no line after deleted lines, selects nearest preceeding
//   - If no lines left in buffer after delete selects (1,0)
// - Lines deleted are moved to the clipboard
// - Sets unsaved if selection is valid/it is executed
