// Tests for ':' command

mod shared;
use shared::fixtures::{
  BasicTest,
};

// Verify behaviour of ':' command
//
// - Takes no selection
//   - Accepts space separated arguements for the macro after the command character
// - First argument: name of a macro
// - Remaining arguments: list of arguments supplied to the macro
// - Errors if macro execution errors depending on macro configuration

