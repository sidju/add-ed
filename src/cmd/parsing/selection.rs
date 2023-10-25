/// Since default selections vary between commands and access to the
/// history is needed for realisation we parse into an intermediate
/// struct which is then interpreted using additional data.

use crate::error::*;
use crate::Ed;

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

pub fn parse_index(
  input: &str,
) -> Result<(usize, Option<Ind<'_>>)> {
  // Set up state variables for one-pass parse
  let mut end = None;
  let mut state = State::Default(0);
  let mut current_ind = None;
  // Loop over chars and parse
  let iter = input.char_indices();
  for (i, ch) in iter {
    // Handle based on state
    match state {
      // If a state change is coming, populate current ind and make the change
      State::Default(start) => {
        match ch {
          // Offset is always valid
          // We need to see if there is a literal before it, before entering state
          '+' | '-' => {
            if start != i {
              // Catches that we had a special index, then some random numbers,
              // then an offset. Not caught earlier so we can give a more
              // detailed error. Same error logic as post loop State::Default
              if let Some(_) = current_ind {return Err(
                // Note that this reports getting digits after another index.
                // We catch it here to get all the digits before erroring.
                EdError::IndicesUnrelated{
                  prior_index: input[..start].to_owned(),
                  unrelated_index: input[start..i].to_owned(),
                }
              )}
              // If there is numeric input before, handle that
              let literal = input[start .. i].parse::<usize>()
                .map_err(|_|EdError::IndexNotInt(input[start..i].to_owned()))?;
              current_ind = Some(Ind::Literal(literal));
            }
            state = State::Offset(i + 1, ch == '-');
          },
          // Invalid if current_ind is some, but we catch that in their handlers
          // to be able to give a clearer error
          '/' | '\'' | '?' | '.' | '$' => {
            // These are only valid at the start of an index
            if start != i { return Err(EdError::IndexSpecialAfterStart{
              prior_index: input[start..i].to_owned(),
              special_index: ch,
            }); }
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
              '.' | '$' => {
                if let Some(_) = current_ind { return Err(
                  EdError::IndicesUnrelated{
                    prior_index: input[..i].to_owned(),
                    unrelated_index: input[i..i+1].to_owned(),
                  }
                )}
                current_ind = Some(
                  if ch == '.' { Ind::Selection } else { Ind::BufferLen }
                );
                state = State::Default(i + 1); // reset start after moving into current_ind
              },
              _ => panic!("Unreachable"),
            }
          }
          // Numeric input isn't always valid, but we catch that after breaking
          // so we can have a more detailed report in the error message
          _ => {
            // If not numeric (base 10) it must be the end of the index
            // Break the loop and handle the last outside
            if ! ch.is_ascii_digit() {
              // Mark current character as the end of the index
              end = Some(i);
              break;
            }
          },
        }
      },
      // If the tag state was entered, save the next char as tag and return to default
      State::Tag => {
        if let Some(_) = current_ind { return Err(
          EdError::IndicesUnrelated{
            // Safe, as the -1 is to exclude the ' (which is 1 byte long)
            prior_index: input[..i-1].to_owned(),
            // As the start -1 includes the ' the start is safe, but we can't
            // assume the given tag is 1 byte long.
            unrelated_index: {
              let mut index = String::new();
              for ch in input[i-1..].chars().take(2) {
                index.push(ch);
              }
              index
            },
          }
        )}
        current_ind = Some(Ind::Tag(ch));
        state = State::Default( i + ch.len_utf8() );
      },
      // If the pattern state was entered, save as pattern until end char is given and return to default
      State::Pattern(start) => {
        if ch == '/' {
          if let Some(_) = current_ind { return Err(
            EdError::IndicesUnrelated{
              prior_index: input[..start-1].to_owned(),
              unrelated_index: input[start-1..i+1].to_owned(),
            }
          )}
          current_ind = Some(Ind::Pattern(&input[start .. i]));
          state = State::Default( i + 1 );
        }
      },
      // Same as pattern with different end char
      State::RevPattern(start) => {
        if ch == '?' {
          if let Some(_) = current_ind { return Err(
            EdError::IndicesUnrelated{
              prior_index: input[..start-1].to_owned(),
              unrelated_index: input[start-1..i+1].to_owned(),
            }
          )}
          current_ind = Some(Ind::RevPattern(&input[start .. i]));
          state = State::Default( i + 1 );
        }
      },
      // For Offset we never return to Default, since the only state valid after a non-normal state is Offset
      // As such we ourselves check for the end of the index or subsequent offsets and handle accordingly
      State::Offset(start, negative) => {
        // Check if a known state change. If so, handle it
        match ch {
          // If we are recursing we parse current offset, put it in current_ind and change state accordingly
          '+' | '-' => {
            let offset = if start != i {
              input[start .. i].parse::<usize>()
                .map_err(|_|EdError::OffsetNotInt(input[start..i].to_owned()))?
            } else { 1 };
            current_ind = Some( if negative {
              Ind::Sub(Box::new(current_ind.unwrap_or(Ind::Selection)), offset)
            } else {
              Ind::Add(Box::new(current_ind.unwrap_or(Ind::Selection)), offset)
            });
            state = State::Offset( i + ch.len_utf8(), ch == '-' );
          },
          x if x.is_ascii_digit() => {}, // Ignore until we find the end
          _ => { // Means this is the end
            // The parsing logic outside the loop does what we need, so save index as end and break
            end = Some(i);
            break;
          },
        } 
      },
    } // End of match
  } // End of for-each

  // When we get here we have either gone through the whole history or
  // found a command/separator that marks the end of this index
  // If end is none we reached the end, rather than a denoting character, and should set end to len()
  let end = match end {
    Some(i) => i,
    None => input.len(),
  };

  // Check the state
  match state {
    // If the string ends in Default mode its contents should be sane 
    State::Default(start) => {
      if start < end {
        // If there is both a current ind and a numeral literal it is error
        // Occurs if a special index receives a non-offset number after
        // (Caught here to find end of index for better error message)
        if let Some(_) = current_ind { Err(
          EdError::IndicesUnrelated{
            prior_index: input[..start].to_owned(),
            unrelated_index: input[start..end].to_owned(),
          }
        )}
        // Else we parse the literal and return it
        else {
          let literal = input[start .. end].parse::<usize>()
            .map_err(|_|EdError::IndexNotInt(input[start..end].to_owned()))?;
          Ok((end, Some(Ind::Literal(literal))))
        }
      }
      // If there is no literal we return current_ind as-is, since None is the correct return if nothing was parsed
      else {
        Ok((end, current_ind))
      }
    },
    // If the string ended abruptly in offset mode the contents may be usable
    State::Offset(start, negative) => {
      // If the string ends on a + we will have an incorrect i
      let offset = if start < end {
        input[start .. end].parse::<usize>()
          .map_err(|_|EdError::OffsetNotInt(input[start .. end].to_owned()))?
      } else {
        1
      };
      Ok((end, Some(
        if negative {
          Ind::Sub(Box::new(current_ind.unwrap_or(Ind::Selection)), offset)
        } else {
          Ind::Add(Box::new(current_ind.unwrap_or(Ind::Selection)), offset)
        }
      )))
    },
    // If we get here in a state that isn't terminated (returned to Default) there is an error
    _ => {
      ed_unreachable!()
    },
  }
}

