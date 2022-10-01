use bevy::prelude::Color;

use crate::{Tile, TileFormat, TileModifier};

/// Border glyphs used in box drawing functions.
///
/// Specifies the style of lines to use along the border of the box.
/// Colors can optionally be specified.
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
    pub title: Option<BorderTitle>,
    /// The foreground color for the border. If not specified, the colors
    /// for the terminal clear tile will be used.
    pub fg_color: Option<Color>,
    /// The background color for the border. If not specified, the colors
    /// for the terminal clear tile will be used.
    pub bg_color: Option<Color>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BorderTitle {
    pub string: String,
    pub color: Color,
    pub align: f32,
}

impl BorderTitle {
    pub fn new(title: impl AsRef<str>) -> Self {
        let def = Tile::default();
        Self {
            string: title.as_ref().to_string(),
            align: 0.0,
            color: def.fg_color,
        }
    }

    pub fn left(title: impl AsRef<str>) -> Self {
        Self::new(title).align(0.0)
    }

    pub fn center(title: impl AsRef<str>) -> Self {
        Self::new(title).align(0.5)
    }

    pub fn right(title: impl AsRef<str>) -> Self {
        Self::new(title).align(1.0)
    }

    pub fn color(mut self, col: Color) -> Self {
        self.color = col;
        self
    }

    pub fn align(mut self, align: f32) -> Self {
        self.align = align;
        self
    }
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
        title: None,
        fg_color: None,
        bg_color: None,
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
        title: None,
        fg_color: Some(Color::WHITE),
        bg_color: Some(Color::BLACK),
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

    pub fn with_title(mut self, title: impl Into<BorderTitle>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Border glyphs will be colored with the default [`Tile`] foreground
    /// and background colors.
    pub fn with_default_colors(self) -> Self {
        let t = Tile::default();
        self.fg(t.fg_color).bg(t.bg_color)
    }

    /// Add a foreground color to the border glyphs.
    pub fn fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    /// Add a background color to the border glyphs.
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    pub fn set_title(&mut self, title: impl Into<BorderTitle>) {
        self.title = Some(title.into());
    }

    pub fn title_mut(&mut self) -> &mut BorderTitle {
        self.title.as_mut().unwrap()
    }

    pub fn get_title_mut(&mut self) -> Option<&mut BorderTitle> {
        self.title.as_mut()
    }

    pub fn set_title_string(&mut self, string: impl AsRef<str>) {
        if let Some(title) = &mut self.title {
            title.string = string.as_ref().to_owned();
        } else {
            self.title = Some(BorderTitle::new(string));
        }
    }

    /// Retrieve the foreground color if it was set.
    pub fn get_fgcol(&self) -> Option<Color> {
        self.fg_color
    }

    /// Retrieve the background color if it was set.
    pub fn get_bgcol(&self) -> Option<Color> {
        self.bg_color
    }

    /// Retrieve the [TileFormat] for a given glyph, with color modifiers
    /// applied, if any.
    fn get_formatted_tile(&self, glyph: char) -> TileFormat {
        let mut fmt = TileFormat::default().glyph(glyph);

        if let Some(fg) = self.fg_color {
            fmt = fmt.fg(fg);
        }

        if let Some(bg) = self.bg_color {
            fmt = fmt.bg(bg);
        }

        fmt
    }

    /// Returns the glyph for a given border edge.
    fn edge_glyph(&self, edge: Edge) -> char {
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

    /// Returns the [TileFormat] for a given box edge.
    pub(crate) fn edge_tile(&self, edge: Edge) -> TileFormat {
        self.get_formatted_tile(self.edge_glyph(edge))
    }
}

impl<'a> From<&'a str> for BorderTitle {
    fn from(s: &'a str) -> Self {
        BorderTitle::new(s)
    }
}

impl From<String> for BorderTitle {
    fn from(s: String) -> Self {
        BorderTitle::new(s)
    }
}

#[derive(Debug, Clone, Copy)]
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
        let fmt = Border::SINGLE_LINE.fg(Color::RED);

        let tile: Tile = fmt.edge_tile(Edge::Top).into();
        assert_eq!('─', tile.glyph);
        assert_eq!(Color::RED, tile.fg_color);

        let tile: Tile = fmt.edge_tile(Edge::TopLeft).into();
        assert_eq!('┌', tile.glyph);
        assert_eq!(Color::RED, tile.fg_color);
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
