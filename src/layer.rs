use std::{iter::Enumerate, slice};

use bevy::{
    math::IVec2,
    reflect::Reflect,
    render::color::Color,
    utils::{hashbrown::hash_map, HashMap},
};

use crate::{GridPoint, PivotedPoint, Tile};

pub struct TerminalLayer {
    tiles: TilesLayer,
    size: IVec2,
    clear_tile: Tile,
}

#[derive(Reflect)]
pub enum TilesLayer {
    Dense(Vec<Tile>),
    Sparse(HashMap<(i32, i32), Tile>),
}

impl TerminalLayer {
    /// Attempt to retrieve a tile at the given grid position. For a sparse
    /// layer this will return [None] for an empty tile. For a dense terminal
    /// this will return [None] from an out of bounds position.
    pub fn get_tile_mut(&mut self, xy: impl Into<PivotedPoint>) -> Option<&mut Tile> {
        let xy = xy.into().calc_from_size(self.size);
        if xy.cmplt(IVec2::ZERO).any() || xy.cmpge(self.size).any() {
            return None;
        }
        match &mut self.tiles {
            TilesLayer::Dense(tiles) => {
                let i = xy.as_index(self.size.x as usize);
                Some(&mut tiles[i])
            }
            TilesLayer::Sparse(tiles) => {
                let [x, y] = xy.as_array();
                tiles.get_mut(&(x, y))
            }
        }
    }

    pub fn insert(&mut self, xy: impl Into<PivotedPoint>, tile: Tile) {
        let xy = xy.into().calc_from_size(self.size);
        match &mut self.tiles {
            TilesLayer::Dense(tiles) => {
                let i = xy.as_index(self.size.x as usize);
                tiles[i] = tile;
            }
            TilesLayer::Sparse(tiles) => {
                let [x, y] = xy.as_array();
                tiles.insert((x, y), tile);
            }
        }
    }

    /// Retrieve a tile at the given grid position.
    ///
    /// For a sparse layer if no tile exists then a tile will be created at
    /// the grid position using the layer's clear tile and that will be returned.
    pub fn tile_mut(&mut self, xy: impl Into<PivotedPoint>) -> &mut Tile {
        let xy = xy.into().calc_from_size(self.size);
        match &mut self.tiles {
            TilesLayer::Dense(tiles) => {
                let i = xy.as_index(self.size.x as usize);
                &mut tiles[i]
            }
            TilesLayer::Sparse(tiles) => {
                let [x, y] = xy.as_array();
                tiles.entry((x, y)).or_insert(self.clear_tile)
            }
        }
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
}

/// Transform a 1d index to it's corresponding 2d grid position.
fn index_to_xy(i: usize, width: usize) -> IVec2 {
    let x = (i % width) as i32;
    let y = (i / width) as i32;
    IVec2::new(x, y)
}

impl TilesLayer {
    pub fn iter_row_mut(&mut self, row: usize) -> impl Iterator<Item = &mut Tile> {}
}

pub enum TilesIterMut<'a> {
    Dense(slice::IterMut<'a, Tile>),
    Sparse(hash_map::ValuesMut<'a, (i32, i32), Tile>),
}

impl<'a> Iterator for TilesIterMut<'a> {
    type Item = &'a mut Tile;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            TilesIterMut::Dense(iter) => iter.next(),
            TilesIterMut::Sparse(iter) => iter.next(),
        }
    }
}

pub enum TilesXyIterMut<'a> {
    Dense {
        size: IVec2,
        iter: Enumerate<slice::IterMut<'a, Tile>>,
    },
    Sparse(hash_map::IterMut<'a, (i32, i32), Tile>),
}

impl<'a> Iterator for TilesXyIterMut<'a> {
    type Item = (IVec2, &'a mut Tile);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            TilesXyIterMut::Dense { size, iter } => iter.next().map(|(i, t)| {
                let xy = index_to_xy(i, size.x as usize);
                (xy, t)
            }),
            TilesXyIterMut::Sparse(iter) => iter.next().map(|((x, y), t)| (IVec2::new(*x, *y), t)),
        }
    }
}
