use arrayvec::{ArrayVec, IntoIter};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum TileWrite {
    Glyph(char),
    FGColor(Color),
    BGColor(Color),
}

/// A trait for building a formatted terminal tile.
pub trait TileWriter: Clone {
    fn glyph(self, glyph: char) -> FormattedTile;
    /// Change the foreground color.
    fn fg(self, color: Color) -> FormattedTile;
    /// Change the background color.
    fn bg(self, color: Color) -> FormattedTile;

    fn iter(self) -> IntoIter<TileWrite, 3>;
}


#[derive(Default, Clone)]
pub struct FormattedTile {
    writes: ArrayVec<TileWrite, 3>,
}

impl TileWriter for FormattedTile {
    fn fg(mut self, color: Color) -> FormattedTile {
        self.writes.push(TileWrite::FGColor(color));
        self
    }

    fn bg(mut self, color: Color) -> FormattedTile {
        self.writes.push(TileWrite::BGColor(color));
        self
    }

    fn glyph(mut self, glyph: char) -> FormattedTile {
        self.writes.push(TileWrite::Glyph(glyph));
        self
    }

    fn iter(self) -> IntoIter<TileWrite, 3> {
        self.writes.into_iter()
    }
}

impl TileWriter for char {
    fn glyph(self, glyph: char) -> FormattedTile {
        let mut fmt = FormattedTile::default();
        fmt.writes.push(TileWrite::Glyph(glyph));
        fmt
    }

    fn fg(self, color: Color) -> FormattedTile {
        let mut fmt = FormattedTile::default();
        fmt.writes.push(TileWrite::Glyph(self));
        fmt.writes.push(TileWrite::FGColor(color));
        fmt
    }

    fn bg(self, color: Color) -> FormattedTile {
        let mut fmt = FormattedTile::default();
        fmt.writes.push(TileWrite::Glyph(self));
        fmt.writes.push(TileWrite::BGColor(color));
        fmt
    }

    fn iter(self) -> IntoIter<TileWrite, 3> {
        let mut fmt = FormattedTile::default();
        fmt.writes.push(TileWrite::Glyph(self));
        fmt.writes.into_iter()
    }
}