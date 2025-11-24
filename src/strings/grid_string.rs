use std::{ops::Sub, str::Chars};

use bevy::{color::LinearRgba, math::IVec2};
use sark_grids::{GridPoint, GridRect, GridSize, Pivot, PivotedPoint};

use crate::strings::{StringDecoration, StringFormatting};

/// Precalculate the number of vertical lines a wrapped string will occupy.
// TODO: Integrate with `wrap_string` to avoid the duplicate work
fn line_count(mut input: &str, max_len: usize, word_wrap: bool) -> usize {
    let mut line_count = 0;
    while let Some((_, rem)) = wrap_line(input, max_len, word_wrap) {
        line_count += 1;
        input = rem;
    }
    line_count
}

/// Calculate the number of characters to offset a line by horizontally based
/// on it's pivot and length.
fn hor_pivot_offset(pivot: Pivot, line_len: usize) -> i32 {
    match pivot {
        Pivot::TopLeft | Pivot::LeftCenter | Pivot::BottomLeft => 0,
        _ => -(line_len.saturating_sub(1) as f32 * pivot.normalized().x).round() as i32,
    }
}

/// Calculate the amount of vertical lines to offset a wrapped string by based
/// on the pivot.
fn ver_pivot_offset(string: &str, pivot: Pivot, max_width: usize, word_wrap: bool) -> i32 {
    match pivot {
        Pivot::TopLeft | Pivot::TopCenter | Pivot::TopRight => 0,
        _ => {
            let line_count = line_count(string, max_width, word_wrap);
            (line_count.saturating_sub(1) as f32 * (1.0 - pivot.normalized().y)).round() as i32
        }
    }
}

/// Wrap the next line of a string to fit within the given line length. It will
/// first try to split at the first newline before max_len, then if word_wrap is
/// true, it will split at the last whitespace character before max_len,
/// otherwise the string will be split at max_len.
///
/// Returns a tuple with the wrapped line and the remaining text in the string.
fn wrap_line(string: &str, max_len: usize, word_wrap: bool) -> Option<(&str, &str)> {
    debug_assert!(max_len > 0, "max_len for wrap_line must be greater than 0");
    if string.trim_end().is_empty() {
        return None;
    }

    // Handle newlines first
    if let Some(newline_index) = string
        // Accounts for unicode chars, this can panic if using string.find
        .char_indices()
        .take(max_len)
        .find(|(_, c)| *c == '\n')
        .map(|(i, _)| i)
    {
        let (a, b) = string.split_at(newline_index);
        return Some((a.trim_end(), b.trim_start()));
    };

    let len = string.chars().count();
    if len <= max_len {
        return Some((string.trim_end(), ""));
    };

    let mut move_back = if word_wrap {
        string
            .chars()
            .rev()
            .skip(len - max_len - 1)
            .position(|c| c.is_whitespace())
            .unwrap_or_default()
    } else {
        0
    };
    // Make sure we only split on unicode char boundaries
    while !string.is_char_boundary(max_len.sub(move_back)) {
        move_back += 1;
    }

    let (a, b) = string.split_at(max_len.sub(move_back));
    Some((a.trim_end(), b.trim_start()))
}

/// An iterator for positioning string characters on a sized rectangular grid.
/// Will attempt to respect string formatting and the size of the given area while
/// yielding each string character and grid position.
///
/// The iterator will always wrap at newlines and will strip leading and trailing
/// whitespace past the first line.
pub struct GridStringIterator<'a> {
    remaining: &'a str,
    rect: GridRect,
    /// The current local grid position of the iterator
    xy: IVec2,
    pivot: Pivot,
    /// Iterator over the current line
    current: Chars<'a>,
    formatting: StringFormatting,
    decoration: StringDecoration,
}

