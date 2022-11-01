use bevy::{
    math::uvec2,
    prelude::{Component, IVec2, UVec2, Vec2},
};
use sark_grids::{geometry::GridRect, Pivot};

use crate::{Border, Terminal, Tile};

use super::TileScaling;

/// Layout settings for the terminal renderer.
///
/// You can modify the `scaling`, `term_pivot`, or `tile_pivot` of the layout
/// to change how the terminal is rendered.
#[derive(Debug, Component, Clone)]
pub struct TerminalLayout {
    pub scaling: TileScaling,
    pub(crate) pivot: Pivot,
    //pub(crate) border_entity: Option<Entity>,
    border: Option<Border>,
    pub(crate) pixels_per_tile: UVec2,
    pub(crate) tile_size: Vec2,
    pub(crate) pos: IVec2,
    bounds: GridRect,
    clear_tile: Tile,
}

impl Default for TerminalLayout {
    fn default() -> Self {
        Self {
            tile_size: Vec2::ONE,
            scaling: TileScaling::World,
            pixels_per_tile: uvec2(8, 8),
            pivot: Pivot::Center,
            border: None,
            pos: Default::default(),
            bounds: GridRect::new([0, 0], [1, 1]),
            clear_tile: Default::default(),
        }
    }
}

impl TerminalLayout {
    /// Returns the bottom left point of the terminal in world space.
    pub fn origin(&self) -> Vec2 {
        let pivot = Vec2::from(self.pivot);
        -(self.bounds.size().as_vec2() * self.tile_size * pivot)
    }

    pub fn term_size(&self) -> UVec2 {
        self.bounds.size().as_uvec2()
    }

    pub fn pixels_per_tile(&self) -> UVec2 {
        self.pixels_per_tile
    }

    pub(crate) fn update_state(&mut self, term: &Terminal, pos: IVec2) {
        if self.border.as_ref() != term.border() {
            self.border = term.border().cloned();
        }
        if self.bounds != term.bounds() {
            self.bounds = term.bounds();
        }
        if self.pos != pos {
            self.pos = pos;
        }
        if self.clear_tile != term.clear_tile {
            self.clear_tile = term.clear_tile;
        }
    }

    pub fn width(&self) -> usize {
        self.bounds.size().x as usize
    }

    pub fn height(&self) -> usize {
        self.bounds.size().y as usize
    }

    pub fn bounds(&self) -> GridRect {
        self.bounds.pivoted(self.pivot).translated(self.pos)
    }

    pub fn bounds_with_border(&self) -> GridRect {
        if self.border.is_some() {
            self.bounds().resized([2, 2])
        } else {
            self.bounds()
        }
    }

    pub fn has_border(&self) -> bool {
        self.border.is_some()
    }

    pub fn clear_tile(&self) -> Tile {
        self.clear_tile
    }

    pub fn border(&self) -> Option<&Border> {
        self.border.as_ref()
    }
}

impl From<&Terminal> for TerminalLayout {
    fn from(t: &Terminal) -> Self {
        Self {
            clear_tile: t.clear_tile,
            border: t.border().cloned(),
            bounds: t.bounds(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Terminal, TerminalLayout};

    #[test]
    fn bounds() {
        let term = Terminal::new([10, 10]);
        let layout = TerminalLayout::from(&term);
        println!("{:?}", layout.bounds);
    }
}
