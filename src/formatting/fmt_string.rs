use std::borrow::Cow;

use arrayvec::ArrayVec;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum StringColor {
    FgColor(Color),
    BgColor(Color),
}

/// A trait for building a formatted terminal string.
pub trait StringWriter<'a>: Clone {
    fn string(&self) -> &str;
    /// Change the foreground color.
    fn fg(self, color: Color) -> FormattedString<'a>; //FormattedString<'a>;
    /// Change the background color.
    fn bg(self, color: Color) -> FormattedString<'a>;

    /// Get the formatted string.
    fn formatted(self) -> FormattedString<'a>;
}

#[derive(Default, Clone)]
pub struct FormattedString<'a> {
    pub string: Cow<'a, str>,
    pub writes: ArrayVec<StringColor, 2>,
}

impl<'a> FormattedString<'a> {
    fn new(string: impl Into<Cow<'a, str>>) -> Self {
        FormattedString {
            string: string.into(),
            ..Default::default()
        }
    }
}

impl<'a> StringWriter<'a> for FormattedString<'a> {
    fn string(&self) -> &str {
        self.string.as_ref()
    }

    fn fg(mut self, color: Color) -> FormattedString<'a> {
        self.writes.push(StringColor::FgColor(color));
        self
    }

    fn formatted(self) -> FormattedString<'a> {
        self
    }

    fn bg(mut self, color: Color) -> FormattedString<'a> {
        self.writes.push(StringColor::BgColor(color));
        self
    }
}

impl<'a> StringWriter<'a> for &'a str {
    fn string(&self) -> &str {
        self
    }

    fn fg(self, color: Color) -> FormattedString<'a> {
        FormattedString::new(self).fg(color)
    }

    fn formatted(self) -> FormattedString<'a> {
        FormattedString::new(self)
    }

    fn bg(self, color: Color) -> FormattedString<'a> {
        FormattedString::new(self).bg(color)
    }
}

impl<'a> StringWriter<'a> for String {
    fn string(&self) -> &str {
        self.as_str()
    }

    fn fg(self, color: Color) -> FormattedString<'a> {
        let mut fmt = FormattedString::new(self);
        fmt.writes.push(StringColor::FgColor(color));
        fmt
    }

    fn formatted(self) -> FormattedString<'a> {
        FormattedString::new(self)
    }

    fn bg(self, color: Color) -> FormattedString<'a> {
        FormattedString::new(self).bg(color)
    }
}

impl<'a> StringWriter<'a> for &'a String {
    fn string(&self) -> &str {
        self.as_str()
    }

    fn fg(self, color: Color) -> FormattedString<'a> {
        FormattedString::new(self.to_owned()).fg(color)
    }

    fn formatted(self) -> FormattedString<'a> {
        FormattedString::new(self.to_owned())
    }

    fn bg(self, color: Color) -> FormattedString<'a> {
        FormattedString::new(self.to_owned()).bg(color)
    }
}

impl<'a> From<FormattedString<'a>> for (Cow<'a, str>, ArrayVec<StringColor, 2>) {
    fn from(fmt: FormattedString<'a>) -> Self {
        (fmt.string, fmt.writes)
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::Color;

    use crate::Terminal;

    use super::StringWriter;

    #[test]
    fn write() {
        let mut term = Terminal::with_size([15, 15]);
        term.put_string([5, 5], "Hello".fg(Color::GREEN));
    }
}
