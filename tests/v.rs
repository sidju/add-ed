// Test 'g' and 'G' command
// 'g' first, 'G' after

mod shared;
use shared::fixtures::PrintTest;
use shared::mock_ui::Print;

// Verify behaviour of 'v'
//
// - Takes optional index
//   - If given, marks all non-matching lines in that selection
//   - If none, same using state.selection
// - Takes a list of arguments separated by the first char following 'v'
//   - First is the regex that lines are marked if not matching
//   - Then it takes any number of commands to run on all marked lines.
//   - If the last argument on the line doesn't have a separator after:
//     - Starts taking input with the separator from above as terminator.
//       Each input line is another command to run on all matching lines.
//       (No additional per-line termination required, unlike GNU Ed.)
//   - If no commands are given it defaults to one invocation of 'p'.
// - If all lines match the regex the command aborts, leaving state unchanged.
// - Selection after command is the selection left by execution of macro on
//   the last matching line. (chosen to handle deletions correctly)
// - Doesn't set/unset unsaved, but the commands executed affect as usual.

// The most normal test case, v/re/p/, using that 'v' defaults to 'p'
#[test]
fn invglobal_grep_defaultcommand() {
  let buffer = vec![
    "hello",
    "1",
    "4",
    "there",
  ];
  PrintTest{
    init_buffer: buffer.clone(),
    init_clipboard: vec![],
    command_input: vec![r",v/\d/"],
    expected_buffer: buffer,
    expected_buffer_saved: true,
    expected_selection: (4,4),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["hello\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["there\n".to_string(),],
        n: false,
        l: false,
      },
    ],
  }.run();
}

// Test a slightly fancier multiline invocation
#[test]
fn invglobal_vrep_and_delete() {
  PrintTest{
    init_buffer: vec![
      "hello",
      "1",
      "4",
      "there",
      "9",
    ],
    init_clipboard: vec![],
    command_input: vec![r",v/\d/p","d","/"],
    expected_buffer: vec![
      "1",
      "4",
      "9",
    ],
    expected_buffer_saved: false,
    expected_selection: (3,3),
    expected_clipboard: vec!["there"],
    expected_prints: vec![
      Print{
        text: vec!["hello\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["there\n".to_string(),],
        n: false,
        l: false,
      },
    ],
  }.run();
}

// The most normal test case, g/re/p/, but with default selection
// Also run a command after, to verify that 'g' doesn't incorrectly take input
#[test]
fn invglobal_grep_noselection_commandafter() {
  let buffer = vec![
    "hello",
    "1",
    "4",
    "there",
  ];
  PrintTest{
    init_buffer: buffer.clone(),
    init_clipboard: vec![],
    command_input: vec![r",v/\d/p/","l",],
    expected_buffer: buffer,
    expected_buffer_saved: true,
    expected_selection: (4,4),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["hello\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["there\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["there\n".to_string(),],
        n: false,
        l: true,
      },
    ],
  }.run();
}

// Verify behaviour of 'V'
//
// - Takes optional selection.
//   - If given marks all nonmatching lines in that selection.
//   - If none, same using state.selection.
// - Takes a list of arguments separated by the first char following 'V'.
//   - First is the regex that lines are marked if not matching.
//   - No further arguments currently supported.
// - If all lines match the regex the command aborts, leaving state unchanged.
// - For each marked line:
//   - That line is printed.
//   - Commands to run on that line are taken in input mode using the separator
//     as terminator.
//     (If none are given defaults to one invocation of 'p')
//   - Commands are run in the order given, aborting on any error.
// - Selection after command is the selection left by commands executed on the
//   last line (that line, if no commands given).
// - Doesn't set/unset unsaved, but the commands executed affect as usual.

// Run interactive with default/no command and selection
// Additionally run a command after, to verify input is done
#[test]
fn invglobal_interactive_defaultcommand_defaultselection() {
  let buffer = vec![
    "hello",
    "1",
    "4",
    "there",
  ];
  PrintTest{
    init_buffer: buffer.clone(),
    init_clipboard: vec![],
    command_input: vec![r"V/\d/","/","/","l",],
    expected_buffer: buffer,
    expected_buffer_saved: true,
    expected_selection: (4,4),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["hello\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["there\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["there\n".to_string(),],
        n: false,
        l: true,
      },
    ],
  }.run();
}

// Run interactive command with delete command
// Additionally run a command after, to verify input is done
#[test]
fn invglobal_interactive_delete() {
  PrintTest{
    init_buffer: vec![
      "hello",
      "1",
      "4",
      "there",
    ],
    init_clipboard: vec![],
    command_input: vec![r"V/\d/","d","/","d","/","l",],
    expected_buffer: vec![
      "1",
      "4",
    ],
    expected_buffer_saved: false,
    expected_selection: (2,2),
    expected_clipboard: vec!["there"],
    expected_prints: vec![
      Print{
        text: vec!["hello\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["there\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["4\n".to_string(),],
        n: false,
        l: true,
      },
    ],
  }.run();
}
