/// Since default selections vary between commands and access to the
/// buffer is needed for realisation we parse into an intermediate
/// struct which is then interpreted using additional data.

use crate::error_consts::*;

use super::Buffer;

// A struct to formalise all the kinds of indices
#[derive(PartialEq, Debug)]
pub enum Ind <'a> {
  Selection,
  BufferLen,
  Literal(usize),
  Tag(char),
  Pattern(&'a str),
  RevPattern(&'a str),
  Add(Box<Ind<'a>>, usize),
  Sub(Box<Ind<'a>>, usize),
}

pub enum Sel <'a> {
  Pair(Ind<'a>, Ind<'a>),
  Lone(Ind<'a>)
}

enum State {
  Default(usize),
  Tag,
  Pattern(usize),
  RevPattern(usize),
  Offset(usize, bool),
}

pub fn parse_index<'a> (
  input: &'a str,
) -> Result<(usize, Option<Ind<'a>>), &'static str> {
  // Set up state variables for one-pass parse
  let mut i = 0;
  let mut state = State::Default(0);
  let mut current_ind = None;
  // Loop over chars and parse
  let mut iter = input.char_indices().peekable();
  while let Some((index, ch)) = iter.next() {
    // Save over index into i
    i = index;
    // Handle based on state
    match state {
      // If a state change is coming, populate current ind and make the change
      State::Default(start) => {
        match ch {
          '/' | '\'' | '?' => {
            // These are only valid at the start of an index
            if start != i { return Err(INDEX_PARSE); }
            // Since prior tags or patterns reset start, check that current_ind is none
            if current_ind.is_some() { return Err(INDEX_PARSE); }
            match ch {
              '\'' => {
                state = State::Tag;
              },
              '/' => {
                state = State::Pattern(i + 1); // Since we know the length of these chars to be one byte
              },
              '?' => {
                state = State::RevPattern(i + 1); // Since we know the length of these chars to be one byte
              },
              '.' => {
                current_ind = Some(Ind::Selection);
                state = State::Default(i + 1); // reset start after moving into current_ind
              },
              '$' => {
                current_ind = Some(Ind::BufferLen);
                state = State::Default(i + 1); // reset start after moving into current_ind
              },
              _ => panic!("Unreachable"),
            }
          }
          '+' => { state = State::Offset(i, false); },
          '-' => { state = State::Offset(i, true); },
          _ => {
            // If not numeric (base 10) it must be the end of the index
            // Break the loop and handle the last outside
            if ! ch.is_digit(10) {
              break;
            }
          },
        }
      },
      // If the tag state was entered, save the next char as tag and return to default
      State::Tag => {
        current_ind = Some(Ind::Tag(ch));
        state = State::Default( i + ch.len_utf8() );
      },
      // If the pattern state was entered, save as pattern until end char is given and return to default
      State::Pattern(start) => {
        if ch == '/' {
          current_ind = Some(Ind::Pattern(&input[start .. i]));
          state = State::Default( i + ch.len_utf8() );
        }
      },
      // Same as pattern with different end char
      State::RevPattern(start) => {
        if ch == '?' {
          current_ind = Some(Ind::RevPattern(&input[start .. i]));
          state = State::Default( i + ch.len_utf8() );
        }
      },
      // For the Add and Sub cases we need to check for separator or command to know when to stop
      // This means we also need to recognize separators or commands as Default would
      // We do this by returning to default when we find a non-numeric character
      State::Offset(start, negative) => {
        // Offset only takes numeric input, so at end of numeric input we hand back over to Default
        // This requires peeking at the next char
        let is_end = match iter.peek() {
          None => true,
          Some((_index, cha)) => {
            ! cha.is_digit(10)
          },
        };
        if is_end {
          // Parse our offset and put it together in current_ind
          let offset = if start != i {
            input[start .. i].parse::<usize>().map_err(|_| INDEX_PARSE)?
          } else {
            1
          };
          current_ind = Some( if negative {
            Ind::Sub(Box::new(current_ind.unwrap_or(Ind::Selection)), offset)
          } else {
            Ind::Add(Box::new(current_ind.unwrap_or(Ind::Selection)), offset)
          });
          // Then return to default state
          state = State::Default( i + ch.len_utf8() );
        }
      },
    } // End of match
  } // End of for-each

  // When we get here we have either gone through the whole buffer or
  // found a command/separator that marks the end of this index

  // Check the state
  match state {
    // If the string ends in Default mode its contents should be sane 
    State::Default(start) => {
      if start != i {
        // If there is both a current ind and a numeral literal it is error
        if current_ind.is_some() { Err(INDEX_PARSE) }
        // Else we parse the literal and return it
        else {
          let literal = input[start .. i].parse::<usize>().map_err(|_| INDEX_PARSE)?;
          Ok((i, Some(Ind::Literal(literal))))
        }
      }
      // If there is no literal we return current_ind as-is, since None is the correct return if nothing was parsed
      else {
        Ok((i, current_ind))
      }
    },
    // If the string ended abruptly in offset mode the contents may be usable
    State::Offset(start, negative) => {
      let offset = if start != i {
        input[start .. i].parse::<usize>().map_err(|_| INDEX_PARSE)?
      } else {
        1
      };
      Ok((i, Some(
        if negative {
          Ind::Sub(Box::new(current_ind.unwrap_or(Ind::Selection)), offset)
        } else {
          Ind::Add(Box::new(current_ind.unwrap_or(Ind::Selection)), offset)
        }
      )))
    },
    // If we get here in a state that isn't terminated (returned to Default) there is an error
    _ => {
      Err(INDEX_PARSE)
    },
  }
}

pub fn parse_selection<'a>(
  input: &'a str,
) -> Result<(usize, Option<Sel<'a>>), &'static str> {
  // First parse, getting an index and the offset it stopped parsing at
  let (offset, ind) = parse_index(input)?;
  // Match the next char to see what kind of selection this is
  match input[offset .. ].chars().next() {
    // Unwraps nothing in the first index into selection
    Some(';') => {
      let (offset2, ind2) = parse_index(&input[offset + 1 ..])?;
      let unwrapped1 = ind.unwrap_or(Ind::Selection);
      let unwrapped2 = ind2.unwrap_or(Ind::BufferLen);
      Ok((offset2 + 1 + offset, Some(Sel::Pair(unwrapped1, unwrapped2))))
    },
    // Unwraps nothing in the first index into start of buffer
    Some(',') => {
      let (offset2, ind2) = parse_index(&input[offset + 1 ..])?;
      let unwrapped1 = ind.unwrap_or(Ind::Literal(0));
      let unwrapped2 = ind2.unwrap_or(Ind::BufferLen);
      Ok((offset2 + 1 + offset, Some(Sel::Pair(unwrapped1, unwrapped2))))
    },
    _ => { // Either no more input or the command char itself
      // This means it is a lone index
      // Map the potential index to a Sel::Lone, since None should remain None
      Ok(( offset, ind.map(|i| Sel::Lone(i)) ))
    },
  }
}

pub fn interpret_index<'a> (
  index: Ind<'a>,
  buffer: &dyn Buffer,
  old_selection: Option<usize>,
) -> Result<usize, &'static str> {
  match index {
    Ind::Selection => match old_selection {
      Some(sel) => Ok(sel),
      None => Err(NO_SELECTION),
    },
    Ind::BufferLen => Ok(buffer.len()),
    Ind::Literal(index) => Ok(index),

    // Decide on option or result from the buffer API
    Ind::Tag(tag) => buffer.get_tag(tag),
    Ind::Pattern(pattern) => buffer.get_matching(pattern, false),
    Ind::RevPattern(pattern) => buffer.get_matching(pattern, true),

    Ind::Add(inner, offset) => {
      let inner = interpret_index(*inner, buffer, old_selection)?;
      Ok(inner + offset)
    },
    Ind::Sub(inner, offset) => {
      let inner = interpret_index(*inner, buffer, old_selection)?;
      if offset > inner { Err(NEGATIVE_INDEX) }
      else { Ok(inner - offset) }
    },
  }
}

pub fn interpret_selection<'a>(
  input: Option<Sel<'a>>,
  old_selection: Option<(usize, usize)>,
  buffer: &dyn Buffer,
  default_all: bool,
) -> Result<(usize, usize), &'static str> {
  let selection = if default_all {
    input.unwrap_or(Sel::Pair( Ind::Literal(1), Ind::BufferLen ))
  }
  // If not default all default is old selection
  else {
    input.unwrap_or(Sel::Pair( Ind::Selection, Ind::Selection ))
  };
  match selection {
    Sel::Lone(ind) => {
      // Just interpret the lone index and make it a selection
      let i = interpret_index(ind, buffer, old_selection.map(|x| x.0) )?;
      Ok((i, i + 1))
    },
    Sel::Pair(ind1, ind2) => {
      let i1 = interpret_index(ind1, buffer, old_selection.map(|x| x.0) )?;
      let i2 = interpret_index(ind2, buffer, old_selection.map(|x| x.1) )?;
      Ok((i1, i2))
    },
  }
}
