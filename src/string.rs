use std::{ops::Sub, str::Chars};

use bevy::{color::LinearRgba, math::IVec2};

use crate::{GridRect, Pivot, PivotedPoint};

/// Optional decoration for a string being written to a terminal.
#[derive(Debug, Clone, Copy, Default)]
pub struct StringDecoration {
    /// Sets the foreground color for string tiles.
    pub fg_color: Option<LinearRgba>,
    /// Sets the background color for string tiles.
    pub bg_color: Option<LinearRgba>,
    pub delimiters: [Option<char>; 2],
}

/// A trait for creating a [DecoratedString].
pub trait StringDecorator<T: AsRef<str>> {
    /// Sets the foreground color for string tiles.
    fn fg(self, color: impl Into<LinearRgba>) -> DecoratedString<T>;
    /// Sets the background color for string tiles.
    fn bg(self, color: impl Into<LinearRgba>) -> DecoratedString<T>;
    /// Adds delimiter characters to the front and back of a string.
    ///
    /// The string is expected to be one or two characters.
    fn delimiters(self, delimiters: impl AsRef<str>) -> DecoratedString<T>;

    fn get_decorated_string(self) -> DecoratedString<T>;
    fn parse_delimiters(&self, string: impl AsRef<str>) -> [Option<char>; 2] {
        let mut chars = string.as_ref().chars();
        [chars.next(), chars.next()]
    }
}

/// A string with optional [StringDecoration].
#[derive(Default)]
pub struct DecoratedString<T: AsRef<str>> {
    pub string: T,
    pub decoration: StringDecoration,
}

impl<T: AsRef<str>> DecoratedString<T> {
    /// Define [StringFormatting::word_wrap] for the string.
    pub fn word_wrap(self, word_wrap: bool) -> DecoratedFormattedText<T> {
        let mut dft: DecoratedFormattedText<T> = self.into();
        dft.formatting.word_wrap = word_wrap;
        dft
    }

    /// Define [StringFormatting::ignore_spaces] for the string.
    pub fn ignore_spaces(self, ignore_spaces: bool) -> DecoratedFormattedText<T> {
        let mut dft: DecoratedFormattedText<T> = self.into();
        dft.formatting.ignore_spaces = ignore_spaces;
        dft
    }
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

    fn delimiters(self, delimiters: impl AsRef<str>) -> DecoratedString<T> {
        let delimiters = self.parse_delimiters(delimiters);
        DecoratedString {
            string: self,
            decoration: StringDecoration {
                delimiters,
                ..Default::default()
            },
        }
    }

