use add_ed::{
  buffer::{
    VecBuffer,
    Buffer,
  },
  ui::DummyUI,
  Ed,
};

/// Application, file and print commands not tested here. Manual testing adviced.

// a is tested when initialising all other tests, therefor has no separate test.

#[test]
fn insert() {
  // Create the testing editor
  let mut buffer = VecBuffer::new();

  {
    let mut ui = DummyUI{
      input: vec![
        // Create initial buffer contents
        ",a\n".to_string(),
        "1\n".to_string(),
        "2\n".to_string(),
        "3\n".to_string(),
        "4\n".to_string(),
        "5\n".to_string(),
        "6\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error creating initial buffer contents.");
    assert_eq!(
      ed.see_state().selection,
      (1,6),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"],
    "Initialising buffer didn't yield expected buffer contents."
  );

  {
    let mut ui = DummyUI{
      input: vec![
        "2,4i\n".to_string(), // ,4 should be ignored
        "1.5\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (2,2),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","1.5\n","2\n","3\n","4\n","5\n","6\n"],
    "Wrong buffer state after run."
  );
}

#[test]
fn change_and_paste() {
  // Create the testing editor
  let mut buffer = VecBuffer::new();

  {
    let mut ui = DummyUI{
      input: vec![
        // Create initial buffer contents
        ",a\n".to_string(),
        "1\n".to_string(),
        "2\n".to_string(),
        "3\n".to_string(),
        "4\n".to_string(),
        "5\n".to_string(),
        "6\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error creating initial buffer contents.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"],
    "Initialising buffer didn't yield expected buffer contents."
  );

  {
    let mut ui = DummyUI{
      input: vec![
        "2,4c\n".to_string(),
        "3\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (2,2),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
    // Also paste right before new line
    let mut ui = DummyUI{
      input: vec![
        "2X".to_string(),
      ].into(),
      print_ui: None,
    };
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (2,4),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","3\n","5\n","6\n"]
  );
}
#[test]
fn delete_and_paste() {
  // Create the testing editor
  let mut buffer = VecBuffer::new();

  {
    let mut ui = DummyUI{
      input: vec![
        // Create initial buffer contents
        ",a\n".to_string(),
        "1\n".to_string(),
        "2\n".to_string(),
        "3\n".to_string(),
        "4\n".to_string(),
        "5\n".to_string(),
        "6\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error creating initial buffer contents.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"],
    "Initialising buffer didn't yield expected buffer contents."
  );

  {
    let mut ui = DummyUI{
      input: vec![
        "2,4d\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (1,1),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
    // Also paste at end of buffer
    let mut ui = DummyUI{
      input: vec![
        "$x".to_string(),
      ].into(),
      print_ui: None,
    };
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (4,6),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","5\n","6\n","2\n","3\n","4\n"]
  );
}
#[test]
fn copy_and_paste() {
  // Create the testing editor
  let mut buffer = VecBuffer::new();

  {
    let mut ui = DummyUI{
      input: vec![
        // Create initial buffer contents
        ",a\n".to_string(),
        "1\n".to_string(),
        "2\n".to_string(),
        "3\n".to_string(),
        "4\n".to_string(),
        "5\n".to_string(),
        "6\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error creating initial buffer contents.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"],
    "Initialising buffer didn't yield expected buffer contents."
  );

  {
    let mut ui = DummyUI{
      input: vec![
        "2,4y\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (2,4),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
    // Also paste at end of buffer
    let mut ui = DummyUI{
      input: vec![
        "$x".to_string(),
      ].into(),
      print_ui: None,
    };
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (7,9),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n","2\n","3\n","4\n"]
  );
}
#[test]
fn mov_copy() {
  // Create the testing editor
  let mut buffer = VecBuffer::new();

  {
    let mut ui = DummyUI{
      input: vec![
        // Create initial buffer contents
        ",a\n".to_string(),
        "1\n".to_string(),
        "2\n".to_string(),
        "3\n".to_string(),
        "4\n".to_string(),
        "5\n".to_string(),
        "6\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error creating initial buffer contents.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"],
    "Initialising buffer didn't yield expected buffer contents."
  );

  {
    let mut ui = DummyUI{
      input: vec![
        "2,4t6\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (7,9),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }

  {
    let mut ui = DummyUI{
      input: vec![
        "7,9t0\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (1,3),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["2\n","3\n","4\n","1\n","2\n","3\n","4\n","5\n","6\n","2\n","3\n","4\n"]
  );
}

#[test]
fn mov() {
  // Create the testing editor
  let mut buffer = VecBuffer::new();

  {
    let mut ui = DummyUI{
      input: vec![
        // Create initial buffer contents
        ",a\n".to_string(),
        "1\n".to_string(),
        "2\n".to_string(),
        "3\n".to_string(),
        "4\n".to_string(),
        "5\n".to_string(),
        "6\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error creating initial buffer contents.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"],
    "Initialising buffer didn't yield expected buffer contents."
  );

  {
    let mut ui = DummyUI{
      input: vec![
        "2,3m5\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (4,5),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","4\n","5\n","2\n","3\n","6\n"],
    "Moving forward didn't yield expected result."
  );

  {
    let mut ui = DummyUI{
      input: vec![
        "3,4m0\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (1,2),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["5\n","2\n","1\n","4\n","3\n","6\n"],
    "Moving backwards didn't yield expected result."
  );
}

#[test]
fn join() {
  // Create the testing editor
  let mut buffer = VecBuffer::new();

  {
    let mut ui = DummyUI{
      input: vec![
        // Create initial buffer contents
        ",a\n".to_string(),
        "1\n".to_string(),
        "2\n".to_string(),
        "3\n".to_string(),
        "4\n".to_string(),
        "5\n".to_string(),
        "6\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error creating initial buffer contents.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"],
    "Initialising buffer didn't yield expected buffer contents."
  );

  {
    let mut ui = DummyUI{
      input: vec![
        "2,4j\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (2,2),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","234\n","5\n","6\n"]
  );
}

#[test]
fn insert_inline() {
  // Create the testing editor
  let mut buffer = VecBuffer::new();

  {
    let mut ui = DummyUI{
      input: vec![
        // Create initial buffer contents
        ",a\n".to_string(),
        "1\n".to_string(),
        "2\n".to_string(),
        "3\n".to_string(),
        "4\n".to_string(),
        "5\n".to_string(),
        "6\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error creating initial buffer contents.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"],
    "Initialising buffer didn't yield expected buffer contents."
  );

  {
    let mut ui = DummyUI{
      input: vec![
        "2,4I\n".to_string(), // ,4 should be ignored
        "5.\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (2,2),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","5.2\n","3\n","4\n","5\n","6\n"],
    "Wrong buffer state after run."
  );
}

#[test]
fn append_inline() {
  // Create the testing editor
  let mut buffer = VecBuffer::new();

  {
    let mut ui = DummyUI{
      input: vec![
        // Create initial buffer contents
        ",a\n".to_string(),
        "1\n".to_string(),
        "2\n".to_string(),
        "3\n".to_string(),
        "4\n".to_string(),
        "5\n".to_string(),
        "6\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error creating initial buffer contents.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"],
    "Initialising buffer didn't yield expected buffer contents."
  );

  {
    let mut ui = DummyUI{
      input: vec![
        "1,2A\n".to_string(), // 1, should be ignored
        ".5\n".to_string(),
        ".\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (2,2),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2.5\n","3\n","4\n","5\n","6\n"],
    "Wrong buffer state after run."
  );
}
