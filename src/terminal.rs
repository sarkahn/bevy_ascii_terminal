use std::iter::FromIterator;
use std::iter::StepBy;
use std::slice::Iter;
use std::slice::IterMut;

use bevy::prelude::*;

use crate::color::*;

use sark_grids::Grid;

/// A single tile of the terminal.
///
/// Defaults to a blank glyph with a black background and a white foreground.
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    /// The glyph for the tile. Glyphs are mapped to sprites via the
    /// terminal's [GlyphMapping](super::renderer::glyph_mapping::GlyphMapping)
    pub glyph: char,
    /// The forergound color for the tile.
    pub fg_color: TileColor,
    /// The background color for the tile.
    pub bg_color: TileColor,
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
    pub tiles: Grid<Tile>,
    size: UVec2,
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            glyph: ' ',
            fg_color: WHITE,
            bg_color: BLACK,
        }
    }
}

/// Border glyphs used in box drawing functions.
///
/// Specifies the style of lines to use along the border of the box.
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
const SINGLE_LINE_GLYPHS: BorderGlyphs = BorderGlyphs {
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
const DOUBLE_LINE_GLYPHS: BorderGlyphs = BorderGlyphs {
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
    pub fn with_size(size: [u32; 2]) -> Terminal {
        Terminal {
            tiles: Grid::default(size),
            size: UVec2::from(size),
        }
    }

    /// Resize the terminal's internal tile data.
    pub fn resize(&mut self, size: [u32; 2]) {
        self.tiles = Grid::default(size);
        self.size = UVec2::from(size);
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
    ///
    /// Note that in the terminal the y axis goes from top to bottom.
    #[inline]
    pub fn to_index(&self, xy: [i32; 2]) -> usize {
        self.tiles.pos_to_index(xy)
    }

    /// Convert a 2D position to it's corresponding 1D index
    /// in the terminal with the y axis flipped.
    ///
    /// In the terminal the y axis goes from top to bottom.
    /// This should be used when passing in coordinates where
    /// the y axis goes from bottom to top.
    #[inline]
    pub fn to_index_flipped(&self, xy: [i32; 2]) -> usize {
        let (x, y) = self.y_flip(xy).into();
        (y * self.width() as i32 + x) as usize
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

    /// Convert 1D index to it's 2D position given the dimensions
    /// of the terminal.
    ///
    /// In the terminal the y axis goes from top to bottom.
    /// This should be used when passing in an index derived from
    /// coordinates where the y axis goes from bottom to top.
    #[inline]
    pub fn to_xy_flipped(&self, i: usize) -> IVec2 {
        let xy = self.to_xy(i);
        self.y_flip(xy.into())
    }

    /// Flip the given position based on the height of the terminal.
    ///
    /// In the terminal the y axis goes from top to bottom.
    /// This should be used when passing in coordinates where
    /// the y axis goes from bottom to top.
    #[inline]
    pub fn y_flip(&self, pos: [i32; 2]) -> IVec2 {
        let [x, y] = pos;
        let y = self.height() as i32 - 1 - y;
        IVec2::new(x, y)
    }

    /// Insert a character.
    ///
    /// The existing foreground and background color of the tile will remain.
    pub fn put_char(&mut self, xy: [i32; 2], glyph: char) {
        self.get_tile_mut(xy).glyph = glyph;
    }

    /// Attempt to insert a character.
    ///
    /// The existing foreground and background colors of the tile will remain.
    /// Returns an error if the position is out of bounds.
    pub fn try_put_char_pos(&mut self, xy: [i32; 2], glyph: char) -> Result<(), String> {
        if !self.is_in_bounds(xy) {
            return Err(format!(
                "try_put_char_pos error, p {} is out of bounds {}",
                IVec2::from(xy),
                self.size
            ));
        }
        self.put_char(xy, glyph);
        Ok(())
    }

    /// Insert a character with colors.
    pub fn put_char_color(
        &mut self,
        xy: [i32; 2],
        glyph: char,
        fg_color: TileColor,
        bg_color: TileColor,
    ) {
        let t = self.get_tile_mut(xy);
        t.glyph = glyph;
        t.fg_color = fg_color;
        t.bg_color = bg_color;
    }

    /// Insert a [Tile].
    pub fn put_tile(&mut self, xy: [i32; 2], tile: Tile) {
        let t = self.get_tile_mut(xy);
        *t = tile;
    }

    /// Write a string to the terminal.
    ///
    /// The string will move to the next line if it reaches the edge
    /// and will truncate at the end of the terminal.
    pub fn put_string(&mut self, xy: [i32; 2], string: &str) {
        let i = self.to_index(xy);
        let tiles = self.tiles.slice_mut(i..).iter_mut().take(string.len());
        let chars = string.chars().take(tiles.len());

        for (char, mut t) in chars.zip(tiles) {
            t.glyph = char;
        }
    }

    /// Write a string to the terminal with colors.
    ///
    /// The string will move to the next line if it reaches the edge
    /// and will truncate at the end of the terminal.
    pub fn put_string_color(
        &mut self,
        xy: [i32; 2],
        string: &str,
        fg_color: TileColor,
        bg_color: TileColor,
    ) {
        let i = self.to_index(xy);
        let tiles = self.tiles.slice_mut(i..).iter_mut().take(string.len());
        let chars = string.chars().take(tiles.len());

        for (char, mut t) in chars.zip(tiles) {
            t.glyph = char;
            t.fg_color = fg_color;
            t.bg_color = bg_color;
        }
    }

    /// Set the foreground color of a tile.
    ///
    /// The existing background color and glyph of the tile will remain.
    pub fn put_fg_color(&mut self, xy: [i32; 2], col: TileColor) {
        self.get_tile_mut(xy).fg_color = col;
    }

    /// Set the background color of a tile.
    ///
    /// The existing foreground color and glyph of the tile will remain.
    pub fn put_bg_color(&mut self, xy: [i32; 2], col: TileColor) {
        self.get_tile_mut(xy).bg_color = col;
    }

    /// Retrieve the char from a tile.
    pub fn get_char(&self, xy: [i32; 2]) -> char {
        self.get_tile(xy).glyph
    }

    /// Retrieve a string from the terminal.
    pub fn get_string(&self, xy: [i32; 2], len: usize) -> String {
        let i = self.to_index(xy);
        let slice = self.tiles.slice(i..).iter().take(len);
        let mut chars: Vec<char> = vec![' '; slice.len()];

        for (i, t) in slice.enumerate() {
            chars[i] = t.glyph;
        }

        String::from_iter(chars)
    }

    /// Retrieve an immutable reference to a tile in the terminal.
    pub fn get_tile(&self, xy: [i32; 2]) -> &Tile {
        &self.tiles[self.to_index(xy)]
    }

    /// Retrieve a mutable reference to a tile in the terminal.
    pub fn get_tile_mut(&mut self, xy: [i32; 2]) -> &mut Tile {
        let i = self.to_index(xy);
        &mut self.tiles[i]
    }

    /// Clear an area of the terminal to the default [Tile].
    pub fn clear_box(&mut self, xy: [i32; 2], size: [u32; 2]) {
        let [width, height] = size;
        let [x, y] = xy;
        for y in y..y + height as i32 {
            for x in x..x + width as i32 {
                self.put_tile([x, y], Tile::default());
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
        let top = y as usize;
        let bottom = y as usize + height - 1;

        for t in self.row_iter_mut(top).skip(left).take(width) {
            t.glyph = border_glyphs.top;
        }
        for t in self.row_iter_mut(bottom).skip(left).take(width) {
            t.glyph = border_glyphs.bottom;
        }
        for t in self.column_iter_mut(left).skip(top).take(height) {
            t.glyph = border_glyphs.left;
        }
        for t in self.column_iter_mut(right).skip(top).take(height) {
            t.glyph = border_glyphs.right;
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

    /// Draw a box with box with the specified colors and [BorderGlyphs].
    pub fn draw_box_color(
        &mut self,
        xy: [i32; 2],
        size: [u32; 2],
        fg_color: TileColor,
        bg_color: TileColor,
        border_glyphs: BorderGlyphs,
    ) {
        let [x, y] = xy;
        let [width, height] = size;
        let width = width as usize;
        let height = height as usize;
        let x = x as usize;
        let y = y as usize;
        let left = x;
        let right = x + width - 1;
        let top = y;
        let bottom = y + height - 1;

        for t in self.row_iter_mut(top).skip(left).take(width) {
            t.glyph = border_glyphs.top;
            t.fg_color = fg_color;
            t.bg_color = bg_color;
        }
        for t in self.row_iter_mut(bottom).skip(left).take(width) {
            t.glyph = border_glyphs.bottom;
            t.fg_color = fg_color;
            t.bg_color = bg_color;
        }
        for t in self.column_iter_mut(left).skip(top).take(height) {
            t.glyph = border_glyphs.left;
            t.fg_color = fg_color;
            t.bg_color = bg_color;
        }
        for t in self.column_iter_mut(right).skip(top).take(height) {
            t.glyph = border_glyphs.right;
            t.fg_color = fg_color;
            t.bg_color = bg_color;
        }

        let left = left as i32;
        let right = right as i32;
        let top = top as i32;
        let bottom = bottom as i32;

        self.put_char_color(
            [left, bottom],
            border_glyphs.bottom_left,
            fg_color,
            bg_color,
        );
        self.put_char_color([left, top], border_glyphs.top_left, fg_color, bg_color);
        self.put_char_color([right, top], border_glyphs.top_right, fg_color, bg_color);
        self.put_char_color(
            [right, bottom],
            border_glyphs.bottom_right,
            fg_color,
            bg_color,
        );
    }

    /// Draw a box with a single-line border.
    pub fn draw_box_single(&mut self, xy: [i32; 2], size: [u32; 2]) {
        self.draw_box(xy, size, SINGLE_LINE_GLYPHS);
    }
    /// Draw a box with a colored single-line border.
    pub fn draw_box_single_color(
        &mut self,
        xy: [i32; 2],
        size: [u32; 2],
        fg_color: TileColor,
        bg_color: TileColor,
    ) {
        self.draw_box_color(xy, size, fg_color, bg_color, SINGLE_LINE_GLYPHS);
    }

    /// Draw a box with a double-line border.
    pub fn draw_box_double(&mut self, xy: [i32; 2], size: [u32; 2]) {
        self.draw_box(xy, size, DOUBLE_LINE_GLYPHS);
    }
    /// Draw a box with a colored double-line border.
    pub fn draw_box_double_color(
        &mut self,
        xy: [i32; 2],
        size: [u32; 2],
        fg_color: TileColor,
        bg_color: TileColor,
    ) {
        self.draw_box_color(xy, size, fg_color, bg_color, DOUBLE_LINE_GLYPHS);
    }

    /// Draw a single-line border around the edge of the whole terminal.
    pub fn draw_border_single(&mut self) {
        self.draw_box_single([0, 0], self.size.into());
    }

    /// Draw a colored single-line border around the edge of the whole terminal.
    pub fn draw_border_single_color(&mut self, fg_color: TileColor, bg_color: TileColor) {
        self.draw_box_single_color([0, 0], self.size.into(), fg_color, bg_color);
    }

    /// Draw a double-line border around the edge of the whole terminal.
    pub fn draw_border_double(&mut self) {
        self.draw_box_double([0, 0], self.size.into());
    }
    /// Draw a colored double-line border around the edge of the whole terminal.
    pub fn draw_border_double_color(&mut self, fg_color: TileColor, bg_color: TileColor) {
        self.draw_box_double_color([0, 0], self.size.into(), fg_color, bg_color);
    }

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
        (x as u32) < self.width() && (y as u32) < self.height()
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
    pub fn row_iter_mut(&mut self, y: usize) -> IterMut<Tile> {
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
}

#[cfg(test)]
mod tests {
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

        let chars: Vec<_> = term.column_iter(3).take(5).map(|t| t.glyph).collect();
        assert_eq!("Hello", String::from_iter(chars));
    }

    #[test]
    fn column_put() {
        let mut term = Terminal::with_size([15, 10]);
        let text = "Hello".chars();
        for (mut t, c) in term.column_iter_mut(5).take(5).zip(text) {
            t.glyph = c;
        }

        assert_eq!('H', term.get_char([5, 0]));
        assert_eq!('e', term.get_char([5, 1]));
        assert_eq!('l', term.get_char([5, 2]));
        assert_eq!('l', term.get_char([5, 3]));
        assert_eq!('o', term.get_char([5, 4]));
    }

    #[test]
    fn border_test() {
        let mut term = Terminal::with_size([10, 10]);
        term.draw_box_single([0, 0], [5, 5]);

        assert_eq!(term.get_char([0, 0]), SINGLE_LINE_GLYPHS.top_left);
        assert_eq!(term.get_char([0, 4]), SINGLE_LINE_GLYPHS.bottom_left);
        assert_eq!(term.get_char([4, 0]), SINGLE_LINE_GLYPHS.top_right);
        assert_eq!(term.get_char([4, 4]), SINGLE_LINE_GLYPHS.bottom_right);
    }
}
