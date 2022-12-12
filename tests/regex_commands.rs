use add_ed::{
  buffer::Buffer,
  ui::DummyUI,
  Ed,
};
use std::collections::HashMap;

mod dummy_io;
use dummy_io::DummyIO;

/// Application, file and print commands not tested here. Manual testing adviced.

#[test]
fn regex_find_line() {
  // Create the testing editor
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();

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
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
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
        "/3/".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (3,3),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
}

#[test]
fn regex_rfind_line() {
  // Create the testing editor
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();

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
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
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
        // Needed since default startup selection is 1,bufferlen
        // Forward happens to work since default index is the start of sel index
        "6".to_string(),
        "?3?".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (3,3),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
}

#[test]
fn regex_removing_line() {
  // Create the testing editor
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();

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
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
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
        "2,4s_\\d\\n__\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
    assert_eq!(
      ed.see_state().selection,
      (2,3),
      "Wrong selection after run. (note: selections are 1 indexed & inclusive)"
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","3\n","4\n","5\n","6\n"]
  );
}

#[test]
fn multiline_regex_removing_lines() {
  // Create the testing editor
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();

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
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
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
        "2,4s_\\d\\n__g\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
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
    vec!["1\n","4\n","5\n","6\n"]
  );
}

#[test]
fn regex_substitute_no_match() {
  // Create the testing editor
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();

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
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
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
        "2,4s_5__g\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    let res = ed.run_macro(&mut ui);
    assert_eq!(
      res,
      Err(add_ed::error_consts::NO_MATCH),
      "When 's' finds no match to substitute it should error, to show that."
    );
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"],
    "Initialising buffer didn't yield expected buffer contents."
  );
}

#[test]
fn regex_substitute_no_args() {
  // Create the testing editor
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();

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
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
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
        "2s_\\d_digit_g\n".to_string(),
        "3,4s\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui)
      .expect("Error running test, s without args should use prior successful call's arguments.")
    ;
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","digit\n","digit\n","digit\n","5\n","6\n"],
    "Running s without arguments should use prior successful call's arguments."
  );
}
