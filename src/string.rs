//! Utilities  for writing formatted/decorated strings to the terminal
//! without any extra allocations.
use std::{ops::Sub, str::Chars};

use bevy::{color::LinearRgba, math::IVec2, reflect::Reflect};
use sark_grids::{GridRect, GridSize, Pivot, PivotedPoint};

/// A string with optional [StringDecoration] and [StringFormatting] applied.
///
/// `dont_word_wrap` Can be used to disable word wrapping, which is enabled by
/// default for terminal strings.
///
/// `clear_colors` can be used to set the fg and bg colors of the string
/// tiles to match the terminal's clear tile.
///
/// The `bg` and `fg` methods can be used to set the background and foreground
/// colors of the string tiles if clear_colors isn't set. Otherwise the existing
/// colors in the terminal will remain unchanged.
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

    pub fn ignore_spaces(mut self) -> Self {
        self.formatting.ignore_spaces = true;
        self
    }

    pub fn dont_word_wrap(mut self) -> Self {
        self.formatting.word_wrap = false;
        self
    }
}

/// Optional decoration to be applied to a string being written to a terminal.
#[derive(Default, Debug, Clone, Copy, Reflect)]
pub struct StringDecoration {
    /// An optional foreground color for the string. If set to None then the
    /// terminal's clear tile color will be used.
    pub fg_color: Option<LinearRgba>,
    /// An optional background color for the string. If set to None then the
    /// terminal's clear tile color will be used.
    pub bg_color: Option<LinearRgba>,
    /// An optional pair of delimiters to be placed around the string.
    pub delimiters: (Option<char>, Option<char>),
    /// If true, then the terminal's clear tile colors will be used for the
    /// string. If false then the fg and bg colors will be used if they are set.
    /// Otherwise the existing colors in the terminal will remain unchanged.
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
    /// override the string's fg and bg colors.
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
#[derive(Debug, Clone, Reflect, Copy)]
pub struct StringFormatting {
    /// Defines whether or not 'empty' (" ") tiles will be modified when writing
    /// strings to the terminal. If set to false then decorations will be
    /// applied even to empty tiles.
    ///
    /// Defaults to false.
    // TODO: move to decoration?
    pub ignore_spaces: bool,
    /// Word wrap prevents words from being split across lines.
    ///
    /// Defaults to true.
    pub word_wrap: bool,
}

impl StringFormatting {
    pub fn without_word_wrap() -> Self {
        Self {
            word_wrap: false,
            ..Self::default()
        }
    }
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

    // pub fn clear_colors(self) -> TerminalString<T> {
    //     TerminalString {
    //         string: self.string,
    //         decoration: StringDecoration {
    //             clear_colors: true,
    //             ..Default::default()
    //         },
    //         formatting: self.formatting,
    //     }
    // }
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
            (line_count.saturating_sub(1) as f32 * (1.0 - pivot.normalized().y)).round() as i32
        }
    }
}

/// Wrap a string to fit within a given line length. It will first try to split
/// at the first newline before max_len, then if word_wrap is true, it will
/// split at the last whitespace character before max_len, otherwise the string
/// will be split at max_len.
fn wrap_string(string: &str, max_len: usize, word_wrap: bool) -> Option<(&str, &str)> {
    debug_assert!(
        max_len > 0,
        "max_len for wrap_string must be greater than 0"
    );
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
    remaining: &'a str,
    rect: GridRect,
    xy: IVec2,
    pivot: Pivot,
    current: Chars<'a>,
    formatting: StringFormatting,
    decoration: StringDecoration,
}

impl<'a> StringIter<'a> {
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
            rect.size.contains_point(local_xy),
            "Local xy {} passed to StringIter must be within the bounds of the given rect size {}",
            local_xy,
            rect.size
        );

        let first_max_len = rect.width().saturating_sub(local_xy.x as usize);
        let (first, remaining) =
            wrap_string(string, first_max_len, formatting.word_wrap).unwrap_or_default();

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

    fn line_feed(&mut self, line_len: usize) {
        let x = self.rect.pivot_point(self.pivot).x;
        let hor_offset = hor_pivot_offset(self.pivot, line_len);
        self.xy.x = x + hor_offset;
        self.xy.y -= 1;
    }
}

impl Iterator for StringIter<'_> {
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
                    wrap_string(self.remaining, self.rect.width(), self.formatting.word_wrap)?;

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
    use bevy::utils::HashMap;

    use crate::{GridPoint, GridRect};

    use super::*;

    /// Map each character in the string to it's grid position
    fn make_map(string: StringIter<'_>) -> HashMap<[i32; 2], char> {
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
        let iter = StringIter::new(
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
    fn iter_no_word_wrap() {
        let area = GridRect::new([0, 0], [12, 20]);
        let iter = StringIter::new(
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
        let iter = StringIter::new(
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
        let offset = ver_pivot_offset(string, Pivot::BottomLeft, 12, wrap);
        assert_eq!(6, line_count(string, 12, false));
        assert_eq!(5, offset);
    }

    #[test]
    fn right_pivot() {
        let string = "A somewhat longer line\nWith a newline";
        let area = GridRect::new([0, 0], [12, 20]);
        let iter = StringIter::new(
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
        let iter = StringIter::new(
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
        let iter = StringIter::new(string, area, [0, 0], None, None);
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
        let iter = StringIter::new(string, rect, p, None, None);
        let map = make_map(iter);
        assert_eq!('L', get_char(&map, [-1, 12]));
        assert_eq!('e', get_char(&map, [-1, 11]));
        assert_eq!('f', get_char(&map, [-1, 10]));
        assert_eq!('t', get_char(&map, [-1, 9]));
        assert_eq!('B', get_char(&map, [-1, 8]));
        assert_eq!('o', get_char(&map, [-1, 7]));
        assert_eq!('t', get_char(&map, [-1, 6]));
    }
}
