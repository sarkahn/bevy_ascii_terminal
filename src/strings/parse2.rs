use bevy::{
    color::LinearRgba,
    math::{IVec2, ivec2},
};
use sark_grids::Pivot;

use crate::{
    Tile,
    strings::{Token, TokenIterator},
};

pub struct Cell {
    pub tile: Tile,
    pub pos: IVec2,
}

pub struct LayoutIterator<'a> {
    text: &'a str,
    start: IVec2,
    xy: IVec2,

    /// Byte index for the start of the current line within text
    line_start: usize,

    fg: LinearRgba,
    bg: LinearRgba,
    default_fg: LinearRgba,
    default_bg: LinearRgba,

    max_width: usize,

    current_line: Option<TokenIterator<'a>>,
    pivot: Pivot,
}

impl<'a> LayoutIterator<'a> {
    pub fn new(
        input: &'a str,
        start: IVec2,
        wrap_width: usize,
        pivot: Pivot,
        default_fg: LinearRgba,
        default_bg: LinearRgba,
    ) -> LayoutIterator {
        LayoutIterator {
            text: input,
            start,
            xy: start,
            line_start: 0,
            fg: default_fg,
            bg: default_bg,
            default_fg,
            default_bg,
            max_width: wrap_width,
            current_line: None,
            pivot,
        }
    }
}

impl<'a> Iterator for LayoutIterator<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(line_iter) = &mut self.current_line
                && let Some(Ok(token)) = line_iter.next()
            {
                // Process token
                match token {
                    Token::Text(text) => {
                        if let Some(ch) = text.chars().next() {
                            let cell = Cell {
                                pos: ivec2(self.xy.x, self.xy.y),
                                tile: Tile::new(ch, self.fg, self.bg),
                            };

                            self.xy.x += 1;

                            return Some(cell);
                        }
                    }

                    Token::Space => {
                        let cell = Cell {
                            pos: ivec2(self.xy.x, self.xy.y),
                            tile: Tile::new(' ', self.fg, self.bg),
                        };

                        self.xy.x += 1;

                        return Some(cell);
                    }

                    Token::Newline => {
                        self.current_line = None;
                        self.xy.y += 1;
                        continue;
                    }

                    Token::FgStart(c) => self.fg = c,
                    Token::BgStart(c) => self.bg = c,
                    Token::FgEnd => self.fg = self.default_fg,
                    Token::BgEnd => self.bg = self.default_bg,
                }
            } else {
                self.current_line = None;
                self.xy.y += 1;
            }

            // Start a new line
            if self.line_start >= self.text.len() {
                return None; // Done
            }

            let (line_end, charcount) = measure_line(self.text, self.line_start, self.max_width);
            let offset = align_offset(self.pivot, charcount);

            self.xy.x = self.start.x + offset;

            self.current_line = Some(TokenIterator::new(&self.text[self.line_start..line_end]));

            self.line_start = line_end;
        }
    }
}

/// Parse ahead to get the line ending byte offset and the character count for
/// the wrapped line
fn measure_line(text: &str, start: usize, max_width: usize) -> (usize, usize) {
    let mut iter = TokenIterator::new(&text[start..]);

    let mut eol_byte = start;
    let mut char_count = 0;

    while let Some(Ok(token)) = iter.next() {
        match token {
            Token::Text(text) => {
                for c in text.chars() {
                    if char_count >= max_width {
                        return (eol_byte, char_count);
                    }
                    char_count += 1;
                    eol_byte += c.len_utf8();
                }
            }

            Token::Space => {
                if char_count + 1 > max_width {
                    return (eol_byte, char_count);
                }
                char_count += 1;
                eol_byte += 1;
            }

            Token::Newline => {
                eol_byte += 1;
                break;
            }

            _ => {}
        }
    }

    (eol_byte, char_count)
}

fn align_offset(pivot: Pivot, line_width: usize) -> i32 {
    let line_width = line_width as i32;
    match pivot {
        Pivot::TopLeft | Pivot::LeftCenter | Pivot::BottomLeft => 0,
        Pivot::BottomCenter | Pivot::Center | Pivot::TopCenter => -line_width / 2,
        Pivot::TopRight | Pivot::RightCenter | Pivot::BottomRight => -line_width,
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::ivec2;
    use sark_grids::Pivot;

    use crate::color;

    use super::LayoutIterator;

    #[test]
    fn wrap() {
        let input = "<fg=blue>Hello</fg> world! Here is a decently long string. Let's see how it gets split.";
        for cell in LayoutIterator::new(
            input,
            ivec2(1, 1),
            25,
            Pivot::TopRight,
            color::WHITE,
            color::BLACK,
        ) {}
    }
}
