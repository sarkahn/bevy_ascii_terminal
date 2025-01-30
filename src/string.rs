use std::{ops::Sub, str::Chars};

use bevy::{color::LinearRgba, math::IVec2, reflect::Reflect};
use sark_grids::{GridRect, Pivot, PivotedPoint};

/// A string with optional [StringDecoration] and [StringFormatting] applied.
#[derive(Default, Debug, Clone)]
pub struct TerminalString<T> {
    pub string: T,
    pub decoration: StringDecoration,
    pub formatting: StringFormatting,
}

impl<T: AsRef<str>> TerminalString<T> {
    pub fn fg(mut self, color: impl Into<LinearRgba>) -> Self {
        self.decoration.fg_color = Some(color.into());
        self
    }

    pub fn bg(mut self, color: impl Into<LinearRgba>) -> Self {
        self.decoration.bg_color = Some(color.into());
        self
    }

    pub fn delimiters(mut self, delimiters: impl AsRef<str>) -> Self {
        let mut chars = delimiters.as_ref().chars();
        self.decoration.delimiters = (chars.next(), chars.next());
        self
    }

    pub fn clear_colors(mut self) -> Self {
        self.decoration.clear_colors = true;
        self
    }
}

/// Optional decoration to be applied to a string being written to a terminal.
#[derive(Default, Debug, Clone, Reflect)]
pub struct StringDecoration {
    pub fg_color: Option<LinearRgba>,
    pub bg_color: Option<LinearRgba>,
    pub delimiters: (Option<char>, Option<char>),
    /// If true the string's foreground and background colors will be set to
    /// match the terminal's clear tile. This will override the fg and bg colors.
    /// colors.
    pub clear_colors: bool,
}

/// A string with optional [StringDecoration].
#[derive(Default)]
pub struct DecoratedString<T: AsRef<str>> {
    pub string: T,
    pub decoration: StringDecoration,
}

/// A trait for creating a [DecoratedString].
pub trait StringDecorator<T: AsRef<str>> {
    /// Sets the foreground color for string tiles.
    fn fg(self, color: impl Into<LinearRgba>) -> DecoratedString<T>;
    /// Sets the background color for string tiles.
    fn bg(self, color: impl Into<LinearRgba>) -> DecoratedString<T>;
    /// Add a pair of delimiters to the string. The first character will be the
    /// opening delimiter and the second character will be the closing delimiter.
    fn delimiters(self, delimiters: impl AsRef<str>) -> DecoratedString<T>;
    /// Sets the string tile colors to match the terminal's clear tile. This will
    /// override any previously set colors.
    fn clear_colors(self) -> DecoratedString<T>;
}

impl<T: AsRef<str>> StringDecorator<T> for T {
    fn fg(self, color: impl Into<LinearRgba>) -> DecoratedString<T> {
        DecoratedString {
            string: self,
            decoration: StringDecoration {
                fg_color: Some(color.into()),
                ..Default::default()
            },
        }
    }

    fn bg(self, color: impl Into<LinearRgba>) -> DecoratedString<T> {
        DecoratedString {
            string: self,
            decoration: StringDecoration {
                bg_color: Some(color.into()),
                ..Default::default()
            },
        }
    }

    fn clear_colors(self) -> DecoratedString<T> {
        DecoratedString {
            string: self,
            decoration: StringDecoration {
                clear_colors: true,
                ..Default::default()
            },
        }
    }

    fn delimiters(self, delimiters: impl AsRef<str>) -> DecoratedString<T> {
        let mut chars = delimiters.as_ref().chars();
        DecoratedString {
            string: self,
            decoration: StringDecoration {
                delimiters: (chars.next(), chars.next()),
                ..Default::default()
            },
        }
    }
}

impl<T: AsRef<str>> StringDecorator<T> for DecoratedString<T> {
    fn fg(mut self, color: impl Into<LinearRgba>) -> DecoratedString<T> {
        self.decoration.fg_color = Some(color.into());
        self
    }

    fn bg(mut self, color: impl Into<LinearRgba>) -> DecoratedString<T> {
        self.decoration.bg_color = Some(color.into());
        self
    }

    fn clear_colors(mut self) -> DecoratedString<T> {
        self.decoration.clear_colors = true;
        self
    }

    fn delimiters(self, delimiters: impl AsRef<str>) -> DecoratedString<T> {
        let mut chars = delimiters.as_ref().chars();
        DecoratedString {
            string: self.string,
            decoration: StringDecoration {
                delimiters: (chars.next(), chars.next()),
                ..self.decoration
            },
        }
    }
}

