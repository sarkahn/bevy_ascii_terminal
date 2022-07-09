use arrayvec::ArrayVec;
use bevy::prelude::*;
use sark_grids::GridPoint;

use crate::{Terminal, Tile};

/// A trait for building a formatted terminal tile.
///
/// You can use the `fg` or `bg` functions to specify
/// a foreground or background color for the tile. If no
/// color is specified then the color in the terminal will
/// be unaffected.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::Color;
/// use bevy_ascii_terminal::*;
/// let mut term = Terminal::with_size([10,10]);
///
/// // Insert a an 'a' character with a blue foreground and green background.
/// term.put_char([1,1], 'a'.fg(Color::BLUE).bg(Color::GREEN));
/// ```
pub trait TileModifier: Clone {
    /// Change the glyph of a tile.
    fn glyph(self, glyph: char) -> TileFormat;
    /// Change the foreground color of a tile.
    fn fg(self, color: Color) -> TileFormat;
    /// Change the background color of a tile.
    fn bg(self, color: Color) -> TileFormat;

    /// Get the [TileFormat] which can be used to apply tile modifications.
    fn format(self) -> TileFormat;
}

/// Formatting that can be applied to a terminal tile.
///
/// Formatting allows you to create an object that specifies certain aspects
/// to modify without necessarily replacing an entire tile.
#[derive(Debug, Default, Clone)]
pub struct TileFormat {
    /// Modifications to be applied to a tile.
    modifications: ArrayVec<TileModification, 3>,
}

/// Modifications that can be applied to a terminal tile.
#[derive(Debug, Clone, Copy)]
pub enum TileModification {
    /// Change the glyph of a tile.
    Glyph(char),
    /// Change the foreground color of a tile.
    FgColor(Color),
    /// Change the background color of a tile.
    BgColor(Color),
}

impl TileFormat {
    pub fn new() -> TileFormat {
        TileFormat::default()
    }

    #[inline]
    /// Apply formatting to an existing tile without necessarily replacing it completely.
    pub fn apply(&self, tile: &mut Tile) {
        for modification in self.modifications.iter() {
            match modification {
                TileModification::Glyph(glyph) => tile.glyph = *glyph,
                TileModification::FgColor(col) => tile.fg_color = *col,
                TileModification::BgColor(col) => tile.bg_color = *col,
            }
        }
    }

    /// Create a [TileFormat] which will clear a tile to default
    /// when applied.
    pub fn clear() -> TileFormat {
        TileFormat::from(Tile::default())
    }

    /// Iterate over tile modifications.
    pub fn iter(&self) -> impl Iterator<Item = &TileModification> {
        self.modifications.iter()
    }

    /// Apply the tile modifications to the terminal at the given position.
    #[inline]
    pub(crate) fn draw(&self, xy: impl GridPoint, term: &mut Terminal) {
        let t = term.get_tile_mut(xy);
        self.apply(t);
    }
}

impl TileModifier for TileFormat {
    /// Change the forergound color of a tile.
    fn fg(mut self, color: Color) -> TileFormat {
        for modifier in self.modifications.iter_mut() {
            if let TileModification::FgColor(col) = modifier {
                *col = color;
                return self;
            }
        }
        self.modifications.push(TileModification::FgColor(color));
        self
    }

    /// Change the background color of a tile.
    fn bg(mut self, color: Color) -> TileFormat {
        for modifier in self.modifications.iter_mut() {
            if let TileModification::BgColor(col) = modifier {
                *col = color;
                return self;
            }
        }
        self.modifications.push(TileModification::BgColor(color));
        self
    }

    /// Change the glyph of a tile.
    fn glyph(mut self, ch: char) -> TileFormat {
        for modifier in self.modifications.iter_mut() {
            if let TileModification::Glyph(glyph) = modifier {
                *glyph = ch;
                return self;
            }
        }
        self.modifications.push(TileModification::Glyph(ch));
        self
    }
    /// Get the [TileFormat] which can be used to apply tile modifications.
    fn format(self) -> TileFormat {
        self
    }
}

impl TileModifier for char {
    /// Replace the original character with a given one.
    ///
    /// This is pointless.
    fn glyph(self, glyph: char) -> TileFormat {
        TileFormat::default().glyph(glyph)
    }

    /// Modify the foreground color of the tile.
    fn fg(self, color: Color) -> TileFormat {
        TileFormat::default().glyph(self).fg(color)
    }

    /// Modify the background color of the tile.
    fn bg(self, color: Color) -> TileFormat {
        TileFormat::default().glyph(self).bg(color)
    }

    /// Get the [TileFormat] for this character.
    fn format(self) -> TileFormat {
        TileFormat::default().glyph(self)
    }
}

impl From<TileFormat> for Tile {
    fn from(fmt: TileFormat) -> Self {
        let mut tile = Tile::default();
        fmt.apply(&mut tile);
        tile
    }
}

impl From<Tile> for TileFormat {
    fn from(tile: Tile) -> Self {
        TileFormat::default()
            .glyph(tile.glyph)
            .fg(tile.fg_color)
            .bg(tile.bg_color)
    }
}
