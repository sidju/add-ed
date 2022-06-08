use super::Buffer;

/// Big test function to validate Buffer behaviour
/// Does not test file interactions or saved tracking.
///
/// Takes an empty Buffer instance, returns it empty.
pub fn api_validation(buffer: &mut impl Buffer) {
  // Verify that the buffer is empty / len works
  assert_eq!(buffer.len(), 0,
    ".len didn't return 0 at start of api test. Please provide empty buffer instance."
  );

  // Put some test lines into the buffer
  // and verify that they came in correctly
  let data = vec![
    "1\n",
    "2\n",
    "3\n",
    "4\n",
    "5\n",
    "6\n",
    "7\n",
    "8\n",
    "9\n"
  ];
  buffer.insert(&mut data.clone().into_iter(), 0).unwrap();
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().collect();
  assert_eq!(output, data,
    ".get_selection((0,buffer.len()-1)) didn't return the data we put in."
  );
  assert_eq!(buffer.len(), data.len(),
    "Length of buffer didn't match length of inserted data."
  );

  // Test move, since we use it in tag testing
  buffer.mov((1,1), 4).unwrap();
  assert_eq!(buffer.get_selection((4,4)).unwrap().next().unwrap(), "1\n",
    "Moving forward didn't place moved line as expected. Remember that moving appends."
  );
  buffer.mov((4,4), 0).unwrap();
  assert_eq!(buffer.get_selection((1,1)).unwrap().next().unwrap(), "1\n",
    "Moving backward didn't place moved line as expected. Remember that moving appends."
  );

  // Test tagging and getting tags
  buffer.tag_line(2, 't').unwrap();
  assert_eq!(buffer.get_tag('t').unwrap(), 2,
    ".get_tag didn't return index set by .tag_line"
  );
  // Moving the line should move the tag with it
  buffer.mov((2,2), 5).unwrap();
  assert_eq!(buffer.get_tag('t').unwrap(), 5,
    ".get_tag didn't return index we moved tagged line to"
  );
  // Return the line to its original position
  buffer.mov((5,5), 1).unwrap();

  // Test get_matching with literal
  // regex support is encouraged but not required
  assert_eq!(buffer.get_matching(r"3\n", 0, false).unwrap(), 3,
    ".get_matching didn't find 3\\n at expected index. Buffer should handle \\n \\\\."
  );

  // Test copy
  buffer.mov_copy((3,3), 7).unwrap();
  assert_eq!(buffer.get_selection((8,8)).unwrap().next().unwrap(), data[2],
    "Copying forward didn't place new line as expected. Remember that copying appends."
  );

  // Test cut as a way to cleanup
  buffer.cut((8,8)).unwrap();
  // Verify the cleanup by checking the full buffer contents
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().collect();
  assert_eq!(output, data,
    "After deleting with .cut didn't get the buffer contents we expected."
  );

  // Test join
  buffer.join((3,6)).unwrap();
  assert_eq!(buffer.get_selection((3,3)).unwrap().next().unwrap(), "3456\n",
    "Joining didn't create the expected contents on the expected line."
  );

  // Test change as a way to cleanup
  buffer.change(&mut vec!["3\n","4\n","5\n","6\n"].into_iter(), (3,3)).unwrap();
  // Verify the cleanup by checking the full buffer contents
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().collect();
  assert_eq!(output, data,
    "After deleting with .cut didn't get the buffer contents we expected."
  );

  // Test that pasting after cutting puts back the cut lines
  buffer.cut((2,8)).unwrap();
  buffer.paste(1).unwrap();
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().collect();
  assert_eq!(output, data,
    "After cutting and pasting back the buffer contents had changed."
  );

  // Test copy by emulating mov_copy
  buffer.copy((3,3)).unwrap();
  buffer.paste(3).unwrap();
  assert_eq!(buffer.get_selection((3,3)).unwrap().next().unwrap(), "3\n",
    "after copying and pasting the buffer contents weren't as expected."
  );

  // Test search_replace as a way to clean up
  assert_eq!(
    buffer.search_replace((r"3\n",""), (3,3), true).unwrap(),
    2,
    "After removing line with regex selection wasn't the prior line."
  );
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().collect();
  assert_eq!(output, data,
    concat!(
      "After substituting to remove a duplicate line the buffer contents weren't as expected. ",
      "Buffer should handle \\n and \\\\."
    )
  );

  // Check that search_replace errors when it finds nothing to replace
  assert_eq!(
    buffer.search_replace((r"6\n",""), (3,3), true),
    Err(crate::error_consts::NO_MATCH),
    "If search_replace doesn't find anything to replace it should error to show this."
  );
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().collect();
  assert_eq!(output, data,
    "After search_replacing with a pattern that shouldn't match the buffer had changed."
  );
}
