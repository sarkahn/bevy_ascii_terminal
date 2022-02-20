use std::iter::FromIterator;

use bevy::prelude::*;

use crate::TileWriter;
use crate::TilesWriter;
use crate::point::Point2d;
use crate::point::Point2dFormatter;
use crate::point::Size2d;
use crate::renderer::TilePivot;

/// A single tile of the terminal.
///
/// Defaults to a blank glyph with a black background and a white foreground.
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    /// The glyph for the tile. Glyphs are mapped to sprites via the
    /// terminal's [UvMapping](super::renderer::uv_mapping::UvMapping)
    pub key: u16,
    /// The forergound color for the tile.
    pub fg_color: Color,
    /// The background color for the tile.
    pub bg_color: Color,
}

impl Tile {
    /// Retrieve the tile key as a glyph
    pub fn glyph(&self) -> char {
        self.key as u8 as char
    }

    pub fn set_glyph(&mut self, glyph: char) {
        self.key = glyph as u16;
    }
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
/// term.put_char([1,1], 'h');
/// term.put_string([2,1], "ello");
///
/// let hello = term.get_string([1,1], 5);
/// ```
#[derive(Component, Default)]
pub struct Terminal {
    pub tiles: Vec<Tile>,
    size: UVec2,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            key: ' ' as u16,
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
    top: '─',
    left: '│',
    right: '│',
    bottom: '─',
    top_left: '┌',
    top_right: '┐',
    bottom_left: '└',
    bottom_right: '┘',
};

/// Double line border glyphs. Can be used in box drawing functions.
pub const DOUBLE_LINE_GLYPHS: BorderGlyphs = BorderGlyphs {
    top: '═',
    left: '║',
    right: '║',
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
            tiles: vec![Tile::default(); size.len()],
            size: size.size(),
        }
    }

    /// Resize the terminal's internal tile data.
    /// 
    /// Erases all existing data.
    pub fn resize(&mut self, size: impl Size2d) {
        self.tiles = vec![Tile::default(); size.len()];
        self.size = size.size();
    }

    pub fn width(&self) -> u32 {
        self.size.x
    }

    pub fn height(&self) -> u32 {
        self.size.y
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    /// Convert a 2D position to it's corresponding 1D index
    /// in the terminal.
    #[inline]
    pub fn to_index(&self, xy: impl Point2d) -> usize {
        (xy.y() * self.width() as i32 + xy.x()) as usize
    }

    /// Convert 1D index to it's 2D position given the dimensions
    /// of the terminal.
    #[inline]
    pub fn to_xy(&self, i: usize) -> IVec2 {
        let i = i as i32;
        let w = self.width() as i32;
        let x = i % w;
        let y = i / w;
        IVec2::new(x, y)
    }



    /// Write to the terminal.
    /// 
    /// ### Formatting
    /// Formatting can be applied to the position and the content applied to the write. Positions can be pivot aligned:
    /// 
    /// ```rust
    /// put_char([2,2].pivot(Pivot::TopRight), 'q'); // Write 'q' to the position 2 left and 
    ///                                              // 2 down from the top right of the terminal.
    /// ```
    /// 
    /// Any type implementing `TileWriter` can be applied to the terminal. There are various built in
    /// types for different operations:
    /// 
    /// ```rust
    /// put_char([1,1], 'a'.fg(Color::BLUE)); // Write a blue 'a' to position [1,1].
    ///                                       // Existing tile background color is not affected.
    /// 
    /// put_char([2,2], BGColor(Color::RED)); // Set the background color for position [2,2].
    ///                                       // Existing tile glyph and foreground color are
    ///                                       // not affected.
    /// ```
    pub fn put_char(&mut self, xy: impl Point2dFormatter, write: impl TileWriter) {
        let xy = xy.point(self.size());
        let i = self.to_index(xy);
        write.write(&mut self.tiles[i]);
    }

    /// Write to the terminal.
    /// 
    /// ### Formatting
    /// Formatting can be applied to the position and the content applied to the write. Positions can be pivot aligned:
    /// 
    /// ```rust
    /// put_tile([2,2].pivot(Pivot::TopRight), 'q'); // Write 'q' to the position 2 left and 
    ///                                              // 2 down from the top right of the terminal.
    /// ```
    /// 
    /// Any type implementing `TileWriter` can be applied to the terminal. There are various built in
    /// types for different operations:
    /// 
    /// ```rust
    /// put_tile([1,1], 'a'.fg(Color::BLUE)); // Write a blue 'a' to position [1,1].
    ///                                       // Existing tile background color is not affected.
    /// 
    /// put_tile([2,2], BGColor(Color::RED)); // Set the background color for position [2,2].
    ///                                       // Existing tile glyph and foreground color are
    ///                                       // not affected.
    /// ```
    pub fn put_tile(&mut self, xy: impl Point2dFormatter, tile: impl TileWriter) {
        self.put_char(xy, tile);
    }

    /// Apply a write to whole terminal.
    pub fn fill(&mut self, tile: impl TileWriter) {
        self.iter_mut().for_each(move |t| tile.write(t));
    }

    /// Write a string to the terminal.
    /// 
    /// ### Formatting
    /// 
    /// The position can be aligned and the string colors can be formatted:
    /// 
    /// ```rust
    /// put_string([1,1].pivot(Pitot::TopLeft), "Hello".fg(Color::RED)); // Write a red "Hello" starting from the
    ///                                                                  // position 1 right and 1 down from the top
    ///                                                                  // left corner of the terminal.
    /// ```
    pub fn put_string<'a>(&'a mut self, xy: impl Point2dFormatter, writer: impl TilesWriter<'a>) {
        let len = writer.len();
        let xy = xy.point(self.size());
        let i = self.to_index(xy);
        let tiles = self.tiles[i..].iter_mut().take(len);

        writer.write(tiles);
    }

    /// Retrieve the glyph from a tile.
    pub fn get_char(&self, xy: impl Point2dFormatter) -> char {
        let xy = xy.point(self.size());
        self.get_tile(xy).glyph()
    }

    /// Retrieve a string from the terminal.
    pub fn get_string(&self, xy: impl Point2dFormatter, len: usize) -> String {
        let xy = xy.point(self.size());
        let i = self.to_index(xy);
        let tiles = self.tiles[i..].iter().take(len).map(|t|t.glyph());
        String::from_iter(tiles)
    }

    /// Retrieve an immutable reference to a tile in the terminal.
    pub fn get_tile(&self, xy: impl Point2dFormatter) -> &Tile {
        let xy = xy.point(self.size());
        &self.tiles[self.to_index(xy)]
    }

    /// Retrieve a mutable reference to a tile in the terminal.
    pub fn get_tile_mut(&mut self, xy: impl Point2dFormatter) -> &mut Tile {
        let xy = xy.point(self.size());
        let i = self.to_index(xy);
        &mut self.tiles[i]
    }

    /// Clear an area of the terminal to the default [Tile].
    pub fn clear_box(&mut self, xy: impl Point2dFormatter, size: impl Size2d) {
        let origin = xy.point(self.size());

        for y in 0..size.height() as i32 {
            for x in 0..size.width() as i32 {
                let p = origin + xy.relative_point([x,y]);
                self.put_tile(p, Tile::default());
            }
        }
    }

    /// Draw a box on the terminal using [BorderGlyphs].
    pub fn draw_box(&mut self, xy: [i32; 2], size: [u32; 2], border_glyphs: BorderGlyphs) {

        
        let [x, y] = xy;
        let [width, height] = size;
        let width = width as usize;
        let height = height as usize;
        let left = x as usize;
        let right = x as usize + width - 1;
        let bottom = y as usize;
        let top = y as usize + height - 1;

        for t in self.row_iter_mut(top).skip(left).take(width) {
            t.set_glyph(border_glyphs.top);
        }
        for t in self.row_iter_mut(bottom).skip(left).take(width) {
            t.set_glyph(border_glyphs.bottom);
        }
        for t in self.column_iter_mut(left).skip(bottom).take(height) {
            t.set_glyph(border_glyphs.left);
        }
        for t in self.column_iter_mut(right).skip(bottom).take(height) {
            t.set_glyph(border_glyphs.right);
        }

        let left = left as i32;
        let right = right as i32;
        let top = top as i32;
        let bottom = bottom as i32;

        self.put_char([left, bottom], border_glyphs.bottom_left);
        self.put_char([left, top], border_glyphs.top_left);
        self.put_char([right, top], border_glyphs.top_right);
        self.put_char([right, bottom], border_glyphs.bottom_right);
    }

    /// Draw a box with a single-line border.
    pub fn draw_box_single(&mut self, xy: [i32; 2], size: [u32; 2]) {
        self.draw_box(xy, size, SINGLE_LINE_GLYPHS);
    }

    // /// Draw a box with a double-line border.
    pub fn draw_box_double(&mut self, xy: [i32; 2], size: [u32; 2]) {
        self.draw_box(xy, size, DOUBLE_LINE_GLYPHS);
    }

    // /// Draw a box with a colored double-line border.
    // pub fn draw_box_double_formatted(&mut self, xy: [i32; 2], size: [u32; 2], format: CharFormat) {
    //     self.draw_box_formatted(xy, size, DOUBLE_LINE_GLYPHS, format);
    // }

    /// Draw a border around the edges of the terminal.
    pub fn draw_border(&mut self, border_glyphs: BorderGlyphs) {
        self.draw_box([0, 0], self.size().into(), border_glyphs);
    }

    /// Draw a single-line border around the edge of the whole terminal.
    pub fn draw_border_single(&mut self) {
        self.draw_box_single([0, 0], self.size.into());
    }

    // /// Draw a double-line border around the edge of the whole terminal.
    pub fn draw_border_double(&mut self) {
        self.draw_box_double([0, 0], self.size.into());
    }
    // /// Draw a colored double-line border around the edge of the whole terminal.
    // pub fn draw_border_double_formatted(&mut self, format: CharFormat) {
    //     self.draw_box_double_formatted([0, 0], self.size.into(), format);
    // }

    /// Draw a fill bar with default formatting.
    pub fn draw_horizontal_bar(&mut self, xy: [i32; 2], width: i32, value: i32, max: i32) {
        self.draw_horizontal_bar_custom(xy, width, value, max, 
            '▓'.fg(Color::WHITE), 
            '░'.fg(Color::GRAY));
    }

    /// Draw a horizontal fill bar with custom filled and empty formatting.
    pub fn draw_horizontal_bar_custom(
        &mut self,
        xy: [i32; 2],
        width: i32,
        value: i32,
        max: i32,
        empty: impl TileWriter,
        filled: impl TileWriter,
    ) {
        let [x, y] = xy;
        let normalized = match max {
            0 => 0.0,
            _ => value as f32 / max as f32,
        };

        let v = f32::ceil(normalized * width as f32) as i32;

        for i in 0..v {
            self.put_char([x + i, y], filled);
        }
        for i in v..width {
            self.put_char([x + i, y], empty);
        }
    }

    /// Clear the terminal tiles to default - empty tiles with
    /// a black background
    pub fn clear(&mut self) {
        self.fill(Tile::default())
    }

    /// Returns true if the given position is inside the bounds of the terminal.
    pub fn is_in_bounds(&self, size: impl Size2d) -> bool {
        size.size().cmplt(self.size()).all()
    }

    /// An immutable iterator over the tiles of the terminal.
    pub fn iter(&self) -> impl Iterator<Item=&Tile> {
        self.tiles.iter()
    }

    /// A mutable iterator over the tiles of the terminal.
    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut Tile> {
        self.tiles.iter_mut()
    }

    /// An immutable iterator over an entire row of tiles in the terminal.
    pub fn row_iter(&self, y: usize) -> impl Iterator<Item=&Tile> {
        let w = self.width() as usize;
        let i = y * w;
        self.tiles[i..i + w].iter()
    }

    /// A mutable iterator over an entire row of tiles in the terminal.
    pub fn row_iter_mut(&mut self, y: usize) -> impl Iterator<Item = &mut Tile> {
        let w = self.width() as usize;
        let i = y * w;
        self.tiles[i..i + w].iter_mut()
    }

    /// An immutable iterator over an entire column of tiles in the terminal.
    pub fn column_iter(&self, x: usize) -> impl Iterator<Item=&Tile> {
        let w = self.width() as usize;
        return self.tiles[x..].iter().step_by(w);
    }

    /// A mutable iterator over an entire column of tiles in the terminal.
    pub fn column_iter_mut(&mut self, x: usize) -> impl Iterator<Item=&mut Tile> {
        let w = self.width() as usize;
        return self.tiles[x..].iter_mut().step_by(w);
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

    /// The index of the right-most column of the terminal.
    pub fn right_index(&self) -> usize {
        self.width() as usize - 1
    }
}