impl<'a> GridStringIterator<'a> {
    pub fn new(
        string: &'a str,
        rect: GridRect,
        local_xy: impl Into<PivotedPoint>,
        formatting: Option<StringFormatting>,
        decoration: Option<StringDecoration>,
    ) -> Self {
        let pivoted_point: PivotedPoint = local_xy.into().with_default_pivot(Pivot::TopLeft);
        let pivot = pivoted_point.pivot.unwrap();
        let local_xy = pivoted_point.point;

        let formatting = formatting.unwrap_or_default();
        let decoration = decoration.unwrap_or_default();

        debug_assert!(
            rect.size
                .contains_point(local_xy.pivot(pivot).calculate(rect.size)),
            "Local position {} passed to StringIter must be within the bounds of the given rect size {}",
            local_xy,
            rect.size
        );

        let first_max_len = rect
            .width()
            .saturating_sub(local_xy.x.unsigned_abs() as usize);
        let (first, remaining) =
            wrap_line(string, first_max_len, formatting.word_wrap).unwrap_or_default();

        let horizontal_offset = hor_pivot_offset(pivot, first.len());
        let vertical_offset = ver_pivot_offset(string, pivot, rect.width(), formatting.word_wrap);

        let mut xy = rect.pivoted_point(pivoted_point);

        xy.x += horizontal_offset;
        xy.y += vertical_offset;

        Self {
            remaining,
            rect,
            xy,
            pivot,
            current: first.chars(),
            formatting,
            decoration,
        }
    }

    /// Move the xy position to the next line of text in the string
    fn line_feed(&mut self, line_len: usize) {
        let x = self.rect.pivot_point(self.pivot).x;
        let hor_offset = hor_pivot_offset(self.pivot, line_len);
        self.xy.x = x + hor_offset;
        self.xy.y -= 1;
    }
}

impl Iterator for GridStringIterator<'_> {
    type Item = (IVec2, (char, Option<LinearRgba>, Option<LinearRgba>));

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self
            .decoration
            .delimiters
            .0
            .take()
            .or_else(|| self.current.next())
            .or_else(|| {
                let (next_line, remaining) =
                    wrap_line(self.remaining, self.rect.width(), self.formatting.word_wrap)?;

                self.line_feed(next_line.len());
                if self.xy.y < 0 {
                    return None;
                }
                self.remaining = remaining;
                self.current = next_line.chars();
                self.current.next()
            })
            .or_else(|| self.decoration.delimiters.1.take())?;
        let p = self.xy;
        self.xy.x += 1;
        if ch == ' ' && self.formatting.ignore_spaces {
            return self.next();
        }
        let fg = self.decoration.fg_color;
        let bg = self.decoration.bg_color;
        Some((p, (ch, fg, bg)))
    }
}

#[cfg(test)]
mod tests {
    use bevy_platform::collections::HashMap;

    use crate::{GridPoint, GridRect, ascii, strings::formatting::StringFormatting};

    use super::*;

    /// Map each character in the string to it's grid position
    fn make_map(string: GridStringIterator<'_>) -> HashMap<[i32; 2], char> {
        string.map(|(p, (ch, _, _))| (p.to_array(), ch)).collect()
    }

    fn get_char(map: &HashMap<[i32; 2], char>, xy: [i32; 2]) -> char {
        *map.get(&xy).unwrap_or(&' ')
    }

    fn read_string(map: &HashMap<[i32; 2], char>, xy: [i32; 2], len: usize) -> String {
        (0..len)
            .map(|i| get_char(map, [xy[0] + i as i32, xy[1]]))
            .collect()
    }

    #[test]
    fn word_wrap() {
        let rem = "Use wasd to resize terminal";
        let (split, rem) = wrap_line(rem, 8, true).unwrap();
        assert_eq!("Use wasd", split);
        assert_eq!("to resize terminal", rem);
        let (split, rem) = wrap_line(rem, 8, true).unwrap();
        assert_eq!("to", split);
        assert_eq!("resize terminal", rem);
        let (split, rem) = wrap_line(rem, 8, true).unwrap();
        assert_eq!("resize", split);
        assert_eq!("terminal", rem);
        let (split, rem) = wrap_line(rem, 8, true).unwrap();
        assert_eq!("terminal", split);
        assert_eq!("", rem);
    }

