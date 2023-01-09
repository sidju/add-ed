use std::collections::HashMap;
use add_ed::{
  buffer::Buffer,
  ui::DummyUI,
  Ed,
};

mod dummy_io;
use dummy_io::DummyIO;

#[test]
fn undo() {
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
        ",d\n".to_string(),
        "u\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","2\n","3\n","4\n","5\n","6\n"]
  );
}

#[test]
fn redo() {
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
        ",i\n".to_string(),
        "0\n".to_string(),
        ".\n".to_string(),
        "u\n".to_string(),
        "U\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, &mut io, "".to_string(),HashMap::new(),false,false)
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["0\n","1\n","2\n","3\n","4\n","5\n","6\n"]
  );
}
