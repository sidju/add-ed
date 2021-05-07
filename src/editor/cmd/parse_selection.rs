/// Since default selections vary between commands and access to the
/// buffer is needed for realisation we parse into an intermediate
/// struct which is then interpreted using additional data.

use crate::error_consts::*;

use super::Buffer;

// A struct to formalise all the kinds of indices
#[derive(PartialEq)]
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
  let mut selection_default = None;
  let mut current_ind = None;
  // Loop over chars and parse
  let iter = input.char_indices().peekable();
  for (index, ch) in iter {
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
                state = State::Pattern(i);
              },
              '?' => {
                state = State::RevPattern(i);
              },
              '.' => {
                current_ind = Some(Ind::Selection);
              },
              '$' => {
                current_ind = Some(Ind::BufferLen)
              },
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
) -> Result<Ind<'a>, &'static str> {
  // First parse, getting an index and the offset it stopped parsing at
  let (offset, ind) = parse_index(input)?;
  match input[offset .. ].chars().next() {
    None => {},
    Some(ch) => match ch {
      ';' => {
        let (offset2, ind2) = parse_index(&input[offset + 1 ..]);
        return Ok((offset2, Sel::FromSelection(ind, ind2)));
      },
      ',' => {
        let (offset2, ind2) = parse_index(&input[offset + 1 ..]);
        return Ok((offset2, Sel::FromStart(ind, ind2)));
      },
      _ => {},
    }
  }
  return Ok((offset, Sel::Lone(ind)));
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
      let inner = interpret_index(inner.into_inner(), buffer, old_selection)?;
      Ok(inner + offset)
    },
    Ind::Sub(inner, offset) => {
      let inner = interpret_index(inner.into_inner(), buffer, old_selection)?;
      if offset > inner { Err(NEGATIVE_INDEX) }
      else { Ok(inner - offset) }
    },
  }
}

pub fn interpret_selection<'a>(
  selection: Sel<'a>,
  buffer: &dyn Buffer,
  old_selection: Option<(usize, usize)>,
  default_all: bool,
) -> Result<(usize, usize), &'static str> {
  match selection {
    Sel::Lone(ind) => {
      match ind {
        Ind::Default => {
          if default_all { Ok((0, buffer.len())) }
          else { Ok(old_selection.unwrap_or((0, buffer.len()))) }
        }

      }
    }
  }
}
