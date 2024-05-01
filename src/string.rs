use std::{ops::Sub, str::Chars};

use bevy::{math::IVec2, render::color::Color};

use crate::{GridRect, Pivot, PivotedPoint};

#[derive(Debug, Clone, Copy)]
pub struct Formatting {
    pub(crate) word_wrapped: bool,
    pub(crate) ignore_spaces: bool,
    pub(crate) fg_color: Option<Color>,
    pub(crate) bg_color: Option<Color>,
}

impl Default for Formatting {
    fn default() -> Self {
        Self {
            word_wrapped: true,
            ignore_spaces: Default::default(),
            fg_color: Default::default(),
            bg_color: Default::default(),
        }
    }
}

#[derive(Default)]
/// A string for writing to a terminal with optional formatting applied.
pub struct FormattedString<'a> {
    pub(crate) string: &'a str,
    pub(crate) formatting: Formatting,
}

/// Allows you to customize a string before it gets written to the terminal.
pub trait StringFormatter<'a> {
    /// By default any string written to the terminal will be wrapped at any
    /// newline and also "word wrapped". If disabled, strings will only be
    /// wrapped at newlines and the terminal edge.
    fn no_word_wrap(self) -> FormattedString<'a>;

    /// Set the foreground color for the string tiles
    fn fg(self, color: Color) -> FormattedString<'a>;

    /// Set the background color for the string tiles
    fn bg(self, color: Color) -> FormattedString<'a>;

    /// If set then no colors or glyphs will be written for space (' ')
    /// characters.
    fn ignore_spaces(self) -> FormattedString<'a>;

    fn string(&self) -> &'a str;
    fn formatting(&self) -> Formatting;
}

impl<'a> StringFormatter<'a> for &'a str {
    fn no_word_wrap(self) -> FormattedString<'a> {
        FormattedString {
            string: self,
            formatting: Formatting {
                word_wrapped: false,
                ..Default::default()
            },
        }
    }

    fn fg(self, color: Color) -> FormattedString<'a> {
        FormattedString {
            string: self,
            formatting: Formatting {
                fg_color: Some(color),
                ..Default::default()
            },
        }
    }

    fn bg(self, color: Color) -> FormattedString<'a> {
        FormattedString {
            string: self,
            formatting: Formatting {
                bg_color: Some(color),
                ..Default::default()
            },
        }
    }

    fn ignore_spaces(self) -> FormattedString<'a> {
        FormattedString {
            string: self,
            formatting: Formatting {
                ignore_spaces: true,
                ..Default::default()
            },
        }
    }

    fn string(&self) -> &'a str {
        self
    }

    fn formatting(&self) -> Formatting {
        Formatting::default()
    }
}

impl<'a> StringFormatter<'a> for &'a String {
    fn no_word_wrap(self) -> FormattedString<'a> {
        FormattedString {
            string: self,
            formatting: Formatting {
                word_wrapped: false,
                ..Default::default()
            },
        }
    }

    fn fg(self, color: Color) -> FormattedString<'a> {
        FormattedString {
            string: self,
            formatting: Formatting {
                fg_color: Some(color),
                ..Default::default()
            },
        }
    }

    fn bg(self, color: Color) -> FormattedString<'a> {
        FormattedString {
            string: self,
            formatting: Formatting {
                bg_color: Some(color),
                ..Default::default()
            },
        }
    }

    fn ignore_spaces(self) -> FormattedString<'a> {
        FormattedString {
            string: self,
            formatting: Formatting {
                ignore_spaces: true,
                ..Default::default()
            },
        }
    }

    fn string(&self) -> &'a str {
        self
    }

    fn formatting(&self) -> Formatting {
        Formatting::default()
    }
}

impl<'a> StringFormatter<'a> for FormattedString<'a> {
    fn no_word_wrap(mut self) -> FormattedString<'a> {
        self.formatting.word_wrapped = false;
        self
    }

    fn fg(mut self, color: Color) -> FormattedString<'a> {
        self.formatting.fg_color = Some(color);
        self
    }

    fn bg(mut self, color: Color) -> FormattedString<'a> {
        self.formatting.bg_color = Some(color);
        self
    }

    fn ignore_spaces(mut self) -> FormattedString<'a> {
        self.formatting.ignore_spaces = true;
        self
    }

    fn string(&self) -> &'a str {
        self.string
    }

    fn formatting(&self) -> Formatting {
        self.formatting
    }
}

