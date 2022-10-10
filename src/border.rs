use bevy::prelude::Color;

use crate::{Tile, FormattedTile, TileFormatter};

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
}

impl Default for Border {
    fn default() -> Self {
        Border::SINGLE_LINE
    }
}

impl Border {
    pub const SINGLE_LINE: Border = Border {
        top: '─',
        left: '│',
        right: '│',
        bottom: '─',
        top_left: '┌',
        top_right: '┐',
        bottom_left: '└',
        bottom_right: '┘',
    };

    pub const DOUBLE_LINE: Border = Border {
        top: '═',
        left: '║',
        right: '║',
        bottom: '═',
        top_left: '╔',
        top_right: '╗',
        bottom_left: '╚',
        bottom_right: '╝',
    };

    /// Construct a new set of [BorderGlyphs] from the given string.
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

#[cfg(test)]
mod test {
    use crate::Tile;
    use bevy::prelude::*;

    use super::*;

    #[test]
    fn format_test() {
        let border = Border::SINGLE_LINE;

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


const BSTRING: &str = 
"┌─[CoolTitle]─┐
 │ │
 └─┘";