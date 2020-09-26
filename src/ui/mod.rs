/// IO abstractions
use crate::State;

use crossterm::{QueueableCommand, ErrorKind, style::Print};
use std::io::{Write, stdout};

// Start with the printing helpers

fn syntect_to_crossterm_color(c: syntect::highlighting::Color)
  -> crossterm::style::Color
{
  use crossterm::style::Color;
  // If the alpha is zero, read as 16 color
  if c.a == 0 {
    match c.r {
      0 => Color::Black,
      1 => Color::DarkRed,
      2 => Color::DarkGreen,
      3 => Color::DarkYellow,
      4 => Color::DarkBlue,
      5 => Color::DarkMagenta,
      6 => Color::DarkCyan,
      7 => Color::Grey,

      8 => Color::DarkGrey,
      9 => Color::Red,
      10 => Color::Green,
      11 => Color::Yellow,
      12 => Color::Blue,
      13 => Color::Magenta,
      14 => Color::Cyan,
      15 => Color::White,

      _ => panic!("Invalid theme. Alpha = 0 indicates 16 color in red."),
    }
  }
  else {
    Color::Rgb{r: c.r, g: c.g, b: c.b}
  }
}

// Set the style given, all parts explicitly set to given style
fn apply_style(style: syntect::highlighting::Style, out: &mut impl Write)
  -> Result<(), ErrorKind>
{
  use syntect::highlighting::FontStyle;
  use crossterm::style::{SetColors, SetAttribute, Colors, Attribute};

  // Prepare and apply colors
  let colors = Colors::new(
    syntect_to_crossterm_color(style.foreground),
    syntect_to_crossterm_color(style.background)
  );
  out.queue(SetColors(colors))?;
  
  // Prepare and apply styling
  if style.font_style.contains(FontStyle::BOLD) {
    out.queue(SetAttribute(Attribute::Bold))?;
  }
  if style.font_style.contains(FontStyle::ITALIC) {
    out.queue(SetAttribute(Attribute::Italic))?;
  }
  if style.font_style.contains(FontStyle::UNDERLINE) {
    out.queue(SetAttribute(Attribute::Underlined))?;
  }
  Ok(())
}
fn reset_style(out: &mut impl Write)
  -> Result<(), ErrorKind>
{
  use crossterm::style::{ResetColor, SetAttribute, Attribute};
  // Reset colors
  out.queue(ResetColor)?;
  // Reset attributes
  out.queue(SetAttribute(Attribute::Reset))?;
  Ok(())
}

fn print_separator(out: &mut impl Write, width: usize)
  -> Result<(), ErrorKind>
{
  // Create the string to hold the separator with the capacity we know it will use
  let mut sep = String::with_capacity(width);
  for _ in 0 .. width {
    sep.push('─');
  }
  sep.push('\n');
  // Print the generated separator
  out.queue(Print(sep))?;
  Ok(())
}

pub fn print_view(
  state: &State,
  //text: &[&str],
  text: &[String],
  mut line_nr: usize,
  n: bool,
  l: bool,
) -> Result<(), ErrorKind> {
  // Get the highlighting settings
  let theme_source = include_bytes!("../../assets/theme.xml");
  let mut theme_reader = std::io::Cursor::new(&theme_source[..]);
  let theme = syntect::highlighting::ThemeSet::load_from_reader(&mut theme_reader).unwrap();

  let syntax = state.syntax_lib.find_syntax_for_file(&state.file)
    .unwrap_or(None)
    .unwrap_or_else(|| state.syntax_lib.find_syntax_plain_text());
  // Create the highlighter, which statefully styles the text over lines.
  let mut highlighter = syntect::easy::HighlightLines::new(syntax, &theme);

  // Create a connection to the terminal, that we print through
  let mut out = stdout();

  // Track lines printed, to break when we have printed the terminal height
  let mut lines_printed = 0;

  // Count characters printed, for wrapping. Use 'i' since everything uses it
  let mut i = 0;

  // Arguably one should give the highlighter all lines before the selection.
  // Otherwise it fails to understand multiline stuff, but not worth it to me. 
  // PR's welcome
  for line in text {
    // To handle wrapping, print character by character
    
    // If i isn't a multiple of the screen size we want to pad with space, to ensure colors are set correctly
    for _ in 0 .. (state.term_size.0 - (i % state.term_size.0)) {
      out.queue(Print(' '))?;
    }

    // Then we can clear i
    i = 0;

    // Highlight the text.
    let highlighted = highlighter.highlight(line, &state.syntax_lib);

    // Iterate over the segments, setting style before each
    for (style, text) in highlighted {
      // Check the style and apply it using Crossterm
      apply_style(style, &mut out)?;

      for ch in text.chars() {
        // If i is divisible by terminal width it is time to split
        if i % state.term_size.0 == 0 {
          // To not colour our line numbers we reset styling here.
          // Can be changed to theme the line numbers
          out.queue(Print('\n'))?;
          reset_style(&mut out)?;

          lines_printed += 1;

          // Break if we have printed the height of the view window - 2
          if lines_printed + 1 >= state.term_size.1 { break; }

          // If we are printing line numbers
          if n {
            // convert the internal 0-indexed integer to a 1-indexed number string
            let tmp_num = (line_nr + 1).to_string();
            let tmp_num_len = tmp_num.len();
            // If this is the first line of the internal line, print the line number
            if i == 0 {
              out.queue(Print(tmp_num))?;
              line_nr += 1;
            }
            // Else print padding to keep the edge even
            else {
              for _ in 0 .. tmp_num.len() { out.queue(Print(' '))?; }
            }
            out.queue(Print('│'))?;
            i += tmp_num_len + 1; // Mark that we added some chars to the line
            // Restore the styling
            apply_style(style, &mut out)?;
          }
        }
        // No matter line changes and all that, look at the actual char
        match ch {
          // Special handling for 'l' printing mode
          // we don't print '\n', since we must trust the buffer in them only occuring
          // at ends of lines (where i == 0, so we print '\n' anyways)
          '\n' => if l { out.queue(Print('$'))?; },
          '$' => if l { out.queue(Print("\\$"))?; i += 1;} else { out.queue(Print('$'))?; },
          c => { out.queue(Print(c))?; },
        }
        // Increment number of chars printed. Not techically correct if it was '\n'
        // but that should be end of line, which means i is never looked at again
        i += 1;
      }
    }
  }
  // Add one newline, since we print newlines at starts of lines, not ends.
  reset_style(&mut out)?;
  out.queue(Print('\n'))?;
  // Add the separator
  print_separator(&mut out, state.term_size.0)?;
  // Finally we flush the buffer, to make sure we actually have printed everything
  out.flush()?;
  Ok(())
}
