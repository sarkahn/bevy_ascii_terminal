//! A grid of tiles for rendering colorful ascii.

use bevy::{
    color::{ColorToPacked, LinearRgba},
    math::{IVec2, UVec2},
    prelude::{Component, Mesh2d},
    reflect::Reflect,
    sprite_render::MeshMaterial2d,
};
use sark_grids::{GridRect, GridSize, Pivot, PivotedPoint};

use crate::{
    Tile, ascii,
    render::{
        RebuildMeshVerts, TerminalFont, TerminalMaterial, TerminalMeshPivot, UvMappingHandle,
    },
    rexpaint::reader::XpFile,
    strings::{GridStringIterator, TerminalString},
    transform::TerminalTransform,
};

/// A grid of tiles for rendering colorful ascii.
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
    /// An internal buffer to minimize allocations when processing strings.
    string_buffer: String,
}

impl Terminal {
    pub fn new(size: impl GridSize) -> Self {
        Self {
            size: size.to_uvec2(),
            tiles: vec![Tile::default(); size.tile_count()],
            clear_tile: Tile::default(),
            string_buffer: String::default(),
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
                    let glyph = ascii::try_index_to_char(glyph as u8).unwrap_or(' ');
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

    /// Create a terminal from a string, where each line is a row of the terminal.
    /// Empty lines will be ignored, add a space if you want an actual empty row
    /// built into the terminal.
    ///
    /// # Example
    /// ```
    /// use bevy_ascii_terminal::Terminal;
    /// let terminal = Terminal::from_string("Hello\nWorld").unwrap();
    /// ```
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
    /// clearing a terminal.
    pub fn with_clear_tile(mut self, clear_tile: Tile) -> Self {
        self.clear_tile = clear_tile;
        self.fill(clear_tile);
        self
    }

    /// A utility function to add a string to the terminal during creation.
    pub fn with_string<T: AsRef<str>>(
        mut self,
        xy: impl Into<PivotedPoint>,
        string: impl Into<TerminalString<T>>,
    ) -> Self {
        self.put_string(xy, string);
        self
    }

    /// Insert a character to the terminal.
    ///
    /// This returns a mutable reference to the terminal tile which can be used
    /// to further modify it.
    ///
    /// # Example
    /// ```
    /// use bevy_ascii_terminal::*;
    /// let mut terminal = Terminal::new([10, 10]);
    /// terminal.put_char([5, 5], 'X').fg(color::RED);
    /// ```
    pub fn put_char(&mut self, xy: impl Into<PivotedPoint>, ch: char) -> &mut Tile {
        self.tile_mut(xy).char(ch)
    }

    /// Set the foreground color of a tile.
    ///
    /// This returns a mutable reference to the terminal tile which can be used
    /// to further modify it.
    ///
    /// # Example
    /// ```
    /// use bevy_ascii_terminal::*;
    /// let mut terminal = Terminal::new([10, 10]);
    /// terminal.put_fg_color([5, 5], color::RED).bg(color::BLUE);
    /// ```
    pub fn put_fg_color(
        &mut self,
        xy: impl Into<PivotedPoint>,
        color: impl Into<LinearRgba>,
    ) -> &mut Tile {
        self.tile_mut(xy).fg(color)
    }

    /// Set the background color of a tile.
    ///
    /// This returns a mutable reference to the terminal tile which can be used
    /// to further modify it.
    ///
    /// # Example
    /// ```
    /// use bevy_ascii_terminal::*;
    /// let mut terminal = Terminal::new([10, 10]);
    /// terminal.put_bg_color([5, 5], color::BLUE).fg(color::RED);
    /// ```
    pub fn put_bg_color(
        &mut self,
        xy: impl Into<PivotedPoint>,
        color: impl Into<LinearRgba>,
    ) -> &mut Tile {
        self.tile_mut(xy).bg(color)
    }

    /// Insert a tile into the terminal.
    pub fn put_tile(&mut self, xy: impl Into<PivotedPoint>, tile: Tile) -> &mut Tile {
        let xy = xy.into();
        let t = self.tile_mut(xy);
        *t = tile;
        t
    }

    /// Clear the terminal, setting all tiles to the terminal's `clear_tile`.
    pub fn clear(&mut self) {
        self.tiles.fill(self.clear_tile);
    }

    pub fn fill(&mut self, tile: Tile) {
        self.tiles.fill(tile);
    }

    /// Write a formatted string to the terminal.
    ///
    /// Formatting options can be applied to the string before writing it to the terminal,
    /// see [TerminalString].
    ///
    /// By default strings will be written to the top left of the terminal. You
    /// can apply a pivot to the xy position to change this.
    ///
    /// # Example
    /// ```
    /// use bevy_ascii_terminal::*;
    /// let mut terminal = Terminal::new([10, 10]);
    /// terminal.put_string([5, 5], "Hello, World!".bg(color::BLUE));
    /// terminal.put_string([1, 1].pivot(Pivot::BottomLeft), "Beep beep!");
    /// ```
    pub fn put_string<T: AsRef<str>>(
        &mut self,
        xy: impl Into<PivotedPoint>,
        string: impl Into<TerminalString<T>>,
    ) {
        let bounds = self.bounds();
        let ts: TerminalString<T> = string.into();
        let clear_tile = self.clear_tile;
        let clear_colors = ts.decoration.clear_colors;
        let mut iter = GridStringIterator::new(
            ts.string.as_ref(),
            bounds,
            xy,
            Some(ts.formatting),
            Some(ts.decoration),
        );
        for (xy, (ch, fg, bg)) in iter.by_ref() {
            if !self.bounds().contains_point(xy) {
                continue;
            }
            let tile = self.tile_mut(xy);
            tile.glyph = ch;
            if clear_colors {
                tile.fg_color = clear_tile.fg_color;
                tile.bg_color = clear_tile.bg_color;
            } else {
                if let Some(col) = fg {
                    tile.fg_color = col;
                }
                if let Some(col) = bg {
                    tile.bg_color = col;
                }
            }
        }
    }

    /// Read a line of characters starting from a grid position on the terminal.
    ///
    /// As with [Terminal::put_string] the xy position will default to a top-left
    /// pivot.
    pub fn read_line(
        &self,
        xy: impl Into<PivotedPoint>,
        width: usize,
    ) -> impl Iterator<Item = char> + '_ {
        let xy: PivotedPoint = xy.into();
        let xy = xy.with_default_pivot(Pivot::TopLeft);
        let i = self.tile_to_index(xy);
        let remaining_width = (self.width() - i % self.width()).min(width);
        self.tiles[i..i + remaining_width].iter().map(|t| t.glyph)
    }

    /// Transform a local 2d tile index into it's corresponding 1d index into the
    /// terminal tile data.
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

    /// Retrieve a tile at the grid position. This will panic if the position is
    /// out of bounds.
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

    /// Retrieve a tile at the grid position. This will panic if the position is
    /// out of bounds.
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
        self.tiles
            .chunks(self.width())
            .skip(rect.bottom() as usize)
            .flat_map(move |tiles| tiles[rect.left() as usize..=rect.right() as usize].iter())
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

    /// The local grid bounds of the terminal. For world bounds see [TerminalTransform].
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

#[cfg(test)]
mod tests {
    use crate::{GridPoint, Pivot, Terminal, ascii};

    #[test]
    fn put_string_negative() {
        let mut terminal = Terminal::new([10, 10]);
        terminal.put_string([-2, -2].pivot(Pivot::Center), "Hello");
        assert_eq!(terminal.tile([1, 3]).glyph, 'H');
    }

    #[test]
    fn read_line() {
        let mut terminal = Terminal::new([20, 10]);
        terminal.put_string([2, 2], "Hello, World!");
        let line: String = terminal.read_line([2, 2], 5).collect();
        assert_eq!(line, "Hello");
    }

    #[test]
    fn big_string() {
        let mut term = Terminal::new([16, 16]);
        let string = String::from_iter(ascii::CP_437_ARRAY.iter());
        term.put_string([0, 0], string);
    }
}
