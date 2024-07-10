use bevy::{
    color::{Color, Gray, LinearRgba},
    reflect::Reflect,
};

/// A single tile of a [crate::Terminal].
#[derive(Debug, PartialEq, Clone, Copy, Reflect)]
pub struct Tile {
    pub glyph: char,
    pub fg_color: LinearRgba,
    pub bg_color: LinearRgba,
}

impl Tile {
    pub const DEFAULT: Tile = Tile {
        glyph: ' ',
        fg_color: LinearRgba::WHITE,
        bg_color: LinearRgba::BLACK,
    };

    /// Set the glyph for this tile.
    pub fn glyph(&mut self, glyph: char) -> &mut Self {
        self.glyph = glyph;
        self
    }

    /// Set the foreground color for this tile.
    pub fn fg(&mut self, color: impl Into<LinearRgba>) -> &mut Self {
        self.fg_color = color.into();
        self
    }

    /// Set the background color for this tile.
    pub fn bg(&mut self, color: impl Into<LinearRgba>) -> &mut Self {
        self.bg_color = color.into();
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

/// A set of properties to apply to a Terminal tile, used in the
/// [crate::Terminal::format_tile] function.
#[derive(Debug, Default, Clone, Copy)]
pub struct FormattedTile {
    pub glyph: Option<char>,
    pub fg_color: Option<LinearRgba>,
    pub bg_color: Option<LinearRgba>,
}

/// A trait for building a [FormattedTile] from a single tile property.
pub trait TileFormatter {
    fn glyph(self, glyph: char) -> FormattedTile;
    fn fg_color(self, color: impl Into<LinearRgba>) -> FormattedTile;
    fn bg_color(self, color: impl Into<LinearRgba>) -> FormattedTile;
    fn fmt(self) -> FormattedTile;
}

impl TileFormatter for char {
    fn glyph(self, glyph: char) -> FormattedTile {
        FormattedTile {
            glyph: Some(glyph),
            ..Default::default()
        }
    }

    fn fg_color(self, color: impl Into<LinearRgba>) -> FormattedTile {
        FormattedTile {
            glyph: Some(self),
            fg_color: Some(color.into()),
            ..Default::default()
        }
    }

    fn bg_color(self, color: impl Into<LinearRgba>) -> FormattedTile {
        FormattedTile {
            glyph: Some(self),
            bg_color: Some(color.into()),
            ..Default::default()
        }
    }

    fn fmt(self) -> FormattedTile {
        FormattedTile {
            glyph: Some(self),
            ..Default::default()
        }
    }
}

impl TileFormatter for FormattedTile {
    fn glyph(mut self, glyph: char) -> FormattedTile {
        self.glyph = Some(glyph);
        self
    }

    fn fg_color(mut self, color: impl Into<LinearRgba>) -> FormattedTile {
        self.fg_color = Some(color.into());
        self
    }

    fn bg_color(mut self, color: impl Into<LinearRgba>) -> FormattedTile {
        self.bg_color = Some(color.into());
        self
    }

    fn fmt(self) -> FormattedTile {
        self
    }
}

impl From<Tile> for FormattedTile {
    fn from(value: Tile) -> Self {
        FormattedTile {
            glyph: Some(value.glyph),
            fg_color: Some(value.fg_color),
            bg_color: Some(value.bg_color),
        }
    }
}

/// A trait for generating a [FormattedTile] from a [Color], used by the
/// [crate::Terminal::format_tile] function.
pub trait ColorWriter {
    fn fg(self) -> FormattedTile;
    fn bg(self) -> FormattedTile;
}

impl ColorWriter for Color {
    /// Create a [FormattedTile] from a foreground color.
    fn fg(self) -> FormattedTile {
        FormattedTile {
            fg_color: Some(self.into()),
            ..Default::default()
        }
    }

    /// Create a [FormattedTile] from a background color.
    fn bg(self) -> FormattedTile {
        FormattedTile {
            bg_color: Some(self.into()),
            ..Default::default()
        }
    }
}
