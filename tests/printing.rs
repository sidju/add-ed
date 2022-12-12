use add_ed::{
  buffer::Buffer,
  ui::DummyUI,
  Ed,
};
use std::collections::HashMap;

mod dummy_io;
use dummy_io::DummyIO;

mod aggregator_ui;
use aggregator_ui::AggregatorUI;

#[test]
fn print() {
  // Create the testing editor
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();
  let mut aggregator = AggregatorUI{ prints_history: Vec::new() };

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
        ",p\n".to_string(),
      ].into(),
      print_ui: Some(&mut aggregator),
    };
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
  assert_eq!(
    aggregator.prints_history[0].text.as_ref(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
  assert_eq!(
    aggregator.prints_history[0].n,
    false
  );
  assert_eq!(
    aggregator.prints_history[0].l,
    false
  );
}

#[test]
fn printing_flags() {
  // Create the testing editor
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();
  let mut aggregator = AggregatorUI{ prints_history: Vec::new() };

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
        ",n\n".to_string(),
        ",l\n".to_string(),
        ",nl\n".to_string(),
      ].into(),
      print_ui: Some(&mut aggregator),
    };
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
  assert_eq!(
    aggregator.prints_history[0].text.as_ref(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
  assert_eq!(
    aggregator.prints_history[1].text.as_ref(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
  assert_eq!(
    aggregator.prints_history[2].text.as_ref(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
  assert_eq!(
    aggregator.prints_history[0].n,
    true
  );
  assert_eq!(
    aggregator.prints_history[0].l,
    false
  );
  assert_eq!(
    aggregator.prints_history[1].n,
    false
  );
  assert_eq!(
    aggregator.prints_history[1].l,
    true
  );
  assert_eq!(
    aggregator.prints_history[2].n,
    true
  );
  assert_eq!(
    aggregator.prints_history[2].l,
    true
  );
}

#[test]
fn change_printing_defaults() {
  // Create the testing editor
  let mut io = DummyIO::new();
  let mut buffer = Buffer::new();
  let mut aggregator = AggregatorUI{ prints_history: Vec::new() };

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
        "L".to_string(),
        ",p".to_string(),
        "N".to_string(),
        "p".to_string(),
        "L".to_string(),
        "p".to_string(),
      ].into(),
      print_ui: Some(&mut aggregator),
    };
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
  assert_eq!(
    aggregator.prints_history[0].text.as_ref(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
  assert_eq!(
    aggregator.prints_history[0].n,
    false
  );
  assert_eq!(
    aggregator.prints_history[0].l,
    true
  );
  assert_eq!(
    aggregator.prints_history[1].text.as_ref(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
  assert_eq!(
    aggregator.prints_history[1].n,
    true
  );
  assert_eq!(
    aggregator.prints_history[1].l,
    true
  );
  assert_eq!(
    aggregator.prints_history[2].text.as_ref(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
  assert_eq!(
    aggregator.prints_history[2].n,
    true
  );
  assert_eq!(
    aggregator.prints_history[2].l,
    false
  );
}
