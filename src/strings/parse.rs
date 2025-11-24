use bevy::color::LinearRgba;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case},
    character::complete::alphanumeric1,
    sequence::{preceded, terminated},
};

use crate::color::parse_color_string;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TerminalStringToken<'a> {
    Text(&'a str),
    StartFg(LinearRgba),
    EndFg,
    StartBg(LinearRgba),
    EndBg,
}

fn parse_start_fg(input: &str) -> IResult<&str, TerminalStringToken<'_>> {
    let (rest, color) =
        preceded(tag_no_case("<fgcol="), terminated(alphanumeric1, tag(">"))).parse(input)?;
    let Some(color) = parse_color_string(color) else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };
    Ok((rest, TerminalStringToken::StartFg(color)))
}

fn parse_end_fg(input: &str) -> IResult<&str, TerminalStringToken<'_>> {
    let (rest, _) = tag_no_case("</fgcol>")(input)?;
    Ok((rest, TerminalStringToken::EndFg))
}

fn parse_start_bg(input: &str) -> IResult<&str, TerminalStringToken<'_>> {
    let (rest, color) =
        preceded(tag_no_case("<bgcol="), terminated(alphanumeric1, tag(">"))).parse(input)?;
    let Some(color) = parse_color_string(color) else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };
    Ok((rest, TerminalStringToken::StartBg(color)))
}

fn parse_end_bg(input: &str) -> IResult<&str, TerminalStringToken<'_>> {
    let (rest, _) = tag_no_case("</bgcol>")(input)?;
    Ok((rest, TerminalStringToken::EndBg))
}

fn parse_text(input: &str) -> IResult<&str, TerminalStringToken<'_>> {
    let (rest, text) = is_not("<")(input)?; // Stop at next tag
    Ok((rest, TerminalStringToken::Text(text)))
}

// pub fn parse_tokens(input: &str) -> IResult<&str, Vec<Token<'_>>> {
//     many0(alt((
//         parse_start_fg,
//         parse_end_fg,
//         parse_start_bg,
//         parse_end_bg,
//         parse_text,
//     )))
//     .parse(input)
// }

pub fn parse_tokens(input: &str) -> TokenIter<'_> {
    TokenIter::new(input)
}

#[derive(Clone)]
pub struct TokenIter<'a> {
    rest: &'a str,
}

impl<'a> TokenIter<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { rest: input }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = TerminalStringToken<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }

        let mut parser = alt((
            parse_start_fg,
            parse_end_fg,
            parse_start_bg,
            parse_end_bg,
            parse_text,
        ));

        if let Ok((next, tok)) = parser.parse(self.rest) {
            self.rest = next;
            Some(tok)
        } else {
            None
        }
    }
}

/// Wraps a single line of text at a specified maximum length.
///
/// It prioritizes wrapping at newlines or existing whitespace, otherwise it hard wraps.
/// Returns a tuple of `(wrapped_line, remaining_text)`.
pub fn wrap_line(text: &str, max_len: usize) -> (&str, &str) {
    if text.is_empty() || max_len == 0 {
        return ("", text);
    }

    let mut last_space_index = None;

    for (i, ch) in text.char_indices() {
        if ch.is_whitespace() && ch != '\n' {
            last_space_index = Some(i);
        }

        if ch == '\n' {
            let line = &text[..i];
            let rest = &text[i + ch.len_utf8()..];
            return (line, rest);
        }

        if i + 1 > max_len {
            if let Some(space_index) = last_space_index {
                // Wrap at the last found space before max_len.
                let line = &text[..space_index];
                let rest = &text[space_index + 1..];
                return (line, rest);
            } else {
                // Hard wrap at the current index if no suitable space was found.
                let line = &text[..i];
                let rest = &text[i..];
                return (line, rest);
            }
        }
    }

    // Remaining text fits within the line length.
    (text, "")
}

/// Creates new string of lines by word wrapping at max_len
pub fn wrap_string(text: &str, max_len: usize) -> String {
    let mut wrapped_string = String::with_capacity(text.len() + text.len() / max_len + 1);
    let mut rem = text;

    // Use a loop with the wrap_line function to process the text in chunks.
    while !rem.is_empty() {
        let (line, rest) = wrap_line(rem, max_len);

        // Append the wrapped line.
        wrapped_string.push_str(line);

        if !rest.is_empty() {
            // If there's remaining text, it means a break occurred.
            // Add a newline character to the output.
            wrapped_string.push('\n');
        }

        rem = rest;
    }

    wrapped_string
}

/// Create a wrapped string from an iterator of substrings that form a complete string.
pub fn wrap_strings<'a>(strings: impl Iterator<Item = &'a str>, max_len: usize) -> String {
    let string = String::from_iter(strings);
    wrap_string(string.as_ref(), max_len)
}

#[cfg(test)]
mod tests {
    use bevy::math::IVec2;
    use sark_grids::PivotedPoint;

    use crate::{
        Terminal,
        strings::{GridStringIterator, TerminalString},
    };

    use super::*;

    #[test]
    fn testiter() {
        let input = "hello <bgcol=white>good <fgcol=blue>world</fgcol> it's</bgcol> a good day";

        for t in parse_tokens(input) {
            println!("{:?}", t);
        }
    }

    #[test]
    fn consume_tags() {
        let input = "hello <bgcol=white>good <fgcol=blue>world</fgcol> it's</bgcol> a good day";

        let mut string = String::new();
        let mut tags = Vec::new();
        let mut i: usize = 0;

        let mut term = Terminal::new([80, 40]);

        for t in parse_tokens(input) {
            match t {
                TerminalStringToken::Text(t) => {
                    string.push_str(t);
                }
                _ => tags.push((i, t)),
            }
        }
    }

