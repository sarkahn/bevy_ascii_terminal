use arrayvec::ArrayVec;
use bevy::prelude::Color;
use sark_grids::GridPoint;
use sark_grids::Size2d;

use crate::Terminal;
use crate::Tile;

pub use super::fmt_tile::TileFormat;
pub use super::fmt_tile::TileModifier;

/// Trait for building a formatted box that can be drawn to a terminal.
pub trait BoxWriter {
    /// Specify the border glyphs for the box.
    fn glyphs(self, glyphs: BorderGlyphs) -> FormattedBox;
    /// Specify the fill tile for the box.
    /// 
    /// The fill tile is applied before border drawing if any.
    fn fill_tile(self, fill_tile: TileFormat) -> FormattedBox;
    /// Clear all tiles contained in the box to default before drawing the border drawing if any.
    fn cleared(self) -> FormattedBox;
    /// Get the [FormattedBox].
    fn formatted(self) -> FormattedBox;
}

#[derive(Debug, Default, Clone)]
/// Specifies the glyphs and colors to use when drawing a box on the terminal.
pub struct FormattedBox {
    /// Optional glyphs to fill the border of the box with.
    /// 
    /// Foreground and/or background color can optionally be provided with the `fg(Color)` and
    /// `bg(Color)` functions. 
    glyphs: Option<BorderGlyphs>,
    fill_tile: Option<TileFormat>,
}

impl FormattedBox {
    pub fn single_line() -> Self {
        FormattedBox {
            glyphs: Some(BorderGlyphs::single_line()),
            fill_tile: None
        }
    }

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
                    fill.draw([x,y], term);
                }
            }
        }

        if let Some(glyphs) = &self.glyphs {
            for t in term.row_iter_mut(top).skip(left).take(width) {
                glyphs.top().apply(t);
            }
            for t in term.row_iter_mut(bottom).skip(left).take(width) {
                glyphs.bottom().apply(t);
            }
            for t in term.column_iter_mut(left).skip(bottom).take(height) {
                glyphs.left().apply(t);
            }
            for t in term.column_iter_mut(right).skip(bottom).take(height) {
                glyphs.right().apply(t);
            }
            
            let left = left as i32;
            let right = right as i32;
            let top = top as i32;
            let bottom = bottom as i32;

            term.put_char([left, bottom], glyphs.bottom_left());
            term.put_char([left, top], glyphs.top_left());
            term.put_char([right, top], glyphs.top_right());
            term.put_char([right, bottom], glyphs.bottom_right());
        }
    }
}

impl BoxWriter for FormattedBox {
    fn glyphs(mut self, glyphs: BorderGlyphs) -> FormattedBox {
        self.glyphs = Some(glyphs);
        self
    }

    fn fill_tile(mut self, fill_tile: TileFormat) -> FormattedBox {
        self.fill_tile = Some(fill_tile);
        self
    }

    fn formatted(self) -> FormattedBox {
        self
    }

    fn cleared(self) -> FormattedBox {
        self.fill_tile(TileFormat::clear())
    }
}

impl BoxWriter for BorderGlyphs {
    fn glyphs(self, glyphs: BorderGlyphs) -> FormattedBox {
        FormattedBox {
            glyphs: Some(glyphs),
            fill_tile: None
        }
    }

    fn fill_tile(self, fill_tile: TileFormat) -> FormattedBox {
        FormattedBox {
            glyphs: None,
            fill_tile: Some(fill_tile),
        }
    }

    fn formatted(self) -> FormattedBox {
        FormattedBox::default().glyphs(self)
    }

