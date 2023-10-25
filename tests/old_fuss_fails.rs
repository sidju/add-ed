mod shared;
use shared::fixtures::{
  ErrorTest,
};
use add_ed::EdError;

#[test]
fn unrelated_indice_with_unicode_tag() {
  ErrorTest{
    init_buffer: vec![],
    command_input: vec!["'''🏷 G"],
    expected_error: EdError::IndicesUnrelated{
      prior_index: "''".to_owned(),
      unrelated_index: "'🏷".to_owned(),
    },
  }.run()
}

#[test]
fn index_overflow() {
  ErrorTest{
    init_buffer: vec![],
    command_input: vec!["9999999999999999999+9999999999999999999"],
    expected_error: EdError::IndexTooBig{
      index: 18_446_744_073_709_551_615,
      buffer_len: 0,
    },
  }.run()
}