    #[test]
    fn iter_newline() {
        let area = GridRect::new([0, 0], [40, 40]);
        let iter = GridStringIterator::new(
            "A simple string\nWith a newline",
            area,
            [0, 0],
            Some(StringFormatting {
                word_wrap: true,
                ..Default::default()
            }),
            None,
        );
        let map = make_map(iter);
        assert_eq!('g', get_char(&map, [14, 39]));
        assert_eq!('W', get_char(&map, [0, 38]))
    }

    #[test]
    fn newline_line_wrap() {
        let (split, remaining) = wrap_line("A simple string\nWith a newline", 12, false).unwrap();
        assert_eq!("A simple str", split);
        assert_eq!("ing\nWith a newline", remaining);
        let (split, remaining) = wrap_line(remaining, 12, false).unwrap();
        assert_eq!("ing", split);
        assert_eq!("With a newline", remaining);
        let (split, remaining) = wrap_line(remaining, 12, false).unwrap();
        assert_eq!("With a newli", split);
        assert_eq!("ne", remaining);
        let (split, remaining) = wrap_line(remaining, 12, false).unwrap();
        assert_eq!("ne", split);
        assert_eq!("", remaining);
    }

    #[test]
    fn newline_word_wrap() {
        let (wrapped, remaining) = wrap_line("A simple string\nWith a newline", 12, true).unwrap();
        assert_eq!("A simple", wrapped);
        assert_eq!("string\nWith a newline", remaining);
        let (wrapped, remaining) = wrap_line(remaining, 12, true).unwrap();
        assert_eq!("string", wrapped);
        assert_eq!("With a newline", remaining);
        let (wrapped, remaining) = wrap_line(remaining, 12, true).unwrap();
        assert_eq!("With a", wrapped);
        assert_eq!("newline", remaining);
        let (wrapped, remaining) = wrap_line(remaining, 12, true).unwrap();
        assert_eq!("newline", wrapped);
        assert_eq!("", remaining);
    }

    #[test]
    fn iter_no_word_wrap() {
        let area = GridRect::new([0, 0], [12, 20]);
        let iter = GridStringIterator::new(
            "A simple string\nWith a newline",
            area,
            [0, 0],
            Some(StringFormatting::without_word_wrap()),
            None,
        );
        let map = make_map(iter);
        assert_eq!("A simple str", read_string(&map, [0, 19], 12));
        assert_eq!("ing", read_string(&map, [0, 18], 3));
        assert_eq!("With a newli", read_string(&map, [0, 17], 12));
        assert_eq!("ne", read_string(&map, [0, 16], 2));
    }

    #[test]
    fn iter_word_wrap() {
        let area = GridRect::new([0, 0], [12, 20]);
        let iter = GridStringIterator::new(
            "A simple string\nWith a newline",
            area,
            [0, 0],
            Some(StringFormatting {
                word_wrap: true,
                ..Default::default()
            }),
            None,
        );
        let map = make_map(iter);
        assert_eq!("A simple", read_string(&map, [0, 19], 8));
        assert_eq!("string", read_string(&map, [0, 18], 6));
        assert_eq!("With a", read_string(&map, [0, 17], 6));
        assert_eq!("newline", read_string(&map, [0, 16], 7));
    }

    #[test]
    fn wrap_line_count() {
        let string = "A somewhat longer line\nWith a newline or two\nOkay? WHEEEEEE.";
        assert_eq!(7, line_count(string, 12, true));
        assert_eq!(6, line_count(string, 12, false));
    }

    #[test]
    fn y_offset_wrap() {
        let string = "A somewhat longer line\nWith a newline or two\nOkay? WHEEEEEE.";
        let line_len = 12;
        let wrap = true;
        let offset = ver_pivot_offset(string, Pivot::TopLeft, line_len, wrap);
        assert_eq!(0, offset);
        assert_eq!(7, line_count(string, 12, wrap));
        assert_eq!(6, ver_pivot_offset(string, Pivot::BottomLeft, 12, wrap));
    }

