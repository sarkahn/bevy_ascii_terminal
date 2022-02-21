//! Various traits and utility types for formatting of the
//! glyphs drawn to the terminal.

use bevy::prelude::*;

use crate::Tile;

/// A trait for writing to the terminal.
pub trait TileWriter: Clone + Copy {
    /// Return the formatted tile from a base type.
    fn formatted(self) -> FormattedTile;
    /// Change the foreground color.
    fn fg(self, color: Color) -> FormattedTile;
    /// Change the background color.
    fn bg(self, color: Color) -> FormattedTile;
    /// Perform a write on the given tile.
    fn write(&self, tile: &mut Tile);
}

impl TileWriter for char {
    fn formatted(self) -> FormattedTile {
        FormattedTile {
            key: Some(self as u16),
            ..Default::default()
        }
    }

    fn fg(self, fg_color: Color) -> FormattedTile {
        FormattedTile {
            key: Some(self as u16),
            fg_color: Some(fg_color),
            ..Default::default()
        }
    }

    fn bg(self, bg_color: Color) -> FormattedTile {
        FormattedTile {
            key: Some(self as u16),
            bg_color: Some(bg_color),
            ..Default::default()
        }
    }

    fn write(&self, tile: &mut Tile) {
        tile.set_glyph(*self);
    }
}

impl TileWriter for FormattedTile {
    fn formatted(self) -> FormattedTile {
        self
    }

    fn fg(mut self, color: Color) -> FormattedTile {
        self.fg_color = Some(color);
        self
    }

    fn bg(mut self, color: Color) -> FormattedTile {
        self.bg_color = Some(color);
        self
    }

    fn write(&self, tile: &mut Tile) {
        self.apply(tile);
    }
}

impl TileWriter for u16 {
    fn formatted(self) -> FormattedTile {
        FormattedTile {
            key: Some(self),
            ..Default::default()
        }
    }

    fn fg(self, color: Color) -> FormattedTile {
        FormattedTile {
            key: Some(self),
            fg_color: Some(color),
            ..Default::default()
        }
    }

    fn bg(self, color: Color) -> FormattedTile {
        FormattedTile {
            key: Some(self),
            bg_color: Some(color),
            ..Default::default()
        }
    }

    fn write(&self, tile: &mut Tile) {
        tile.key = *self;
    }
}

#[derive(Default, Clone, Copy)]
pub struct FormattedTile {
    key: Option<u16>,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
}
impl FormattedTile {
    /// Set the foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg_color = Some(color);
        self
    }

    /// Set the background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Apply the formatting to the given tile.
    ///
    /// Omitted writer operations should not effect existing tile state.
    pub(crate) fn apply(&self, tile: &mut Tile) {
        match self.key {
            Some(key) => tile.key = key,
            None => {}
        }
        match self.fg_color {
            Some(c) => tile.fg_color = c,
            None => {}
        }
        match self.bg_color {
            Some(c) => tile.bg_color = c,
            None => {}
        }
    }
}

impl From<Tile> for FormattedTile {
    fn from(t: Tile) -> Self {
        FormattedTile {
            key: Some(t.key),
            fg_color: Some(t.fg_color),
            bg_color: Some(t.bg_color),
        }
    }
}

impl TileWriter for Tile {
    fn formatted(self) -> FormattedTile {
        self.into()
    }

    fn fg(mut self, color: Color) -> FormattedTile {
        self.fg_color = color;
        self.into()
    }

    fn bg(mut self, color: Color) -> FormattedTile {
        self.bg_color = color;
        self.into()
    }

    #[inline]
    fn write(&self, tile: &mut Tile) {
        *tile = *self
    }
}

#[allow(clippy::len_without_is_empty)]
/// A trait for writing a string of formatted tiles to the terminal.
pub trait TilesWriter<'a> {
    /// Returns the formatted tiles from a base type.
    fn formatted(self) -> FormattedTiles<'a>;
    /// Set the foreground color for the tiles.
    fn fg(self, fg_color: Color) -> FormattedTiles<'a>;
    /// Set the background color for the tiles.
    fn bg(self, bg_color: Color) -> FormattedTiles<'a>;
    /// Apply the formatting to the given set of tiles.
    fn write(&self, tiles: impl Iterator<Item = &'a mut Tile>);
    /// The length of the writer tiles.
    fn len(&self) -> usize;
}

