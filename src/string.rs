use std::{
    ops::{Add, Sub},
    str::{Chars, Lines},
};

use bevy::{math::IVec2, render::color::Color};

use crate::{GridPoint, GridRect, Pivot, PivotedPoint};

/// A string for writing to a terminal with optional formatting applied.
///
/// Note most string types can convert directly into formatted strings with no
/// allocations.
pub struct FormattedString<'a> {
    pub(crate) string: &'a str,
    pub(crate) word_wrapped: bool,
    pub(crate) ignore_spaces: bool,
    pub(crate) fg_color: Option<Color>,
    pub(crate) bg_color: Option<Color>,
}

impl<'a> Default for FormattedString<'a> {
    fn default() -> Self {
        Self {
            string: Default::default(),
            word_wrapped: true,
            ignore_spaces: false,
            fg_color: Default::default(),
            bg_color: Default::default(),
        }
    }
}

/// Allows you to customize a string before it gets written to the terminal.
pub trait StringFormatter<'a> {
    /// By default any string written to the terminal will be wrapped at any
    /// newline and also "word wrapped". If disabled, strings will only be
    /// wrapped at the edge of the terminal.
    fn no_word_wrap(self) -> FormattedString<'a>;

    /// Set the foreground color for the string tiles
    fn fg(self, color: Color) -> FormattedString<'a>;

    /// Set the background color for the string tiles
    fn bg(self, color: Color) -> FormattedString<'a>;

    /// If set then no colors or glyphs will be written for space (' ')
    /// characters.
    fn ignore_spaces(self) -> FormattedString<'a>;
}

impl<'a> StringFormatter<'a> for &'static str {
    fn no_word_wrap(self) -> FormattedString<'a> {
        FormattedString {
            string: self,
            word_wrapped: false,
            ..Default::default()
        }
    }

    fn fg(self, color: Color) -> FormattedString<'a> {
        FormattedString {
            string: self,
            fg_color: Some(color),
            ..Default::default()
        }
    }

    fn bg(self, color: Color) -> FormattedString<'a> {
        FormattedString {
            string: self,
            bg_color: Some(color),
            ..Default::default()
        }
    }

    fn ignore_spaces(self) -> FormattedString<'a> {
        FormattedString {
            string: self,
            ignore_spaces: true,
            ..Default::default()
        }
    }
}

impl<'a> StringFormatter<'a> for &'static String {
    fn no_word_wrap(self) -> FormattedString<'a> {
        FormattedString {
            string: self,
            word_wrapped: false,
            ..Default::default()
        }
    }

    fn fg(self, color: Color) -> FormattedString<'a> {
        FormattedString {
            string: self,
            fg_color: Some(color),
            ..Default::default()
        }
    }

    fn bg(self, color: Color) -> FormattedString<'a> {
        FormattedString {
            string: self,
            bg_color: Some(color),
            ..Default::default()
        }
    }

    fn ignore_spaces(self) -> FormattedString<'a> {
        FormattedString {
            string: self,
            ignore_spaces: true,
            ..Default::default()
        }
    }
}

impl<'a> StringFormatter<'a> for FormattedString<'a> {
    fn no_word_wrap(mut self) -> FormattedString<'a> {
        self.word_wrapped = false;
        self
    }

    fn fg(mut self, color: Color) -> FormattedString<'a> {
        self.fg_color = Some(color);
        self
    }

    fn bg(mut self, color: Color) -> FormattedString<'a> {
        self.bg_color = Some(color);
        self
    }

    fn ignore_spaces(mut self) -> FormattedString<'a> {
        self.ignore_spaces = true;
        self
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
    if let Some(newline_index) = string[..max_len.min(string.len())]
        .chars()
        .position(|c| c == '\n')
    {
        let (a, b) = string.split_at(newline_index + 1);
        return Some((a.trim_end(), b));
    };

    if string.len() <= max_len {
        return Some((string.trim_end(), ""));
    };

    let move_back = string[..max_len]
        .chars()
        .rev()
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
    if let Some(newline_index) = string[..max_len.min(string.len())]
        .chars()
        .position(|c| c == '\n')
    {
        let (a, b) = string.split_at(newline_index + 1);
        return Some((a.trim_end(), b));
    };

    if string.len() <= max_len {
        return Some((string.trim_end(), ""));
    };

    Some(string.split_at(max_len + 1))
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
            (line_count.saturating_sub(1) as f32 * (1.0 - pivot.normalized().y)).round() as i32
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
            (line_count.sub(1) as f32 * (1.0 - pivot.normalized().y)).round() as i32
        }
    }
}

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
        string: impl Into<FormattedString<'a>>,
        rect: GridRect,
    ) -> Self {
        let xy: PivotedPoint = xy.into().with_default_pivot(Pivot::TopLeft);
        let pivot = xy.pivot().unwrap();
        let mut origin = rect.pivot_point(pivot);
        let origin_offset = xy.point_without_pivot() * pivot.axis();

        let fmt: FormattedString = string.into();
        let wrapped = fmt.word_wrapped;

        let first_max_len = rect
            .width()
            .saturating_sub(origin_offset.x.unsigned_abs() as usize);
        let (first, remaining) = if wrapped {
            wrap_string(fmt.string, first_max_len)
        } else {
            split_string(fmt.string, first_max_len)
        }
        .unwrap_or_default();

        let horizontal_offset =
            horizontal_pivot_offset(pivot, first.len() + origin_offset.x.unsigned_abs() as usize);
        let vertical_offset = if wrapped {
            wrapped_y_pivot_offset(remaining, pivot, rect.width())
        } else {
            split_y_pivot_offset(remaining, pivot, rect.width())
        } + origin_offset.y;

        // println!("FIRST LINE: '{}' First remaining: '{}'", first, remaining);
        // println!(
        //     "Pivot: {:?}. origin before offset: {}, origin offset: {:?}, final offset: {}, {}",
        //     pivot, origin, origin_offset, horizontal_offset, vertical_offset
        // );

        origin.x += horizontal_offset;
        origin.y += vertical_offset;

        //println!("XY START {}", origin);

        Self {
            word_wrapped: wrapped,
            remaining,
            current: first.chars(),
            rect,
            pivot,
            xy: origin,
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
    use crate::Terminal;

    use super::*;

    #[test]
    fn write() {
        thing(Pivot::TopLeft);
        thing(Pivot::TopRight);
        thing(Pivot::BottomLeft);
        thing(Pivot::BottomRight);
        thing(Pivot::Center);
    }

    fn thing(pivot: Pivot) {
        let size = [18, 5];
        let mut term = Terminal::with_clear_tile(size, '.'.into());
        let string = StringIter::new(
            [3, 1].pivot(pivot),
            "Hello\nHow are you?",
            GridRect::new([0, 0], size),
        );
        for (xy, ch) in string {
            //println!("{}: {}", xy, ch);
            term.put_char(xy, ch);
        }
        print_terminal(&term);
        println!("----");
    }

    fn print_terminal(term: &Terminal) {
        for row in (0..term.height()).rev() {
            let row = String::from_iter(term.iter_row(row).map(|t| t.glyph));
            println!("{}", row);
        }
    }
}
