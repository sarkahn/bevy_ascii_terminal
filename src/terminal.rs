use std::iter::FromIterator;
use std::slice::Iter;
use std::slice::IterMut;

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

    pub fn put_string(&mut self, x: usize, y: usize, string: &str) {
        let tiles = self.mut_slice(x, y, string.len());
        let chars = string.chars().take(tiles.len());

        for (i, char) in chars.enumerate() {
            tiles[i].glyph = char;
        }
    }

    pub fn get_char(&self, x: usize, y: usize) -> char {
        self.get_tile(x, y).glyph
    }

    pub fn get_string(&self, x: usize, y: usize, len: usize) -> String {
        let slice = self.slice(x, y, len);
        let mut chars: Vec<char> = vec![' '; slice.len()];

        for (i, t) in slice.iter().enumerate() {
            chars[i] = t.glyph;
        }

        String::from_iter(chars)
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        let x = x as usize;
        let y = y as usize;
        self.tiles.get(y * self.width() + x).unwrap()
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        let x = x as usize;
        let y = y as usize;
        let width = self.width();
        self.tiles.get_mut(y * width + x).unwrap()
    }

    pub fn clear_box(&mut self, x: usize, y: usize, width: usize, height: usize) {
        for x in x..x + width {
            for y in y..y + height {
                self.put_tile(x, y, Tile::default());
            }
        }
    }

    pub fn draw_box_single(&mut self, x: usize, y: usize, width: usize, height: usize) {
        let width = width;
        let height = height;

        let left = x;
        let right = x + width - 1;
        let top = y;
        let bottom = y + height - 1;

        for y in top + 1..bottom {
            self.put_char(left, y, '│');
            self.put_char(right, y, '│');
        }

        for x in left + 1..right {
            self.put_char(x, top, '─');
            self.put_char(x, bottom, '─');
        }

        self.put_char(left, bottom, '└');
        self.put_char(left, top, '┌');
        self.put_char(right, top, '┐');
        self.put_char(right, bottom, '┘');
    }

    pub fn draw_border_single(&mut self) {
        self.draw_box_single(0, 0, self.width(), self.height());
    }

    pub fn clear(&mut self) {
        for tile in self.tiles.iter_mut() {
            *tile = Tile::default();
        }
    }

    pub fn iter(&self) -> Iter<Tile> {
        self.tiles.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Tile> {
        self.tiles.iter_mut()
    }

    pub fn slice(&self, x: usize, y: usize, len: usize) -> &[Tile] {
        let i = y * self.width() + x;
        let end = usize::min(i + len, self.tiles.len());

        &self.tiles[i..end]
    }

    pub fn mut_slice(&mut self, x: usize, y: usize, len: usize) -> &mut [Tile] {
        let i = y * self.width() + x;
        let end = usize::min(i + len, self.tiles.len());

        &mut self.tiles[i..end]
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
}
