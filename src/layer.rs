use bevy::math::IVec2;
use std::collections::BTreeMap;

use crate::{
    grid::{direction::Dir4, IndexRect},
    tile::TileWriter,
    GridPoint, GridRect, PivotedPoint, Tile,
};

use self::iterators::{TilesLayerIterXY, TilesLayerIterXYMut};

/// A layer of tiles within a terminal. Can be either [Dense], or [Sparse].
///
/// [Dense]: TilesLayer::Dense
/// [Sparse]: TilesLayer::Sparse
#[derive(Debug)]
pub struct TerminalLayer {
    tiles: TilesLayer,
    size: IVec2,
    clear_tile: Tile,
    state: LayerState,
    index: i32,
}

impl TerminalLayer {
    pub fn new_dense(size: impl GridPoint, index: i32, clear_tile: Tile) -> Self {
        Self {
            size: size.as_ivec2(),
            tiles: TilesLayer::Dense(vec![Tile::default(); size.len()]),
            clear_tile,
            state: LayerState::Resized,
            index,
        }
    }

    pub fn new_sparse(size: impl GridPoint, index: i32, clear_tile: Tile) -> Self {
        Self {
            size: size.as_ivec2(),
            tiles: TilesLayer::Sparse(BTreeMap::new()),
            clear_tile,
            state: LayerState::Resized,
            index,
        }
    }

