use std::iter::FromIterator;
use std::iter::StepBy;
use std::slice::Iter;
use std::slice::IterMut;

use bevy::prelude::*;

// use crate::formatting::CharFormat;
// use crate::formatting::StringFormat;

use sark_grids::Grid;
use sark_grids::GridPoint;
use sark_grids::Size2d;
use thiserror::Error;

use crate::formatting::StringWrite;
use crate::formatting::TileWrite;
//use crate::formatting::FormattedTile;
use crate::formatting::TileWriter;
use crate::formatting::fmt_string::StringWriter;

/// A single tile of the terminal.
///
/// Defaults to a blank glyph with a black background and a white foreground.
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    /// The glyph for the tile. Glyphs are mapped to sprites via the
    /// terminal's [UvMapping](super::renderer::uv_mapping::UvMapping)
    pub glyph: char,
    /// The forergound color for the tile.
    pub fg_color: Color,
    /// The background color for the tile.
    pub bg_color: Color,
}

/// A simple terminal for writing text in a readable grid.
///
/// Contains various functions for drawing colorful text to a
/// terminal.
///
/// # Example
/// ```
/// use bevy_ascii_terminal::Terminal;
///
/// let mut term = Terminal::with_size([10,10]);
///
/// term.put_char([1,1], 'h'.fg(Color::RED));
/// term.put_string([2,1], "ello");
///
/// let hello = term.get_string([1,1], 5);
/// ```
#[derive(Component, Default)]
pub struct Terminal {
    pub tiles: Grid<Tile>,
    size: UVec2,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            glyph: ' ',
            fg_color: Color::WHITE,
            bg_color: Color::BLACK,
        }
    }
}

/// Border glyphs used in box drawing functions.
///
/// Specifies the style of lines to use along the border of the box.
#[derive(Clone, Copy)]
pub struct BorderGlyphs {
    pub top: char,
    pub left: char,
    pub right: char,
    pub bottom: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
}

/// Single line border glyphs. Can be used in box drawing functions.
pub const SINGLE_LINE_GLYPHS: BorderGlyphs = BorderGlyphs {
    left: '│',
    right: '│',
    bottom: '─',
    top: '─',
    top_left: '┌',
    top_right: '┐',
    bottom_left: '└',
    bottom_right: '┘',
};

/// Double line border glyphs. Can be used in box drawing functions.
pub const DOUBLE_LINE_GLYPHS: BorderGlyphs = BorderGlyphs {
    left: '║',
    right: '║',
    top: '═',
    bottom: '═',
    top_left: '╔',
    top_right: '╗',
    bottom_left: '╚',
    bottom_right: '╝',
};

impl Terminal {
    /// Construct a terminal with the given size
    pub fn with_size(size: impl Size2d) -> Terminal {
        Terminal {
            tiles: Grid::default(size),
            size: size.as_uvec2(),
        }
    }

    /// Resize the terminal's internal tile data.
    pub fn resize(&mut self, size: [u32; 2]) {
        self.tiles = Grid::default(size);
        self.size = UVec2::from(size);
    }

    pub fn width(&self) -> usize {
        self.size.x as usize
    }
    pub fn height(&self) -> usize {
        self.size.y as usize
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    /// Convert a 2D position to it's corresponding 1D index
    /// in the terminal.
    ///
    /// Note that in the terminal the y axis goes from top to bottom.
    #[inline]
    pub fn to_index(&self, xy: impl GridPoint) -> usize {
        self.tiles.pos_to_index(xy)
    }

    /// Convert 1D index to it's 2D position given the dimensions
    /// of the terminal.
    ///
    /// Note that in the terminal the y axis goes from top to bottom.
    #[inline]
    pub fn to_xy(&self, i: usize) -> IVec2 {
        let i = i as i32;
        let w = self.width() as i32;
        let x = i % w;
        let y = i / w;
        IVec2::new(x, y)
    }

    /// Insert a formatted character into the terminal.
    ///
    /// You can optionally specify the foreground and/or background color for the
    /// tile as well.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// 
    /// ```
    pub fn put_char(&mut self, xy: impl GridPoint, writer: impl TileWriter) {
        let tile = self.get_tile_mut(xy);

        for write in writer.iter() {
            match write {
                TileWrite::Glyph(glyph) => tile.glyph = glyph,
                TileWrite::FGColor(col) => tile.fg_color = col,
                TileWrite::BGColor(col) => tile.bg_color = col,
            }
        }
    }

    /// Insert a [Tile].
    pub fn put_tile(&mut self, xy: impl GridPoint, tile: Tile) {
        let t = self.get_tile_mut(xy);
        *t = tile;
    }

    /// Write a formatted string to the terminal.
    /// 
    /// You can optionally specify a foreground and/or background color for the
    /// string as well.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let mut term = Terminal::with_size([10,10]);
    /// term.put_string("Hello".fg(Color::BLUE)); // Write a blue "Hello" to the terminal
    /// ```
    pub fn put_string<'a>(&mut self, xy: impl GridPoint, writer: impl StringWriter<'a>) {
        let i = self.to_index(xy);

        let (string,writes) = writer.formatted().into();

        let tiles = self.tiles.slice_mut(i..).iter_mut().take(string.len());
        let chars = string.chars().take(tiles.len());

        for (char, mut t) in chars.zip(tiles) {
            t.glyph = char;
        }

        for write in writes {
            match write {
                StringWrite::FGColor(col) => {   
                    let tiles = self.tiles.slice_mut(i..).iter_mut().take(string.len());
                    for t in tiles {
                        t.fg_color = col;
                    }
                },
                StringWrite::BGColor(col) => {   
                    let tiles = self.tiles.slice_mut(i..).iter_mut().take(string.len());
                    for t in tiles {
                        t.bg_color = col;
                    }
                },
            }
        }

    }

