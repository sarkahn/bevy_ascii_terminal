use arrayvec::ArrayVec;
use bevy::prelude::{Color, IVec2, Vec2};
use sark_grids::{Size2d, GridPoint};
use std::borrow::Cow;
use bitflags::bitflags;

bitflags! {
    pub struct Align: u8 {
        const Left =    1 << 1;
        const CenterX = 1 << 2;
        const Right =   1 << 3;
        const Top =     1 << 4;
        const CenterY = 1 << 5;
        const Bottom =  1 << 6;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StringModifier {
    FgColor(Color),
    BgColor(Color),
    /// usize == width to align to
    Aligned(Align, usize),
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
    pub writes: ArrayVec<StringModifier, 3>,
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
        self.writes.push(StringModifier::FgColor(color));
        self
    }

    fn formatted(self) -> FormattedString<'a> {
        self
    }

    fn bg(mut self, color: Color) -> FormattedString<'a> {
        self.writes.push(StringModifier::BgColor(color));
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
        fmt.writes.push(StringModifier::FgColor(color));
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

impl<'a> From<FormattedString<'a>> for (Cow<'a, str>, ArrayVec<StringModifier, 3>) {
    fn from(fmt: FormattedString<'a>) -> Self {
        (fmt.string, fmt.writes)
    }
}

fn get_aligned_point(p: impl GridPoint, size: impl Size2d, align: Vec2) -> IVec2 {
    (p.as_vec2() + size.as_vec2() * align).as_ivec2()
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
