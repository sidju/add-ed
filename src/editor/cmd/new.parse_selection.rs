/// Since default selections vary between commands and access to the
/// buffer is needed for realisation we parse into an intermediate
/// struct which is then interpreted using additional data.

use crate::error_consts::*;

// A struct to formalise all the kinds of indices
#[derive(PartialEq)]
pub enum Ind <'a> {
  Default,
  Selection,
  BuffenLen,
  Literal(usize),
  Tag(char),
  Pattern(&'a str),
  RevPattern(&'a str),
  Add(Box<Ind<'a>>, usize),
  Sub(Box<Ind<'a>>, usize),
}

pub enum Sel <'a> {
  Pair(Ind, Ind),
  Lone(Ind)
}

enum State {
  Default(usize),
  Tag,
  Pattern(usize),
  RevPattern(usize),
  Offset(usize, bool),
}

pub fn <'a> parse_index(
  input: &'a str,
  lone_only: bool,
) -> Result<(usize, Ind<'a>), &'static str> {
  // Set up state variables for one-pass parse
  let mut state = State::Default(0);
  let mut selection_default = None;
  let mut current_ind = None;
  // Loop over chars and parse
  let iter = input.char_indices().peekable();
  for (i, ch) in iter {
    // Handle based on state
    match state {
      State::Default(start) => { // If default, check for state changes
        match ch {
          '/' | '\'' | '?' => {
            // These are only valid at the start of an index
            if start != i { return Err(PARSE_INDEX); }
            // Since prior tags or patterns reset start, check that current_ind is none
            if current_ind.is_some() { return Err(PARSE_INDEX); }
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
            }
          }
          '+' => { state = State::Offset(i, false); },
          '-' => { state = State::Offset(i, true); },
          _ => {
            // If not numeric (base 10) it must be the end of the index
            if ! ch.is_digit(10) {
              return (i, current_ind.unwrap_or(Ind::Default));
            }
          },
        }
      },
      State::Tag => {
        current_ind = Some(Ind::Tag(ch));
        state = State::Default( i + ch.len_utf8() );
      },
      State::Pattern(start) {
        if ch == '/' {
          current_ind = Some(Ind::Pattern(&input[start .. i]));
          state = State::Default( i + ch.len_utf8() );
        }
      },
      State::RevPattern(start) {
        if ch == '?' {
          current_ind = Some(Ind::RevPattern(&input[start .. i]));
          state = State::Default( i + ch.len_utf8() );
        }
      },
      // For the Add and Sub cases we need to check for separator or command to know when to stop
      // This means we also need to parse separator or command as Default would
      // There is also a possibility to allow chaining additions by recognising + and - here, but I am too tired
      State::Offset(start, negative) => {
        // Offset only takes numeric input, so at end of numeric input we hand back over to Default
        // This requires peeking at the next char
        if ! iter.peek().1.is_digit(10) {
          // Parse our offset and put it together in current_ind
          let offset = if start != i {
            input[start .. i].parse::<usize>().map_err(|_| PARSE_INDEX)?
          } else {
            1
          };
          current_ind = Some( if negative {
            Ind::Sub(Box::new(current_ind), offset)
          } else {
            Ind::Add(Box::new(current_ind), offset)
          });
          // Then return to default state
          state = State::Default( i + ch.len_utf8() );
        }
      },
    } // End of match
  } // End of for-each
  // If we got all the way through the command wasn't newline terminated. So error
  Err(PARSE_INDEX)
}

pub fn parse_selection(
  input: &'a str,
) -> Result<Ind<'a>, &'a static str> {
  // First parse, getting an index and the offset it stopped parsing at
  let (offset, ind) = parse_index(input);
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


pub fn <'a> interpret_index(
  index: Ind<'a>,
  buffer: &dyn Buffer,
  old_selection: Option<usize>,
  default: usize,
) -> Result<usize, &'static str> {
  let sel = old_selection.unwrap_or(default);
  match index {
    Ind::Default => default,
    Ind::Selection => sel,
    Ind::BufferLen => buffer.len(),
    Ind::Literal(index) => index,
    Ind::Tag(tag) => buffer.get_tag(tag),
    Ind::Pattern(pattern) => buffer.get_matching(pattern, false),
    Ind::RevPattern(pattern) => buffer.get_matching(pattern, true),
    Ind::Add(inner, offset) => {
      let inner = interpret_index(inner.into_inner(), buffer, old_selection, default)?;
      inner + offset
    },
    Ind::Sub(inner, offset) => {
      let inner = interpret_index(inner.into_inner(), buffer, old_selection, default)?;
      if offset > inner { Err(PARSE_INDEX) }
      else { inner - offset }
    },
  }
}

pub fn <'a> interpret_selection(
  selection: Sel<'a>,
  buffer: &dyn Buffer,
  old_selection: Option<(usize, usize)>,
  default_all: bool,
) -> Result<(usize, usize), &'static str> {
}

            if ch.is_ascii_alphabetic()  || '\n' {
              // Build a selection from what we parsed so far and return it
              if start != i {
                if current_ind.is_some() { return Err(PARSE_INDEX); }
                // This means we should parse for a number
                current_ind = Some(input[start .. i].parse::<usize>().map_err(|_| PARSE_INDEX)?);
              }
              return Ok(( i, match selection_default {
                // With semi-colon default start and end are selection start and buffer end
                Some(true) => Sel::Pair(
                  first_ind.unwrap_or(Ind::Selection),
                  current_ind.unwrap_or(Ind::BufferLen)
                ),
                // With comma default start and end are buffer start and end
                Some(false) => Sel::Pair(
                  first_ind.unwrap_or(Ind::Literal(0)),
                  current_ind.unwrap_or(Ind::BufferLen)
                ),
                // With no separator we create a lone index
                None => {
                  current_ind
                    .map(|i| Sel::Lone(i))
                    // If nothing at all entered, set default selection
                    .unwrap_or(Sel::Default)
                },
              }));
            }