/// A set of formatted tiles for writing to the terminal.
#[derive(Debug, Clone, Default)]
pub struct FormattedTiles<'a> {
    pub string: &'a str,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

impl<'a> FormattedTiles<'a> {
    pub fn new(chars: &'a str) -> Self {
        FormattedTiles {
            string: chars,
            ..Default::default()
        }
    }
}

impl<'a> TilesWriter<'a> for &'a str {
    fn formatted(self) -> FormattedTiles<'a> {
        FormattedTiles {
            string: self,
            ..Default::default()
        }
    }

    fn fg(self, fg_color: Color) -> FormattedTiles<'a> {
        FormattedTiles {
            string: self,
            fg_color: Some(fg_color),
            ..Default::default()
        }
    }

    fn bg(self, bg_color: Color) -> FormattedTiles<'a> {
        FormattedTiles {
            string: self,
            bg_color: Some(bg_color),
            ..Default::default()
        }
    }

    fn write(&self, tiles: impl Iterator<Item = &'a mut Tile>) {
        for (tile, ch) in tiles.zip(self.chars()) {
            tile.set_glyph(ch);
        }
    }

    fn len(&self) -> usize {
        (self as &str).len()
    }
}

impl<'a> TilesWriter<'a> for FormattedTiles<'a> {
    fn formatted(self) -> FormattedTiles<'a> {
        self
    }

    fn fg(mut self, fg_color: Color) -> FormattedTiles<'a> {
        self.fg_color = Some(fg_color);
        self
    }

    fn bg(mut self, bg_color: Color) -> FormattedTiles<'a> {
        self.bg_color = Some(bg_color);
        self
    }

    fn write(&self, tiles: impl Iterator<Item = &'a mut Tile>) {
        for (tile, ch) in tiles.zip(self.string.chars()) {
            tile.set_glyph(ch);
            match self.fg_color {
                Some(col) => tile.fg_color = col,
                None => {}
            }

            match self.bg_color {
                Some(col) => tile.bg_color = col,
                None => {}
            };
        }
    }

    fn len(&self) -> usize {
        self.string.len()
    }
}

/// Set the foreground color for a tile without affecting anything else.
#[derive(Default, Clone, Copy)]
pub struct FGColor(pub Color);

impl TileWriter for FGColor {
    /// Return the formatted tile from the base type.
    fn formatted(self) -> FormattedTile {
        FormattedTile {
            fg_color: Some(self.0),
            ..Default::default()
        }
    }

    /// Change the foreground color.
    fn fg(self, color: Color) -> FormattedTile {
        FormattedTile {
            fg_color: Some(color),
            ..Default::default()
        }
    }

    /// Returns formatting for setting the background color as well.
    fn bg(self, color: Color) -> FormattedTile {
        FormattedTile {
            fg_color: Some(self.0),
            bg_color: Some(color),
            ..Default::default()
        }
    }

    /// Apply the foreground color to a tile.
    ///
    /// Has no effect on the glyph or background color.
    fn write(&self, tile: &mut Tile) {
        tile.fg_color = self.0;
    }
}

/// Set the background color for a tile without affecting anything else.
#[derive(Default, Clone, Copy)]
pub struct BGColor(pub Color);

impl TileWriter for BGColor {
    /// Return the formatted tile from the base type.
    fn formatted(self) -> FormattedTile {
        FormattedTile {
            bg_color: Some(self.0),
            ..Default::default()
        }
    }

    /// Returns formatting for setting the foreground color as well.
    fn fg(self, color: Color) -> FormattedTile {
        FormattedTile {
            bg_color: Some(color),
            ..Default::default()
        }
    }

    /// Change the background color.
    fn bg(self, color: Color) -> FormattedTile {
        FormattedTile {
            bg_color: Some(self.0),
            fg_color: Some(color),
            ..Default::default()
        }
    }

    /// Apply the background color to a tile without affecting anything else.
    fn write(&self, tile: &mut Tile) {
        tile.bg_color = self.0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let a = 'H'.fg(Color::GREEN).bg(Color::YELLOW);
        println!("{}", a.key.unwrap());
    }
}