    // /// Write a string to the terminal.
    // ///
    // /// The string will move to the next line if it reaches the edge
    // /// and will truncate at the end of the terminal.
    // pub fn put_string(&mut self, xy: [i32; 2], string: &str) {
    //     let i = self.to_index(xy);
    //     let tiles = self.tiles.slice_mut(i..).iter_mut().take(string.len());
    //     let chars = string.chars().take(tiles.len());

    //     for (char, mut t) in chars.zip(tiles) {
    //         t.glyph = char;
    //     }
    // }

    /// Retrieve the char from a tile.
    pub fn get_char(&self, xy: impl GridPoint) -> char {
        self.get_tile(xy).glyph
    }

    /// Retrieve a string from the terminal.
    pub fn get_string(&self, xy: impl GridPoint, len: usize) -> String {
        let i = self.to_index(xy);
        let slice = self.tiles.slice(i..).iter().take(len).map(|t|t.glyph);
        
        String::from_iter(slice)
    }

    /// Retrieve an immutable reference to a tile in the terminal.
    pub fn get_tile(&self, xy: impl GridPoint) -> &Tile {
        &self.tiles[self.to_index(xy)]
    }

    #[inline]
    /// Retrieve a mutable reference to a tile in the terminal.
    pub fn get_tile_mut(&mut self, xy: impl GridPoint) -> &mut Tile {
        let i = self.to_index(xy);
        &mut self.tiles[i]
    }

    // /// Clear an area of the terminal to the default [Tile].
    // pub fn clear_box(&mut self, xy: [i32; 2], size: [u32; 2]) {
    //     let [width, height] = size;
    //     let [x, y] = xy;
    //     for y in y..y + height as i32 {
    //         for x in x..x + width as i32 {
    //             todo!();
    //             //self.put_tile([x, y], Tile::default());
    //         }
    //     }
    // }

    // /// Draw a box on the terminal using [BorderGlyphs].
    // pub fn draw_box(&mut self, xy: [i32; 2], size: [u32; 2], border_glyphs: BorderGlyphs) {
    //     let [x, y] = xy;
    //     let [width, height] = size;
    //     let width = width as usize;
    //     let height = height as usize;
    //     let left = x as usize;
    //     let right = x as usize + width - 1;
    //     let bottom = y as usize;
    //     let top = y as usize + height - 1;

    //     for t in self.row_iter_mut(top).skip(left).take(width) {
    //         t.glyph = border_glyphs.top;
    //     }
    //     for t in self.row_iter_mut(bottom).skip(left).take(width) {
    //         t.glyph = border_glyphs.bottom;
    //     }
    //     for t in self.column_iter_mut(left).skip(bottom).take(height) {
    //         t.glyph = border_glyphs.left;
    //     }
    //     for t in self.column_iter_mut(right).skip(bottom).take(height) {
    //         t.glyph = border_glyphs.right;
    //     }

    //     let left = left as i32;
    //     let right = right as i32;
    //     let top = top as i32;
    //     let bottom = bottom as i32;

    //     self.put_char([left, bottom], border_glyphs.bottom_left);
    //     self.put_char([left, top], border_glyphs.top_left);
    //     self.put_char([right, top], border_glyphs.top_right);
    //     self.put_char([right, bottom], border_glyphs.bottom_right);
    // }

    // /// Draw a box with box with the specified colors and [BorderGlyphs].
    // pub fn draw_box_formatted(
    //     &mut self,
    //     xy: [i32; 2],
    //     size: [u32; 2],
    //     border_glyphs: BorderGlyphs,
    //     format: CharFormat,
    // ) {
    //     let [x, y] = xy;
    //     let [width, height] = size;
    //     let width = width as usize;
    //     let height = height as usize;
    //     let x = x as usize;
    //     let y = y as usize;
    //     let left = x;
    //     let right = x + width - 1;
    //     let bottom = y;
    //     let top = y + height - 1;

