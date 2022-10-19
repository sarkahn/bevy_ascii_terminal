use arrayvec::ArrayVec;
use bevy::prelude::Color;
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
/// use bevy_ascii_terminal::prelude::*;
/// let mut term = Terminal::with_size([10,10]);
///
/// // Insert a an 'a' character with a blue foreground and green background.
/// term.put_char([1,1], 'a'.fg(Color::BLUE).bg(Color::GREEN));
/// ```
pub trait TileFormatter: Clone {
    /// Change the glyph of a tile.
    fn glyph(self, glyph: char) -> FormattedTile;
    /// Change the foreground color of a tile.
    fn fg(self, color: Color) -> FormattedTile;
    /// Change the background color of a tile.
    fn bg(self, color: Color) -> FormattedTile;

    /// Get the [TileFormat] which can be used to apply tile modifications.
    fn format(self) -> FormattedTile;
}

/// Formatting that can be applied to a terminal tile.
///
/// Formatting allows you to create an object that specifies certain aspects
/// to modify without necessarily replacing an entire tile.
#[derive(Debug, Default, Clone)]
pub struct FormattedTile {
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

impl FormattedTile {
    pub fn new() -> FormattedTile {
        FormattedTile::default()
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
    pub fn clear() -> FormattedTile {
        FormattedTile::from(Tile::default())
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

impl TileFormatter for FormattedTile {
    /// Change the forergound color of a tile.
    fn fg(mut self, color: Color) -> FormattedTile {
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
    fn bg(mut self, color: Color) -> FormattedTile {
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
    fn glyph(mut self, ch: char) -> FormattedTile {
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
    fn format(self) -> FormattedTile {
        self
    }
}

impl TileFormatter for char {
    /// Replace the original glyph with the given one.
    fn glyph(self, glyph: char) -> FormattedTile {
        FormattedTile::default().glyph(glyph)
    }

    /// Modify the foreground color of the tile.
    fn fg(self, color: Color) -> FormattedTile {
        FormattedTile::default().glyph(self).fg(color)
    }

    /// Modify the background color of the tile.
    fn bg(self, color: Color) -> FormattedTile {
        FormattedTile::default().glyph(self).bg(color)
    }

    /// Get the [TileFormat] for this character.
    fn format(self) -> FormattedTile {
        FormattedTile::default().glyph(self)
    }
}

impl From<FormattedTile> for Tile {
    fn from(fmt: FormattedTile) -> Self {
        let mut tile = Tile::default();
        fmt.apply(&mut tile);
        tile
    }
}

impl From<Tile> for FormattedTile {
    fn from(tile: Tile) -> Self {
        FormattedTile::default()
            .glyph(tile.glyph)
            .fg(tile.fg_color)
            .bg(tile.bg_color)
    }
}

/// A trait for setting the foreground or background color of a tile in
/// `put_color` functions.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::Color;
/// use bevy_ascii_terminal::prelude::*;
/// let mut term = Terminal::with_size([10,10]);
///
/// // Set the background color for the given tile to blue.
/// term.put_color([3,3], Color::BLUE.bg());
/// ```
pub trait ColorFormatter {
    fn fg(&self) -> ColorFormat;
    fn bg(&self) -> ColorFormat;
}

impl ColorFormatter for Color {
    fn fg(&self) -> ColorFormat {
        ColorFormat::FgColor(*self)
    }

    fn bg(&self) -> ColorFormat {
        ColorFormat::BgColor(*self)
    }
}

/// Used in `put_color` functions for setting the foreground or background color
/// of a tile without affecting the glyph.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorFormat {
    FgColor(Color),
    BgColor(Color),
}
