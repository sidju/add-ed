/// Since default selections vary between commands and access to the
/// history is needed for realisation we parse into an intermediate
/// struct which is then interpreted using additional data.

use crate::error::*;
use crate::Ed;

// A struct to formalise all the kinds of indices
#[derive(PartialEq, Debug)]
pub enum HistoryInd {
  Current,
  Last,
  Absolute(usize),
  // Delay implementation until we figure out how to make it user friendly
  // (straight regex would be a footgun, searching for a i would return
  // every single line containing the letter i)
//  Pattern(&'a str),
//  RevPattern(&'a str),
  Add(Box<HistoryInd>, usize),
  Sub(Box<HistoryInd>, usize),
}

#[derive(PartialEq, Debug)]
enum State {
  Default(usize),
  Absolute(usize),
//  Pattern(usize),
//  RevPattern(usize),
  Offset(usize, bool),
}

pub fn parse_history_index(
  input: &str,
) -> Result<(usize, Option<HistoryInd>)> {
  // Set up state variables for one-pass parse
  let mut i;
  let mut ch;
  let mut state = State::Default(0);
  let mut current_ind = None;
  // Loop over chars and parse
  let input = input.trim_end_matches('\n');
  let mut iter = input.char_indices();
  // This loop is only exited by the Default and Offset state handlers, which
  // handle finalizing and returning.
  // If the other states don't switch to one of them it will loop indefinitely.
  loop {
    // Get next character, if none give None but set i to len of input
    (i, ch) = iter.next()
      .map(|(i, ch)| (i, Some(ch)))
      .unwrap_or((input.len(), None))
    ;
    // Handle based on state
    match state {
      // If a state change is coming, populate current ind and make the change
      State::Default(start) => {
        // While we are receiving data we check for state changes
        match ch {
          // Check for the direct literals, require that current_ind is None
          /* Some('/') | Some('?') | */ Some('*') | Some('.') | Some('$') => {
            let c = ch.unwrap();
            // These are only valid at the start of an index
            if start != i { return Err(EdError::HistoryIndexSpecialAfterStart{
              prior_index: input[start..i].to_owned(),
              special_index: c,
            }); }
            match c {
//              '/' => {
//                state = State::Pattern(i + 1); // Since we know the length of these chars to be one byte
//              },
//              '?' => {
//                state = State::RevPattern(i + 1); // Since we know the length of these chars to be one byte
//              },
              '*' => {
                state = State::Absolute(i + 1);
              },
              '.' | '$' => {
                // The other special indices handle this error upon termination,
                // but since these are only one character we do it here.
                if let Some(_) = current_ind { return Err(
                  EdError::HistoryIndicesUnrelated{
                    prior_index: input[..i].to_owned(),
                    unrelated_index: input[i..i+1].to_owned(),
                  }
                )}
                current_ind = Some(
                  match c {
                    '.' => HistoryInd::Current,
                    '$' => HistoryInd::Last,
                    _ => ed_unreachable!()?,
                  }
                );
                state = State::Default(i + 1); // reset start after moving into current_ind
              },
              _ => ed_unreachable!()?,
            }
          }
          // Offset is valid even if current_ind is set
          // We need to see if there is a literal before it, before entering state
          Some('+') | Some('-') => {
            if start != i {
              // Catches that we had a special index, then some random numbers,
              // then an offset. Not caught earlier so we can give a more
              // detailed error. Same error logic as post loop State::Default
              if let Some(_) = current_ind {return Err(
                // Note that this reports getting digits after another index.
                // We catch it here to get all the digits before erroring.
                EdError::HistoryIndicesUnrelated{
                  prior_index: input[..start].to_owned(),
                  unrelated_index: input[start..i].to_owned(),
                }
              )}
              // If there is numeric input before, handle that
              let literal = input[start .. i].parse::<usize>()
                .map_err(|_|EdError::HistoryIndexNotInt(input[start..i].to_owned()))?;
              current_ind = Some(HistoryInd::Sub(Box::new(HistoryInd::Current), literal));
            }
            state = State::Offset(i + 1, ch == Some('-'));
          },
          // This input may not be valid, but we catch it later to get better
          // errors
          _ => {
            // If this is valid base 10 input, keep on looping until it ends
            if ch.is_some_and(|x| x.is_ascii_digit()) {
            }
            // Otherwise we are out of index input, parse up and return
            else {
              // If there is input since last finalized state
              return if start < i {
                // And a current ind it is error
                // Occurs if a special index receives a non-offset number after
                // (Caught here to find end of index for better error message)
                if let Some(_) = current_ind { Err(
                  EdError::HistoryIndicesUnrelated{
                    prior_index: input[..start].to_owned(),
                    unrelated_index: input[start..i].to_owned(),
                  }
                )}
                // Else we parse the literal and return it
                else {
                  let literal = input[start..i].parse::<usize>()
                    .map_err(|_|EdError::HistoryIndexNotInt(input[start..i].to_owned()))?;
                  Ok((i, Some(HistoryInd::Sub(Box::new(HistoryInd::Current), literal))))
                }
              }
              // If there is no literal we return current_ind as-is, since None is the correct return if nothing was parsed
              else {
                Ok((i, current_ind))
              }
            }
          },
        }
      },
      State::Absolute(start) => {
        // If still digits, keep looping until end
        if ch.is_some_and(|x| x.is_ascii_digit()) {}
        else {
          // A prior index is not valid
          if let Some(_) = current_ind { return Err(
            EdError::HistoryIndicesUnrelated{
              prior_index: input[..start-1].to_owned(),
              unrelated_index: input[start-1..input.len().min(i+1)].to_owned(),
            }
          )}
          let index = input[start..i].parse::<usize>()
            .map_err(|_|EdError::HistoryIndexNotInt(input[start..i].to_owned()))?;
          current_ind = Some(HistoryInd::Absolute(index));
          state = State::Default( i + 1 );
        }
      },
//      // If the pattern state was entered, save as pattern until end char is given and return to default
//      State::Pattern(start) => {
//        // terminator or end of input, either way we are done
//        if ch.is_none_or(|x| x == '/') {
//          if let Some(_) = current_ind { return Err(
//            EdError::IndicesUnrelated{
//              prior_index: input[..start-1].to_owned(),
//              unrelated_index: input[start-1..input.len().min(i+1)].to_owned(),
//            }
//          )}
//          current_ind = Some(Ind::Pattern(&input[start .. i]));
//          // Moving to state default means that state handles return as needed
//          state = State::Default( i + 1 );
//        }
//      },
//      // Same as pattern with different end char
//      State::RevPattern(start) => {
//        if ch.is_none_or(|x| x == '?') {
//          if let Some(_) = current_ind { return Err(
//            EdError::IndicesUnrelated{
//              prior_index: input[..start-1].to_owned(),
//              unrelated_index: input[start-1..input.len().min(i+1)].to_owned(),
//            }
//          )}
//          current_ind = Some(Ind::RevPattern(&input[start .. i]));
//          // Moving to state default means that state handles return as needed
//          state = State::Default( i + 1 );
//        }
//      },
      // For Offset we never return to Default, since the only state valid after a non-normal state is Offset
      // As such we ourselves check for the end of the index or subsequent offsets and handle accordingly
      State::Offset(start, negative) => {
        // Check if a known state change. If so, handle it
        match ch {
          // If we are recursing we parse current offset, put it in current_ind and change state accordingly
          Some('+') | Some('-') => {
            let offset = if start != i {
              input[start .. i].parse::<usize>()
                .map_err(|_|EdError::HistoryOffsetNotInt(input[start..i].to_owned()))?
            } else { 1 };
            current_ind = Some( if negative {
              HistoryInd::Sub(Box::new(current_ind.unwrap_or(HistoryInd::Current)), offset)
            } else {
              HistoryInd::Add(Box::new(current_ind.unwrap_or(HistoryInd::Current)), offset)
            });
            state = State::Offset( i + ch.unwrap().len_utf8(), ch == Some('-') );
          },
          x if x.is_some_and(|x| x.is_ascii_digit()) => {}, // Ignore until we find the end
          _ => { // Means this is the end
            // Handle that if the index ends on a + there's nothing to parse
            let offset = if start < i {
              input[start .. i].parse::<usize>()
               .map_err(|_|EdError::HistoryOffsetNotInt(input[start .. i].to_owned()))?
            } else {
              1
            };
            return Ok((i, Some(
              if negative {
                HistoryInd::Sub(Box::new(current_ind.unwrap_or(HistoryInd::Current)), offset)
              } else {
                HistoryInd::Add(Box::new(current_ind.unwrap_or(HistoryInd::Current)), offset)
              }
            )))
          },
        } 
      },
    } // End of match
  } // End of loop
}

pub fn interpret_history_index(
  state: &Ed<'_>,
  index: HistoryInd,
  current_ind: usize,
) -> Result<usize> {
  match index {
    HistoryInd::Current => Ok(current_ind),
    HistoryInd::Last => Ok(state.history.len().saturating_sub(1)),
    HistoryInd::Absolute(i) => Ok(i),
//    Ind::Pattern(pattern) =>
//      super::get_matching(
//        state.history.current(),
//        pattern,
//        old_selection,
//        super::Direction::Forwards,
//      ),
//    Ind::RevPattern(pattern) =>
//      super::get_matching(
//        state.history.current(),
//        pattern,
//        old_selection,
//        super::Direction::Backwards
//      ),
    // These are relative to the prior, so have no indexing per-se
    HistoryInd::Add(inner, offset) => {
      let inner = interpret_history_index(state, *inner, current_ind)?;
      Ok(inner.saturating_add(offset))
    },
    HistoryInd::Sub(inner, offset) => {
      let inner = interpret_history_index(state, *inner, current_ind)?;
      Ok(inner.saturating_sub(offset))
    },
  }
}
