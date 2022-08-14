use add_ed::{
  buffer::{
    VecBuffer,
    Buffer,
  },
  ui::DummyUI,
  Ed,
};
use std::collections::HashMap;

#[test]
fn test_name() {
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
    let mut ed = Ed::new(&mut buffer, "".to_string(),HashMap::new(),false,false)
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
        "2,3:test\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut macros = HashMap::new();
    macros.insert("test".to_string(), "i\na\n.".to_string());
    let mut ed = Ed::new(&mut buffer, "".to_string(),macros,false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec!["1\n","a\n","2\n","3\n","4\n","5\n","6\n"]
  );
}