    fn get_decorated_string(self) -> DecoratedString<T> {
        DecoratedString {
            string: self,
            decoration: Default::default(),
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

    fn delimiters(mut self, delimiters: impl AsRef<str>) -> DecoratedString<T> {
        self.decoration.delimiters = self.parse_delimiters(delimiters);
        self
    }

    fn get_decorated_string(self) -> DecoratedString<T> {
        self
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StringFormatting {
    /// Defines whether or not strings written to the terminal will be word wrapped.
    ///
    /// Defaults to true.
    pub word_wrap: bool,

    /// Defines whether or not 'empty' tiles will be modified when writing strings
    /// to the terminal. If set to true, then empty tiles will be ignored
    /// when writing strings to the terminal. Otherwise, decorations will be
    /// applied even to empty tiles.
    ///
    /// Defaults to false.
    pub ignore_spaces: bool,
}

pub struct FormattedString<T: AsRef<str>> {
    pub string: T,
    pub formatting: StringFormatting,
}

impl<T: AsRef<str>> FormattedString<T> {
    pub fn bg(self, color: impl Into<LinearRgba>) -> DecoratedFormattedText<T> {
        let mut dft = DecoratedFormattedText::from(self);
        dft.decoration.bg_color = Some(color.into());
        dft
    }

    pub fn fg(self, color: impl Into<LinearRgba>) -> DecoratedFormattedText<T> {
        let mut dft = DecoratedFormattedText::from(self);
        dft.decoration.fg_color = Some(color.into());
        dft
    }
}

pub trait StringFormatter<T: AsRef<str>> {
    fn word_wrap(self, wrap: bool) -> FormattedString<T>;
    fn ignore_spaces(self, ignore_spaces: bool) -> FormattedString<T>;
    fn get_formatted_string(self) -> FormattedString<T>;
}

impl<T: AsRef<str>> StringFormatter<T> for T {
    fn word_wrap(self, word_wrap: bool) -> FormattedString<T> {
        FormattedString {
            string: self,
            formatting: StringFormatting {
                word_wrap,
                ..Default::default()
            },
        }
    }

    fn ignore_spaces(self, ignore_spaces: bool) -> FormattedString<T> {
        FormattedString {
            string: self,
            formatting: StringFormatting {
                ignore_spaces,
                ..Default::default()
            },
        }
    }

    fn get_formatted_string(self) -> FormattedString<T> {
        FormattedString {
            string: self,
            formatting: Default::default(),
        }
    }
}

impl<T: AsRef<str>> StringFormatter<T> for FormattedString<T> {
    fn word_wrap(mut self, wrap: bool) -> FormattedString<T> {
        self.formatting.word_wrap = wrap;
        self
    }

    fn ignore_spaces(mut self, ignore_spaces: bool) -> FormattedString<T> {
        self.formatting.ignore_spaces = ignore_spaces;
        self
    }

    fn get_formatted_string(self) -> FormattedString<T> {
        self
    }
}

/// Text to be written to the terminal with optional formatting and/or decorations.
pub struct DecoratedFormattedText<T: AsRef<str>> {
    pub string: T,
    pub decoration: StringDecoration,
    pub formatting: StringFormatting,
}

impl<T: AsRef<str>> From<DecoratedString<T>> for DecoratedFormattedText<T> {
    fn from(value: DecoratedString<T>) -> Self {
        DecoratedFormattedText {
            string: value.string,
            decoration: value.decoration,
            formatting: Default::default(),
        }
    }
}

impl<T: AsRef<str>> From<FormattedString<T>> for DecoratedFormattedText<T> {
    fn from(value: FormattedString<T>) -> Self {
        DecoratedFormattedText {
            string: value.string,
            formatting: value.formatting,
            decoration: Default::default(),
        }
    }
}

impl<T: AsRef<str>> From<T> for DecoratedFormattedText<T> {
    fn from(value: T) -> Self {
        DecoratedFormattedText {
            string: value,
            formatting: Default::default(),
            decoration: Default::default(),
        }
    }
}

// impl<T: AsRef<str>> TextFormatter<T> for DecoratedText<T> {
//     fn no_word_wrap(mut self) -> DecoratedText<T> {
//         self.formatting.word_wrap = false;
//         self
//     }

//     fn fg(mut self, color: impl Into<LinearRgba>) -> DecoratedText<T> {
//         self.decoration.fg_color = Some(color.into());
//         self
//     }

//     fn bg(mut self, color: impl Into<LinearRgba>) -> DecoratedText<T> {
//         self.decoration.bg_color = Some(color.into());
//         self
//     }

//     fn ignore_spaces(mut self) -> DecoratedText<T> {
//         self.formatting.ignore_spaces = true;
//         self
//     }

//     fn string(self) -> T {
//         self.text
//     }

//     fn formatting(self) -> TextFormatting {
//         self.formatting
//     }
// }

// /// Allows you to customize text before it gets written to the terminal.
// pub trait TextFormatter<T: AsRef<str>> {
//     /// By default any string written to the terminal will be wrapped at any
//     /// newline and also "word wrapped". If disabled, strings will only be
//     /// wrapped at newlines and the terminal edge.
//     fn no_word_wrap(self) -> DecoratedText<T>;

//     /// Set the foreground color for the string tiles
//     fn fg(self, color: Color) -> DecoratedText<T>;

//     /// Set the background color for the string tiles
//     fn bg(self, color: Color) -> DecoratedText<T>;

//     /// If set then no colors or glyphs will be written for space (' ')
//     /// characters.
//     fn ignore_spaces(self) -> DecoratedText<T>;

//     fn string(self) -> T;
//     fn formatting(self) -> TextFormatting;
// }

// impl<T: AsRef<str>> TextFormatter<T> for T {
//     fn no_word_wrap(self) -> DecoratedText<T> {
//         DecoratedText {
//             text: self,
//             formatting: TextFormatting {
//                 word_wrap: false,
//                 ..Default::default()
//             },
//             decoration: Default::default(),
//         }
//     }

//     fn fg(self, color: Color) -> DecoratedText<T> {
//         DecoratedText {
//             text: self,
//             decoration: StringDecoration {
//                 fg_color: Some(color),
//                 ..Default::default()
//             },
//             formatting: Default::default(),
//         }
//     }

//     fn bg(self, color: Color) -> DecoratedText<T> {
//         DecoratedText {
//             text: self,
//             decoration: StringDecoration {
//                 bg_color: Some(color),
//                 ..Default::default()
//             },
//             formatting: Default::default(),
//         }
//     }

//     fn ignore_spaces(self) -> DecoratedText<T> {
//         DecoratedText {
//             text: self,
//             formatting: TextFormatting {
//                 ignore_spaces: true,
//                 ..Default::default()
//             },
//             decoration: Default::default(),
//         }
//     }

//     fn string(self) -> T {
//         self
//     }

//     fn formatting(self) -> TextFormatting {
//         TextFormatting::default()
//     }
// }

// impl<T: AsRef<str>> From<T> for DecoratedText<T> {
//     fn from(value: T) -> Self {
//         DecoratedText {
//             text: value,
//             decoration: Default::default(),
//             formatting: Default::default(),
//         }
//     }
// }

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

    // fn title_writer<T: AsRef<str>>(string: impl Into<TitleString<T>>) {
    //     let string: TitleString<T> = string.into();
    //     println!("{}", string.string.as_ref());
    // }

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
