use std::collections::HashMap;
use add_ed::{
  buffer::{
    VecBuffer,
    Buffer,
  },
  ui::DummyUI,
  Ed,
};

#[test]
fn reflow() {
  // Create the testing editor
  let mut buffer = VecBuffer::new();

  {
    let mut ui = DummyUI{
      input: vec![
        // Create initial buffer contents
        ",a\n".to_string(),
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam ac urna sit amet enim elementum efficitur non eget lorem. Quisque orci enim, gravida in lorem nec, varius porta velit. Donec\n".to_string(),
        "at mollis urna. Curabitur cursus lectus in maximus accumsan. Cras et luctus diam. Vivamus tristique, sem vitae condimentum euismod, tellus lectus tincidunt sem, faucibus tempor dolor ante vitae ante.\n".to_string(),
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
    vec!["Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam ac urna sit amet enim elementum efficitur non eget lorem. Quisque orci enim, gravida in lorem nec, varius porta velit. Donec\n","at mollis urna. Curabitur cursus lectus in maximus accumsan. Cras et luctus diam. Vivamus tristique, sem vitae condimentum euismod, tellus lectus tincidunt sem, faucibus tempor dolor ante vitae ante.\n",],
    "Initialising buffer didn't yield expected buffer contents."
  );

  {
    let mut ui = DummyUI{
      input: vec![
        ",J".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),HashMap::new(),false,false)
      .expect("Failed to open no file. Should be noop.")
    ;
    ed.run_macro(&mut ui).expect("Error running test.");
  }
  assert_eq!(
    buffer.get_selection((1,buffer.len())).unwrap().map(|(_,s)|s).collect::<Vec<&str>>(),
    vec![
      "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam ac urna sit\n",
      "amet enim elementum efficitur non eget lorem. Quisque orci enim, gravida in\n",
      "lorem nec, varius porta velit. Donec at mollis urna. Curabitur cursus lectus in\n",
      "maximus accumsan. Cras et luctus diam. Vivamus tristique, sem vitae condimentum\n",
      "euismod, tellus lectus tincidunt sem, faucibus tempor dolor ante vitae ante.\n",
    ]
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
        "2,4j\n".to_string(),
      ].into(),
      print_ui: None,
    };
    let mut ed = Ed::new(&mut buffer, "".to_string(),HashMap::new(),false,false)
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
