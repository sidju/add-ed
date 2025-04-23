// Test 'g' and 'G' command
// 'g' first, 'G' after

mod shared;
use shared::fixtures::PrintTest;
use shared::mock_ui::Print;

// Verify behaviour of 'g'
//
// - Takes optional index
//   - If given, marks all matching lines in that selection
//   - If none, same using state.selection
// - Takes a list of arguments separated by the first char following 'g'
//   - First is the regex that lines are marked if matching
//   - Then it takes any number of commands to run on all matching lines.
//   - If the last argument on the line doesn't have a separator after:
//     - Starts taking input with the separator from above as terminator.
//       Each input line is another command to run on all matching lines.
//       (No additional per-line termination required, unlike GNU Ed.)
//   - If no commands are given it defaults to one invocation of 'p'.
// - If no line matches the regex the command aborts, leaving state unchanged.
// - Selection after command is the selection left by execution of macro on
//   the last matching line. (chosen to handle deletions correctly)
// - Doesn't set/unset unsaved, but the commands executed affect as usual.

// The most normal test case, g/re/p/, using that 'g' defaults to 'p'
#[test]
fn global_grep_defaultcommand() {
  let buffer = vec![
    "hello",
    "1",
    "4",
    "there",
  ];
  PrintTest{
    init_buffer: buffer.clone(),
    init_clipboard: vec![],
    command_input: vec![r",g/\d/"],
    expected_buffer: buffer,
    expected_buffer_saved: true,
    expected_selection: (3,3),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["1\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["4\n".to_string(),],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec![],
  }.run();
}

// Test a slightly fancier multiline invocation
#[test]
fn global_grep_and_delete() {
  PrintTest{
    init_buffer: vec![
      "hello",
      "1",
      "4",
      "there",
      "9",
    ],
    init_clipboard: vec![],
    command_input: vec![r",g/\d/p","d","/"],
    expected_buffer: vec![
      "hello",
      "there",
    ],
    expected_buffer_saved: false,
    expected_selection: (2,2),
    expected_clipboard: vec!["9"],
    expected_prints: vec![
      Print{
        text: vec!["1\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["4\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["9\n".to_string(),],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec![r",g/\d/p"],
  }.run();
}

// And one where g feeds a command line-input in the same line
#[test]
fn global_grep_and_change() {
  PrintTest{
    init_buffer: vec![
      "hello",
      "1",
      "4",
      "there",
      "9",
    ],
    init_clipboard: vec![],
    command_input: vec![r",g/\d/p/c/number/"],
    expected_buffer: vec![
      "hello",
      "number",
      "number",
      "there",
      "number",
    ],
    expected_buffer_saved: false,
    expected_selection: (5,5),
    expected_clipboard: vec!["9"],
    expected_prints: vec![
      Print{
        text: vec!["1\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["4\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["9\n".to_string(),],
        n: false,
        l: false,
      },
    ],
    expected_history_tags: vec![r",g/\d/p/c/number/"],
  }.run();
}

// The most normal test case, g/re/p/, but with default selection
// Also run a command after, to verify that 'g' doesn't incorrectly take input
#[test]
fn global_grep_noselection_commandafter() {
  let buffer = vec![
    "hello",
    "1",
    "4",
    "there",
  ];
  PrintTest{
    init_buffer: buffer.clone(),
    init_clipboard: vec![],
    command_input: vec![",n",r",g/\d/p/","l",],
    expected_buffer: buffer,
    expected_buffer_saved: true,
    expected_selection: (3,3),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "hello\n".to_string(),
          "1\n".to_string(),
          "4\n".to_string(),
          "there\n".to_string(),
        ],
        n: true,
        l: false,
      },
      Print{
        text: vec!["1\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["4\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["4\n".to_string(),],
        n: false,
        l: true,
      },
    ],
    expected_history_tags: vec![],
  }.run();
}

// Tracks an old bug, where a shared variable was used to mark which lines had
// been matched by a 'g', 'G', 'v' or 'V' command. This could cause infinite
// recursion, as helpfully shown by fuzzing (though it took me a while to
// figure out what was going on.)
//
// In this test this is shown since the inner 'g' invocation marks line 2, acts
// on it, and then unmarks it (causing it to be skipped in the outer 'g', and
// some more trouble I don't care to figure out).
#[test]
fn global_nested() {
  let buffer = vec![
    "hello",
    "1",
    "4",
  ];
  PrintTest{
    init_buffer: buffer.clone(),
    init_clipboard: vec![],
    command_input: vec![r",g/.*/p/,g_1_n_/"],
    expected_buffer: buffer,
    expected_buffer_saved: true,
    expected_selection: (2,2), // Since 1 is printed last and that is line 2
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec!["hello\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["1\n".to_string(),],
        n: true,
        l: false,
      },
      Print{
        text: vec!["1\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["1\n".to_string(),],
        n: true,
        l: false,
      },
      Print{
        text: vec!["4\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["1\n".to_string(),],
        n: true,
        l: false,
      },
    ],
    expected_history_tags: vec![],
  }.run();
}

// Verify behaviour of 'G'
//
// - Takes optional selection.
//   - If given marks all matching lines in that selection.
//   - If none, same using state.selection.
// - Takes a list of arguments separated by the first char following 'G'.
//   - First is the regex that lines are marked if matching.
//   - No further arguments currently supported.
// - If no line matches the regex the command aborts, leaving state unchanged.
// - For each matching line:
//   - That line is printed.
//   - Commands to run on that line are taken in input mode using the separator
//     as terminator.
//     (If none are given defaults to one invocation of 'p')
//   - Commands are run in the order given, aborting on any error.
// - Selection after command is the selection left by commands executed on the
//   last line (that line, if no commands given).
// - Doesn't set/unset unsaved, but the commands executed affect as usual.

// Run interactive global command with default/no command and selection
// Additionally run a command after, to verify input is done
#[test]
fn global_interactive_defaultcommand_defaultselection() {
  let buffer = vec![
    "hello",
    "1",
    "4",
    "there",
  ];
  PrintTest{
    init_buffer: buffer.clone(),
    init_clipboard: vec![],
    command_input: vec![",n",r"G/\d/","/","/","l",],
    expected_buffer: buffer,
    expected_buffer_saved: true,
    expected_selection: (3,3),
    expected_clipboard: vec![],
    expected_prints: vec![
      Print{
        text: vec![
          "hello\n".to_string(),
          "1\n".to_string(),
          "4\n".to_string(),
          "there\n".to_string(),
        ],
        n: true,
        l: false,
      },
      Print{
        text: vec!["1\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["4\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["4\n".to_string(),],
        n: false,
        l: true,
      },
    ],
    expected_history_tags: vec![],
  }.run();
}

// Run interactive global command with delete command
// Additionally run a command after, to verify input is done
#[test]
fn global_interactive_delete() {
  PrintTest{
    init_buffer: vec![
      "hello",
      "1",
      "4",
      "there",
    ],
    init_clipboard: vec![],
    command_input: vec![r"G/\d/","d","/","d","/","l",],
    expected_buffer: vec![
      "hello",
      "there",
    ],
    expected_buffer_saved: false,
    expected_selection: (2,2),
    expected_clipboard: vec!["4"],
    expected_prints: vec![
      Print{
        text: vec!["1\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["4\n".to_string(),],
        n: false,
        l: false,
      },
      Print{
        text: vec!["there\n".to_string(),],
        n: false,
        l: true,
      },
    ],
    expected_history_tags: vec![r"G/\d/"],
  }.run();
}