    /// Iterate over all tiles in the layer.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Tile> {
        self.tiles.iter()
    }

    /// Iterate over all tiles in the layer.
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Tile> {
        self.tiles.iter_mut()
    }

    /// Iterate over every tile in the layer along with it's 2d position.
    pub fn iter_xy(&self) -> TilesLayerIterXY {
        self.tiles.iter_xy(self.size.x as usize)
    }

    /// Iterate over every tile in the layer along with it's 2d position.
    pub fn iter_xy_mut(&mut self) -> TilesLayerIterXYMut {
        self.tiles.iter_xy_mut(self.size.x as usize)
    }

    /// Iterate over a row of tiles
    pub fn iter_row(&self, row: usize) -> impl DoubleEndedIterator<Item = &Tile> {
        iterators::TilesRowIter::new(row, self)
    }

    /// Iterate over a row of tiles
    pub fn iter_row_mut(&mut self, row: usize) -> impl DoubleEndedIterator<Item = &mut Tile> {
        iterators::TilesRowIterMut::new(row, self)
    }

    /// Iterate over a column of tiles
    pub fn iter_column(&self, column: usize) -> impl DoubleEndedIterator<Item = &Tile> {
        iterators::TilesColumnIter::new(column, self)
    }

    /// Iterate over a column of tiles
    pub fn iter_column_mut(&mut self, column: usize) -> impl DoubleEndedIterator<Item = &mut Tile> {
        iterators::TilesColumnIterMut::new(column, self)
    }

    /// Iterator over a rectangular subsection of tiles
    pub fn iter_rect(&self, rect: GridRect) -> impl Iterator<Item = &Tile> {
        iterators::TilesRectIter::new(rect, self)
    }

    /// Iterator over a rectangular subsection of tiles
    pub fn iter_rect_mut(&mut self, rect: GridRect) -> impl Iterator<Item = &mut Tile> {
        iterators::TilesRectIterMut::new(rect, self)
    }

    /// Iterate over an orthogonal line of tiles.
    pub fn iter_line(
        &self,
        xy: impl GridPoint,
        dir: Dir4,
        len: usize,
    ) -> impl Iterator<Item = &Tile> {
        iterators::TilesLineIter::new(xy.as_ivec2(), dir, len, self)
    }

    /// Iterate over an orthogonal line of tiles.
    pub fn iter_line_mut(
        &mut self,
        xy: impl GridPoint,
        dir: Dir4,
        len: usize,
    ) -> impl Iterator<Item = &mut Tile> {
        iterators::TilesLineIterMut::new(xy.as_ivec2(), dir, len, self)
    }

    pub fn state(&self) -> LayerState {
        self.state
    }

    pub fn reset_state(&mut self) {
        self.state = LayerState::Unchanged;
    }

    fn set_changed(&mut self) {
        self.state = LayerState::Changed;
    }

    fn set_resized(&mut self) {
        self.state = LayerState::Resized;
    }

    /// Retrieve a tile at the given position.
    ///
    /// For a dense layer this will panic of the index is out of bounds.
    /// For a sparse layer this will panic if no tile exists at the position.
    pub fn tile(&self, xy: impl Into<PivotedPoint>) -> &Tile {
        let xy = xy.into().calc_from_size(self.size);

        match &self.tiles {
            TilesLayer::Dense(tiles) => {
                assert!(
                    xy.cmpge(IVec2::ZERO).all() && xy.cmplt(self.size).all(),
                    "tile_mut acessing dense layer index {}, out of layer bounds {}",
                    xy,
                    self.size
                );
                let i = xy.as_index(self.size.width());
                &tiles[i]
            }
            TilesLayer::Sparse(tiles) => tiles.get(&xy.into()).unwrap_or_else(|| {
                panic!("Error retrieving tile from sparse layer, no tile at {}", xy)
            }),
        }
    }

    /// Retrieve a tile from the terminal layer.
    ///
    /// For a dense layer this will panic if the index is out of bounds.
    /// For a sparse layer this will create a clear tile for an empty index.
    pub fn tile_mut(&mut self, xy: impl Into<PivotedPoint>) -> &mut Tile {
        let xy = xy.into().calc_from_size(self.size);

        match &self.tiles {
            TilesLayer::Dense(_) => self.set_changed(),
            TilesLayer::Sparse(tiles) => {
                if tiles.contains_key(&xy.into()) {
                    self.set_changed()
                } else {
                    self.set_resized()
                }
            }
        };

        match &mut self.tiles {
            TilesLayer::Dense(tiles) => {
                assert!(
                    xy.cmpge(IVec2::ZERO).all() && xy.cmplt(self.size).all(),
                    "tile_mut acessing position {}, out of layer bounds {}",
                    xy,
                    self.size
                );
                let i = xy.as_index(self.size.width());
                &mut tiles[i]
            }
            TilesLayer::Sparse(tiles) => tiles.entry(xy.into()).or_insert(self.clear_tile),
        }
    }

    /// Retrieve a tile.
    ///
    /// Will return [None] if there is no tile at the given position.
    pub fn get_tile(&self, xy: impl Into<PivotedPoint>) -> Option<&Tile> {
        let xy = xy.into().calc_from_size(self.size);

        match &self.tiles {
            TilesLayer::Dense(tiles) => {
                let i = xy.as_index(self.size.width());
                tiles.get(i)
            }
            TilesLayer::Sparse(tiles) => tiles.get(&xy.into()),
        }
    }

    /// Retrieve a mutable tile.
    ///
    /// Will return [None] if there is no tile at the given position.
    pub fn get_tile_mut(&mut self, xy: impl Into<PivotedPoint>) -> Option<&mut Tile> {
        let xy = xy.into().calc_from_size(self.size);

        match &mut self.tiles {
            TilesLayer::Dense(tiles) => {
                let i = xy.as_index(self.size.width());
                tiles.get_mut(i)
            }
            TilesLayer::Sparse(tiles) => tiles.get_mut(&xy.into()),
        }
    }

    pub fn index(&self) -> i32 {
        self.index
    }
}

impl TileWriter for TerminalLayer {
    fn tile_mut(&mut self, xy: impl Into<PivotedPoint>) -> &mut Tile {
        self.tile_mut(xy)
    }

    fn clear_tile(&self) -> Tile {
        self.clear_tile
    }
}

impl IndexRect for TerminalLayer {
    fn size(&self) -> IVec2 {
        self.size
    }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub enum LayerState {
    Changed,
    Resized,
    #[default]
    Unchanged,
}

#[derive(Debug)]
pub enum TilesLayer {
    /// Tiles are stored in a vec. The layer has a constant size that never
    /// changes, and all tiles are created immediately along with the layer.
    Dense(Vec<Tile>),
    /// Tiles are stored in a BTreeMap. The layer begins empty and tiles must be
    /// added manually.
    Sparse(BTreeMap<LayerPosition, Tile>),
}

impl TilesLayer {
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Tile> {
        match self {
            TilesLayer::Dense(tiles) => iterators::TilesLayerIter::Dense(tiles.iter()),
            TilesLayer::Sparse(tiles) => iterators::TilesLayerIter::Sparse(tiles.values()),
        }
    }

    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Tile> {
        match self {
            TilesLayer::Dense(tiles) => iterators::TilesLayerIterMut::Dense(tiles.iter_mut()),
            TilesLayer::Sparse(tiles) => iterators::TilesLayerIterMut::Sparse(tiles.values_mut()),
        }
    }