    fn cleared(self) -> FormattedBox {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum BorderGlyphWrite {
    FgColor(Color),
    BgColor(Color),
}

/// Border glyphs used in box drawing functions.
///
/// Specifies the style of lines to use along the border of the box.
#[derive(Debug, Default, Clone)]
pub struct BorderGlyphs {
    top: char,
    left: char,
    right: char,
    bottom: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,

    color_modifiers: ArrayVec<BorderGlyphWrite, 2>,
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
    /// let border = BorderGlyphs::from_string(
    ///     "┌─┐
    ///      │ │
    ///      └─┘"
    /// );
    /// ```
    pub fn from_string(box_string: &str) -> Self {
        let mut string = box_string.to_string();
        string.retain(|c| !c.is_whitespace());
        debug_assert!(string.chars().count() == 8, 
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
            ..Default::default()
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
        }.default_colors()
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
        }.default_colors()
    }

    /// Add a foreground color to the box formatting.
    /// 
    /// This will only be applied to border glyphs. To apply color to the
    /// entire box use `BoxWriter::fill_tile` instead.
    pub fn fg(mut self, color: Color) -> Self {
        self.color_modifiers.push(BorderGlyphWrite::FgColor(color));
        self
    }

    /// Add a background color to the box formatting.
    /// 
    /// This will only be applied to border glyphs. To apply color to the
    /// entire box use `BoxWriter::fill_tile` instead.
    pub fn bg(mut self, color: Color) -> Self {
        self.color_modifiers.push(BorderGlyphWrite::BgColor(color));
        self
    }

    /// Will apply default colors to border glyphs.
    /// 
    /// If not specified the existing tile colors in the terminal will be
    /// unaffected - only glyphs will be drawn.
    pub fn default_colors(self) -> Self {
        let t = Tile::default();
        self.fg(t.fg_color).bg(t.bg_color)
    }

    /// Iterator over the color modifiers to be applied to border glyphs.
    pub fn color_mod_iter(&self) -> impl Iterator<Item=&BorderGlyphWrite> {
        self.color_modifiers.iter()
    }

    /// Retrieve the [TileFormat] for a given glyph, with color modifiers
    /// applied, if any.
    fn get_tile(&self, glyph: char) -> TileFormat {
        let mut fmt = TileFormat::default().glyph(glyph);

        for write in self.color_modifiers.iter() {
            match write {
                BorderGlyphWrite::FgColor(col) => fmt = fmt.fg(*col),
                BorderGlyphWrite::BgColor(col) => fmt = fmt.bg(*col),
            };
        }

        fmt
    }

    /// Retrieve the [TileFormat] for the top border tile.
    pub fn top(&self) -> TileFormat {
        self.get_tile(self.top)
    }
    
    /// Retrieve the [TileFormat] for the left border tile.
    pub fn left(&self) -> TileFormat {
        self.get_tile(self.left)
    }
    
    /// Retrieve the [TileFormat] for the right border tile.
    pub fn right(&self) -> TileFormat {
        self.get_tile(self.right)
    }
    
    /// Retrieve the [TileFormat] for the bottom border tile.
    pub fn bottom(&self) -> TileFormat {
        self.get_tile(self.bottom)
    }
    
    /// Retrieve the [TileFormat] for the top left border tile.
    pub fn top_left(&self) -> TileFormat {
        self.get_tile(self.top_left)
    }
    
    /// Retrieve the [TileFormat] for the top right border tile.
    pub fn top_right(&self) -> TileFormat {
        self.get_tile(self.top_right)
    }
    
    /// Retrieve the [TileFormat] for the bottom left border tile.
    pub fn bottom_left(&self) -> TileFormat {
        self.get_tile(self.bottom_left)
    }
    
    /// Retrieve the [TileFormat] for the bottom right border tile.
    pub fn bottom_right(&self) -> TileFormat {
        self.get_tile(self.bottom_right)
    }

    pub fn set_top(&mut self, glyph: char) {
        self.top = glyph;
    }
    
    pub fn set_left(&mut self, glyph: char) {
        self.left = glyph;
    }
    
    pub fn set_right(&mut self, glyph: char) {
        self.right = glyph;
    }
    
    pub fn set_bottom(&mut self, glyph: char) {
        self.bottom = glyph;
    }
    
    pub fn set_top_left(&mut self, glyph: char) {
        self.top_left = glyph;
    }
    
    pub fn set_top_right(&mut self, glyph: char) {
        self.top_right = glyph;
    }
    
    pub fn set_bottom_left(&mut self, glyph: char) {
        self.bottom_left = glyph;
    }
    
    pub fn set_bottom_right(&mut self, glyph: char) {
        self.bottom_right = glyph;
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::*;
    use crate::Tile;

    use super::*;

    #[test]
    fn test() {
        let fmt = BorderGlyphs::single_line().fill_tile('q'.fg(Color::BLUE));
    }

    #[test]
    fn format_test() {
        let fmt = BorderGlyphs::single_line().fg(Color::RED);

        let tile: Tile = fmt.top().into();
        assert_eq!('─', tile.glyph);
        assert_eq!(Color::RED, tile.fg_color);

        let tile: Tile = fmt.top_left().into();
        assert_eq!('┌', tile.glyph);
        assert_eq!(Color::RED, tile.fg_color);
    }

    #[test]
    fn from_string() {
        let glyphs = BorderGlyphs::from_string(
            "┌─┐
             │ │
             └─┘"
        );

        assert_eq!(glyphs.top, '─' );
        assert_eq!(glyphs.bottom, '─' );
        assert_eq!(glyphs.left, '│' );
        assert_eq!(glyphs.right, '│' );
        assert_eq!(glyphs.top_left, '┌' );
        assert_eq!(glyphs.top_right, '┐' );
        assert_eq!(glyphs.bottom_left, '└' );
        assert_eq!(glyphs.bottom_right, '┘' );
    }
}