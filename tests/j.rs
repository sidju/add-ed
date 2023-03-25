// Tests for 'j' and 'J' command
// 'j' tests are immediately after imports
// 'J tests are after the 'j' tests

mod shared;
use shared::fixtures::{
  BasicTest,
};

// Verify behaviour of 'j' command
//
// - Takes optional selection
//   - If given joins selection into one line (remove newlines)
//   - If none given does the same on state.selection
//   - Special: If selection is one line returns error, cannot join single line.
// - Selection after execution is the resulting line after joining.
// - Clipboard after execution is the original selection before joining.

// Normal use-case, join two lines
#[test]
fn join() {
  BasicTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["2,3j"],
    expected_buffer: vec!["a","bc","d"],
    expected_buffer_saved: false,
    expected_clipboard: vec!["b","c"],
    expected_selection: (2,2),
  }.run()
}

// Test with default selection
#[test]
fn join_noselection() {
  BasicTest{
    init_buffer: vec!["a","b","c","d"],
    init_clipboard: vec![],
    command_input: vec!["j"],
    expected_buffer: vec!["abcd"],
    expected_buffer_saved: false,
    expected_clipboard: vec!["a","b","c","d"],
    expected_selection: (1,1),
  }.run()
}

// Verify behaviour of 'J'
//
// - Takes optional selection
//   - If given reflows lines to width
//   - If none given same for state.selection
// - Takes optional unsigned integer argument
//   - If given reflows to that number of columns width
//   - If none given reflows to 80 columns width
// - Selection after execution is the span of resulting lines
// - Clipboard after execution is the original selection before reflowing

// Lorem ipsum helper, since we will need a larger buffer to test reflow
fn lorem() -> Vec<&'static str> {
  vec![
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Pretium viverra suspendisse potenti nullam ac tortor vitae purus faucibus. Arcu risus quis varius quam quisque id diam. Turpis in eu mi bibendum neque. Vitae sapien pellentesque habitant morbi tristique senectus et netus et. Dui nunc mattis enim ut tellus. Feugiat vivamus at augue eget arcu dictum. Ante metus dictum at tempor commodo. Non quam lacus suspendisse faucibus interdum posuere lorem ipsum. Congue eu consequat ac felis donec et odio. Ullamcorper malesuada proin libero nunc consequat. Semper risus in hendrerit gravida rutrum quisque. Sem viverra aliquet eget sit. Mollis aliquam ut porttitor leo a diam sollicitudin tempor. Faucibus scelerisque eleifend donec pretium. Vulputate odio ut enim blandit volutpat. Adipiscing elit duis tristique sollicitudin. Pretium viverra suspendisse potenti nullam ac tortor vitae purus faucibus. Nibh sed pulvinar proin gravida. Vitae justo eget magna fermentum iaculis eu non.",
    "Diam donec adipiscing tristique risus nec feugiat in fermentum posuere. Feugiat sed lectus vestibulum mattis ullamcorper velit sed. In hac habitasse platea dictumst vestibulum. Amet volutpat consequat mauris nunc congue nisi vitae suscipit tellus. Cursus metus aliquam eleifend mi in nulla posuere. Adipiscing at in tellus integer. Ultricies lacus sed turpis tincidunt id aliquet risus feugiat. Eu tincidunt tortor aliquam nulla facilisi cras fermentum odio. Vitae proin sagittis nisl rhoncus mattis rhoncus urna neque viverra. Scelerisque viverra mauris in aliquam sem fringilla ut morbi tincidunt. Pellentesque habitant morbi tristique senectus et. Risus sed vulputate odio ut enim blandit volutpat maecenas.",
  ]
}

