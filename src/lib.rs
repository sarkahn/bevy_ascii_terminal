use bevy::math::{UVec2};
use array2d::*;
use std::iter::FromIterator;

use bevy::prelude::*;
use bevy_render::prelude::Color;

#[derive(Clone, Copy)]
pub struct Tile {
    glyph: char,
    fg_color: Color,
    bg_color: Color,
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

trait Terminal {
    fn put_char(&mut self, x: i32, y: i32, glyph: char);
    fn put_string(&mut self, x: i32, y: i32, string: &str);
    fn get_char(&self, x: i32, y: i32) -> char;
    fn get_string(&self, x: i32, y: i32, len: usize) -> String;
    fn get_tile(&self, x: i32, y: i32) -> &Tile;
    fn get_tile_mut(&mut self,x: i32, y: i32) -> &mut Tile;
}

pub struct FixedTerminal {
    size: (usize,usize),
    data: Array2D<Tile>,
}

impl FixedTerminal {
    pub fn new(width: usize, height: usize) -> FixedTerminal {
        FixedTerminal {
            size: (width, height),
            data: Array2D::filled_with(Tile::default(), width, height),
        }
    }
}

impl Terminal for FixedTerminal {
    fn put_char(&mut self, x: i32, y: i32, glyph: char) {
        self.get_tile_mut(x,y).glyph = glyph;
    }

    fn put_string(&mut self, x: i32, y: i32, string: &str) {
        let len = string.len();
        let chars = string.chars();

        let mut i = 0_usize;
        let mut dy = y as usize;
        let mut dx =  x as usize + i;

        for ch in chars {
            if dx >= self.size.0 {
                dy += 1;
                if dy >= self.size.1 {
                    return;
                }
                dx = dx % self.size.0;
            }

            let mut t = self.data.get_mut(dx, dy).unwrap();
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
        debug_assert!((x as usize) < self.size.0 &&
                      (y as usize) < self.size.1, "Trying to get string out of bounds");

        let mut y = y as usize;
        let mut chars: Vec<char> = vec![' '; len];
        for i in 0..len {
            let mut dx = i + x as usize;
            if dx >= self.size.0 {
                y += 1;
                if y >= self.size.1 {
                    return String::from_iter(chars);
                }
                dx = dx % self.size.0;
            }
            chars[i] = self.get_char(dx as i32,y as i32);
        }

        String::from_iter(chars)
    }

    fn get_tile(&self, x: i32, y: i32) -> &Tile {
        self.data.get(x as usize, y as usize).unwrap()
    }

    fn get_tile_mut(&mut self, x: i32, y: i32) -> &mut Tile {
        self.data.get_mut(x as usize,y as usize).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_char() {
        let mut term = FixedTerminal::new(20,20);

        term.put_char(5,5, 'h');

        assert_eq!('h', term.get_char(5,5));
    }

    #[test]
    fn put_string() {
        let mut term = FixedTerminal::new(20, 20);
        term.put_string(0,0, "Hello");
        assert_eq!("Hello", term.get_string(0,0,5));
    }
}