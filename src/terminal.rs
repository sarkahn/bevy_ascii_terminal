use bevy::{
    color::{ColorToPacked, LinearRgba},
    math::{IVec2, UVec2},
    prelude::{Component, Mesh2d},
    reflect::Reflect,
    sprite::MeshMaterial2d,
};
use sark_grids::{GridRect, GridSize, PivotedPoint};

use crate::{
    ascii,
    render::{
        RebuildMeshVerts, TerminalFont, TerminalMaterial, TerminalMeshPivot, UvMappingHandle,
    },
    rexpaint::reader::XpFile,
    string::{StringIter, TerminalString},
    transform::TerminalTransform,
    Tile,
};

/// A terminal to present a sized grid of colored tiles.
#[derive(Debug, Reflect, Component, Clone)]
#[require(
    TerminalTransform,
    TerminalFont,
    TerminalMeshPivot,
    UvMappingHandle,
    Mesh2d,
    MeshMaterial2d<TerminalMaterial>,
    RebuildMeshVerts,
)]
pub struct Terminal {
    size: UVec2,
    tiles: Vec<Tile>,
    clear_tile: Tile,
}

impl Terminal {
    pub fn new(size: impl GridSize) -> Self {
        Self {
            size: size.to_uvec2(),
            tiles: vec![Tile::default(); size.tile_count()],
            clear_tile: Tile::default(),
        }
    }