impl<T: AsRef<str>> DecoratedString<T> {
    pub fn ignore_spaces(self) -> TerminalString<T> {
        TerminalString {
            string: self.string,
            decoration: self.decoration,
            formatting: StringFormatting {
                ignore_spaces: true,
                ..Default::default()
            },
        }
    }
}

impl<T: AsRef<str>> From<T> for DecoratedString<T> {
    fn from(value: T) -> Self {
        DecoratedString {
            string: value,
            decoration: Default::default(),
        }
    }
}

/// Optional formatting to be applied to a string being written to a terminal.
#[derive(Debug, Clone, Reflect)]
pub struct StringFormatting {
    /// Defines whether or not 'empty' (" ") tiles will be modified when writing
    /// strings to the terminal. If set to false then decorations will be
    /// applied even to empty tiles.
    ///
    /// Defaults to false.
    // TODO: move to decoration?
    pub ignore_spaces: bool,
    /// Word wrap prevents words from being split across lines.
    pub word_wrap: bool,
}

impl Default for StringFormatting {
    fn default() -> Self {
        Self {
            ignore_spaces: Default::default(),
            word_wrap: true,
        }
    }
}

#[derive(Default)]
pub struct FormattedString<T: AsRef<str>> {
    pub string: T,
    pub formatting: StringFormatting,
}

pub trait StringFormatter<T: AsRef<str>> {
    fn ignore_spaces(self) -> FormattedString<T>;
    fn dont_word_wrap(self) -> FormattedString<T>;
}

impl<T: AsRef<str>> StringFormatter<T> for T {
    fn ignore_spaces(self) -> FormattedString<T> {
        FormattedString {
            string: self,
            formatting: StringFormatting {
                ignore_spaces: true,
                ..Default::default()
            },
        }
    }

    fn dont_word_wrap(self) -> FormattedString<T> {
        FormattedString {
            string: self,
            formatting: StringFormatting {
                word_wrap: false,
                ..Default::default()
            },
        }
    }
}

impl<T: AsRef<str>> StringFormatter<T> for FormattedString<T> {
    fn ignore_spaces(mut self) -> FormattedString<T> {
        self.formatting.ignore_spaces = true;
        self
    }

    fn dont_word_wrap(mut self) -> FormattedString<T> {
        self.formatting.word_wrap = false;
        self
    }
}

impl<T: AsRef<str>> From<DecoratedString<T>> for TerminalString<T> {
    fn from(value: DecoratedString<T>) -> Self {
        TerminalString {
            string: value.string,
            decoration: value.decoration,
            formatting: Default::default(),
        }
    }
}

impl<T: AsRef<str>> From<FormattedString<T>> for TerminalString<T> {
    fn from(value: FormattedString<T>) -> Self {
        TerminalString {
            string: value.string,
            formatting: value.formatting,
            decoration: Default::default(),
        }
    }
}

impl<T: AsRef<str>> FormattedString<T> {
    pub fn fg(self, color: impl Into<LinearRgba>) -> TerminalString<T> {
        TerminalString {
            string: self.string,
            decoration: StringDecoration {
                fg_color: Some(color.into()),
                ..Default::default()
            },
            formatting: self.formatting,
        }
    }
    pub fn bg(self, color: impl Into<LinearRgba>) -> TerminalString<T> {
        TerminalString {
            string: self.string,
            decoration: StringDecoration {
                bg_color: Some(color.into()),
                ..Default::default()
            },
            formatting: self.formatting,
        }
    }

    pub fn delimiters(self, delimiters: impl AsRef<str>) -> TerminalString<T> {
        let mut chars = delimiters.as_ref().chars();
        TerminalString {
            string: self.string,
            decoration: StringDecoration {
                delimiters: (chars.next(), chars.next()),
                ..Default::default()
            },
            formatting: self.formatting,
        }
    }

    pub fn clear_colors(self) -> TerminalString<T> {
        TerminalString {
            string: self.string,
            decoration: StringDecoration {
                clear_colors: true,
                ..Default::default()
            },
            formatting: self.formatting,
        }
    }
}

impl<T: AsRef<str> + Default> From<T> for TerminalString<T> {
    fn from(value: T) -> Self {
        Self {
            string: value,
            ..Default::default()
        }
    }
}

/// Precalculate the number of vertical lines a wrapped string will occupy.
// TODO: Integrate with `wrap_string` to avoid the duplicate work
fn line_count(mut input: &str, max_len: usize, wrap: bool) -> usize {
    let mut line_count = 0;
    while let Some((_, rem)) = wrap_string(input, max_len, wrap) {
        line_count += 1;
        input = rem;
    }
    line_count
}

