
mod glyph_mapping;
mod render;

use std::slice::Iter;
use std::slice::IterMut;
use std::iter::FromIterator;

use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct Tile {
    glyph: char,
    fg_color: Color,
    bg_color: Color,
}

pub struct TerminalSize {
    size: (usize,usize),
}

impl From<&TerminalSize> for (usize,usize) {
    fn from(val: &TerminalSize) -> Self {
        val.size
    }
}

pub struct Terminal {
    data: Vec<Tile>,
    size: (usize, usize),
}


impl Default for Tile {
    fn default() -> Self {
        Tile {
            glyph: '.',
            fg_color: Color::WHITE,
            bg_color: Color::BLACK,
        }
    }
}

impl Terminal {
    pub fn new(width: usize, height: usize) -> Terminal {
        Terminal {
            data: vec![Tile::default(); width* height],
            size: (width, height),
        }
    }

    fn width(&self) -> usize {
        self.size.0
    }

    fn put_char(&mut self, x: i32, y: i32, glyph: char) {
        self.get_tile_mut(x,y).glyph = glyph;
    }

    fn put_string(&mut self, x: i32, y: i32, string: &str) {
        let chars = string.chars();

        let mut i = 0_usize;
        let mut dy = y as usize;
        let mut dx =  x as usize + i;

        let (width, height) = self.size;
        for ch in chars {
            if dx >= width {
                dy += 1;
                if dy >= height {
                    return;
                }
                dx = dx % width;
            }

            let mut t = self.data.get_mut(dy * width + dx).unwrap();
            t.glyph = ch;

            self.put_char(dx as i32, dy as i32, ch);

            dx += 1;
            i += 1;

        }
    }

    fn get_char(&self, x: i32, y: i32) -> char {
        self.get_tile(x,y).glyph
    }

    fn get_string(&self, x: i32, y: i32, len: usize) -> String {
        let (width,height) = self.size;

        debug_assert!((x as usize) < width &&
                      (y as usize) < height, "Trying to get string out of bounds");

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
            chars[i] = self.get_char(dx as i32,y as i32);
        }

        String::from_iter(chars)
    }

    fn get_tile(&self, x: i32, y: i32) -> &Tile {
        let x = x as usize;
        let y = y as usize;
        self.data.get(y * self.width() + x).unwrap()
    }

    fn get_tile_mut(&mut self, x: i32, y: i32) -> &mut Tile {
        let x = x as usize;
        let y = y as usize;
        let width = self.width();
        self.data.get_mut(y * width + x).unwrap()
    }

    fn iter(&self) -> Iter<Tile> {
        self.data.iter()
    }

    fn iter_mut(&mut self) -> IterMut<Tile> {
        self.data.iter_mut()
    }
}

#[derive(Bundle)]
pub struct TerminalBundle {
    terminal: Terminal,
    size: TerminalSize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_char() {
        let mut term = Terminal::new(20,20);

        term.put_char(5,5, 'h');

        assert_eq!('h', term.get_char(5,5));
    }

    #[test]
    fn put_string() {
        let mut term = Terminal::new(20, 20);
        term.put_string(0,0, "Hello");
        assert_eq!("Hello", term.get_string(0,0,5));
    }
}