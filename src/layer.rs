use bevy::{
    color::Color, ecs::component::Component, math::IVec2, reflect::Reflect, utils::HashMap,
};

use crate::{GridPoint, PivotedPoint, Tile};

/// A grid of tiles stored in a hashmap. Tiles do not exist until inserted
#[derive(Component, Reflect)]
pub struct TerminalLayer {
    tiles: HashMap<(i32, i32), Tile>,
    size: IVec2,
    clear_tile: Tile,
}

impl TerminalLayer {
    /// Attempt to retrieve a tile at the given grid position. Will return [None]
    /// for an empty tile.
    pub fn get_tile_mut(&mut self, xy: impl Into<PivotedPoint>) -> Option<&mut Tile> {
        let xy = xy.into().calc_from_size(self.size);
        let [x, y] = xy.as_array();
        self.tiles.get_mut(&(x, y))
    }

    pub fn insert(&mut self, xy: impl Into<PivotedPoint>, tile: Tile) {
        let xy = xy.into().calc_from_size(self.size);
        self.size = self.size.max(xy + 1);
        let [x, y] = xy.as_array();
        self.tiles.insert((x, y), tile);
    }

    /// Retrieve a tile at the given grid position.
    ///
    /// For a sparse layer if no tile exists then a tile will be created at
    /// the grid position using the layer's clear tile, and that will be returned.
    pub fn tile_mut(&mut self, xy: impl Into<PivotedPoint>) -> &mut Tile {
        let xy = xy.into().calc_from_size(self.size);
        self.size = self.size.max(xy + 1);
        let [x, y] = xy.as_array();
        self.tiles.entry((x, y)).or_insert(self.clear_tile)
    }

    pub fn put_char(&mut self, xy: impl Into<PivotedPoint>, ch: char) -> &mut Tile {
        self.tile_mut(xy).glyph(ch)
    }

    pub fn put_fg_color(&mut self, xy: impl Into<PivotedPoint>, color: Color) -> &mut Tile {
        self.tile_mut(xy).fg(color)
    }

    pub fn put_bg_color(&mut self, xy: impl Into<PivotedPoint>, color: Color) -> &mut Tile {
        self.tile_mut(xy).bg(color)
    }

    pub fn iter_xy(&self) -> impl Iterator<Item = (IVec2, &Tile)> {
        self.tiles.iter().map(|((x, y), t)| (IVec2::new(*x, *y), t))
    }

    pub fn iter_xy_mut(&mut self) -> impl Iterator<Item = (IVec2, &mut Tile)> {
        self.tiles
            .iter_mut()
            .map(|((x, y), t)| (IVec2::new(*x, *y), t))
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.values()
    }

    pub fn iter_tiles_mut(&mut self) -> impl Iterator<Item = &mut Tile> {
        self.tiles.values_mut()
    }
}