/// Calculate the number of tiles to offset a string by horizontally based
/// on it's pivot.
fn hor_pivot_offset(pivot: Pivot, line_len: usize) -> i32 {
    match pivot {
        Pivot::TopLeft | Pivot::LeftCenter | Pivot::BottomLeft => 0,
        _ => -(line_len.saturating_sub(1) as f32 * pivot.normalized().x).round() as i32,
    }
}

/// Calculate the amount of lines to offset a wrapped string by based on a pivot
fn ver_pivot_offset(string: &str, pivot: Pivot, max_width: usize, wrap: bool) -> i32 {
    match pivot {
        Pivot::TopLeft | Pivot::TopCenter | Pivot::TopRight => 0,
        _ => {
            let line_count = line_count(string, max_width, wrap);
            (line_count as f32 * (1.0 - pivot.normalized().y)).round() as i32
        }
    }
}

/// Wrap a string to fit within a given line length. It will first try to split
/// at the first newline before max_len, then if word_wrap is true, it will
/// split at the last whitespace character before max_len, otherwise the string
/// will be split at max_len.
fn wrap_string(string: &str, max_len: usize, word_wrap: bool) -> Option<(&str, &str)> {
    debug_assert!(max_len > 0, "max_len must be greater than 0");
    if string.trim_end().is_empty() {
        return None;
    }

    // Handle newlines first
    if let Some(newline_index) = string.chars().take(max_len).position(|c| c == '\n') {
        let (a, b) = string.split_at(newline_index);
        return Some((a.trim_end(), b.trim_start()));
    };

    let len = string.chars().count();
    if len <= max_len {
        return Some((string.trim_end(), ""));
    };

    let move_back = if word_wrap {
        string
            .chars()
            .rev()
            .skip(len - max_len - 1)
            .position(|c| c.is_whitespace())
            .unwrap_or_default()
    } else {
        0
    };

    let (a, b) = string.split_at(max_len.sub(move_back));
    Some((a.trim_end(), b.trim_start()))
}

/// An iterator for writing wrapped strings to a rectangular grid. Will attempt
/// to respect formatting and the size of the given area while yielding
/// each string character and grid position.
///
/// The iterator will always wrap at newlines and will strip leading and trailing
/// whitespace past the first line.
pub struct StringIter<'a> {
    word_wrapped: bool,
    remaining: &'a str,
    current: Chars<'a>,
    rect: GridRect,
    pivot: Pivot,
    xy: IVec2,
}

impl<'a> StringIter<'a> {
    pub fn new(
        xy: impl Into<PivotedPoint>,
        string: &'a str,
        rect: GridRect,
        wrapped: bool,
    ) -> Self {
        let xy: PivotedPoint = xy.into().with_default_pivot(Pivot::TopLeft);
        let pivot = xy.pivot.unwrap();
        let offset = xy.point;

        let first_max_len = rect.width().saturating_sub(offset.abs().x as usize);
        let (first, remaining) = wrap_string(string, first_max_len, wrapped).unwrap_or_default();

        let horizontal_offset = hor_pivot_offset(pivot, first.len());
        let vertical_offset = ver_pivot_offset(string, pivot, rect.width(), wrapped);

        let mut xy = xy.calculate(rect.size);

        xy.x += horizontal_offset;
        xy.y += vertical_offset;

        Self {
            word_wrapped: wrapped,
            remaining,
            current: first.chars(),
            rect,
            pivot,
            xy,
        }
    }

    fn line_feed(&mut self, line_len: usize) {
        let x = self.rect.pivot_point(self.pivot).x;
        let hor_offset = hor_pivot_offset(self.pivot, line_len);
        self.xy.x = x + hor_offset;
        self.xy.y -= 1;
    }
}

impl Iterator for StringIter<'_> {
    type Item = (IVec2, char);

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.current.next().or_else(|| {
            let (next_line, remaining) =
                wrap_string(self.remaining, self.rect.width(), self.word_wrapped)?;

            // if self.word_wrapped {
            //     wrap_string(self.remaining, self.rect.width())?
            // } else {
            //     split_string(self.remaining, self.rect.width())?
            // };
            self.line_feed(next_line.len());
            if self.xy.y < 0 {
                return None;
            }
            self.remaining = remaining;
            self.current = next_line.chars();
            self.current.next()
        })?;

        let ret = Some((self.xy, ch));
        self.xy.x += 1;
        ret
    }
}

#[cfg(test)]
mod tests {
    use bevy::utils::HashMap;