#[cfg(test)]
mod tests {
    use crate::Pivot;

    use super::*;

    #[test]
    fn put_char() {
        let mut term = Terminal::with_size([20, 20]);

        term.put_char([5, 5], 'h');

        assert_eq!('h', term.get_char([5, 5]));
    }

    #[test]
    fn put_string() {
        let mut term = Terminal::with_size([20, 20]);
        term.put_string([0, 0], "Hello");
        assert_eq!("Hello", term.get_string([0, 0], 5));

        term.put_string([18, 19], "Hello");
        assert_eq!("He", term.get_string([18, 19], 2));
    }

    #[test]
    fn edges() {
        let mut term = Terminal::with_size([25, 20]);
        term.put_char([0, 0], 'a');
        term.put_char([24, 19], 'a');
    }

    #[test]
    fn column_get() {
        let mut term = Terminal::with_size([15, 10]);
        term.put_char([3, 0], 'H');
        term.put_char([3, 1], 'e');
        term.put_char([3, 2], 'l');
        term.put_char([3, 3], 'l');
        term.put_char([3, 4], 'o');

        let chars: Vec<_> = term.column_iter(3).take(5).map(|t| t.glyph()).collect();
        assert_eq!("Hello", String::from_iter(chars));
    }

    #[test]
    fn column_put() {
        let mut term = Terminal::with_size([15, 10]);
        let text = "Hello".chars();
        for (t, c) in term.column_iter_mut(5).take(5).zip(text) {
            t.set_glyph(c);
        }

        assert_eq!('H', term.get_char([5, 0]));
        assert_eq!('e', term.get_char([5, 1]));
        assert_eq!('l', term.get_char([5, 2]));
        assert_eq!('l', term.get_char([5, 3]));
        assert_eq!('o', term.get_char([5, 4]));
    }