    //     for t in self.row_iter_mut(top).skip(left).take(width) {
    //         *t = format.tile(border_glyphs.top);
    //     }
    //     for t in self.row_iter_mut(bottom).skip(left).take(width) {
    //         *t = format.tile(border_glyphs.bottom);
    //     }
    //     for t in self.column_iter_mut(left).skip(bottom).take(height) {
    //         *t = format.tile(border_glyphs.left);
    //     }
    //     for t in self.column_iter_mut(right).skip(bottom).take(height) {
    //         *t = format.tile(border_glyphs.right);
    //     }

    //     let left = left as i32;
    //     let right = right as i32;
    //     let top = top as i32;
    //     let bottom = bottom as i32;

    //     self.put_char_formatted([left, bottom], border_glyphs.bottom_left, format);
    //     self.put_char_formatted([left, top], border_glyphs.top_left, format);
    //     self.put_char_formatted([right, top], border_glyphs.top_right, format);
    //     self.put_char_formatted([right, bottom], border_glyphs.bottom_right, format);
    // }

    // /// Draw a box with a single-line border.
    // pub fn draw_box_single(&mut self, xy: [i32; 2], size: [u32; 2]) {
    //     self.draw_box_formatted(xy, size, SINGLE_LINE_GLYPHS, CharFormat::default());
    // }
    // /// Draw a box with a colored single-line border.
    // pub fn draw_box_single_formatted(&mut self, xy: [i32; 2], size: [u32; 2], format: CharFormat) {
    //     self.draw_box_formatted(xy, size, SINGLE_LINE_GLYPHS, format);
    // }

    // /// Draw a box with a double-line border.
    // pub fn draw_box_double(&mut self, xy: [i32; 2], size: [u32; 2]) {
    //     self.draw_box(xy, size, DOUBLE_LINE_GLYPHS);
    // }
    // /// Draw a box with a colored double-line border.
    // pub fn draw_box_double_formatted(&mut self, xy: [i32; 2], size: [u32; 2], format: CharFormat) {
    //     self.draw_box_formatted(xy, size, DOUBLE_LINE_GLYPHS, format);
    // }

    // pub fn draw_border(&mut self, border_glyphs: BorderGlyphs) {
    //     self.draw_box([0, 0], self.size().into(), border_glyphs);
    // }

    // /// Draw a single-line border around the edge of the whole terminal.
    // pub fn draw_border_single(&mut self) {
    //     self.draw_box_single([0, 0], self.size.into());
    // }

    // /// Draw a colored single-line border around the edge of the whole terminal.
    // pub fn draw_border_single_formatted(&mut self, format: CharFormat) {
    //     self.draw_box_single_formatted([0, 0], self.size.into(), format);
    // }

    // /// Draw a double-line border around the edge of the whole terminal.
    // pub fn draw_border_double(&mut self) {
    //     self.draw_box_double([0, 0], self.size.into());
    // }
    // /// Draw a colored double-line border around the edge of the whole terminal.
    // pub fn draw_border_double_formatted(&mut self, format: CharFormat) {
    //     self.draw_box_double_formatted([0, 0], self.size.into(), format);
    // }

    // pub fn draw_horizontal_bar(&mut self, xy: [i32; 2], width: i32, value: i32, max: i32) {
    //     self.draw_horizontal_bar_color(xy, width, value, max, Color::WHITE, Color::GRAY);
    // }

    // pub fn draw_horizontal_bar_color(
    //     &mut self,
    //     xy: [i32; 2],
    //     width: i32,
    //     value: i32,
    //     max: i32,
    //     filled_color: Color,
    //     empty_color: Color,
    // ) {
    //     let [x, y] = xy;
    //     let normalized = match max {
    //         0 => 0.0,
    //         _ => value as f32 / max as f32,
    //     };

    //     let v = f32::ceil(normalized * width as f32) as i32;

    //     let filled_format = CharFormat::default().with_fg_color(filled_color);
    //     let empty_format = CharFormat::default().with_fg_color(empty_color);
    //     for i in 0..v {
    //         self.put_char_formatted([x + i, y], '▓', filled_format);
    //     }
    //     for i in v..width {
    //         self.put_char_formatted([x + i, y], '░', empty_format);
    //     }
    // }

    /// Clear the terminal tiles to default - empty tiles with
    /// a black background
    pub fn clear(&mut self) {
        for t in self.tiles.iter_mut() {
            *t = Tile::default()
        }
    }

