use arrayvec::ArrayVec;
use bevy::prelude::Color;
use sark_grids::GridPoint;
use sark_grids::Size2d;

use crate::formatting::TileFormat;
use crate::formatting::TileModifier;
use crate::terminal::ColorModifier;
use crate::Terminal;
use crate::Tile;

#[derive(Debug, Default, Clone)]
/// Specifies the glyphs and colors to use when drawing a box on the terminal.
pub struct UiBox {
    /// Optional glyphs to be draw on the border of the box.
    ///
    /// Foreground and/or background color can optionally be provided with the `fg(Color)` and
    /// `bg(Color)` functions.
    pub glyphs: Option<BorderGlyphs>,

    /// Optional tile to fill the box with.
    pub fill_tile: Option<TileFormat>,
}

impl UiBox {
    pub fn new() -> Self {
        UiBox::default()
    }

    /// A box with a single-line border.
    pub fn single_line() -> Self {
        UiBox::default().border_glyphs(BorderGlyphs::single_line())
    }

    /// A box with a double line border.
    pub fn double_line() -> Self {
        UiBox::new().border_glyphs(BorderGlyphs::double_line())
    }

    /// Specify the [BorderGlyphs] for the box.
    pub fn border_glyphs(mut self, glyphs: BorderGlyphs) -> UiBox {
        self.glyphs = Some(glyphs);
        self
    }

    /// Specify the optional fill tile for the box.
    ///
    /// If specified the fill tile will be drawn to the whole area of the box before
    /// the border is drawn.
    pub fn fill_tile(mut self, fill_tile: TileFormat) -> UiBox {
        self.fill_tile = Some(fill_tile);
        self
    }

    /// Clear the entire area of the box to default tiles before drawing the border.
    pub fn cleared(self) -> UiBox {
        self.fill_tile(TileFormat::clear())
    }

    /// Draw the box to a terminal.
    pub(crate) fn draw(&self, xy: impl GridPoint, size: impl Size2d, term: &mut Terminal) {
        let [x, y] = xy.to_array();
        let [width, height] = size.as_ivec2().to_array();
        let width = width as usize;
        let height = height as usize;
        let left = x as usize;
        let right = x as usize + width - 1;
        let bottom = y as usize;
        let top = y as usize + height - 1;

        if let Some(fill) = &self.fill_tile {
            let [width, height] = size.to_array();
            let [x, y] = xy.to_array();
            for y in y..y + height as i32 {
                for x in x..x + width as i32 {
                    fill.draw([x, y], term);
                }
            }
        }

        if let Some(glyphs) = &self.glyphs {
            for t in term.row_iter_mut(top).skip(left).take(width) {
                glyphs.edge_tile(BoxEdge::Top).apply(t);
            }
            for t in term.row_iter_mut(bottom).skip(left).take(width) {
                glyphs.edge_tile(BoxEdge::Bottom).apply(t);
            }
            for t in term.column_iter_mut(left).skip(bottom).take(height) {
                glyphs.edge_tile(BoxEdge::Left).apply(t);
            }
            for t in term.column_iter_mut(right).skip(bottom).take(height) {
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

    color_modifiers: ArrayVec<ColorModifier, 2>,
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
        .default_clear_colors()
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
        .default_clear_colors()
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
        .default_clear_colors()
    }

    /// Add a foreground color to the box formatting.
    ///
    /// This will only be applied to border glyphs. To apply color to the
    /// entire box use `BoxWriter::fill_tile` instead.
    pub fn fg_color(mut self, color: Color) -> Self {
        for modifier in self.color_modifiers.iter_mut() {
            match modifier {
                ColorModifier::FgColor(col) => {
                    *col = color;
                    return self;
                }
                ColorModifier::BgColor(_) => {}
            }
        }
        self.color_modifiers.push(ColorModifier::FgColor(color));
        self
    }

    /// Add a background color to the box formatting.
    ///
    /// This will only be applied to border glyphs. To apply color to the
    /// entire box use `BoxWriter::fill_tile` instead.
    pub fn bg_color(mut self, color: Color) -> Self {
        for modifier in self.color_modifiers.iter_mut() {
            match modifier {
                ColorModifier::FgColor(_) => {}
                ColorModifier::BgColor(col) => {
                    *col = color;
                    return self;
                }
            }
        }
        self.color_modifiers.push(ColorModifier::BgColor(color));
        self
    }

    /// When specified the border glyphs will not affect existing tile colors.
    pub fn dont_clear_colors(mut self) -> Self {
        self.color_modifiers.clear();
        self
    }

    /// Iterator over the color modifiers to be applied to border glyphs.
    pub fn color_mod_iter(&self) -> impl Iterator<Item = &ColorModifier> {
        self.color_modifiers.iter()
    }

    /// Retrieve the [TileFormat] for a given glyph, with color modifiers
    /// applied, if any.
    fn get_formatted_tile(&self, glyph: char) -> TileFormat {
        let mut fmt = TileFormat::default().glyph(glyph);

        for write in self.color_modifiers.iter() {
            match write {
                ColorModifier::FgColor(col) => fmt = fmt.fg(*col),
                ColorModifier::BgColor(col) => fmt = fmt.bg(*col),
            };
        }

        fmt
    }

    /// Returns the glyph for a given border edge.
    pub fn edge_glyph(&self, edge: BoxEdge) -> char {
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

    /// Returns the mutable glyph for a given border edge.
    pub fn edge_glyph_mut(&mut self, edge: BoxEdge) -> &mut char {
        match edge {
            BoxEdge::Top => &mut self.top,
            BoxEdge::Left => &mut self.left,
            BoxEdge::Right => &mut self.right,
            BoxEdge::Bottom => &mut self.bottom,
            BoxEdge::TopLeft => &mut self.top_left,
            BoxEdge::TopRight => &mut self.top_right,
            BoxEdge::BottomLeft => &mut self.bottom_left,
            BoxEdge::BottomRight => &mut self.bottom_right,
        }
    }

    /// Returns the [TileFormat] for a given box edge.
    pub fn edge_tile(&self, edge: BoxEdge) -> TileFormat {
        self.get_formatted_tile(self.edge_glyph(edge))
    }

    fn default_clear_colors(mut self) -> Self {
        let t = Tile::default();
        self.color_modifiers
            .push(ColorModifier::FgColor(t.fg_color));
        self.color_modifiers
            .push(ColorModifier::BgColor(t.bg_color));
        self
    }
}

#[cfg(test)]
mod test {
    use crate::Tile;
    use bevy::prelude::*;

    use super::*;

    #[test]
    fn format_test() {
        let fmt = BorderGlyphs::single_line().fg_color(Color::RED);

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