pub fn parse_selection(
  input: &str,
) -> Result<(usize, Option<Sel<'_>>)> {
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
    // Unwraps nothing in the first index into start of history
    Some(',') => {
      let (offset2, ind2) = parse_index(&input[offset + 1 ..])?;
      let unwrapped1 = ind.unwrap_or(Ind::Literal(1));
      let unwrapped2 = ind2.unwrap_or(Ind::BufferLen);
      Ok((offset2 + 1 + offset, Some(Sel::Pair(unwrapped1, unwrapped2))))
    },
    _ => { // Either no more input or the command char itself
      // This means it is a lone index
      // Map the potential index to a Sel::Lone, since None should remain None
      Ok(( offset, ind.map(Sel::Lone) ))
    },
  }
}

// Interprets index struct into 1-indexed usize.
// (1-indexed so append operations can append to line 0 to insert before line 1)
// Should not be able to return a index bigger than history.len().
pub fn interpret_index(
  state: &Ed<'_>,
  index: Ind<'_>,
  old_selection: usize,
) -> Result<usize> {
  let ind = match index {
    Ind::Selection => Ok(old_selection),
    // Since we want 1-indexed len() points at the last valid line or 0 if none
    Ind::BufferLen => Ok(state.history.current().len()),
    // May be invalid, history is expected to check
    Ind::Literal(i) => Ok(i),
    // These return values are 0 indexed like the rest of the Buffer API
    // Subtract/add 1 on input/output
    Ind::Tag(tag) => super::get_tag(state.history.current(), tag),
    Ind::Pattern(pattern) =>
      super::get_matching(
        state.history.current(),
        pattern,
        old_selection,
        super::Direction::Forwards,
      ),
    Ind::RevPattern(pattern) =>
      super::get_matching(
        state.history.current(),
        pattern,
        old_selection,
        super::Direction::Backwards
      ),
    // These are relative to the prior, so have no indexing per-se
    Ind::Add(inner, offset) => {
      let inner = interpret_index(state, *inner, old_selection)?;
      Ok(inner.saturating_add(offset))
    },
    Ind::Sub(inner, offset) => {
      let inner = interpret_index(state, *inner, old_selection)?;
      Ok(inner.saturating_sub(offset))
    },
  }?;
  Ok(ind)
}

// Interprets a given selection into two usize.
// 1-indexed just like indices, since 'i'/'a' use selection start/end as index
// This function tries to make every selection inclusive towards its ending index
pub fn interpret_selection(
  state: &Ed<'_>,
  input: Option<Sel<'_>>,
  old_selection: (usize, usize),
) -> Result<(usize, usize)> {
  let selection = input.unwrap_or(Sel::Pair( Ind::Selection, Ind::Selection ));
  let interpreted = match selection {
    Sel::Lone(ind) => {
      // Just interpret the lone index and make it a selection
      let i = interpret_index(state, ind, old_selection.0 )?;
      (i, i)
    },
    Sel::Pair(ind1, ind2) => {
      let i = interpret_index(state, ind1, old_selection.0 )?;
      let i2 = interpret_index(state, ind2, old_selection.1 )?;
      (i, i2)
    },
  };
  Ok(interpreted)
}
