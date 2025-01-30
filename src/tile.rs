use bevy::{
    color::{Color, LinearRgba},
    reflect::Reflect,
};

#[derive(Debug, Clone, Reflect, Copy, PartialEq)]
pub struct Tile {
    pub glyph: char,
    pub fg_color: LinearRgba,
    pub bg_color: LinearRgba,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            glyph: ' ',
            fg_color: Color::WHITE.into(),
            bg_color: Color::BLACK.into(),
        }
    }
}

impl Tile {
    pub const fn new(glyph: char, fg_color: LinearRgba, bg_color: LinearRgba) -> Self {
        Self {
            glyph,
            fg_color,
            bg_color,
        }
    }

    pub fn fg(&mut self, color: impl Into<LinearRgba>) -> &mut Self {
        self.fg_color = color.into();
        self
    }

    pub fn bg(&mut self, color: impl Into<LinearRgba>) -> &mut Self {
        self.bg_color = color.into();
        self
    }

    pub fn char(&mut self, glyph: char) -> &mut Self {
        self.glyph = glyph;
        self
    }

    pub fn with_fg(mut self, color: impl Into<LinearRgba>) -> Self {
        self.fg(color);
        self
    }

    pub fn with_bg(mut self, color: impl Into<LinearRgba>) -> Self {
        self.bg(color);
        self
    }

    pub fn with_char(mut self, glyph: char) -> Self {
        self.char(glyph);
        self
    }
}