    fn iter_xy(&self, width: usize) -> TilesLayerIterXY {
        match self {
            TilesLayer::Dense(tiles) => iterators::TilesLayerIterXY::Dense {
                width,
                iter: tiles.iter().enumerate(),
            },
            TilesLayer::Sparse(tiles) => iterators::TilesLayerIterXY::Sparse(tiles.iter()),
        }
    }

    fn iter_xy_mut(&mut self, width: usize) -> TilesLayerIterXYMut {
        match self {
            TilesLayer::Dense(tiles) => iterators::TilesLayerIterXYMut::Dense {
                width,
                iter: tiles.iter_mut().enumerate(),
            },
            TilesLayer::Sparse(tiles) => iterators::TilesLayerIterXYMut::Sparse(tiles.iter_mut()),
        }
    }
}

/// Wrapper to define proper ordering for tiles in a sparse tile layer. (y ascending then x ascending)
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct LayerPosition(IVec2);

impl From<IVec2> for LayerPosition {
    fn from(value: IVec2) -> Self {
        Self(value)
    }
}

impl From<LayerPosition> for IVec2 {
    fn from(value: LayerPosition) -> Self {
        value.0
    }
}

impl LayerPosition {
    pub fn new(xy: impl GridPoint) -> Self {
        Self(xy.as_ivec2())
    }
}

// Order by y first
impl Ord for LayerPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0.y, self.0.x).cmp(&(other.0.y, other.0.x))
    }
}

impl PartialOrd for LayerPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

mod iterators {
    use std::{
        collections::btree_map::{self, Values, ValuesMut},
        iter::{Enumerate, StepBy},
        ops::Sub,
        slice::{Iter, IterMut},
    };

    use bevy::math::IVec2;

    use crate::{
        grid::{direction::Dir4, IndexRect},
        layer::{LayerPosition, TerminalLayer, TilesLayer},
        GridPoint, GridRect, Tile,
    };

