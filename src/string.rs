use std::{
    ops::Sub,
    str::{Chars, Lines},
};

use bevy::{math::IVec2, render::color::Color};

use crate::{GridRect, Pivot};

pub struct FormattedString<'a> {
    pub(crate) string: &'a str,
    pub(crate) wrapped: bool,
    pub(crate) ignore_spaces: bool,
    pub(crate) fg_color: Option<Color>,
    pub(crate) bg_color: Option<Color>,
}

impl<'a> Default for FormattedString<'a> {
    fn default() -> Self {
        Self {
            string: Default::default(),
            wrapped: true,
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
            wrapped: false,
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
            wrapped: false,
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
        self.wrapped = false;
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

pub enum XyStringIter<'a> {
    Wrapped(WrappedStringIter<'a>),
    NotWrapped(NoWordWrapStringIter<'a>),
}

impl<'a> Iterator for XyStringIter<'a> {
    type Item = (IVec2, char);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            XyStringIter::Wrapped(iter) => iter.next(),
            XyStringIter::NotWrapped(iter) => iter.next(),
        }
    }
}

pub struct WrappedStringIter<'a> {
    rect: GridRect,
    pivot: Pivot,
    xy: IVec2,
    remaining: &'a str,
    chars: Chars<'a>,
}

impl<'a> WrappedStringIter<'a> {
    pub fn new(string: &'a str, rect: GridRect, pivot: Pivot) -> Self {
        let mut xy = rect.pivot_point(pivot);
        let vertical_offset = wrapped_line_count_offset(string, pivot, rect.width());
        // +1 as first iteration will immediately line feed
        xy.y += vertical_offset + 1;
        Self {
            rect,
            pivot,
            xy,
            remaining: string,
            chars: "".chars(),
        }
    }

    fn line_feed(&mut self, line_len: usize) {
        let origin = self.rect.pivot_point(self.pivot);
        let hor_offset = horizontal_pivot_offset(self.pivot, line_len);
        self.xy.x = origin.x + hor_offset;
        self.xy.y -= 1;
    }
}

impl<'a> Iterator for WrappedStringIter<'a> {
    type Item = (IVec2, char);

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.chars.next().or_else(|| {
            let (next_line, remaining) = wrap_string(self.remaining, self.rect.width())?;
            self.line_feed(next_line.len());
            self.remaining = remaining;
            self.chars = next_line.chars();
            self.chars.next()
        })?;
        let ret = Some((self.xy, ch));
        self.xy.x += 1;
        ret
    }
}

// TODO: Need to wrap at width
// TODO: Does this make sense? We wrap on newlines but don't word wrap?
pub struct NoWordWrapStringIter<'a> {
    rect: GridRect,
    pivot: Pivot,
    lines: Lines<'a>,
    chars: Chars<'a>,
    xy: IVec2,
}

impl<'a> Iterator for NoWordWrapStringIter<'a> {
    type Item = (IVec2, char);

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.chars.next().or_else(|| {
            let line = self.lines.next()?;
            let line = line.trim_end();
            self.line_feed(line.len());
            self.chars = line.chars();
            self.chars.next()
        })?;
        let ret = Some((self.xy, ch));
        self.xy.x += 1;
        ret
    }
}

impl<'a> NoWordWrapStringIter<'a> {
    pub fn new(string: &'a str, rect: GridRect, pivot: Pivot) -> Self {
        let mut xy = rect.pivot_point(pivot);
        let vertical_offset = wrapped_line_count_offset(string, pivot, rect.width());
        // +1 as first iteration will immediately line feed
        xy.y += vertical_offset + 1;
        Self {
            rect,
            pivot,
            lines: string.lines(),
            chars: "".chars(),
            xy,
        }
    }

    fn line_feed(&mut self, line_len: usize) {
        let origin = self.rect.pivot_point(self.pivot);
        let hor_offset = horizontal_pivot_offset(self.pivot, line_len);
        self.xy.x = origin.x + hor_offset;
        self.xy.y -= 1;
    }
}

/// Attempts to wrap a string at either the first newline before `max_len` or the
/// next available whitespace from `max_len`. Returns None from an empty string.
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

/// Calculate the number of tiles to offset a string by horizontally based
/// on it's pivot.
fn horizontal_pivot_offset(pivot: Pivot, line_len: usize) -> i32 {
    match pivot {
        Pivot::TopLeft | Pivot::LeftCenter | Pivot::BottomLeft => 0,
        _ => -(line_len.sub(1) as f32 * pivot.normalized().x).round() as i32,
    }
}

