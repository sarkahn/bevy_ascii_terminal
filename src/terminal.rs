use std::iter::FromIterator;
use std::slice::Iter;
use std::slice::IterMut;
use std::iter::StepBy;

use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub glyph: char,
    pub fg_color: Color,
    pub bg_color: Color,
}

#[derive(Default)]
pub struct TerminalSize {
    pub value: UVec2,
}

#[derive(Default)]
pub struct Terminal {
    pub tiles: Vec<Tile>,
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

impl Terminal {
    pub fn new(width: usize, height: usize) -> Terminal {
        Terminal {
            tiles: vec![Tile::default(); width * height],
            size: UVec2::new(width as u32, height as u32),
        }
    }

    pub fn width(&self) -> usize {
        self.size.x as usize
    }
    pub fn height(&self) -> usize {
        self.size.y as usize
    }

    #[inline]
    pub fn to_index(&self, x: usize, y: usize) -> usize {
        y * self.width() + x
    }
    
    #[inline]
    pub fn to_xy(&self, i: usize) -> (usize,usize) {
        let x = i % self.width();
        let y = i / self.width();
        (x,y)
    }

    pub fn put_char(&mut self, x: usize, y: usize, glyph: char) {
        self.get_tile_mut(x, y).glyph = glyph;
    }

    pub fn put_char_color(
        &mut self,
        x: usize,
        y: usize,
        glyph: char,
        fg_color: Color,
        bg_color: Color,
    ) {
        let t = self.get_tile_mut(x, y);
        t.glyph = glyph;
        t.fg_color = fg_color;
        t.bg_color = bg_color;
    }

    pub fn put_tile(&mut self, x: usize, y: usize, tile: Tile) {
        let t = self.get_tile_mut(x, y);
        *t = tile;
    }

    /// Write a string to the console. The string will move to the
    /// next line if it reaches the edge and will truncate at the
    /// end of the console
    pub fn put_string(&mut self, x: usize, y: usize, string: &str) {
        let i = self.to_index(x,y);
        let tiles = self.tiles[i..].iter_mut().take(string.len());
        let chars = string.chars().take(tiles.len());

        for (char, mut t) in chars.zip(tiles) {
            t.glyph = char;
        }
    }

    /// Write a string to the console. The string will move to the
    /// next line if it reaches the edge and will truncate at the
    /// end of the console
    pub fn put_string_color(
        &mut self,
        x: usize,
        y: usize,
        string: &str,
        fg_color: Color,
        bg_color: Color,
    ) {
        let i = self.to_index(x,y);
        let tiles = self.tiles[i..].iter_mut().take(string.len());
        let chars = string.chars().take(tiles.len());

        for (char, mut t) in chars.zip(tiles) {
            t.glyph = char;
            t.fg_color = fg_color;
            t.bg_color = bg_color;
        }
    }

    pub fn get_char(&self, x: usize, y: usize) -> char {
        self.get_tile(x, y).glyph
    }

    pub fn get_string(&self, x: usize, y: usize, len: usize) -> String {
        let i = self.to_index(x,y);
        let slice = self.tiles[i..].iter().take(len);
        let mut chars: Vec<char> = vec![' '; slice.len()];

        for (i, t) in slice.enumerate() {
            chars[i] = t.glyph;
        }

        String::from_iter(chars)
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.tiles[self.to_index(x,y)]
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        let i = self.to_index(x,y);
        &mut self.tiles[i]
    }

    pub fn clear_box(&mut self, x: usize, y: usize, width: usize, height: usize) {
        for y in y..y + height {
            for x in x..x + width {
                self.put_tile(x, y, Tile::default());
            }
        }
    }

    /// Draw a box with a single-line border
    pub fn draw_box_single(&mut self, x: usize, y: usize, width: usize, height: usize) {
        let left = x;
        let right = x + width - 1;
        let top = y;
        let bottom = y + height - 1;

        for t in self.row_iter_mut(top).skip(left).take(width) {
            t.glyph = '─';
        }
        for t in self.row_iter_mut(bottom).skip(left).take(width) {
            t.glyph = '─';
        }
        for t in self.column_iter_mut(left).skip(top).take(height) {
            t.glyph = '│';
        }
        for t in self.column_iter_mut(right).skip(top).take(height) {
            t.glyph = '│';
        }

        self.put_char(left, bottom, '└');
        self.put_char(left, top, '┌');
        self.put_char(right, top, '┐');
        self.put_char(right, bottom, '┘');
    }

    /// Draw a border around the edges of the console with
    // single-line edges.
    pub fn draw_border_single(&mut self) {
        self.draw_box_single(0, 0, self.width(), self.height());
    }

    /// Clear the console tiles to default - empty tiles with
    /// a black background
    pub fn clear(&mut self) {
        for t in self.tiles.iter_mut() {
            *t = Tile::default()
        } 
    }

    pub fn iter(&self) -> Iter<Tile> {
        self.tiles.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Tile> {
        self.tiles.iter_mut()
    }

    /// An iterator over an entire row of tiles in  the terminal.
    pub fn row_iter(&self, y: usize) -> Iter<Tile> {
        self.tiles[y * self.width()..self.width()].iter()
    }

    // A mutable iterator over an entire row of tiles in the terminal.
    pub fn row_iter_mut(&mut self, y: usize) -> IterMut<Tile> {
        let w = self.width();
        let i = y * w;
        self.tiles[i..i + w].iter_mut()
    }

    // An iterator over an entire column of tiles in the terminal.
    pub fn column_iter(&self, x: usize) -> StepBy<Iter<Tile>> {
        return self.tiles[x..].iter().step_by(self.width())
    }

    // A mutable iterator over an entire column of tiles in the terminal.
    pub fn column_iter_mut(&mut self, x: usize) -> StepBy<IterMut<Tile>> {
        let w = self.width();
        return self.tiles[x..].iter_mut().step_by(w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_char() {
        let mut term = Terminal::new(20, 20);

        term.put_char(5, 5, 'h');

        assert_eq!('h', term.get_char(5, 5));
    }

    #[test]
    fn put_string() {
        let mut term = Terminal::new(20, 20);
        term.put_string(0, 0, "Hello");
        assert_eq!("Hello", term.get_string(0, 0, 5));

        term.put_string(18, 19, "Hello");
        assert_eq!("He", term.get_string(18, 19, 2));
    }

    #[test]
    fn edges() {
        let mut term = Terminal::new(25, 20);
        term.put_char(0, 0, 'a');
        term.put_char(24, 19, 'a');
    }

    #[test]
    fn column_get() {
        let mut term = Terminal::new(15, 10);
        term.put_char(3,0,'H');
        term.put_char(3,1,'e');
        term.put_char(3,2,'l');
        term.put_char(3,3,'l');
        term.put_char(3,4,'o');

        let chars: Vec<_> = term.column_iter(3).take(5).map(|t|t.glyph).collect();
        assert_eq!("Hello", String::from_iter(chars));
    }

    #[test]
    fn column_put() {
        let mut term = Terminal::new(15, 10);
        let text = "Hello".chars();
        for (mut t, c) in term.column_iter_mut(5).take(5).zip(text) {
            t.glyph = c;
        }

        assert_eq!('H', term.get_char(5,0));
        assert_eq!('e', term.get_char(5,1));
        assert_eq!('l', term.get_char(5,2));
        assert_eq!('l', term.get_char(5,3));
        assert_eq!('o', term.get_char(5,4));
    }
}