    pub enum TilesLayerIter<'a> {
        Dense(Iter<'a, Tile>),
        Sparse(Values<'a, LayerPosition, Tile>),
    }

    impl<'a> Iterator for TilesLayerIter<'a> {
        type Item = &'a Tile;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                TilesLayerIter::Dense(iter) => iter.next(),
                TilesLayerIter::Sparse(iter) => iter.next(),
            }
        }
    }

    impl<'a> DoubleEndedIterator for TilesLayerIter<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                TilesLayerIter::Dense(iter) => iter.next_back(),
                TilesLayerIter::Sparse(iter) => iter.next_back(),
            }
        }
    }

    pub enum TilesLayerIterMut<'a> {
        Dense(IterMut<'a, Tile>),
        Sparse(ValuesMut<'a, LayerPosition, Tile>),
    }

    impl<'a> Iterator for TilesLayerIterMut<'a> {
        type Item = &'a mut Tile;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                TilesLayerIterMut::Dense(iter) => iter.next(),
                TilesLayerIterMut::Sparse(iter) => iter.next(),
            }
        }
    }

    impl<'a> DoubleEndedIterator for TilesLayerIterMut<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                TilesLayerIterMut::Dense(iter) => iter.next_back(),
                TilesLayerIterMut::Sparse(iter) => iter.next_back(),
            }
        }
    }

    pub enum TilesLayerIterXY<'a> {
        Dense {
            width: usize,
            iter: Enumerate<Iter<'a, Tile>>,
        },
        Sparse(btree_map::Iter<'a, LayerPosition, Tile>),
    }

    impl<'a> TilesLayerIterXY<'a> {
        pub fn new(layer: &'a mut TerminalLayer) -> Self {
            match layer.tiles {
                TilesLayer::Dense(_) => todo!(),
                TilesLayer::Sparse(_) => todo!(),
            }
        }
    }

    impl<'a> Iterator for TilesLayerIterXY<'a> {
        type Item = (IVec2, &'a Tile);

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                TilesLayerIterXY::Dense { width, iter } => iter.next().map(|(i, t)| {
                    let x = (i % *width) as i32;
                    let y = (i / *width) as i32;
                    (IVec2::new(x, y), t)
                }),
                TilesLayerIterXY::Sparse(iter) => iter.next().map(|(p, t)| (IVec2::from(*p), t)),
            }
        }
    }

    impl<'a> DoubleEndedIterator for TilesLayerIterXY<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                TilesLayerIterXY::Dense { width, iter } => iter.next_back().map(|(i, t)| {
                    let x = (i % *width) as i32;
                    let y = (i / *width) as i32;
                    (IVec2::new(x, y), t)
                }),
                TilesLayerIterXY::Sparse(iter) => {
                    iter.next_back().map(|(p, t)| (IVec2::from(*p), t))
                }
            }
        }
    }

    pub enum TilesLayerIterXYMut<'a> {
        Dense {
            width: usize,
            iter: Enumerate<IterMut<'a, Tile>>,
        },
        Sparse(btree_map::IterMut<'a, LayerPosition, Tile>),
    }

    impl<'a> Iterator for TilesLayerIterXYMut<'a> {
        type Item = (IVec2, &'a mut Tile);

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                TilesLayerIterXYMut::Dense { width, iter } => iter.next().map(|(i, t)| {
                    let x = (i % *width) as i32;
                    let y = (i / *width) as i32;
                    (IVec2::new(x, y), t)
                }),
                TilesLayerIterXYMut::Sparse(iter) => iter.next().map(|(p, t)| (IVec2::from(*p), t)),
            }
        }
    }

    impl<'a> DoubleEndedIterator for TilesLayerIterXYMut<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                TilesLayerIterXYMut::Dense { width, iter } => iter.next_back().map(|(i, t)| {
                    let x = (i % *width) as i32;
                    let y = (i / *width) as i32;
                    (IVec2::new(x, y), t)
                }),
                TilesLayerIterXYMut::Sparse(iter) => {
                    iter.next_back().map(|(p, t)| (IVec2::from(*p), t))
                }
            }
        }
    }

    pub enum TilesRowIter<'a> {
        Dense(Iter<'a, Tile>),
        Sparse {
            row: i32,
            iter: TilesLayerIterXY<'a>,
        },
    }

    impl<'a> TilesRowIter<'a> {
        pub fn new(row: usize, layer: &'a TerminalLayer) -> Self {
            let i = row * layer.width();
            let w = layer.width();
            match &layer.tiles {
                TilesLayer::Dense(tiles) => TilesRowIter::Dense(tiles[i..i + w].iter()),
                TilesLayer::Sparse(tiles) => TilesRowIter::Sparse {
                    row: row as i32,
                    iter: TilesLayerIterXY::Sparse(tiles.iter()),
                },
            }
        }
    }

    impl<'a> Iterator for TilesRowIter<'a> {
        type Item = &'a Tile;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                TilesRowIter::Dense(iter) => iter.next(),
                TilesRowIter::Sparse { row, iter } => {
                    for (xy, tile) in iter.by_ref() {
                        if xy.y == *row {
                            return Some(tile);
                        }
                    }
                    None
                }
            }
        }
    }

    impl<'a> DoubleEndedIterator for TilesRowIter<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                TilesRowIter::Dense(iter) => iter.next_back(),
                TilesRowIter::Sparse { row, iter } => {
                    for (xy, tile) in iter.rev().by_ref() {
                        if xy.y == *row {
                            return Some(tile);
                        }
                        continue;
                    }
                    None
                }
            }
        }
    }

    pub enum TilesRowIterMut<'a> {
        Dense(IterMut<'a, Tile>),
        Sparse {
            row: i32,
            iter: TilesLayerIterXYMut<'a>,
        },
    }

    impl<'a> TilesRowIterMut<'a> {
        pub fn new(row: usize, layer: &'a mut TerminalLayer) -> Self {
            let i = row * layer.width();
            let w = layer.width();
            match &mut layer.tiles {
                TilesLayer::Dense(tiles) => TilesRowIterMut::Dense(tiles[i..i + w].iter_mut()),
                TilesLayer::Sparse(tiles) => TilesRowIterMut::Sparse {
                    row: row as i32,
                    iter: TilesLayerIterXYMut::Sparse(tiles.iter_mut()),
                },
            }
        }
    }

    impl<'a> Iterator for TilesRowIterMut<'a> {
        type Item = &'a mut Tile;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                TilesRowIterMut::Dense(iter) => iter.next(),
                TilesRowIterMut::Sparse { row, iter } => {
                    for (xy, tile) in iter.by_ref() {
                        if xy.y == *row {
                            return Some(tile);
                        }
                    }
                    None
                }
            }
        }
    }

    impl<'a> DoubleEndedIterator for TilesRowIterMut<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                TilesRowIterMut::Dense(iter) => iter.next_back(),
                TilesRowIterMut::Sparse { row, iter } => {
                    for (xy, tile) in iter.rev().by_ref() {
                        if xy.y == *row {
                            return Some(tile);
                        }
                        continue;
                    }
                    None
                }
            }
        }
    }

    pub enum TilesColumnIter<'a> {
        Dense(StepBy<Iter<'a, Tile>>),
        Sparse {
            col: i32,
            iter: btree_map::Iter<'a, LayerPosition, Tile>,
        },
    }

    impl<'a> Iterator for TilesColumnIter<'a> {
        type Item = &'a Tile;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                TilesColumnIter::Dense(iter) => iter.next(),
                TilesColumnIter::Sparse { col, iter } => {
                    for (xy, tile) in iter.by_ref() {
                        if xy.0.x == *col {
                            return Some(tile);
                        }
                    }
                    None
                }
            }
        }
    }

    impl<'a> TilesColumnIter<'a> {
        pub fn new(column: usize, layer: &'a TerminalLayer) -> Self {
            let w = layer.width();
            let h = layer.height();
            let xy = IVec2::new(column as i32, 0);

            match &layer.tiles {
                TilesLayer::Dense(tiles) => {
                    let i = column;
                    let end = i + (w * h.sub(1));
                    TilesColumnIter::Dense(tiles[i..=end].iter().step_by(w))
                }
                TilesLayer::Sparse(tiles) => TilesColumnIter::Sparse {
                    col: xy.x,
                    iter: tiles.iter(),
                },
            }
        }
    }

    impl<'a> DoubleEndedIterator for TilesColumnIter<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                TilesColumnIter::Dense(iter) => iter.next_back(),
                TilesColumnIter::Sparse { col, iter } => {
                    for (xy, tile) in iter.rev().by_ref() {
                        if xy.0.x == *col {
                            return Some(tile);
                        }
                    }
                    None
                }
            }
        }
    }

    pub enum TilesColumnIterMut<'a> {
        Dense(StepBy<IterMut<'a, Tile>>),
        Sparse {
            col: i32,
            iter: btree_map::IterMut<'a, LayerPosition, Tile>,
        },
    }

    impl<'a> TilesColumnIterMut<'a> {
        pub fn new(column: usize, layer: &'a mut TerminalLayer) -> Self {
            let w = layer.width();
            let h = layer.height();
            let xy = IVec2::new(column as i32, 0);

            match &mut layer.tiles {
                TilesLayer::Dense(tiles) => {
                    let i = column;
                    let end = i + (w * h.sub(1));
                    TilesColumnIterMut::Dense(tiles[i..=end].iter_mut().step_by(w))
                }
                TilesLayer::Sparse(tiles) => TilesColumnIterMut::Sparse {
                    col: xy.x,
                    iter: tiles.iter_mut(),
                },
            }
        }
    }

    impl<'a> Iterator for TilesColumnIterMut<'a> {
        type Item = &'a mut Tile;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                TilesColumnIterMut::Dense(iter) => iter.next(),
                TilesColumnIterMut::Sparse { col, iter } => {
                    for (xy, tile) in iter.by_ref() {
                        if xy.0.x == *col {
                            return Some(tile);
                        }
                    }
                    None
                }
            }
        }
    }

    impl<'a> DoubleEndedIterator for TilesColumnIterMut<'a> {
        fn next_back(&mut self) -> Option<Self::Item> {
            match self {
                TilesColumnIterMut::Dense(iter) => iter.next_back(),
                TilesColumnIterMut::Sparse { col, iter } => {
                    for (xy, tile) in iter.rev().by_ref() {
                        if xy.0.x == *col {
                            return Some(tile);
                        }
                    }
                    None
                }
            }
        }
    }

    pub enum TilesRectIter<'a> {
        Dense {
            iter: Iter<'a, Tile>,
            i: usize,
            width: usize,
            skip: usize,
        },
        Sparse {
            rect: GridRect,
            iter: TilesLayerIterXY<'a>,
        },
    }

    impl<'a> TilesRectIter<'a> {
        pub fn new(rect: GridRect, layer: &'a TerminalLayer) -> Self {
            let min = rect.min();
            let max = rect.max();
            let skip = layer.width() - rect.width();
            let start = min.as_index(layer.width());
            let end = max.as_index(layer.width());

            match &layer.tiles {
                TilesLayer::Dense(tiles) => TilesRectIter::Dense {
                    iter: tiles[start..=end].iter(),
                    i: 0,
                    width: rect.width(),
                    skip,
                },
                TilesLayer::Sparse(tiles) => TilesRectIter::Sparse {
                    rect,
                    iter: TilesLayerIterXY::Sparse(tiles.iter()),
                },
            }
        }
    }

    impl<'a> Iterator for TilesRectIter<'a> {
        type Item = &'a Tile;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                TilesRectIter::Dense {
                    iter,
                    i,
                    width,
                    skip,
                } => {
                    if i >= width {
                        *i = 0;
                        iter.nth(*skip);
                    }
                    *i += 1;
                    iter.next()
                }
                TilesRectIter::Sparse { rect, iter } => {
                    for (xy, tile) in iter.by_ref() {
                        if xy.cmpgt(rect.max()).all() {
                            return None;
                        }
                        if rect.contains_point(xy) {
                            return Some(tile);
                        }
                    }
                    None
                }
            }
        }
    }

    pub enum TilesRectIterMut<'a> {
        Dense {
            iter: IterMut<'a, Tile>,
            i: usize,
            width: usize,
            skip: usize,
        },
        Sparse {
            rect: GridRect,
            iter: TilesLayerIterXYMut<'a>,
        },
    }

    impl<'a> TilesRectIterMut<'a> {
        pub fn new(rect: GridRect, layer: &'a mut TerminalLayer) -> Self {
            let min = rect.min();
            let max = rect.max();
            let skip = layer.width() - rect.width();
            let start = min.as_index(layer.width());
            let end = max.as_index(layer.width());

            match &mut layer.tiles {
                TilesLayer::Dense(tiles) => TilesRectIterMut::Dense {
                    iter: tiles[start..=end].iter_mut(),
                    i: 0,
                    width: rect.width(),
                    skip,
                },
                TilesLayer::Sparse(tiles) => TilesRectIterMut::Sparse {
                    rect,
                    iter: TilesLayerIterXYMut::Sparse(tiles.iter_mut()),
                },
            }
        }
    }

    impl<'a> Iterator for TilesRectIterMut<'a> {
        type Item = &'a mut Tile;

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                TilesRectIterMut::Dense {
                    iter,
                    i,
                    width,
                    skip,
                } => {
                    if i >= width {
                        *i = 0;
                        iter.nth(*skip);
                    }
                    *i += 1;
                    iter.next()
                }
                TilesRectIterMut::Sparse { rect, iter } => {
                    for (xy, tile) in iter.by_ref() {
                        if xy.cmpgt(rect.max()).all() {
                            return None;
                        }
                        if rect.contains_point(xy) {
                            return Some(tile);
                        }
                    }
                    None
                }
            }
        }
    }

    pub struct TilesLineIter<'a> {
        min: IVec2,
        max: IVec2,
        axis: usize,
        iter: TilesLayerIterXY<'a>,
    }

    impl<'a> TilesLineIter<'a> {
        pub fn new(xy: IVec2, dir: Dir4, len: usize, layer: &'a TerminalLayer) -> Self {
            let a = xy.as_ivec2();
            let b = a + (dir.as_ivec2() * len as i32);
            let axis = match dir {
                Dir4::Left | Dir4::Right => 0,
                Dir4::Up | Dir4::Down => 1,
            };
            TilesLineIter {
                min: a.min(b),
                max: a.max(b),
                axis,
                iter: layer.iter_xy(),
            }
        }
    }

    impl<'a> Iterator for TilesLineIter<'a> {
        type Item = &'a Tile;

        fn next(&mut self) -> Option<Self::Item> {
            for (xy, tile) in self.iter.by_ref() {
                if xy.cmpgt(self.max).all() {
                    return None;
                }
                if xy[self.axis] == self.min[self.axis] {
                    return Some(tile);
                }
            }
            None
        }
    }

    pub struct TilesLineIterMut<'a> {
        min: IVec2,
        max: IVec2,
        axis: usize,
        iter: TilesLayerIterXYMut<'a>,
    }

    impl<'a> TilesLineIterMut<'a> {
        pub fn new(
            xy: impl GridPoint,
            dir: Dir4,
            len: usize,
            layer: &'a mut TerminalLayer,
        ) -> Self {
            let a = xy.as_ivec2();
            let b = a + (dir.as_ivec2() * len as i32);
            let axis = match dir {
                Dir4::Left | Dir4::Right => 0,
                Dir4::Up | Dir4::Down => 1,
            };
            TilesLineIterMut {
                min: a.min(b),
                max: a.max(b),
                axis,
                iter: layer.iter_xy_mut(),
            }
        }
    }

    impl<'a> Iterator for TilesLineIterMut<'a> {
        type Item = &'a mut Tile;

        fn next(&mut self) -> Option<Self::Item> {
            for (xy, tile) in self.iter.by_ref() {
                if xy.cmpgt(self.max).all() {
                    return None;
                }
                if xy[self.axis] == self.min[self.axis] {
                    return Some(tile);
                }
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{tile::TileWriter, Tile};

    use super::{LayerPosition, TerminalLayer};

    #[test]
    fn layerpos() {
        let mut tree = BTreeMap::new();

        tree.insert(LayerPosition::new([10, 11]), 'a');
        tree.insert(LayerPosition::new([12, 12]), 'f');
        tree.insert(LayerPosition::new([10, 13]), 'g');
        tree.insert(LayerPosition::new([12, 11]), 'c');
        tree.insert(LayerPosition::new([11, 12]), 'e');
        tree.insert(LayerPosition::new([11, 13]), 'h');
        tree.insert(LayerPosition::new([11, 11]), 'b');
        tree.insert(LayerPosition::new([12, 13]), 'i');
        tree.insert(LayerPosition::new([10, 12]), 'd');

        let chars: Vec<_> = tree.values().cloned().collect();
        assert_eq!('a', chars[0]);
        assert_eq!('b', chars[1]);
        assert_eq!('c', chars[2]);
        assert_eq!('d', chars[3]);
        assert_eq!('e', chars[4]);
        assert_eq!('f', chars[5]);
        assert_eq!('g', chars[6]);
        assert_eq!('h', chars[7]);
        assert_eq!('i', chars[8]);
    }

    #[test]
    fn column() {
        let mut layer = TerminalLayer::new_dense([10, 10], 0, Tile::DEFAULT);
        layer.put_char([3, 0], 'a');
        layer.put_char([3, 9], 'b');

        let mut col = layer.iter_column_mut(3);

        assert_eq!('a', col.next().unwrap().glyph);
        assert_eq!('b', col.next_back().unwrap().glyph);
    }

    #[test]
    fn xy_iter() {
        let layer = TerminalLayer::new_dense([15, 15], 0, Tile::DEFAULT);
        let mut iter = layer.iter_xy();
        assert_eq!([0, 0], iter.next().unwrap().0.to_array());
        assert_eq!([14, 14], iter.next_back().unwrap().0.to_array());
    }
}
