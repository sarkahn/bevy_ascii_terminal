use bevy::color::LinearRgba;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::{
        complete::{is_not, tag, tag_no_case},
        take_while_m_n,
    },
    character::complete::alphanumeric1,
    combinator::map_res,
    sequence::{preceded, terminated},
};

use crate::color::{hex_color, parse_color_string};

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

pub fn parse_tokens(input: &str) -> impl Iterator<Item = TerminalStringToken<'_>> {
    TokenIter::new(input)
}

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

        match parser.parse(self.rest) {
            Ok((next, tok)) => {
                self.rest = next;
                Some(tok)
            }
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Terminal;

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
}
