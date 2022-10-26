use bevy::{prelude::{Component, UVec2, Vec2, IVec2}, math::uvec2};
use sark_grids::{Pivot, geometry::GridRect};

use crate::{Border, Terminal};

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
    pub(crate) border: Option<Border>,
    pub(crate) pixels_per_tile: UVec2,
    pub(crate) tile_size: Vec2,
    pub(crate) pos: IVec2,
    pub(crate) bounds: GridRect,
}

impl Default for TerminalLayout {
    fn default() -> Self {
        Self {
            tile_size: Vec2::ONE,
            scaling: TileScaling::World,
            pixels_per_tile: uvec2(8, 8),
            pivot: Pivot::Center,
            //border_entity: None,
            border: None,
            pos: Default::default(),
            bounds: GridRect::new([0,0], [1,1]),
        }
    }
}

impl TerminalLayout {
    /// Returns the bottom left point of the terminal in world space.
    pub fn origin(&self) -> Vec2 {
        let pivot = Vec2::splat(0.5);
        -(self.bounds.size().as_vec2() * self.tile_size * pivot)
    }

    pub fn term_size(&self) -> UVec2 {
        self.bounds.size().as_uvec2()
    }

    pub fn pixels_per_tile(&self) -> UVec2 {
        self.pixels_per_tile
    }

    pub(crate) fn update_state(&mut self, term: &Terminal, pos: IVec2) {
        self.border = term.border().cloned();
        let mut bounds = term.bounds_without_border();
        bounds.center += pos;
        self.bounds = bounds;
        self.pos = pos;
    }

    pub fn width(&self) -> usize {
        self.bounds.size().x as usize
    }

    pub fn height(&self) -> usize {
        self.bounds.size().y as usize
    }

    pub fn bounds_without_border(&self) -> GridRect {
        self.bounds
    }

    pub fn bounds_with_border(&self) -> GridRect {
        if self.border.is_some() {
            self.bounds.resized([1,1])
        } else {
            self.bounds
        }
    }
}

impl From<&Terminal> for TerminalLayout {
    fn from(t: &Terminal) -> Self {
        Self {
            border: t.border().cloned(),
            bounds: t.bounds_with_border(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{TerminalLayout, Terminal};

    #[test]
    fn bounds() {
        let term = Terminal::new([10,10]);
        let layout = TerminalLayout::from(&term);
        println!("{:?}", layout.bounds);
    }
}