impl<'a> From<&'static str> for FormattedString<'a> {
    fn from(value: &'static str) -> Self {
        FormattedString {
            string: value,
            ..Default::default()
        }
    }
}

impl<'a> From<&'static String> for FormattedString<'a> {
    fn from(value: &'static String) -> Self {
        FormattedString {
            string: value,
            ..Default::default()
        }
    }
}

impl<'a> AsRef<str> for FormattedString<'a> {
    fn as_ref(&self) -> &str {
        self.string
    }
}

/// Attempts to wrap a string at either the first newline before `max_len` or the
/// next available whitespace back from `max_len`. Returns None from an empty string.
///
/// Will wrap at `max_len` if no whitespace is available.
///
/// Wrapped lines will always have trailing whitespace removed.
///
/// The return value is a tuple of `(wrapped line, remaining string)`
fn wrap_string(string: &str, max_len: usize) -> Option<(&str, &str)> {
    if string.trim_end().is_empty() {
        return None;
    }

    // Handle newlines first
    if let Some(newline_index) = //string[0..max_len.min(string.len())]
        //.chars()
        string.chars().take(max_len).position(|c| c == '\n')
    {
        let (a, b) = string.split_at(newline_index + 1);
        return Some((a.trim_end(), b));
    };

    let len = string.chars().count();

    if len <= max_len {
        return Some((string.trim_end(), ""));
    };

    let move_back = string
        .chars()
        .rev()
        .skip(len - max_len)
        .position(|c| c.is_whitespace())
        .unwrap_or_default();

    let (a, b) = string.split_at(max_len.sub(move_back));
    Some((a.trim_end(), b))
}

/// Attempts to wrap a string at either the first newline before `max_len` or
/// max_len. Returns [None] from an empty string.
fn split_string(string: &str, max_len: usize) -> Option<(&str, &str)> {
    if string.trim_end().is_empty() {
        return None;
    }
    if let Some(newline_index) = string.chars().take(max_len).position(|c| c == '\n') {
        let (a, b) = string.split_at(newline_index + 1);
        return Some((a.trim_end(), b));
    };

    if string.len() <= max_len {
        return Some((string.trim_end(), ""));
    };

    Some(string.split_at(max_len))
}

/// Calculate the number of tiles to offset a string by horizontally based
/// on it's pivot.
fn horizontal_pivot_offset(pivot: Pivot, line_len: usize) -> i32 {
    match pivot {
        Pivot::TopLeft | Pivot::LeftCenter | Pivot::BottomLeft => 0,
        _ => -(line_len.saturating_sub(1) as f32 * pivot.normalized().x).round() as i32,
    }
}

/// Calculate the amount of lines to offset a wrapped string by based on a pivot
fn wrapped_y_pivot_offset(string: &str, pivot: Pivot, max_width: usize) -> i32 {
    match pivot {
        Pivot::TopLeft | Pivot::TopCenter | Pivot::TopRight => 0,
        _ => {
            let line_count = wrapped_line_count(string, max_width);
            (line_count as f32 * (1.0 - pivot.normalized().y)).round() as i32
        }
    }
}

/// Calculate the number of vertical lines a wrapped string will occupy.
fn wrapped_line_count(mut input: &str, max_len: usize) -> usize {
    let mut line_count = 0;
    while let Some((_, rem)) = wrap_string(input, max_len) {
        line_count += 1;
        input = rem;
    }
    line_count
}

/// Calculate the number of vertical lines a split string will occupy.
fn split_line_count(mut input: &str, max_len: usize) -> usize {
    let mut line_count = 0;
    while let Some((_, rem)) = split_string(input, max_len) {
        line_count += 1;
        input = rem;
    }
    line_count
}

/// Calculate the amount of lines to offset a wrapped string by based on a pivot
fn split_y_pivot_offset(string: &str, pivot: Pivot, max_width: usize) -> i32 {
    match pivot {
        Pivot::TopLeft | Pivot::TopCenter | Pivot::TopRight => 0,
        _ => {
            let line_count = split_line_count(string, max_width);
            (line_count as f32 * (1.0 - pivot.normalized().y)).round() as i32
        }
    }
}