// Test default
#[test]
fn reflow_noselection_nowidth() {
  BasicTest{
    init_buffer: lorem(),
    init_clipboard: vec![],
    command_input: vec!["J"],
    expected_buffer: vec![
      "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor",
      "incididunt ut labore et dolore magna aliqua. Pretium viverra suspendisse potenti",
      "nullam ac tortor vitae purus faucibus. Arcu risus quis varius quam quisque id",
      "diam. Turpis in eu mi bibendum neque. Vitae sapien pellentesque habitant morbi",
      "tristique senectus et netus et. Dui nunc mattis enim ut tellus. Feugiat vivamus",
      "at augue eget arcu dictum. Ante metus dictum at tempor commodo. Non quam lacus",
      "suspendisse faucibus interdum posuere lorem ipsum. Congue eu consequat ac felis",
      "donec et odio. Ullamcorper malesuada proin libero nunc consequat. Semper risus",
      "in hendrerit gravida rutrum quisque. Sem viverra aliquet eget sit. Mollis",
      "aliquam ut porttitor leo a diam sollicitudin tempor. Faucibus scelerisque",
      "eleifend donec pretium. Vulputate odio ut enim blandit volutpat. Adipiscing elit",
      "duis tristique sollicitudin. Pretium viverra suspendisse potenti nullam ac",
      "tortor vitae purus faucibus. Nibh sed pulvinar proin gravida. Vitae justo eget",
      "magna fermentum iaculis eu non. Diam donec adipiscing tristique risus nec",
      "feugiat in fermentum posuere. Feugiat sed lectus vestibulum mattis ullamcorper",
      "velit sed. In hac habitasse platea dictumst vestibulum. Amet volutpat consequat",
      "mauris nunc congue nisi vitae suscipit tellus. Cursus metus aliquam eleifend mi",
      "in nulla posuere. Adipiscing at in tellus integer. Ultricies lacus sed turpis",
      "tincidunt id aliquet risus feugiat. Eu tincidunt tortor aliquam nulla facilisi",
      "cras fermentum odio. Vitae proin sagittis nisl rhoncus mattis rhoncus urna neque",
      "viverra. Scelerisque viverra mauris in aliquam sem fringilla ut morbi tincidunt.",
      "Pellentesque habitant morbi tristique senectus et. Risus sed vulputate odio ut",
      "enim blandit volutpat maecenas.",
    ],
    expected_buffer_saved: false,
    expected_clipboard: lorem(),
    expected_selection: (1,23),
  }.run()
}

// Test fully defined
#[test]
fn reflow() {
  let mut expected_clipboard = lorem();
  expected_clipboard.remove(1);
  BasicTest{
    init_buffer: lorem(),
    init_clipboard: vec![],
    command_input: vec!["1J70"],
    expected_buffer: vec![
      "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do",
      "eiusmod tempor incididunt ut labore et dolore magna aliqua. Pretium",
      "viverra suspendisse potenti nullam ac tortor vitae purus faucibus.",
      "Arcu risus quis varius quam quisque id diam. Turpis in eu mi bibendum",
      "neque. Vitae sapien pellentesque habitant morbi tristique senectus et",
      "netus et. Dui nunc mattis enim ut tellus. Feugiat vivamus at augue",
      "eget arcu dictum. Ante metus dictum at tempor commodo. Non quam lacus",
      "suspendisse faucibus interdum posuere lorem ipsum. Congue eu consequat",
      "ac felis donec et odio. Ullamcorper malesuada proin libero nunc",
      "consequat. Semper risus in hendrerit gravida rutrum quisque. Sem",
      "viverra aliquet eget sit. Mollis aliquam ut porttitor leo a diam",
      "sollicitudin tempor. Faucibus scelerisque eleifend donec pretium.",
      "Vulputate odio ut enim blandit volutpat. Adipiscing elit duis",
      "tristique sollicitudin. Pretium viverra suspendisse potenti nullam ac",
      "tortor vitae purus faucibus. Nibh sed pulvinar proin gravida. Vitae",
      "justo eget magna fermentum iaculis eu non.",
      "Diam donec adipiscing tristique risus nec feugiat in fermentum posuere. Feugiat sed lectus vestibulum mattis ullamcorper velit sed. In hac habitasse platea dictumst vestibulum. Amet volutpat consequat mauris nunc congue nisi vitae suscipit tellus. Cursus metus aliquam eleifend mi in nulla posuere. Adipiscing at in tellus integer. Ultricies lacus sed turpis tincidunt id aliquet risus feugiat. Eu tincidunt tortor aliquam nulla facilisi cras fermentum odio. Vitae proin sagittis nisl rhoncus mattis rhoncus urna neque viverra. Scelerisque viverra mauris in aliquam sem fringilla ut morbi tincidunt. Pellentesque habitant morbi tristique senectus et. Risus sed vulputate odio ut enim blandit volutpat maecenas.",
    ],
    expected_buffer_saved: false,
    expected_clipboard: expected_clipboard,
    expected_selection: (1,16),
  }.run()
}
