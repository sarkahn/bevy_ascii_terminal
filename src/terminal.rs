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
    pub size: (usize, usize),
}

impl From<&TerminalSize> for (usize, usize) {
    fn from(val: &TerminalSize) -> Self {
        val.size
    }
}

#[derive(Default)]
pub struct Terminal {
    pub tiles: Vec<Tile>,
    pub size: (usize, usize),
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
            size: (width, height),
        }
    }

    pub fn width(&self) -> usize {
        self.size.0
    }
    pub fn height(&self) -> usize {
        self.size.1
    }

    pub fn put_char(&mut self, x: i32, y: i32, glyph: char) {
        self.get_tile_mut(x, y).glyph = glyph;
    }

    pub fn put_char_color(
        &mut self,
        x: i32,
        y: i32,
        glyph: char,
        fg_color: Color,
        bg_color: Color,
    ) {
        let t = self.get_tile_mut(x, y);
        t.glyph = glyph;
        t.fg_color = fg_color;
        t.bg_color = bg_color;
    }

    pub fn put_tile(&mut self, x: i32, y: i32, tile: Tile) {
        let t = self.get_tile_mut(x, y);
        *t = tile;
    }

    pub fn put_string(&mut self, x: i32, y: i32, string: &str) {
        let chars = string.chars();

        let mut dy = y as usize;
        let mut dx = x as usize;

        let (width, height) = self.size;

        for ch in chars {
            if dx >= width {
                dy += 1;
                if dy >= height {
                    return;
                }
                dx %= width;
            }

            self.put_char(dx as i32, dy as i32, ch);

            dx += 1;
        }
    }

    pub fn put_string_color(&mut self, x: i32, y: i32, string: &str, fg: Color, bg: Color) {
        let chars = string.chars();

        let mut dy = y as usize;
        let mut dx = x as usize;

        let (width, height) = self.size;

        for ch in chars {
            if dx >= width {
                dy += 1;
                if dy >= height {
                    return;
                }
                dx %= width;
            }

            self.put_char_color(dx as i32, dy as i32, ch, fg, bg);

            dx += 1;
        }
    }

    pub fn get_char(&self, x: i32, y: i32) -> char {
        self.get_tile(x, y).glyph
    }

    pub fn get_string(&self, x: i32, y: i32, len: usize) -> String {
        let (width, height) = self.size;

        debug_assert!(
            (x as usize) < width && (y as usize) < height,
            "Trying to get string out of bounds"
        );

        let mut y = y as usize;
        let mut chars: Vec<char> = vec![' '; len];
        for i in 0..len {
            let mut dx = i + x as usize;
            if dx >= width {
                y += 1;
                if y >= height {
                    return String::from_iter(chars);
                }
                dx = dx % width;
            }
            chars[i] = self.get_char(dx as i32, y as i32);
        }

        String::from_iter(chars)
    }

    pub fn get_tile(&self, x: i32, y: i32) -> &Tile {
        let x = x as usize;
        let y = y as usize;
        self.tiles.get(y * self.width() + x).unwrap()
    }

    pub fn get_tile_mut(&mut self, x: i32, y: i32) -> &mut Tile {
        let x = x as usize;
        let y = y as usize;
        let width = self.width();
        debug_assert!(
            x < self.width(),
            "get_tile_mut(x = {}) out of bounds. Width {}",
            x,
            self.width()
        );
        debug_assert!(
            y < self.height(),
            "get_tile_mut(y = {}) out of bounds: {}",
            y,
            self.height()
        );
        let i = y * width + x;
        debug_assert!(
            i < self.tiles.len(),
            "get_tile_mut({},{}) resulting index {} is out of bounds of len {}",
            x,
            y,
            i,
            self.tiles.len()
        );

        self.tiles.get_mut(i).unwrap()
    }

    pub fn clear_box(&mut self, x: i32, y: i32, width: usize, height: usize) {
        for x in x..x + width as i32 {
            for y in y..y + height as i32 {
                self.put_tile(x, y, Tile::default());
            }
        }
    }

    pub fn draw_box_single(&mut self, x: i32, y: i32, width: usize, height: usize) {
        let width = width as i32;
        let height = height as i32;

        let left = x;
        let right = x + width - 1;
        let bottom = y;
        let top = y + height - 1;

        for y in bottom + 1..top {
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
