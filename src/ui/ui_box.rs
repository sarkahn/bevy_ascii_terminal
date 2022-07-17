use arrayvec::ArrayVec;
use bevy::math::Vec2;
use bevy::prelude::Color;
use sark_grids::GridPoint;
use sark_grids::Size2d;

use crate::fmt_tile::ColorFormat;
use crate::fmt_tile::TileFormat;
use crate::formatting::TileModifier;
use crate::Terminal;
use crate::Tile;

#[derive(Debug, Default, Clone)]
/// Specifies the glyphs and colors to use when drawing a box on the terminal.
pub struct UiBox {
    /// Optional glyphs to be draw on the border of the box.
    ///
    /// Foreground and/or background color can optionally be provided with the `fg(Color)` and
    /// `bg(Color)` functions.
    pub border_glyphs: Option<BorderGlyphs>,

    /// Optional tile to fill the box with.
    pub fill_tile: Option<TileFormat>,
}

impl UiBox {
    pub fn new() -> Self {
        UiBox::default()
    }

    /// Create a hollow UiBox with a single line border.
    pub fn single_line() -> Self {
        let border = BorderGlyphs::single_line();
        UiBox::new().with_border(border)
    }

    /// Create a hollow UiBox with a double line border.
    pub fn double_line() -> Self {
        let border = BorderGlyphs::double_line();
        UiBox::new().with_border(border)
    }

    /// Specify the [BorderGlyphs] for the box.
    pub fn with_border(mut self, glyphs: BorderGlyphs) -> Self {
        self.border_glyphs = Some(glyphs);
        self
    }

    /// Specify the optional fill tile for the box.
    ///
    /// If specified, the fill tile will be drawn to the whole area of the box before
    /// the border is drawn. If no border is specified the entire box will be filled.
    pub fn filled(mut self, fill_tile: impl TileModifier) -> UiBox {
        self.fill_tile = Some(fill_tile.format());
        self
    }

    /// A box which will apply the given foreground and background colors, without affecting
    /// existing glyphs.
    pub fn color_fill(self, fg: Color, bg: Color) -> UiBox {
        self.filled(TileFormat::new().fg(fg).bg(bg))
    }

    /// Clear the entire area of the box to default tiles before drawing the border.
    pub fn cleared(self) -> Self {
        self.filled(TileFormat::clear())
    }