    use crate::{GridPoint, GridRect};

    use super::*;

    /// Map each character in the string to it's grid position
    fn make_map(string: StringIter<'_>) -> HashMap<[i32; 2], char> {
        string.map(|(p, ch)| (p.to_array(), ch)).collect()
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
        let (split, rem) = wrap_string(rem, 8, true).unwrap();
        assert_eq!("Use wasd", split);
        assert_eq!("to resize terminal", rem);
        let (split, rem) = wrap_string(rem, 8, true).unwrap();
        assert_eq!("to", split);
        assert_eq!("resize terminal", rem);
        let (split, rem) = wrap_string(rem, 8, true).unwrap();
        assert_eq!("resize", split);
        assert_eq!("terminal", rem);
        let (split, rem) = wrap_string(rem, 8, true).unwrap();
        assert_eq!("terminal", split);
        assert_eq!("", rem);
    }

    #[test]
    fn iter_newline() {
        let area = GridRect::new([0, 0], [40, 40]);
        let iter = StringIter::new([0, 0], "A simple string\nWith a newline", area, true);
        let map = make_map(iter);
        assert_eq!('g', get_char(&map, [14, 39]));
        assert_eq!('W', get_char(&map, [0, 38]))
    }

    #[test]
    fn newline_line_wrap() {
        let (split, remaining) = wrap_string("A simple string\nWith a newline", 12, false).unwrap();
        assert_eq!("A simple str", split);
        assert_eq!("ing\nWith a newline", remaining);
        let (split, remaining) = wrap_string(remaining, 12, false).unwrap();
        assert_eq!("ing", split);
        assert_eq!("With a newline", remaining);
        let (split, remaining) = wrap_string(remaining, 12, false).unwrap();
        assert_eq!("With a newli", split);
        assert_eq!("ne", remaining);
        let (split, remaining) = wrap_string(remaining, 12, false).unwrap();
        assert_eq!("ne", split);
        assert_eq!("", remaining);
    }

    #[test]
    fn newline_word_wrap() {
        let (wrapped, remaining) =
            wrap_string("A simple string\nWith a newline", 12, true).unwrap();
        assert_eq!("A simple", wrapped);
        assert_eq!("string\nWith a newline", remaining);
        let (wrapped, remaining) = wrap_string(remaining, 12, true).unwrap();
        assert_eq!("string", wrapped);
        assert_eq!("With a newline", remaining);
        let (wrapped, remaining) = wrap_string(remaining, 12, true).unwrap();
        assert_eq!("With a", wrapped);
        assert_eq!("newline", remaining);
        let (wrapped, remaining) = wrap_string(remaining, 12, true).unwrap();
        assert_eq!("newline", wrapped);
        assert_eq!("", remaining);
    }

    #[test]
    fn iter_line_wrap() {
        let area = GridRect::new([0, 0], [12, 20]);
        let iter = StringIter::new([0, 0], "A simple string\nWith a newline", area, false);
        let map = make_map(iter);
        assert_eq!("A simple str", read_string(&map, [0, 19], 12));
        assert_eq!("ing", read_string(&map, [0, 18], 3));
        assert_eq!("With a newli", read_string(&map, [0, 17], 12));
        assert_eq!("ne", read_string(&map, [0, 16], 2));
    }

    #[test]
    fn iter_word_wrap() {
        let area = GridRect::new([0, 0], [12, 20]);
        let iter = StringIter::new([0, 0], "A simple string\nWith a newline", area, true);
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
    fn y_offset() {
        let string = "A somewhat longer line\nWith a newline or two\nOkay? WHEEEEEE.";
        let line_len = 12;
        let wrap = true;
        let offset = ver_pivot_offset(string, Pivot::TopLeft, line_len, wrap);
        assert_eq!(0, offset);
        let offset = ver_pivot_offset(string, Pivot::BottomLeft, 12, wrap);
        assert_eq!(7, offset);
    }

    #[test]
    fn right_pivot() {
        let string = "A somewhat longer line\nWith a newline";
        let area = GridRect::new([0, 0], [12, 20]);
        let wrap = true;
        let iter = StringIter::new([0, 0].pivot(Pivot::TopRight), string, area, wrap);
        let map = make_map(iter);
        let assert_string_location = |string: &str, xy: [i32; 2]| {
            assert_eq!(string, read_string(&map, xy, string.len()));
        };
        assert_string_location("A somewhat", [2, 19]);
        assert_string_location("longer line", [1, 18]);
        assert_string_location("With a", [6, 17]);
        assert_string_location("newline", [5, 16]);
    }
}
