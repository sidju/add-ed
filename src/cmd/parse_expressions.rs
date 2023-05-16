use crate::error::*;

/// Splits string into strings separated by the first character in given string
///
/// Handles backslash escaping the separator to not split on it,
/// but does not convert r"\\" into r"\".
pub fn parse_expressions(input: &str) -> Result<Vec<String>> {
  let separator = match input.chars().next() {
    Some(ch) => ch,
    None => return Ok(Vec::new()),
  };
  // Do a bit of fancy stuff to allow escaping the separator
  // Assumes that the underlying system converts r"\\" into r"\"
  let mut expressions = Vec::new();
  let mut partial: Option<String> = None;
  for chunk in input[separator.len_utf8()..].split(separator) {
    // Some fancy code to handle escaping in case chunk ends with \\\
    // First get the coordinate of the last non '\\' char, if any
    let last_non_escape = chunk
      .rfind(|c| c!= '\\')
      .map(|last_i|
        last_i +
        // Handle if last non-escape character is more than one byte long
        // Next can be unwrapped since execution of map implies its existence
        chunk[last_i..].chars().next().unwrap().len_utf8() - 1
      )
    ;
    let nr_of_escapes = match last_non_escape {
      // If none, number of '\\' is chunk.len()
      None => chunk.len(),
      // If some(x), number of '\\' is chunk.len() - (non-slash index + 1)
      Some(x) => chunk.len() - x - 1,
    };
    // If nr of escapes isn't divisible by two the separator is escaped
    if nr_of_escapes % 2 == 1 {
      // If separator is escaped we hold the chunk in partial
      // (after replacing escaped separator with separator)
      let mut tmp = match partial {
        // Safe to slice with -1 here, since the if above only passes if the
        // last character is '\\'
        Some(mut x) => { x.push_str(&chunk[..chunk.len() - 1]); x },
        None => chunk[..chunk.len() - 1].to_string(),
      };
      tmp.push(separator);
      partial = Some(tmp);
    }
    else {
      match partial {
        None => {
          expressions.push(chunk.to_string());
        },
        Some(mut x) => {
          x.push_str(chunk);
          expressions.push(x);
          partial = None;
        },
      }
    }
  }
  if partial.is_some() { 
    Err(EdError::EscapedArgumentListEnd(input.to_owned()))
  }
  else {
    Ok(expressions)
  }
}
