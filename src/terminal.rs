use std::iter::FromIterator;
use std::iter::StepBy;
use std::slice::Iter;
use std::slice::IterMut;

use bevy::prelude::*;

use sark_grids::Grid;
use sark_grids::GridPoint;
use sark_grids::Size2d;
use sark_grids::grid::Side;

use crate::formatting::StringWrite;
use crate::formatting::TileModifier;
use crate::formatting::fmt_string::StringWriter;
use crate::ui::UiProgressBar;
use crate::ui::ui_box::BorderGlyphs;
use crate::ui::ui_box::UiBox;



/// A simple terminal for writing text in a readable grid.
///
/// Contains various functions for drawing colorful text to a
/// terminal.
///
/// # Example
/// ```
/// use bevy_ascii_terminal::*;
/// use bevy::prelude::Color;
///
/// let mut term = Terminal::with_size([10,10]);
///
/// term.put_char([1,1], 'h'.fg(Color::RED));
/// term.put_string([2,1], "ello".bg(Color::BLUE));
///
/// let hello = term.get_string([1,1], 5);
/// ```
#[derive(Component, Default)]
pub struct Terminal {
    pub tiles: Grid<Tile>,
    size: UVec2,
}

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorModifier {
    FgColor(Color),
    BgColor(Color),
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

impl Terminal {
    /// Construct a terminal with the given size
    pub fn with_size(size: impl Size2d) -> Terminal {
        Terminal {
            tiles: Grid::default(size),
            size: size.as_uvec2(),
        }
    }

    /// Resize the terminal's internal tile data.
    /// 
    /// This will clear all tiles to default.
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
    #[inline]
    pub fn to_index(&self, xy: impl GridPoint) -> usize {
        self.tiles.pos_to_index(xy)
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

    /// Insert a formatted character into the terminal.
    ///
    /// The [TileWriter] trait allows you to optionally specify a foreground
    /// and/or background color for the tile as well. If you don't specify a
    /// color then the existing color in the terminal will be unaffected.
    /// 
    /// All tiles in the terminal begin with a white foreground and black background.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use bevy_ascii_terminal::*;
    /// use bevy::prelude::Color;
    /// 
    /// let mut term = Terminal::with_size([10,10]);
    /// // Insert an 'a' with a blue foreground and a red background.
    /// term.put_char([2,3], 'a'.fg(Color::BLUE).bg(Color::RED));
    /// // Replace the 'a' with a 'q'. Foreground and background color will be
    /// // unaffected
    /// term.put_char([2,3], 'q');
    /// ```
    pub fn put_char(&mut self, xy: impl GridPoint, writer: impl TileModifier) {
        let fmt = writer.format();
        fmt.draw(xy, self);
    }

    /// Change the foreground or background color for a single tile in the terminal.
    pub fn put_color(&mut self, xy: impl GridPoint, color: ColorModifier) {
        let tile = self.get_tile_mut(xy);
        match color {
            ColorModifier::FgColor(col) => tile.fg_color = col,
            ColorModifier::BgColor(col) => tile.bg_color = col,
        }
    }

    /// Insert a [Tile].
    pub fn put_tile(&mut self, xy: impl GridPoint, tile: Tile) {
        let t = self.get_tile_mut(xy);
        *t = tile;
    }

    /// Write a formatted string to the terminal.
    /// 
    /// The [StringWriter] trait allows you to optionally specify a foreground
    /// and/or background color for the string as well. If you don't specify a
    /// color then the existing color in the terminal will be unaffected.
    /// 
    /// All tiles in the terminal begin with a white foreground and black background.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use bevy_ascii_terminal::*;
    /// use bevy::prelude::Color;
    /// 
    /// let mut term = Terminal::with_size([10,10]);
    /// // Write a blue "Hello" to the terminal
    /// term.put_string([1,2], "Hello".fg(Color::BLUE));
    /// // Write "Hello" with a green background.
    /// term.put_string([2,1], "Hello".bg(Color::GREEN));
    /// ```
    pub fn put_string<'a>(&mut self, xy: impl GridPoint, writer: impl StringWriter<'a>) {
        let i = self.to_index(xy);

        let (string,writes) = writer.formatted().into();

        let count = string.chars().count();
        let tiles = self.tiles.slice_mut(i..).iter_mut().take(count);
        let count = tiles.len();
        let chars = string.chars().take(count);

        for (char, mut t) in chars.zip(tiles) {
            t.glyph = char;
        }

        for write in writes {
            match write {
                StringWrite::FgColor(col) => {   
                    let tiles = self.tiles.slice_mut(i..).iter_mut().take(count);
                    for t in tiles {
                        t.fg_color = col;
                    }
                },
                StringWrite::BgColor(col) => {   
                    let tiles = self.tiles.slice_mut(i..).iter_mut().take(count);
                    for t in tiles {
                        t.bg_color = col;
                    }
                },
            }
        }

    }

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

    /// Clear an area of the terminal to the default [Tile].
    pub fn clear_box(&mut self, xy: impl GridPoint, size: impl Size2d) {
        let [width, height] = size.to_array();
        let [x, y] = xy.to_array();
        for y in y..y + height as i32 {
            for x in x..x + width as i32 {
                self.put_tile([x, y], Tile::default());
            }
        }
    }

    /// Draw a formatted box to the terminal.
    /// 
    /// The [BoxBuilder] trait allows you to specify the properties of the box.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use bevy_ascii_terminal::*;
    /// use bevy_ascii_terminal::ui::*;
    /// 
    /// let mut term = Terminal::with_size([10,10]);
    /// term.draw_box([0,0], [3,3], &UiBox::single_line());
    /// ```
    pub fn draw_box(&mut self, xy: impl GridPoint, size: impl Size2d, ui_box: &UiBox) {
        // TODO: Make boxes work with alignment.
        ui_box.draw(xy, size, self);
    }

    /// Draw a border around the entire terminal.
    pub fn draw_border(&mut self, glyphs: BorderGlyphs) {
        let bx = UiBox::new().border_glyphs(glyphs);
        bx.draw([0,0], self.size, self);
    }

    pub fn draw_progress_bar(&mut self, xy: impl GridPoint, size: usize, bar: &UiProgressBar) {
        // TODO: Make bars work with alignment.
        bar.draw(xy, size, self);
    }

    /// Clear the terminal tiles to default - empty tiles with
    /// a black background and white foreground.
    pub fn clear(&mut self) {
        for t in self.tiles.iter_mut() {
            *t = Tile::default()
        }
    }

    /// Returns true if the given position is inside the bounds of the terminal.
    pub fn is_in_bounds(&self, xy: impl GridPoint) -> bool {
        let xy = xy.as_uvec2();
        xy.cmpge(UVec2::ZERO).all() && xy.cmplt(self.size).all()
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

    /// Get the index for a given side on the terminal.
    pub fn side_index(&self, side: Side) -> usize {
        self.tiles.side_index(side)
    }
}


#[cfg(test)]
mod tests {
    use crate::ui::ui_box::BorderGlyphs;

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
        // let mut term = Terminal::with_size([20, 20]);
        // term.put_string([0, 0], "Hello");
        // assert_eq!("Hello", term.get_string([0, 0], 5));

        // term.put_string([18, 19], "Hello");
        // assert_eq!("He", term.get_string([18, 19], 2));
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

}
