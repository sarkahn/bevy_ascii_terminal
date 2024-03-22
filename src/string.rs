use std::{ops::Sub, str::Chars};

use bevy::{math::IVec2, render::color::Color};

use crate::{GridRect, Pivot, Terminal};

pub struct FormattedString<'a> {
    string: &'a str,
    wrapped: bool,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
}

impl<'a> Default for FormattedString<'a> {
    fn default() -> Self {
        Self {
            string: Default::default(),
            wrapped: true,
            fg_color: Default::default(),
            bg_color: Default::default(),
        }
    }
}

pub trait StringFormatter<'a> {
    /// By default any string written to the terminal will be wrapped at any
    /// newline and also "word wrapped". If disabled, strings will be written
    /// as is
    fn dont_wrap(self) -> FormattedString<'a>;

    /// Set the foreground color for the string tiles
    fn fg(self, color: Color) -> FormattedString<'a>;

    /// Set the background color for the string tiles
    fn bg(self, color: Color) -> FormattedString<'a>;
}

impl<'a> StringFormatter<'a> for &'static str {
    fn dont_wrap(self) -> FormattedString<'a> {
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
}

impl<'a> StringFormatter<'a> for &'static String {
    fn dont_wrap(self) -> FormattedString<'a> {
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
}

impl<'a> StringFormatter<'a> for FormattedString<'a> {
    fn dont_wrap(mut self) -> FormattedString<'a> {
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
}

pub struct WrappedString<'a> {
    rect: GridRect,
    pivot: Pivot,
    xy: IVec2,
    remaining: &'a str,

    chars: Option<Chars<'a>>,
    chars2: Chars<'a>,
}

impl<'a> Iterator for WrappedString<'a> {
    type Item = (IVec2, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.chars.is_none() {
            if let Some((nextline, remaining)) = wrap_string(self.remaining, self.rect.width()) {
                let len = nextline.len();
                self.remaining = remaining;
                self.chars = Some(nextline.chars());

                let origin = self.rect.pivot_point(self.pivot);
                let len_offset = (len as f32 * self.pivot.normalized().x).round() as i32;
                self.xy.x = origin.x - len_offset;
                self.xy.y -= 1;
            } else {
                return None;
            }
        }

        None

        // if self.x >= self.current_line.len() {
        //     if let Some((nextline, remaining)) = wrap_string(self.remaining, self.rect.width()) {
        //         self.remaining = remaining;
        //         self.chars = nextline.chars();
        //     } else {
        //         return None;
        //     }
        // }
        // let xy = self.rect.min() + IVec2::new(self.x as i32, self.y as i32);
        // Some((xy, ))
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

fn wrapped_line_count(mut input: &str, max_len: usize) -> usize {
    let mut line_count = 0;
    while let Some((_, rem)) = wrap_string(input, max_len) {
        line_count += 1;
        input = rem;
    }
    line_count
}

#[cfg(test)]
mod tests {
    use super::wrap_string;

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
}