    /// Draw the box to a terminal.
    pub(crate) fn draw(&self, xy: impl GridPoint, size: impl Size2d, term: &mut Terminal) {
        let pivoted_point = xy.get_pivot();
        let xy = xy.get_aligned_point(term.size());
        let pivot = pivoted_point.pivot;

        let pivot = Vec2::from(pivot);
        let align_offset = ((size.as_vec2() - Vec2::ONE) * pivot).as_ivec2();
        let xy = xy - align_offset;

        let [x, y] = xy.as_usize_array();
        let [width, height] = size.as_usize_array();
        let left = x;
        let right = x + width - 1;
        let bottom = y;
        let top = y + height - 1;

        if let Some(fill) = &self.fill_tile {
            let [width, height] = size.as_array();
            let [x, y] = xy.to_array();
            for y in y..y + height as i32 {
                for x in x..x + width as i32 {
                    fill.draw([x, y], term);
                }
            }
        }

        if let Some(glyphs) = &self.border_glyphs {
            for t in term.iter_row_mut(top).skip(left).take(width) {
                glyphs.edge_tile(BoxEdge::Top).apply(t);
            }
            for t in term.iter_row_mut(bottom).skip(left).take(width) {
                glyphs.edge_tile(BoxEdge::Bottom).apply(t);
            }
            for t in term.iter_column_mut(left).skip(bottom).take(height) {
                glyphs.edge_tile(BoxEdge::Left).apply(t);
            }
            for t in term.iter_column_mut(right).skip(bottom).take(height) {
                glyphs.edge_tile(BoxEdge::Right).apply(t);
            }

            let left = left as i32;
            let right = right as i32;
            let top = top as i32;
            let bottom = bottom as i32;

            term.put_char([left, bottom], glyphs.edge_tile(BoxEdge::BottomLeft));
            term.put_char([left, top], glyphs.edge_tile(BoxEdge::TopLeft));
            term.put_char([right, top], glyphs.edge_tile(BoxEdge::TopRight));
            term.put_char([right, bottom], glyphs.edge_tile(BoxEdge::BottomRight));
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BoxEdge {
    Top,
    Left,
    Right,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

/// Border glyphs used in box drawing functions.
///
/// Specifies the style of lines to use along the border of the box.
/// Colors can optionally be specified.
#[derive(Debug, Clone)]
pub struct BorderGlyphs {
    top: char,
    left: char,
    right: char,
    bottom: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,

    color_modifiers: ArrayVec<ColorFormat, 2>,
}

impl Default for BorderGlyphs {
    fn default() -> Self {
        BorderGlyphs::single_line()
    }
}

impl BorderGlyphs {
    /// Construct a new set of [BorderGlyphs] from the given string.
    ///
    /// The format of the string should match the example below. Line returns and spaces
    /// will be ignored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bevy_ascii_terminal::ui::BorderGlyphs;
    /// let border = BorderGlyphs::from_string(
    ///     "┌─┐
    ///      │ │
    ///      └─┘"
    /// );
    /// ```
    pub fn from_string(box_string: &str) -> Self {
        let mut string = box_string.to_string();
        string.retain(|c| !c.is_whitespace());
        debug_assert!(
            string.chars().count() == 8,
            "Error building border glyphs from string, string should contain
            exactly 8 glyphs (minus whitespace)"
        );

        let mut chars = string.chars();
        BorderGlyphs {
            top_left: chars.next().unwrap(),
            top: chars.next().unwrap(),
            top_right: chars.next().unwrap(),
            left: chars.next().unwrap(),
            right: chars.next().unwrap(),
            bottom_left: chars.next().unwrap(),
            bottom: chars.next().unwrap(),
            bottom_right: chars.next().unwrap(),
            color_modifiers: ArrayVec::new(),
        }
    }

    /// Single line border glyphs. Can be used in box drawing functions.
    pub fn single_line() -> Self {
        BorderGlyphs {
            top: '─',
            left: '│',
            right: '│',
            bottom: '─',
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            color_modifiers: ArrayVec::new(),
        }
    }

    /// Double line border glyphs. Can be used in box drawing functions.
    pub fn double_line() -> Self {
        BorderGlyphs {
            top: '═',
            left: '║',
            right: '║',
            bottom: '═',
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            color_modifiers: ArrayVec::new(),
        }
    }

    /// Border glyphs will be colored with the default [`Tile`] foreground
    /// and background colors.
    pub fn with_default_colors(self) -> Self {
        let t = Tile::default();
        self.fg(t.fg_color).bg(t.bg_color)
    }

    /// Add a foreground color to the border glyphs.
    pub fn fg(mut self, color: Color) -> Self {
        for modifier in self.color_modifiers.iter_mut() {
            if let ColorFormat::FgColor(col) = modifier {
                *col = color;
                return self;
            }
        }
        self.color_modifiers.push(ColorFormat::FgColor(color));
        self
    }

    /// Add a background color to the border glyphs.
    pub fn bg(mut self, color: Color) -> Self {
        for modifier in self.color_modifiers.iter_mut() {
            match modifier {
                ColorFormat::FgColor(_) => {}
                ColorFormat::BgColor(col) => {
                    *col = color;
                    return self;
                }
            }
        }
        self.color_modifiers.push(ColorFormat::BgColor(color));
        self
    }

    /// Retrieve the [TileFormat] for a given glyph, with color modifiers
    /// applied, if any.
    fn get_formatted_tile(&self, glyph: char) -> TileFormat {
        let mut fmt = TileFormat::default().glyph(glyph);

        for write in self.color_modifiers.iter() {
            match write {
                ColorFormat::FgColor(col) => fmt = fmt.fg(*col),
                ColorFormat::BgColor(col) => fmt = fmt.bg(*col),
            };
        }

        fmt
    }

    /// Returns the glyph for a given border edge.
    fn edge_glyph(&self, edge: BoxEdge) -> char {
        match edge {
            BoxEdge::Top => self.top,
            BoxEdge::Left => self.left,
            BoxEdge::Right => self.right,
            BoxEdge::Bottom => self.bottom,
            BoxEdge::TopLeft => self.top_left,
            BoxEdge::TopRight => self.top_right,
            BoxEdge::BottomLeft => self.bottom_left,
            BoxEdge::BottomRight => self.bottom_right,
        }
    }

    /// Returns the [TileFormat] for a given box edge.
    fn edge_tile(&self, edge: BoxEdge) -> TileFormat {
        self.get_formatted_tile(self.edge_glyph(edge))
    }
}

#[cfg(test)]
mod test {
    use crate::Tile;
    use bevy::prelude::*;

    use super::*;

    #[test]
    fn format_test() {
        let fmt = BorderGlyphs::single_line().fg(Color::RED);

        let tile: Tile = fmt.edge_tile(BoxEdge::Top).into();
        assert_eq!('─', tile.glyph);
        assert_eq!(Color::RED, tile.fg_color);

        let tile: Tile = fmt.edge_tile(BoxEdge::TopLeft).into();
        assert_eq!('┌', tile.glyph);
        assert_eq!(Color::RED, tile.fg_color);
    }

    #[test]
    fn from_string() {
        let glyphs = BorderGlyphs::from_string(
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