/// Calculate the number of vertical lines a wrapped string will occupy.
fn wrapped_line_count_offset(string: &str, pivot: Pivot, max_width: usize) -> i32 {
    match pivot {
        Pivot::TopLeft | Pivot::TopCenter | Pivot::TopRight => 0,
        _ => {
            let line_count = wrapped_line_count(string, max_width);
            (line_count.sub(1) as f32 * (1.0 - pivot.normalized().y)).round() as i32
        }
    }
}

fn wrapped_line_count(mut input: &str, max_len: usize) -> usize {
    let mut line_count = 0;
    while let Some((_, rem)) = wrap_string(input, max_len) {
        line_count += 1;
        input = rem;
    }
    line_count
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

#[cfg(test)]
mod tests {
    use crate::{GridRect, Pivot};

    use super::{wrap_string, WrappedStringIter};

    #[test]
    fn wrap() {
        let (a, b) = wrap_string(
            "hello, how are you doing today? I'm doing well, thanks.",
            30,
        )
        .unwrap();
        assert_eq!("hello, how are you doing", a);
        assert_eq!("today? I'm doing well, thanks.", b);
    }

    #[test]
    fn no_whitespace_wrap() {
        let (a, b) = wrap_string("averylongstringwithnowhitespace", 10).unwrap();
        assert_eq!("averylongs", a);
        assert_eq!("tringwithnowhitespace", b);
    }

    #[test]
    fn newline_wrap() {
        let (a, b) = wrap_string("A string\nwith\nAn early newline.", 17).unwrap();
        assert_eq!("A string", a);
        assert_eq!("with\nAn early newline.", b);

        let (a, b) = wrap_string(b, 17).unwrap();
        assert_eq!("with", a);
        assert_eq!("An early newline.", b);

        let (a, b) = wrap_string(b, 17).unwrap();
        assert_eq!("An early newline.", a);
        assert_eq!("", b);
    }

    #[test]
    fn iter() {
        let rect = GridRect::new([0, 0], [17, 17]);
        let string = "A string\nwith\nAn early newline.";
        let iter = WrappedStringIter::new(string, rect, Pivot::TopLeft);

        let charpoints: Vec<_> = iter.collect();
        let chars = String::from_iter(charpoints.iter().map(|(_, ch)| ch));
        assert_eq!("A stringwithAn early newline.", chars.as_str());

        let points: Vec<_> = charpoints.iter().map(|(p, _)| p.to_array()).collect();

        #[allow(clippy::needless_range_loop)]
        for i in 0..8 {
            let [x, y] = points[i];
            assert_eq!(i as i32, x);
            assert_eq!(16, y);
        }

        for i in 0..4 {
            let [x, y] = points[8 + i];
            assert_eq!(i as i32, x);
            assert_eq!(15, y);
        }

        for i in 0..16 {
            let [x, y] = points[12 + i];
            assert_eq!(i as i32, x);
            assert_eq!(14, y);
        }

        let iter = WrappedStringIter::new(string, rect, Pivot::TopRight);

        let charpoints: Vec<_> = iter.collect();
        let chars = String::from_iter(charpoints.iter().map(|(_, ch)| ch));
        assert_eq!("A stringwithAn early newline.", chars.as_str());

        let points: Vec<_> = charpoints.iter().map(|(p, _)| p.to_array()).collect();

        #[allow(clippy::needless_range_loop)]
        for i in 0..8 {
            let [x, y] = points[i];
            assert_eq!(9 + i as i32, x);
            assert_eq!(16, y);
        }

        for i in 0..4 {
            let [x, y] = points[8 + i];
            assert_eq!(13 + i as i32, x);
            assert_eq!(15, y);
        }

        let iter = WrappedStringIter::new(string, rect, Pivot::BottomRight);

        let charpoints: Vec<_> = iter.collect();
        let chars = String::from_iter(charpoints.iter().map(|(_, ch)| ch));
        assert_eq!("A stringwithAn early newline.", chars.as_str());

        let points: Vec<_> = charpoints.iter().map(|(p, _)| p.to_array()).collect();

        #[allow(clippy::needless_range_loop)]
        for i in 0..8 {
            let [x, y] = points[i];
            assert_eq!(9 + i as i32, x);
            assert_eq!(2, y);
        }

        for i in 0..4 {
            let [x, y] = points[8 + i];
            assert_eq!(13 + i as i32, x);
            assert_eq!(1, y);
        }

        for i in 0..16 {
            let [x, y] = points[12 + i];
            assert_eq!(i as i32, x);
            assert_eq!(0, y);
        }
    }
}