    /// Create a terminal from a REXPaint file. Note this writes all layers to the
    /// same terminal, so it won't preserve the transparent layering aspect of
    /// actual rexpaint files.
    pub fn from_rexpaint_file(file_path: impl AsRef<str>) -> std::io::Result<Self> {
        let mut file = std::fs::File::open(file_path.as_ref())?;
        let xp = XpFile::read(&mut file)?;
        let Some((w, h)) = xp.layers.first().map(|l| (l.width, l.height)) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No layers found in REXPaint file",
            ));
        };
        let mut terminal = Self::new([w, h]);
        for layer in &xp.layers {
            for y in 0..layer.height {
                for x in 0..layer.width {
                    let cell = layer.get(x, y).unwrap();
                    let Some(glyph) = char::from_u32(cell.ch) else {
                        continue;
                    };
                    let glyph = ascii::index_to_char(glyph as u8);
                    let frgb = [cell.fg.r, cell.fg.g, cell.fg.b, 255];
                    let brgb = [cell.bg.r, cell.bg.g, cell.bg.b, 255];
                    let fg = LinearRgba::from_u8_array(frgb);
                    let bg = LinearRgba::from_u8_array(brgb);
                    let t = terminal.tile_mut([x, y]);
                    t.glyph = glyph;
                    t.fg_color = fg;
                    t.bg_color = bg;
                }
            }
        }
        Ok(terminal)
    }

    pub fn from_string(string: impl AsRef<str>) -> Option<Self> {
        let width = string.as_ref().lines().map(|l| l.len()).max()?;
        let height = string.as_ref().lines().filter(|l| !l.is_empty()).count();
        if width == 0 || height == 0 {
            return None;
        }
        let mut terminal = Self::new([width, height]);
        for (y, line) in string.as_ref().lines().rev().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let t = terminal.tile_mut([x as i32, y as i32]);
                t.glyph = ch;
            }
        }
        Some(terminal)
    }

    /// Specify the terminal's `clear tile`. This is the default tile used when
    /// clearing a dense terminal or when creating a new tile on a sparse terminal.
    pub fn with_clear_tile(mut self, clear_tile: Tile) -> Self {
        self.clear_tile = clear_tile;
        self.fill(clear_tile);
        self
    }

    /// Can be used to add a string to the terminal during initialization.
    pub fn with_string<T: AsRef<str>>(
        mut self,
        xy: impl Into<PivotedPoint>,
        string: impl Into<TerminalString<T>>,
    ) -> Self {
        self.put_string(xy, string);
        self
    }

    pub fn put_char(&mut self, xy: impl Into<PivotedPoint>, ch: char) -> &mut Tile {
        self.tile_mut(xy).char(ch)
    }

    pub fn put_fg_color(
        &mut self,
        xy: impl Into<PivotedPoint>,
        color: impl Into<LinearRgba>,
    ) -> &mut Tile {
        self.tile_mut(xy).fg(color)
    }

    pub fn put_bg_color(
        &mut self,
        xy: impl Into<PivotedPoint>,
        color: impl Into<LinearRgba>,
    ) -> &mut Tile {
        self.tile_mut(xy).bg(color)
    }

    pub fn put_tile(&mut self, xy: impl Into<PivotedPoint>, tile: Tile) -> &mut Tile {
        let xy = xy.into();
        let t = self.tile_mut(xy);
        *t = tile;
        t
    }

    /// Clear the terminal, setting all tiles to the terminals `clear_tile`.
    pub fn clear(&mut self) {
        self.tiles.fill(self.clear_tile);
    }

    pub fn fill(&mut self, tile: Tile) {
        self.tiles.fill(tile);
    }

    /// Write a formatted string to the terminal.
    ///
    /// By default strings will be written to the top left of the terminal. You
    /// can apply a pivot to the xy position to change this.
    pub fn put_string<T: AsRef<str>>(
        &mut self,
        xy: impl Into<PivotedPoint>,
        string: impl Into<TerminalString<T>>,
    ) {
        let bounds = self.bounds();
        let ts: TerminalString<T> = string.into();
        let wrap = ts.formatting.word_wrap;
        let fg = if ts.decoration.clear_colors {
            Some(self.clear_tile.fg_color)
        } else {
            ts.decoration.fg_color
        };
        let bg = if ts.decoration.clear_colors {
            Some(self.clear_tile.bg_color)
        } else {
            ts.decoration.bg_color
        };
        let ignore_spaces = ts.formatting.ignore_spaces;
        let mut iter = StringIter::new(xy, ts.string.as_ref(), bounds, wrap);

        for (xy, ch) in iter.by_ref() {
            if ignore_spaces && ch.is_whitespace() {
                continue;
            }
            let tile = self.tile_mut(xy);
            tile.glyph = ch;
            if let Some(fg) = fg {
                tile.fg_color = fg;
            }
            if let Some(bg) = bg {
                tile.bg_color = bg;
            }
        }
    }

    /// Transform a local 2d tile index into 1d index into the terminal tile data.
    #[inline]
    pub fn tile_to_index(&self, xy: impl Into<PivotedPoint>) -> usize {
        let xy: PivotedPoint = xy.into();
        let [x, y] = xy.calculate(self.size).to_array();
        y as usize * self.width() + x as usize
    }

    /// Convert a 1d index into the terminal tile data into it's corresponding
    /// 2d tile index.
    #[inline]
    pub fn index_to_tile(&self, i: usize) -> IVec2 {
        let w = self.width() as i32;
        IVec2::new(i as i32 % w, i as i32 / w)
    }

    /// Retrieve a tile at the grid position. This will panic if the index is
    /// out of bounds.
    ///
    /// For a sparse grid this will insert a clear tile if no tile exists and
    /// return it.
    pub fn tile_mut(&mut self, xy: impl Into<PivotedPoint>) -> &mut Tile {
        let xy = xy.into();
        debug_assert!(
            self.size.contains_point(xy.calculate(self.size)),
            "Attempting to access a tile at an out of bounds grid position {:?} 
        from a terminal of size {}",
            xy,
            self.size
        );
        let i = self.tile_to_index(xy);
        &mut self.tiles[i]
    }

    /// Retrieve a tile at the grid position. This will panic if the index is
    /// out of bounds.
    ///
    /// For a sparse terminal this will panic if no tile exists at the given position.
    /// Note this behavior is different from `tile_mut` which will automatically
    /// insert and return a clear tile if no tile exists at the given position.
    pub fn tile(&self, xy: impl Into<PivotedPoint>) -> &Tile {
        let xy = xy.into();
        debug_assert!(
            self.size.contains_point(xy.calculate(self.size)),
            "Attempting to access a tile at an out of bounds grid position {:?} 
        from a terminal of size {}",
            xy,
            self.size
        );
        let i = self.tile_to_index(xy);
        &self.tiles[i]
    }

    pub fn width(&self) -> usize {
        self.size.x as usize
    }

    pub fn height(&self) -> usize {
        self.size.y as usize
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    /// The terminal tiles as a slice.
    pub fn tiles(&self) -> &[Tile] {
        self.tiles.as_slice()
    }

    /// The terminal tiles as a slice.
    pub fn tiles_mut(&mut self) -> &mut [Tile] {
        self.tiles.as_mut_slice()
    }

    /// Iterate over a row of terminal tiles. Row indices start from 0 at the bottom.
    pub fn iter_row(&self, row: usize) -> impl DoubleEndedIterator<Item = &Tile> {
        let start = self.width() * row;
        let end = start + self.width();
        self.tiles[start..end].iter()
    }

    /// Iterate over a row of terminal tiles. Row indices start from 0 at the bottom.
    pub fn iter_row_mut(&mut self, row: usize) -> impl DoubleEndedIterator<Item = &mut Tile> {
        let start = self.width() * row;
        let end = start + self.width();
        self.tiles[start..end].iter_mut()
    }

    /// Iterate over a column of terminal tiles. Column indices start from 0 at the left.
    pub fn iter_column(&self, column: usize) -> impl DoubleEndedIterator<Item = &Tile> {
        self.tiles.iter().skip(column).step_by(self.width())
    }

    /// Iterate over a column of terminal tiles. Column indices start from 0 at the left.
    pub fn iter_column_mut(&mut self, column: usize) -> impl DoubleEndedIterator<Item = &mut Tile> {
        let w = self.width();
        self.tiles.iter_mut().skip(column).step_by(w)
    }

    /// Iterate over a rectangular section of terminal tiles.
    pub fn iter_rect(&self, rect: GridRect) -> impl DoubleEndedIterator<Item = &Tile> {
        let iter = self
            .tiles
            .chunks(self.width())
            .skip(rect.bottom() as usize)
            .flat_map(move |tiles| tiles[rect.left() as usize..=rect.right() as usize].iter());

        iter
    }

    /// Iterate over a rectangular section of terminal tiles.
    pub fn iter_rect_mut(&mut self, rect: GridRect) -> impl DoubleEndedIterator<Item = &mut Tile> {
        let w = self.width();
        self.tiles
            .chunks_mut(w)
            .skip(rect.bottom() as usize)
            .flat_map(move |tiles| tiles[rect.left() as usize..=rect.right() as usize].iter_mut())
    }

    /// An iterator over all tiles that also yields each tile's 2d grid position
    pub fn iter_xy(&self) -> impl DoubleEndedIterator<Item = (IVec2, &Tile)> {
        self.tiles
            .iter()
            .enumerate()
            .map(|(i, t)| (self.index_to_tile(i), t))
    }

    /// An iterator over all tiles that also yields each tile's 2d grid position
    pub fn iter_xy_mut(&mut self) -> impl DoubleEndedIterator<Item = (IVec2, &mut Tile)> {
        let w = self.width() as i32;
        let index_to_xy = move |i: i32| IVec2::new(i % w, i / w);
        self.tiles
            .iter_mut()
            .enumerate()
            .map(move |(i, t)| (index_to_xy(i as i32), t))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Tile> {
        self.tiles.iter_mut()
    }

    /// The local grid bounds of the terminal. For world bounds, see [TerminalTransform].
    pub fn bounds(&self) -> GridRect {
        GridRect::new([0, 0], self.size)
    }

    pub fn clear_tile(&self) -> Tile {
        self.clear_tile
    }

    pub fn resize(&mut self, new_size: impl GridSize) {
        let new_size = new_size.to_uvec2().max(UVec2::new(2, 2));
        self.tiles = vec![self.clear_tile; new_size.tile_count()];
        self.size = new_size;
    }
}
