// Real tests for the IO implementation
// Mocking these would defeat the point, so we risk side-effects instead,
// wherefore they are locked behind the features test_local_io

use super::*;
// Needed to build fake UI whose UILock we hand in to command tests
use crate::UI;

use crate::ui::dummy_ui::DummyUI;

#[test]
fn test_file_io() {
  let mut io = LocalIO::new();
  let data = vec![
    "1\n",
    "2\n",
  ];
  let path = "io_test_file";

  // Create new file
  io.write_file(
    path,
    false, // don't append
    Box::new("1\n2\n".split_inclusive('\n')).into(),
  ).unwrap();
  let read = std::fs::read_to_string(path).unwrap();
  let read: Vec<&str> = (&read).split_inclusive('\n').collect();
  assert_eq!(
    read,
    data,
    "After creating file with write_file it didn't have the expected contents."
  );
  // Overwrite the file
  io.write_file(
    path,
    false, // don't append
    Box::new("1\n2\n".split_inclusive('\n')).into(),
  ).unwrap();
  let read = std::fs::read_to_string(path).unwrap();
  let read: Vec<&str> = (&read).split_inclusive('\n').collect();
  assert_eq!(
    read,
    data,
    "After overwriting write_file the file didn't have the expected contents."
  );
  // Append to the file
  io.write_file(
    path,
    true, // Append
    Box::new("1\n2\n".split_inclusive('\n')).into(),
  ).unwrap();
  let read = std::fs::read_to_string(path).unwrap();
  let read: Vec<&str> = (&read).split_inclusive('\n').collect();
  let double_data: Vec<&str> = data.iter().chain(data.iter()).map(|x| *x).collect();
  assert_eq!(
    read,
    double_data,
    "After appending with write_file file didn't have the expected contents."
  );
  // Finally verify that read_file reads it correctly
  let read = io.read_file(
    path,
    true, // File should exist
  ).unwrap();
  let read: Vec<&str> = (&read).split_inclusive('\n').collect();
  assert_eq!(
    read,
    double_data,
    "After appending with write_file file didn't have the expected contents."
  );
  // Cleanup
  std::fs::remove_file(path).unwrap();
  // Verify must_exist part of read_file
  let res = io.read_file(path, true);
  assert!(
    res.is_err(),
    "Read file should return an error if must_exist is true and file not found.",
  );
}

#[test]
fn test_command_io() {
  let mut io = LocalIO::new();
  let mut mock_ui = DummyUI{};
  let mut mock_ui_lock = mock_ui.lock_ui();
  // Verify basic command execution via side effects
  io.run_command(
    &mut mock_ui_lock,
    "echo \"hurr\ndurr\" > io_command_test_file".to_owned(),
  ).unwrap();
  let data = io.read_file("io_command_test_file", true).unwrap();
  assert_eq!(
    &data,
    "hurr\ndurr\n",
    "Command running did not have expected effect."
  );
  // Verify that failed execution gives an error
  let res = io.run_command(
    &mut mock_ui_lock,
    "false".to_owned(),
  );
  match res {
    Ok(_) => panic!("No error returned when child process failed to run."),
    Err(e) => {
      let inner = e.downcast_ref::<LocalIOError>()
        .expect("Wrong error type returned when child process failed to run.")
      ;
      assert_eq!(
        inner,
        &super::LocalIOError::ChildReturnedError(1),
        "Wrong error when child process failed to run."
      )
    }
  };
  // Cleanup
  std::fs::remove_file("io_command_test_file").unwrap();
  // Test reading from command
  let data = io.run_read_command(
    &mut mock_ui_lock,
    "echo \"hurr\ndurr\"".to_owned(),
  ).unwrap();
  assert_eq!(
    &data,
    "hurr\ndurr\n",
    "Reading from a command did not return the expected output."
  );
  // Test writing to command
  let input = "hurr\ndurr\ndunn\n";
  let written = io.run_write_command(
    &mut mock_ui_lock,
    "cat > io_command_test_file".to_owned(),
    Box::new("hurr\ndurr\ndunn\n".split_inclusive('\n')).into(),
  ).unwrap();
  assert_eq!(
    written,
    input.len(),
    "Write command call didn't return the number of bytes in the given stream."
  );
  let data = io.read_file("io_command_test_file", true).unwrap();
  assert_eq!(
    &data,
    input,
    "Write command running did not have expected effect"
  );
  // Cleanup
  std::fs::remove_file("io_command_test_file").unwrap();
  // Test transform command
  let output = io.run_transform_command(
    &mut mock_ui_lock,
    "sort -n".to_owned(),
    Box::new("4\n5\n8\n1\n3\n2\n6\n0\n9\n7\n10\n".split_inclusive('\n')).into(),
  ).unwrap();
  assert_eq!(
    &output,
    "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n",
    "Transform command running did not have expected effect."
  );
} 