    /// Returns true if the given position is inside the bounds of the terminal.
    pub fn is_in_bounds(&self, xy: [i32; 2]) -> bool {
        let [x, y] = xy;
        (x as usize) < self.width() && (y as usize) < self.height()
    }

    /// An immutable iterator over the tiles of the terminal.
    pub fn iter(&self) -> Iter<Tile> {
        self.tiles.iter()
    }

    /// A mutable iterator over the tiles of the terminal.
    pub fn iter_mut(&mut self) -> IterMut<Tile> {
        self.tiles.iter_mut()
    }

    /// An immutable iterator over an entire row of tiles in the terminal.
    pub fn row_iter(&self, y: usize) -> Iter<Tile> {
        self.tiles.row_iter(y)
    }

    /// A mutable iterator over an entire row of tiles in the terminal.
    pub fn row_iter_mut(&mut self, y: usize) -> impl Iterator<Item = &mut Tile> {
        self.tiles.row_iter_mut(y)
    }

    /// An immutable iterator over an entire column of tiles in the terminal.
    pub fn column_iter(&self, x: usize) -> StepBy<Iter<Tile>> {
        self.tiles.column_iter(x)
    }

    /// A mutable iterator over an entire column of tiles in the terminal.
    pub fn column_iter_mut(&mut self, x: usize) -> StepBy<IterMut<Tile>> {
        self.tiles.column_iter_mut(x)
    }

    /// The index of the bottom row of the terminal (0).
    pub fn bottom_index(&self) -> usize {
        0
    }

    /// The index of the top row of the terminal.
    pub fn top_index(&self) -> usize {
        self.height() as usize - 1
    }

    /// The index of the left-most column of the terminal (0).
    pub fn left_index(&self) -> usize {
        0
    }

    /// THe index of the right-most column of the terminal.
    pub fn right_index(&self) -> usize {
        self.width() as usize - 1
    }
}

#[derive(Error, Debug)]
pub enum TerminalWriteError {
    #[error("{xy} is out of bounds {size}")]
    OutOfBounds{ xy: IVec2, size: IVec2 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_char() {
        let mut term = Terminal::with_size([20, 20]);

        term.put_char([5, 5], 'h');

        assert_eq!('h', term.get_char([5, 5]));

        term.put_char([1,2], 'q'.fg(Color::RED));

        let t = term.get_tile([1,2]);
        assert_eq!('q', t.glyph);
        assert_eq!(Color::RED, t.fg_color);
    }

    #[test]
    fn put_string() {
        let mut term = Terminal::with_size([20, 20]);
        term.put_string([0, 0], "Hello");
        assert_eq!("Hello", term.get_string([0, 0], 5));

        term.put_string([18, 19], "Hello");
        assert_eq!("He", term.get_string([18, 19], 2));
    }

    // #[test]
    // fn edges() {
    //     let mut term = Terminal::with_size([25, 20]);
    //     term.put_char([0, 0], 'a');
    //     term.put_char([24, 19], 'a');
    // }

    // #[test]
    // fn column_get() {
    //     let mut term = Terminal::with_size([15, 10]);
    //     term.put_char([3, 0], 'H');
    //     term.put_char([3, 1], 'e');
    //     term.put_char([3, 2], 'l');
    //     term.put_char([3, 3], 'l');
    //     term.put_char([3, 4], 'o');

    //     let chars: Vec<_> = term.column_iter(3).take(5).map(|t| t.glyph).collect();
    //     assert_eq!("Hello", String::from_iter(chars));
    // }

    // #[test]
    // fn column_put() {
    //     let mut term = Terminal::with_size([15, 10]);
    //     let text = "Hello".chars();
    //     for (mut t, c) in term.column_iter_mut(5).take(5).zip(text) {
    //         t.glyph = c;
    //     }

    //     assert_eq!('H', term.get_char([5, 0]));
    //     assert_eq!('e', term.get_char([5, 1]));
    //     assert_eq!('l', term.get_char([5, 2]));
    //     assert_eq!('l', term.get_char([5, 3]));
    //     assert_eq!('o', term.get_char([5, 4]));
    // }

    // // #[test]
    // // fn border_test() {
    // //     let mut term = Terminal::with_size([10, 10]);
    // //     term.draw_box_single([0, 0], [5, 5]);

    // //     assert_eq!(term.get_char([0, 4]), SINGLE_LINE_GLYPHS.top_left);
    // //     assert_eq!(term.get_char([0, 0]), SINGLE_LINE_GLYPHS.bottom_left);
    // //     assert_eq!(term.get_char([4, 4]), SINGLE_LINE_GLYPHS.top_right);
    // //     assert_eq!(term.get_char([4, 0]), SINGLE_LINE_GLYPHS.bottom_right);
    // // }
}
