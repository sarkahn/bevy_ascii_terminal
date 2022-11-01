// use bevy::prelude::Color;

// use crate::{Tile, FormattedTile, TileFormatter};

use bevy::{prelude::Color, utils::HashMap};

/// Specifies the style of lines to use along the border of a box.
#[derive(Debug, Clone, PartialEq)]
pub struct Border {
    pub top: char,
    pub left: char,
    pub right: char,
    pub bottom: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub(crate) edge_strings: HashMap<Edge, AlignedString>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct AlignedString {
    pub align: f32,
    pub string: String,
    pub fg_col: Option<Color>,
    pub bg_col: Option<Color>,
}

pub trait AlignedStringFormatter {
    fn fg_col(self, color: Color) -> AlignedString;
    fn bg_col(self, color: Color) -> AlignedString;
    fn aligned(self, align: f32) -> AlignedString;
}

impl AlignedStringFormatter for AlignedString {
    fn fg_col(mut self, color: Color) -> AlignedString {
        self.fg_col = Some(color);
        self
    }

    fn bg_col(mut self, color: Color) -> AlignedString {
        self.bg_col = Some(color);
        self
    }

    fn aligned(mut self, align: f32) -> AlignedString {
        self.align = align;
        self
    }
}

impl Default for Border {
    fn default() -> Self {
        Border::single_line()
    }
}

impl Border {
    /// A border with single-line glyphs.
    pub fn single_line() -> Border {
        Self {
            top: '─',
            left: '│',
            right: '│',
            bottom: '─',
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            edge_strings: Default::default(),
        }
    }

    /// A border with double-line glyphs.
    pub fn double_line() -> Border {
        Border {
            top: '═',
            left: '║',
            right: '║',
            bottom: '═',
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            ..Default::default()
        }
    }

    /// Construct a new [Border] from the given string.
    ///
    /// The format of the string should match the example below. Line returns
    /// and spaces will be ignored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bevy_ascii_terminal::Border;
    /// let border = Border::from_string(
    ///     "┌─┐
    ///      │ │
    ///      └─┘"
    /// );
    /// ```
    pub fn from_string(box_string: impl AsRef<str>) -> Self {
        let mut chars = box_string
            .as_ref()
            .chars()
            .filter(|c| !c.is_whitespace() && *c != '\n');
        debug_assert!(
            chars.clone().count() == 8,
            "Error building border glyphs from string, string should contain
            exactly 8 glyphs (whitespace and line returns are ignored)"
        );

        Border {
            top_left: chars.next().unwrap(),
            top: chars.next().unwrap(),
            top_right: chars.next().unwrap(),
            left: chars.next().unwrap(),
            right: chars.next().unwrap(),
            bottom_left: chars.next().unwrap(),
            bottom: chars.next().unwrap(),
            bottom_right: chars.next().unwrap(),
            ..Default::default()
        }
    }

    /// Returns the glyph for a given border edge.
    pub fn edge_glyph(&self, edge: Edge) -> char {
        match edge {
            Edge::Top => self.top,
            Edge::Left => self.left,
            Edge::Right => self.right,
            Edge::Bottom => self.bottom,
            Edge::TopLeft => self.top_left,
            Edge::TopRight => self.top_right,
            Edge::BottomLeft => self.bottom_left,
            Edge::BottomRight => self.bottom_right,
        }
    }

    pub fn with_title(mut self, title: impl Into<AlignedString>) -> Self {
        self.edge_strings.insert(Edge::Top, title.into());
        self
    }

    pub fn set_title_string(&mut self, title: impl Into<String>) {
        let string = self.edge_strings.entry(Edge::Top).or_default();
        string.string = title.into();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Edge {
    Top,
    Left,
    Right,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl AlignedStringFormatter for &String {
    fn fg_col(self, color: Color) -> AlignedString {
        AlignedString {
            string: self.to_string(),
            fg_col: Some(color),
            ..Default::default()
        }
    }

    fn bg_col(self, color: Color) -> AlignedString {
        AlignedString {
            string: self.to_string(),
            bg_col: Some(color),
            ..Default::default()
        }
    }

    fn aligned(self, align: f32) -> AlignedString {
        AlignedString {
            string: self.to_string(),
            align,
            ..Default::default()
        }
    }
}

impl AlignedStringFormatter for &str {
    fn fg_col(self, color: Color) -> AlignedString {
        AlignedString {
            string: self.to_string(),
            fg_col: Some(color),
            ..Default::default()
        }
    }

    fn bg_col(self, color: Color) -> AlignedString {
        AlignedString {
            string: self.to_string(),
            bg_col: Some(color),
            ..Default::default()
        }
    }

    fn aligned(self, align: f32) -> AlignedString {
        AlignedString {
            string: self.to_string(),
            align,
            ..Default::default()
        }
    }
}

impl From<&str> for AlignedString {
    fn from(string: &str) -> Self {
        AlignedString {
            string: string.to_string(),
            ..Default::default()
        }
    }
}

impl From<String> for AlignedString {
    fn from(string: String) -> Self {
        AlignedString {
            string,
            ..Default::default()
        }
    }
}

impl From<&String> for AlignedString {
    fn from(string: &String) -> Self {
        AlignedString {
            string: string.to_owned(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    // use crate::Tile;
    // use bevy::prelude::*;

    use super::*;

    #[test]
    fn format_test() {
        let border = Border::single_line();

        assert_eq!('─', border.top);
        assert_eq!('┌', border.top_left);
    }

    #[test]
    fn from_string() {
        let glyphs = Border::from_string(
            "┌─┐
             │ │
             └─┘",
        );

        assert_eq!(glyphs.top, '─');
        assert_eq!(glyphs.bottom, '─');
        assert_eq!(glyphs.left, '│');
        assert_eq!(glyphs.right, '│');
        assert_eq!(glyphs.top_left, '┌');
        assert_eq!(glyphs.top_right, '┐');
        assert_eq!(glyphs.bottom_left, '└');
        assert_eq!(glyphs.bottom_right, '┘');
    }
}
