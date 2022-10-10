use arrayvec::ArrayVec;
use bevy::prelude::Color;

use std::borrow::Cow;

use crate::Tile;

#[derive(Debug, Clone, Copy)]
pub enum StringModifier {
    FgColor(Color),
    BgColor(Color),
}

/// A trait for building a formatted terminal string.
pub trait StringFormatter<'a>: Clone {
    fn string(&self) -> &str;
    /// Change the foreground color.
    fn fg(self, color: Color) -> FormattedString<'a>; //FormattedString<'a>;
    /// Change the background color.
    fn bg(self, color: Color) -> FormattedString<'a>;

    /// Get the formatted string.
    fn formatted(self) -> FormattedString<'a>;

    fn apply(&self, tile: &mut Tile);
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

impl<'a> StringFormatter<'a> for FormattedString<'a> {
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

    fn apply(&self, tile: &mut Tile) {
        for write in &self.writes {
            match write {
                StringModifier::FgColor(col) => tile.fg_color = *col,
                StringModifier::BgColor(col) => tile.bg_color = *col,
            }
        }
    }
}

impl<'a> StringFormatter<'a> for &'a str {
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

    fn apply(&self, _tile: &mut Tile) {}
}

impl<'a> StringFormatter<'a> for String {
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

    fn apply(&self, _tile: &mut Tile) {}
}

impl<'a> StringFormatter<'a> for &'a String {
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

    fn apply(&self, _tile: &mut Tile) {}
}

impl<'a> From<FormattedString<'a>> for (Cow<'a, str>, ArrayVec<StringModifier, 3>) {
    fn from(fmt: FormattedString<'a>) -> Self {
        (fmt.string, fmt.writes)
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::Color;
    use sark_grids::{GridPoint, Pivot};

    use crate::Terminal;

    use super::StringFormatter;

    #[test]
    fn string_color() {
        let mut term = Terminal::new([15, 15]);
        term.put_string([5, 5], "Hello".fg(Color::GREEN));

        assert_eq!(term.get_tile([5, 5]).fg_color, Color::GREEN);
    }

    #[test]
    fn pivot_top_right() {
        let mut term = Terminal::new([20, 20]);
        term.put_string([0, 0].pivot(Pivot::TopRight), "Hello");

        assert_eq!("Hello", term.get_string([15, 19], 5));
    }

    #[test]
    fn pivot_top_left() {
        let mut term = Terminal::new([20, 20]);
        term.put_string([0, 0].pivot(Pivot::TopLeft), "Hello");

        assert_eq!("Hello", term.get_string([0, 19], 5));
    }

    #[test]
    fn pivot_center() {
        let mut term = Terminal::new([20, 20]);
        term.put_string([0, 0].pivot(Pivot::Center), "Hello");

        assert_eq!("Hello", term.get_string([7, 9], 5));
    }

    #[test]
    fn pivot_bottom_right() {
        let mut term = Terminal::new([20, 20]);
        term.put_string([0, 0].pivot(Pivot::BottomRight), "Hello");

        assert_eq!("Hello", term.get_string([15, 0], 5));
    }

    #[test]
    fn pivot_multiline_bottom_right() {
        let mut term = Terminal::new([20, 20]);
        term.put_string([0, 0].pivot(Pivot::BottomRight), "Hello\nHow are you?\nOk");

        assert_eq!("Hello", term.get_string([15, 2], 5));
        assert_eq!("How are you?", term.get_string([8, 1], 12));
        assert_eq!("Ok", term.get_string([18, 0], 2));
    }

    #[test]
    fn pivot_multiline_top_right() {
        let mut term = Terminal::new([20, 20]);
        term.put_string([0, 0].pivot(Pivot::TopRight), "Hello\nHow are you?\nOk");

        assert_eq!("Hello", term.get_string([15, 19], 5));
        assert_eq!("How are you?", term.get_string([8, 18], 12));
        assert_eq!("Ok", term.get_string([18, 17], 2));
    }

    #[test]
    fn pivot_multiline_top_left() {
        let mut term = Terminal::new([20, 20]);
        term.put_string([0, 0].pivot(Pivot::TopLeft), "Hello\nHow are you?\nOk");

        assert_eq!("Hello", term.get_string([0, 19], 5));
        assert_eq!("How are you?", term.get_string([0, 18], 12));
        assert_eq!("Ok", term.get_string([0, 17], 2));
    }

    #[test]
    fn pivot_multiline_bottom_left() {
        let mut term = Terminal::new([20, 20]);
        term.put_string([0, 0].pivot(Pivot::BottomLeft), "Hello\nHow are you?\nOk");

        assert_eq!("Hello", term.get_string([0, 2], 5));
        assert_eq!("How are you?", term.get_string([0, 1], 12));
        assert_eq!("Ok", term.get_string([0, 0], 2));
    }

    #[test]
    fn pivot_multiline_center() {
        let mut term = Terminal::new([20, 20]);
        term.put_string([0, 0].pivot(Pivot::Center), "Hello\nHow are you?\nOk");

        assert_eq!("Hello", term.get_string([7, 10], 5));
        assert_eq!("How are you?", term.get_string([4, 9], 12));
        assert_eq!("Ok", term.get_string([9, 8], 2));
    }
}
