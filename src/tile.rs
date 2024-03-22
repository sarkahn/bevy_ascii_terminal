use bevy::{reflect::Reflect, render::color::Color};

#[derive(Debug, Default, Clone, Copy, Reflect)]
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

    pub fn glyph(&mut self, glyph: char) -> &mut Self {
        self.glyph = glyph;
        self
    }

    pub fn fg(&mut self, color: Color) -> &mut Self {
        self.fg_color = color;
        self
    }

    pub fn bg(&mut self, color: Color) -> &mut Self {
        self.bg_color = color;
        self
    }
}
