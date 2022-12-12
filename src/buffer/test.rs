use super::Buffer;

#[test]
fn validate() {
  let mut buf = Buffer::new();
  api_validation(&mut buf);
}

/// Big test function to validate Buffer behaviour
/// Does not test file interactions or saved tracking.
///
/// Takes an empty Buffer instance, returns it empty.
pub fn api_validation(buffer: &mut Buffer) {
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
  buffer.insert(data.clone(), 0).unwrap();
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect();
  assert_eq!(output, data,
    ".get_selection((1,buffer.len())) didn't return the data we put in."
  );
  assert_eq!(buffer.len(), data.len(),
    "Length of buffer didn't match length of inserted data."
  );

  // Test move, since we use it in tag testing
  buffer.mov((1,1), 4).unwrap();
  assert_eq!(buffer.get_selection((4,4)).unwrap().next().unwrap().1, "1\n",
    "Moving forward didn't place moved line as expected. Remember that moving appends."
  );
  buffer.mov((4,4), 0).unwrap();
  assert_eq!(buffer.get_selection((1,1)).unwrap().next().unwrap().1, "1\n",
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
  assert_eq!(buffer.get_selection((8,8)).unwrap().next().unwrap().1, data[2],
    "Copying forward didn't place new line as expected. Remember that copying appends."
  );

  // Test cut as a way to cleanup
  buffer.cut((8,8)).unwrap();
  // Verify the cleanup by checking the full buffer contents
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect();
  assert_eq!(output, data,
    "After deleting with .cut didn't get the buffer contents we expected."
  );

  // Test join
  buffer.join((3,6)).unwrap();
  assert_eq!(buffer.get_selection((3,3)).unwrap().next().unwrap().1, "3456\n",
    "Joining didn't create the expected contents on the expected line."
  );

  // Test change as a way to cleanup
  buffer.change(vec!["3\n","4\n","5\n","6\n"], (3,3)).unwrap();
  // Verify the cleanup by checking the full buffer contents
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect();
  assert_eq!(output, data,
    "After deleting with .cut didn't get the buffer contents we expected."
  );

  // Test that pasting after cutting puts back the cut lines
  buffer.cut((2,8)).unwrap();
  buffer.paste(1).unwrap();
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect();
  assert_eq!(output, data,
    "After cutting and pasting back the buffer contents had changed."
  );

  // Test copy by emulating mov_copy
  buffer.copy((3,3)).unwrap();
  buffer.paste(3).unwrap();
  assert_eq!(buffer.get_selection((3,3)).unwrap().next().unwrap().1, "3\n",
    "after copying and pasting the buffer contents weren't as expected."
  );

  // Test search_replace as a way to clean up
  assert_eq!(
    buffer.search_replace((r"3\n",""), (4,5), true).unwrap(),
    4,
    "Regex cannot remove all lines in selection, selection should only shrink."
  );
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect();
  assert_eq!(output, data,
    concat!(
      "After substituting to remove a duplicate line the buffer contents weren't as expected. ",
      "Buffer should handle \\n and \\\\."
    )
  );

  // Check that search_replace errors when it finds nothing to replace
  // Also checks that last newline in selection isn't match/replace-able
  assert_eq!(
    buffer.search_replace((r".*\n",""), (3,3), true),
    Err(crate::error_consts::NO_MATCH),
    "If search_replace doesn't find anything to replace it should error to show this."
  );
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect();
  assert_eq!(output, data,
    "After search_replacing with a pattern that shouldn't match the buffer had changed."
  );

  // Test undo redo
  assert_eq!(
    buffer.undo_range().unwrap(),
    0..1,
    "Before creating undo checkpoints the undo range should be 0..1."
  );
  buffer.snapshot().unwrap();
  buffer.cut((1,buffer.len())).unwrap();
  assert_eq!(
    buffer.len(),
    0,
    "After deleting whole buffer no data should remain."
  );
  assert_eq!(
    buffer.undo_range().unwrap(),
    0..2,
    "After creating a checkpoint the undo range should be 0..2."
  );
  buffer.undo(1).unwrap();
  assert_eq!(
    buffer.undo_range().unwrap(),
    -1..1,
    "After undoing x checkpoints range should begin at -x."
  );
  let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect();
  assert_eq!(output, data,
    "After undoing to a snapshot the state of the buffer should be same as before."
  );

  // Test text wrapping
  // Requires different buffer contents, snapshot/undo to reset test data after
  buffer.snapshot().unwrap();
  {
    let lorem = vec![
      "Verå et quia ad repellendus. Voluptas debitis id consequatur doloremque sed suscipit et tempora. Odit sed est hic non error. Sint itaque et ut alias voluptatem sit. Et sunt totam amet doloribus unde nam velit voluptatem. Odit nisi ut eius et temporibus et.\n",
      "Enim asperiores sit åt et fugit omnis. Quos tenetur cupiditate velit excepturi est autem dolor. Est earum quidem dolorem. Adipisci earum vero ab enim. Qui rerum sit illum esse deserunt.\n",
      "Eos voluptatem vel corrupti reprehenderit. Voluptas quisquam fuga esse tenetur nesciunt sit corrupti. Odio corporis rerum est sed. Dicta ipsam modi minus voluptas.\n"
    ];
    buffer.change(lorem.clone(), (1,buffer.len())).unwrap();
    let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect();
    assert_eq!(
      lorem, output,
      "Changing buffer contents to lorem ipsum failed."
    );
    let end = buffer.reflow((2,buffer.len()), 55).unwrap();
    assert_eq!(
      end, 8,
      "Returned new end of selection from buffer.reflow is wrong."
    );
    let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect();
    assert_eq!(
      output, vec![
        "Verå et quia ad repellendus. Voluptas debitis id consequatur doloremque sed suscipit et tempora. Odit sed est hic non error. Sint itaque et ut alias voluptatem sit. Et sunt totam amet doloribus unde nam velit voluptatem. Odit nisi ut eius et temporibus et.\n",
        "Enim asperiores sit åt et fugit omnis. Quos tenetur\n",
        "cupiditate velit excepturi est autem dolor. Est earum\n",
        "quidem dolorem. Adipisci earum vero ab enim. Qui rerum\n",
        "sit illum esse deserunt. Eos voluptatem vel corrupti\n",
        "reprehenderit. Voluptas quisquam fuga esse tenetur\n",
        "nesciunt sit corrupti. Odio corporis rerum est sed.\n",
        "Dicta ipsam modi minus voluptas.\n",
      ],
      "Buffer contents after reflow weren't as expected."
    );
    buffer.undo(1).unwrap();
    let output: Vec<&str> = buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect();
    assert_eq!(output, data,
      "After undoing to a snapshot the state of the buffer should be same as before."
    );
  }
}