/// An iterator for writing wrapped strings to the terminal. Will attempt
/// to respect formatting and the size of the terminal while yielding
/// each string character and 2d position.
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
        let pivot = xy.pivot().unwrap();
        let offset = xy.point_without_pivot();

        let first_max_len = rect.width().saturating_sub(offset.abs().x as usize);
        let (first, remaining) = if wrapped {
            wrap_string(string, first_max_len)
        } else {
            split_string(string, first_max_len)
        }
        .unwrap_or_default();

        let horizontal_offset = horizontal_pivot_offset(pivot, first.len());
        let vertical_offset = if wrapped {
            wrapped_y_pivot_offset(remaining, pivot, rect.width())
        } else {
            split_y_pivot_offset(remaining, pivot, rect.width())
        };

        let mut xy = xy.calc_from_size(rect.size);

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
        let hor_offset = horizontal_pivot_offset(self.pivot, line_len);
        self.xy.x = x + hor_offset;
        self.xy.y -= 1;
    }
}

impl<'a> Iterator for StringIter<'a> {
    type Item = (IVec2, char);

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.current.next().or_else(|| {
            let (next_line, remaining) = if self.word_wrapped {
                wrap_string(self.remaining, self.rect.width())?
            } else {
                split_string(self.remaining, self.rect.width())?
            };
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
    use super::*;

    #[test]
    fn newline() {
        let area = GridRect::new([0, 0], [40, 40]);
        let mut iter = StringIter::new([0, 0], "A simple string\nWith a newline", area, true);
        let (p, ch) = iter.nth(14).unwrap();
        assert_eq!('g', ch);
        assert_eq!([14, 39], p.to_array());
        let (p, ch) = iter.next().unwrap();
        assert_eq!('W', ch);
        assert_eq!([0, 38], p.to_array());
    }

    #[test]
    fn split() {
        let (split, remaining) = split_string("A simple string\nWith a newline", 12).unwrap();
        assert_eq!("A simple str", split);
        assert_eq!("ing\nWith a newline", remaining);
        let (split, remaining) = split_string(remaining, 12).unwrap();
        assert_eq!("ing", split);
        assert_eq!("With a newline", remaining);
        let (split, remaining) = split_string(remaining, 12).unwrap();
        assert_eq!("With a newli", split);
        assert_eq!("ne", remaining);
        let (split, remaining) = split_string(remaining, 12).unwrap();
        assert_eq!("ne", split);
        assert_eq!("", remaining);
    }

    #[test]
    fn wrap() {
        let (wrapped, remaining) = wrap_string("A simple string\nWith a newline", 12).unwrap();
        assert_eq!("A simple", wrapped);
        assert_eq!("string\nWith a newline", remaining);
        let (wrapped, remaining) = wrap_string(remaining, 12).unwrap();
        assert_eq!("string", wrapped);
        assert_eq!("With a newline", remaining);
        let (wrapped, remaining) = wrap_string(remaining, 12).unwrap();
        assert_eq!("With a", wrapped);
        assert_eq!("newline", remaining);
        let (wrapped, remaining) = wrap_string(remaining, 12).unwrap();
        assert_eq!("newline", wrapped);
        assert_eq!("", remaining);
    }

    #[test]
    fn iter_split() {
        let area = GridRect::new([0, 0], [12, 20]);
        let mut iter = StringIter::new([0, 0], "A simple string\nWith a newline", area, false);
        let (p, ch) = iter.nth(11).unwrap();
        assert_eq!('r', ch);
        assert_eq!([11, 19], p.to_array());
        let (p, ch) = iter.next().unwrap();
        assert_eq!('i', ch);
        assert_eq!([0, 18], p.to_array());
    }

    #[test]
    fn iter_wrap() {
        let area = GridRect::new([0, 0], [12, 20]);
        let mut iter = StringIter::new([0, 0], "A simple string\nWith a newline", area, true);
        let (p, ch) = iter.nth(7).unwrap();
        assert_eq!('e', ch);
        assert_eq!([7, 19], p.to_array());
        let (p, ch) = iter.next().unwrap();
        assert_eq!('s', ch);
        assert_eq!([0, 18], p.to_array());
        let (p, ch) = iter.nth(4).unwrap();
        assert_eq!('g', ch);
        assert_eq!([5, 18], p.to_array());
    }
}
