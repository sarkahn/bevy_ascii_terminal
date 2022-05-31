use arrayvec::{ArrayVec, IntoIter};
use bevy::prelude::*;
use crate::Tile;

#[derive(Debug, Clone, Copy)]
pub enum TileWrite {
    Glyph(char),
    FGColor(Color),
    BGColor(Color),
}

// pub trait TerminalWriter: Clone + Copy {
//     fn glyph(self, glyph: char) -> FormattedTile;
//     /// Change the foreground color.
//     fn fg(self, color: Color) -> FormattedTile;
//     /// Change the background color.
//     fn bg(self, color: Color) -> FormattedTile;
// }

/// A trait for building a formatted terminal tile.
pub trait TileWriter: Clone {
    fn glyph(self, glyph: char) -> TileWrites;
    /// Change the foreground color.
    fn fg(self, color: Color) -> TileWrites;
    /// Change the background color.
    fn bg(self, color: Color) -> TileWrites;

    fn iter(self) -> IntoIter<TileWrite, 3>;
}

// impl TileWriter for char {
//     fn formatted(self) -> FormattedTile {
//         FormattedTile {
//             glyph: Some(self),
//             ..Default::default()
//         }
//     }

//     fn fg(self, fg_color: Color) -> FormattedTile {
//         FormattedTile {
//             glyph: Some(self),
//             fg_color: Some(fg_color),
//             ..Default::default()
//         }
//     }

//     fn bg(self, bg_color: Color) -> FormattedTile {
//         FormattedTile {
//             glyph: Some(self),
//             bg_color: Some(bg_color),
//             ..Default::default()
//         }
//     }

//     fn write(&self, tile: &mut Tile) {
//         tile.glyph = *self;
//     }
// }

// impl TileWriter for FormattedTile {
//     fn formatted(self) -> FormattedTile {
//         self
//     }

//     fn fg(mut self, color: Color) -> FormattedTile {
//         self.fg_color = Some(color);
//         self
//     }

//     fn bg(mut self, color: Color) -> FormattedTile {
//         self.bg_color = Some(color);
//         self
//     }

//     fn write(&self, tile: &mut Tile) {
//         self.apply(tile);
//     }
// }

#[derive(Default, Clone)]
pub struct TileWrites {
    writes: ArrayVec<TileWrite, 3>,
}

impl TileWriter for TileWrites {
    fn fg(mut self, color: Color) -> TileWrites {
        self.writes.push(TileWrite::FGColor(color));
        self
    }

    fn bg(mut self, color: Color) -> TileWrites {
        self.writes.push(TileWrite::BGColor(color));
        self
    }

    fn glyph(mut self, glyph: char) -> TileWrites {
        self.writes.push(TileWrite::Glyph(glyph));
        self
    }

    fn iter(self) -> IntoIter<TileWrite, 3> {
        self.writes.into_iter()
    }
}

impl TileWriter for char {
    fn glyph(self, glyph: char) -> TileWrites {
        let mut fmt = TileWrites::default();
        fmt.writes.push(TileWrite::Glyph(glyph));
        fmt
    }

    fn fg(self, color: Color) -> TileWrites {
        let mut fmt = TileWrites::default();
        fmt.writes.push(TileWrite::Glyph(self));
        fmt.writes.push(TileWrite::FGColor(color));
        fmt
    }

    fn bg(self, color: Color) -> TileWrites {
        let mut fmt = TileWrites::default();
        fmt.writes.push(TileWrite::Glyph(self));
        fmt.writes.push(TileWrite::BGColor(color));
        fmt
    }

    fn iter(self) -> IntoIter<TileWrite, 3> {
        let mut fmt = TileWrites::default();
        fmt.writes.push(TileWrite::Glyph(self));
        fmt.writes.into_iter()
    }
}

// impl FormattedTile {
//     /// Set the foreground color
//     pub fn fg(mut self, color: Color) -> Self {
//         self.fg_color = Some(color);
//         self
//     }

//     /// Set the background color
//     pub fn bg(mut self, color: Color) -> Self {
//         self.bg_color = Some(color);
//         self
//     }

//     /// Apply the formatting to the given tile.
//     ///
//     /// Omitted writer operations should not effect existing tile state.
//     pub(crate) fn apply(&self, tile: &mut Tile) {
//         match self.glyph {
//             Some(key) => tile.glyph = key,
//             None => {}
//         }
//         match self.fg_color {
//             Some(c) => tile.fg_color = c,
//             None => {}
//         }
//         match self.bg_color {
//             Some(c) => tile.bg_color = c,
//             None => {}
//         }
//     }
// }

// impl From<Tile> for FormattedTile {
//     fn from(t: Tile) -> Self {
//         FormattedTile {
//             glyph: Some(t.glyph),
//             fg_color: Some(t.fg_color),
//             bg_color: Some(t.bg_color),
//         }
//     }
// }

// impl From<FormattedTile> for Tile {
//     fn from(fmt: FormattedTile) -> Self {
//         let mut tile = Tile::default();

//         fmt.apply(&mut tile);

//         tile
//     }
// }

// impl TileWriter for Tile {
//     fn formatted(self) -> FormattedTile {
//         self.into()
//     }

//     fn fg(mut self, color: Color) -> FormattedTile {
//         self.fg_color = color;
//         self.into()
//     }

//     fn bg(mut self, color: Color) -> FormattedTile {
//         self.bg_color = color;
//         self.into()
//     }

//     #[inline]
//     fn write(&self, tile: &mut Tile) {
//         *tile = *self
//     }
// }