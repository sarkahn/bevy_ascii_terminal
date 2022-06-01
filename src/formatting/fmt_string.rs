use arrayvec::{ArrayVec};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum StringWrite {
    FgColor(Color),
    BgColor(Color),
}

/// A trait for building a formatted terminal string.
pub trait StringWriter<'a>: Clone {
    fn string(self) -> &'a str;
    /// Change the foreground color.
    fn fg(self, color: Color) -> FormattedString<'a>;
    /// Change the background color.
    fn bg(self, color: Color) -> FormattedString<'a>;

    fn formatted(self) -> FormattedString<'a>;
}

#[derive(Default, Clone)]
pub struct FormattedString<'a> {
    string: &'a str,
    writes: ArrayVec<StringWrite, 2>,
}

impl<'a> FormattedString<'a> {
    fn new(string: &'a str) -> Self {
        FormattedString {
            string,
            ..Default::default()
        }
    }
}

impl<'a> StringWriter<'a> for FormattedString<'a> {
    fn string(self) -> &'a str {
        self.string
    }

    fn fg(mut self, color: Color) -> FormattedString<'a> {
        self.writes.push(StringWrite::FgColor(color));
        self
    }

    fn bg(mut self, color: Color) -> FormattedString<'a> {
        self.writes.push(StringWrite::BgColor(color));
        self
    }

    fn formatted(self) -> FormattedString<'a> {
        self
    }
} 

impl<'a> StringWriter<'a> for &'a str {
    fn string(self) -> &'a str {
        self
    }

    fn fg(self, color: Color) -> FormattedString<'a> {
        let mut fmt = FormattedString::new(self);
        fmt.writes.push(StringWrite::FgColor(color));
        fmt
    }

    fn bg(self, color: Color) -> FormattedString<'a> {
        let mut fmt = FormattedString::new(self);
        fmt.writes.push(StringWrite::BgColor(color));
        fmt
    }

    fn formatted(self) -> FormattedString<'a> {
        FormattedString::new(self)
    }
} 

impl<'a> From<FormattedString<'a>> for (&'a str, ArrayVec<StringWrite,2>) {
    fn from(fmt: FormattedString<'a>) -> Self {
        (fmt.string,fmt.writes)
    }
}