    #[test]
    fn tile_writers() {
        let mut term = Terminal::with_size([5,5]);
        term.put_tile([0,0], 'a'.fg(Color::GREEN).bg(Color::YELLOW));
        let tile = term.get_tile([0,0]);
        assert_eq!(Color::GREEN, tile.fg_color);
        assert_eq!(Color::YELLOW, tile.bg_color);
    }

    #[test]
    fn string_writer() {
        let mut term = Terminal::with_size([10,10]);
        term.put_string([1,1], "Hello");

        assert_eq!("Hello", term.get_string([1,1], 5));
    }

    #[test]
    fn box_test() {
        let mut term = Terminal::with_size([10,10]);

        term.fill('a');

        term.clear_box([1,1].pivot(Pivot::TopRight), [3,3]);

        assert_eq!('a', term.get_char([1,1]));
        assert_eq!(' ', term.get_char([8,8]));
        assert_eq!(' ', term.get_char([6,6]));
    }

    // #[test]
    // fn border_test() {
    //     let mut term = Terminal::with_size([10, 10]);
    //     term.draw_box_single([0, 0], [5, 5]);

    //     assert_eq!(term.get_char([0, 4]), SINGLE_LINE_GLYPHS.top_left);
    //     assert_eq!(term.get_char([0, 0]), SINGLE_LINE_GLYPHS.bottom_left);
    //     assert_eq!(term.get_char([4, 4]), SINGLE_LINE_GLYPHS.top_right);
    //     assert_eq!(term.get_char([4, 0]), SINGLE_LINE_GLYPHS.bottom_right);
    // }
}
