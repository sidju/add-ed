/// A simple UI based on the original ED editor
use add_ed::EdState;
use add_ed::ui::UI;
use add_ed::error_consts::*;
struct ClassicUI{}
impl UI for ClassicUI {
    fn print(
    &mut self,
    _ed: EdState,
    s: &str
  ) -> Result<(), &'static str> {
    println!("{}", s);
    Ok(())
  }
  fn get_command(
    &mut self,
    _ed: EdState,
    prefix: Option<char>,
  ) -> Result<String, &'static str> {
    if let Some(pre) = prefix {
      print!("{}", pre);
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
      .map_err(|_| TERMINAL_READ)?;
    Ok(input)
  }
  fn get_input(
    &mut self,
    _ed: EdState,
    terminator: char
  ) -> Result<Vec<String>, &'static str> {
    let mut input = Vec::new();
    let stdin = std::io::stdin();
    let terminator = format!("{}\n", terminator);
    loop {
      let mut buf = String::new();
      let res = stdin.read_line(&mut buf);
      if res.is_err() {
        return Err(TERMINAL_READ);
      }
      if buf == terminator { return Ok(input); }
      else { input.push(buf); }
    }
  }
  fn print_selection(
    &mut self,
    ed: EdState,
    selection: (usize, usize),
    numbered: bool,
    literal: bool,
  ) -> Result<(), &'static str> {
    let selected = ed.buffer.get_selection(selection)?;
    let mut line_nr = selection.0;
    for line in selected {
      if numbered {
        line_nr += 1;
        print!("{}: ", line_nr);
      }
      for ch in line.chars() {
        match ch {
          '\n' => {
            if literal { print!("$\n") } else { print!("\n") }
          },
          '$' => {
            if literal { print!("\\$") } else { print!("$") }
          },
          c => print!("{}", c),
        }
      }
    }
    Ok(())
  }
}

fn main() {
  // Here one should add command line argument parsing, to get the filename
  let path = "".to_string();
  let mut ui = ClassicUI{};
  let mut buffer = add_ed::buffer::VecBuffer::new();
  // Read in the file given and instantiate the editor
  let mut ed = add_ed::Ed::new(&mut buffer, path).expect("Failed to open file.");
  // Run the editor with the created UI
  ed.run(&mut ui).unwrap();
}
