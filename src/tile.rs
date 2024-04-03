use bevy::{reflect::Reflect, render::color::Color};

#[derive(Debug, PartialEq, Clone, Copy, Reflect)]
pub struct Tile {
    pub glyph: char,
    pub fg_color: Color,
    pub bg_color: Color,
}

impl Tile {
    pub const DEFAULT: Tile = Tile {
        glyph: ' ',
        fg_color: Color::WHITE,
        bg_color: Color::BLACK,
    };

    /// Set the glyph for this tile.
    pub fn glyph(&mut self, glyph: char) -> &mut Self {
        self.glyph = glyph;
        self
    }

    /// Set the foreground color for this tile.
    pub fn fg(&mut self, color: Color) -> &mut Self {
        self.fg_color = color;
        self
    }

    /// Set the background color for this tile.
    pub fn bg(&mut self, color: Color) -> &mut Self {
        self.bg_color = color;
        self
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::DEFAULT
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Tile {
            glyph: value,
            ..Default::default()
        }
    }
}