    #[test]
    fn wrap_with_tokens() {
        let input = [
            "hello ",
            "good ",
            "world",
            " it's",
            " a good day. How are you doing?",
        ];

        let wrapped = wrap_strings(input.into_iter(), 15);

        println!("{}", wrapped);
    }

    #[test]
    fn wrapped_string_test() {
        let input = "hello good world it's a good day. How are you doing?";
        let wrapped = wrap_string(input, 15);
        println!("{}", wrapped);
    }

    pub fn put_string<T: AsRef<str>>(
        term: &mut Terminal,
        xy: impl Into<PivotedPoint>,
        string: impl Into<TerminalString<T>>,
    ) {
        let ts = string.into();

        let tokens = parse_tokens(ts.string.as_ref());

        let text_tokens = tokens.clone().filter_map(|t| {
            if let TerminalStringToken::Text(s) = t {
                Some(s)
            } else {
                None
            }
        });

        let lines = wrap_strings(text_tokens, term.width());

        // Write pivot aligned lines

        let mut cursor = 0;
        for token in tokens {
            match token {
                TerminalStringToken::Text(s) => cursor += s.chars().count(),
                TerminalStringToken::StartFg(linear_rgba) => todo!(),
                TerminalStringToken::EndFg => todo!(),
                TerminalStringToken::StartBg(linear_rgba) => todo!(),
                TerminalStringToken::EndBg => todo!(),
            }
        }
    }

    // pub fn put_stringy<T: AsRef<str>>(
    //     term: &mut Terminal,
    //     xy: impl Into<PivotedPoint>,
    //     string: impl Into<TerminalString<T>>,
    // ) {
    //     let mut fg_state = None;
    //     let mut bg_state = None;

    //     let ts = string.into();
    //     let word_wrap = ts.formatting.word_wrap;

    //     let mut xy = xy.into().calculate(term.size());

    //     let mut put_char = |xy: IVec2, ch: char, fg: Option<LinearRgba>, bg: Option<LinearRgba>| {
    //         let tile = term.tile_mut(xy);
    //         tile.glyph = ch;
    //         if let Some(fg) = fg {
    //             tile.fg_color = fg;
    //         }

    //         if let Some(bg) = bg {
    //             tile.bg_color = bg;
    //         }
    //     };

    //     for tok in parse_tokens(ts.string.as_ref()) {
    //         match tok {
    //             TerminalStringToken::Text(mut s) => {
    //                 while !s.is_empty() {
    //                     // ---------- 1. consume leading whitespace ----------
    //                     let mut ws_end = 0;
    //                     for (i, ch) in s.char_indices() {
    //                         if !ch.is_whitespace() {
    //                             break;
    //                         }
    //                         ws_end = i + ch.len_utf8();
    //                     }
    //                     let (ws, after_ws) = s.split_at(ws_end);

    //                     // ---------- 2. consume next word ----------
    //                     let mut word_end = 0;
    //                     for (i, ch) in after_ws.char_indices() {
    //                         if ch.is_whitespace() {
    //                             break;
    //                         }
    //                         word_end = i + ch.len_utf8();
    //                     }
    //                     let (word, rest) = after_ws.split_at(word_end);

    //                     // visible lengths
    //                     let ws_len = ws.chars().count();
    //                     let word_len = word.chars().count();

    //                     let max_len = term.width() - xy.x as usize;

    //                     // ---------- 3. wrap if needed ----------
    //                     if word_wrap && xy.x != 0 && xy.x as usize + ws_len + word_len > max_len {
    //                         grid.next_line();
    //                         col = 0;
    //                     }

    //                     // ---------- 4. output whitespace ----------
    //                     for ch in ws.chars() {
    //                         grid.put_char(ch, fg, bg);
    //                         col += 1;
    //                     }

    //                     // ---------- 5. output word ----------
    //                     for ch in word.chars() {
    //                         grid.put_char(ch, fg, bg);
    //                         col += 1;
    //                         if word_wrap && col >= max_len {
    //                             grid.next_line();
    //                             col = 0;
    //                         }
    //                     }

    //                     s = rest; // continue on remaining slice
    //                 }
    //             }

    //             TerminalStringToken::StartFg(c) => fg_state = Some(c),
    //             TerminalStringToken::EndFg => fg_state = None,
    //             TerminalStringToken::StartBg(c) => bg_state = Some(c),
    //             TerminalStringToken::EndBg => bg_state = None,
    //         }
    //     }
    // }
}

// pub fn put_string<T: AsRef<str>>(
//     &mut self,
//     xy: impl Into<PivotedPoint>,
//     string: impl Into<TerminalString<T>>,
// ) {
//     let bounds = self.bounds();
//     let ts: TerminalString<T> = string.into();
//     let clear_tile = self.clear_tile;
//     let clear_colors = ts.decoration.clear_colors;
//     let mut iter = GridStringIterator::new(
//         ts.string.as_ref(),
//         bounds,
//         xy,
//         Some(ts.formatting),
//         Some(ts.decoration),
//     );
//     for (xy, (ch, fg, bg)) in iter.by_ref() {
//         if !self.bounds().contains_point(xy) {
//             continue;
//         }
//         let tile = self.tile_mut(xy);
//         tile.glyph = ch;
//         if clear_colors {
//             tile.fg_color = clear_tile.fg_color;
//             tile.bg_color = clear_tile.bg_color;
//         } else {
//             if let Some(col) = fg {
//                 tile.fg_color = col;
//             }
//             if let Some(col) = bg {
//                 tile.bg_color = col;
//             }
//         }
//     }
// }
