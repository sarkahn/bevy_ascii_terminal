use arrayvec::ArrayVec;
use bevy::prelude::*;

/// The foreground or background color of a string being written to the terminal.
#[derive(Debug, Clone, Copy)]
pub enum StringColor {
    FgColor(Color),
    BgColor(Color),
}

/// A terminal string with optional formatting.
#[derive(Debug, Clone)]
pub struct FormattedString<'a> {
    pub str: &'a str,
    pub colors: ArrayVec<StringColor, 2>,
}

impl<'a> FormattedString<'a> {
    pub fn new(str: &'a str) -> Self {
        Self { str, colors: ArrayVec::new() }
    }

    pub fn from_ref_string(string: &'a mut String) -> Self {
        Self { str: string, colors: default() }
    }

    pub fn with_fg_color(mut self, color: Color) -> Self {
        self.colors.push(StringColor::FgColor(color));
        self
    }
    
    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.colors.push(StringColor::BgColor(color));
        self
    }
}

impl<'a> From<FormattedString<'a>> for (&'a str, ArrayVec<StringColor, 2>) {
    fn from(fmt: FormattedString<'a>) -> Self {
        (fmt.str, fmt.colors)
    }
}

/// A trait for building a formatted terminal string.
pub trait StringWriter<'a> {
    fn string(&self) -> &str;
    /// Change the foreground color.
    fn fg(&'a mut self, color: Color) -> FormattedString<'a>;
    /// Change the background color.
    fn bg(&'a mut self, color: Color) -> FormattedString<'a>;

    /// Get the formatted string.
    fn formatted(self) -> FormattedString<'a>;
}


impl<'a> StringWriter<'a> for &'a String {
    fn string(&self) -> &str {
        self
    }

    fn fg(&'a mut self, color: Color) -> FormattedString<'a> {
        FormattedString::new(self).with_fg_color(color)
    }

    fn bg(&'a mut self, color: Color) -> FormattedString<'a> {
        FormattedString::new(self).with_bg_color(color)
    }

    fn formatted(self) -> FormattedString<'a> {
        FormattedString::new(self)
    }
} 

impl<'a> StringWriter<'a> for &'a str {
    fn string(&self) -> &str {
        self
    }

    fn fg(&'a mut self, color: Color) -> FormattedString<'a> {
        FormattedString::new(self).with_fg_color(color)
    }

    fn bg(&'a mut self, color: Color) -> FormattedString<'a> {
        FormattedString::new(self).with_bg_color(color)
    }

    fn formatted(self) -> FormattedString<'a> {
        FormattedString::new(self)
    }
}

impl <'a> StringWriter<'a> for FormattedString<'a> {
    fn string(&self) -> &str {
        &self.str
    }

    fn fg(&'a mut self, color: Color) -> FormattedString<'a> {
        self.colors.push(StringColor::FgColor(color));
        FormattedString {
            str: self.str,
            colors: self.colors.clone(),
        }
    }

    fn bg(&'a mut self, color: Color) -> FormattedString<'a> {
        self.colors.push(StringColor::BgColor(color));
        FormattedString {
            str: self.str,
            colors: self.colors.clone(),
        }
    }

    fn formatted(self) -> FormattedString<'a> {
        self
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
        term.put_string([1,1], "hi");
        term.put_string([5, 5], "Hello".fg(Color::GREEN).bg(Color::BLUE));
        let tile = term.get_tile([5,5]);
        assert_eq!(Color::BLUE, tile.bg_color);
        assert_eq!('H', tile.glyph);
        let string = "Hello".to_string();
        term.put_string([5, 5], &string);

    }
}