    #[test]
    fn y_offset_no_wrap() {
        let string = "A somewhat longer line\nWith a newline or two\nOkay? WHEEEEEE.";
        let line_len = 12;
        let wrap = false;
        let offset = ver_pivot_offset(string, Pivot::TopLeft, line_len, wrap);
        assert_eq!(0, offset);
        let offset = ver_pivot_offset(string, Pivot::BottomLeft, line_len, wrap);
        assert_eq!(6, line_count(string, line_len, false));
        assert_eq!(5, offset);
    }

    #[test]
    fn right_pivot() {
        let string = "A somewhat longer line\nWith a newline";
        let area = GridRect::new([0, 0], [12, 20]);
        let iter = GridStringIterator::new(
            string,
            area,
            [0, 0].pivot(Pivot::TopRight),
            Some(StringFormatting {
                word_wrap: true,
                ..Default::default()
            }),
            None,
        );
        let map = make_map(iter);
        let assert_string_location = |string: &str, xy: [i32; 2]| {
            assert_eq!(string, read_string(&map, xy, string.len()));
        };
        assert_string_location("A somewhat", [2, 19]);
        assert_string_location("longer line", [1, 18]);
        assert_string_location("With a", [6, 17]);
        assert_string_location("newline", [5, 16]);
    }

    #[test]
    fn delimiters() {
        let string = "A simple string";
        let area = GridRect::new([0, 0], [20, 5]);
        let iter = GridStringIterator::new(
            string,
            area,
            [0, 0],
            None,
            Some(StringDecoration {
                delimiters: (Some('['), Some(']')),
                ..Default::default()
            }),
        );
        let map = make_map(iter);
        assert_eq!("[A simple string]", read_string(&map, [0, 4], 17));
    }

    #[test]
    fn one_wide() {
        let string = "Abcdefg";
        let area = GridRect::new([0, 0], [1, 7]);
        let iter = GridStringIterator::new(string, area, [0, 0], None, None);
        let map = make_map(iter);
        assert_eq!('A', get_char(&map, [0, 6]));
        assert_eq!('b', get_char(&map, [0, 5]));
        assert_eq!('c', get_char(&map, [0, 4]));
        assert_eq!('d', get_char(&map, [0, 3]));
        assert_eq!('e', get_char(&map, [0, 2]));
        assert_eq!('f', get_char(&map, [0, 1]));
        assert_eq!('g', get_char(&map, [0, 0]));
    }

    #[test]
    fn leftbot() {
        let string = "LeftBot";
        let p = [0, 0].pivot(Pivot::BottomLeft);
        let rect = GridRect::new([-1, 6], [1, 40]);
        let iter = GridStringIterator::new(string, rect, p, None, None);
        let map = make_map(iter);
        assert_eq!('L', get_char(&map, [-1, 12]));
        assert_eq!('e', get_char(&map, [-1, 11]));
        assert_eq!('f', get_char(&map, [-1, 10]));
        assert_eq!('t', get_char(&map, [-1, 9]));
        assert_eq!('B', get_char(&map, [-1, 8]));
        assert_eq!('o', get_char(&map, [-1, 7]));
        assert_eq!('t', get_char(&map, [-1, 6]));
    }

    #[test]
    fn centered() {
        let string = "Hello\nThere";
        let p = [0, 0].pivot(Pivot::Center);
        let rect = GridRect::new([0, 0], [11, 11]);
        let iter = GridStringIterator::new(string, rect, p, None, None);
        let map = make_map(iter);
        assert_eq!('H', get_char(&map, [3, 6]));
        assert_eq!('e', get_char(&map, [4, 6]));
        assert_eq!('l', get_char(&map, [5, 6]));
        assert_eq!('l', get_char(&map, [6, 6]));
        assert_eq!('o', get_char(&map, [7, 6]));
    }

    #[test]
    fn wrap_after_unicode() {
        let mut string = String::with_capacity(ascii::CP_437_ARRAY.len() * 2);
        for ch in ascii::CP_437_ARRAY.iter() {
            string.push(*ch);
            string.push('\n');
        }
        let iter = GridStringIterator::new(
            &string,
            GridRect::new([0, 0], [10, 500]),
            [0, 0],
            None,
            None,
        );
        iter.count();
    }
}
