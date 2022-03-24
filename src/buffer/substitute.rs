//! Escape sequence interpreter to aid in buffer implementation

/// Takes input str, returns string with escapes interpreted
pub fn substitute(input: &str) -> String {
  let mut out = String::new();
  let mut escaped = false;

  // Iterate over the string and replace where matching
  for ch in input.chars() {
    // If escaped check for special characters
    if escaped {
      match ch {
        '\\' => out.push('\\'),
        'n' => out.push('\n'),
        'r' => out.push('\r'),
        't' => out.push('\t'),
        c => {
          // If no special, insert the false
          // escape and the character after
          out.push('\\');
          out.push(c);
        },
      }
      escaped = false;
    }
    // If not escaped check if is escaping
    else {
      if ch == '\\' {
        escaped = true;
      }
      else {
        out.push(ch);
      }
    }
  }
  out
}

#[cfg(test)]
mod test {
  use super::substitute;
  #[test]
  fn test_backslash_escape() {
    // Double slash should be reduced into single slash,
    // since otherwise there is no way to write a single slash
    let input = r"\\";
    let output = substitute(input);
    assert_eq!(r"\", &output);
    // Double slash should escape the slash and cause it to not escape into newline
    let input = r"\\n";
    let output = substitute(input);
    assert_eq!(r"\n", &output); 
  }
}
