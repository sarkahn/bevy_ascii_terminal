//! A grid of tiles for rendering colorful ascii.
use bevy::{
    color::{ColorToPacked, LinearRgba},
    math::{IVec2, UVec2, ivec2},
    prelude::{Component, Mesh2d},
    reflect::Reflect,
    sprite_render::MeshMaterial2d,
};

#[allow(deprecated)]
use crate::{
    GridRect, GridSize, Pivot, PivotedPoint, Tile, ascii,
    render::{
        RebuildMeshVerts, TerminalFont, TerminalMaterial, TerminalMeshPivot, UvMappingHandle,
    },
    rexpaint::reader::XpFile,
    strings::TerminalString,
    transform::TerminalTransform,
};
use crate::{
    Token, TokenIterator,
    padding::{BoxStyle, ColorWrite, Padding},
    wrap_line_count, wrap_string, wrap_tagged_line_count, wrap_tagged_string,
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
    pivot: Pivot,
    padding: Padding,
}

impl Terminal {
    #[allow(deprecated)]
    pub fn new(size: impl GridSize) -> Self {
        Self {
            size: size.to_uvec2(),
            tiles: vec![Tile::default(); size.tile_count()],
            clear_tile: Tile::default(),
            pivot: Pivot::default(),
            padding: Padding::default(),
        }
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

    /// Draw a border on the terminal.
    pub fn with_border(mut self, box_style: BoxStyle) -> Self {
        self.put_border(box_style);
        self
    }

    /// Set the padding for the terminal.
    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Set a title
    pub fn with_title<T: AsRef<str>>(mut self, title: impl Into<TerminalString<T>>) -> Self {
        self.put_title(title);
        self
    }

    pub fn with_pivot(mut self, pivot: Pivot) -> Self {
        self.pivot = pivot;
        self
    }

    /// Set the clear character
    pub fn with_clear_char(mut self, glyph: char) -> Self {
        self.clear_tile.glyph = glyph;
        self.maybe_fill(Some(glyph), None, None);
        self
    }

    /// Set the foreground clear color
    pub fn with_fg_clear_color(mut self, col: LinearRgba) -> Self {
        self.clear_tile.fg_color = col;
        self.maybe_fill(None, Some(col), None);
        self
    }

    /// Set the background clear color
    pub fn with_bg_clear_color(mut self, col: LinearRgba) -> Self {
        self.clear_tile.bg_color = col;
        self.maybe_fill(None, None, Some(col));
        self
    }

    /// A utility function to add a string to the terminal during creation.
    #[allow(deprecated)]
    pub fn with_string<T: AsRef<str>>(
        mut self,
        xy: impl Into<PivotedPoint>,
        string: impl Into<TerminalString<T>>,
    ) -> Self {
        let xy = xy.into();

        self.put_string(xy.point, string);
        self
    }

    pub fn set_padding(&mut self, padding: Padding) {
        self.padding = padding;
    }

    pub fn padding(&self) -> Padding {
        self.padding
    }

    pub fn set_pivot(&mut self, pivot: Pivot) {
        self.pivot = pivot;
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
    #[allow(deprecated)]
    pub fn put_char(&mut self, xy: impl Into<PivotedPoint>, ch: char) -> &mut Tile {
        let pp = xy.into();

        let piv = self.pivot;
        if let Some(pivot) = pp.pivot {
            self.pivot = pivot;
        }

        let i = self
            .try_transform_point_to_index(pp.point)
            .expect("Put char out of bounds");

        self.tiles[i].glyph = ch;

        self.pivot = piv;

        &mut self.tiles[i]
    }

    pub fn maybe_put(
        &mut self,
        xy: impl Into<IVec2>,
        glyph: Option<char>,
        fg: Option<LinearRgba>,
        bg: Option<LinearRgba>,
    ) {
        //let xy = xy.into().calculate(self.size());
        let xy = xy.into();

        let i = (xy.y * self.width() as i32 + xy.x) as usize;
        let t = &mut self.tiles[i];
        if let Some(glyph) = glyph {
            t.glyph = glyph;
        }

        if let Some(fg) = fg {
            t.fg_color = fg;
        }

        if let Some(bg) = bg {
            t.bg_color = bg;
        }
    }

    pub fn maybe_fill(
        &mut self,
        glyph: Option<char>,
        fg: Option<LinearRgba>,
        bg: Option<LinearRgba>,
    ) {
        if let Some(ch) = glyph {
            for t in self.tiles.iter_mut() {
                t.glyph = ch;
            }
        }
        if let Some(col) = fg {
            for t in self.tiles.iter_mut() {
                t.fg_color = col;
            }
        }

        if let Some(col) = bg {
            for t in self.tiles.iter_mut() {
                t.bg_color = col;
            }
        }
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
    /// terminal.put_fg_color([5, 5], color::RED);
    /// ```
    #[allow(deprecated)]
    pub fn put_fg_color(&mut self, xy: impl Into<PivotedPoint>, color: impl Into<LinearRgba>) {
        let pp = xy.into();

        // TODO: Remove after pivoted point is gone
        let piv = self.pivot;
        if let Some(pivot) = pp.pivot {
            self.pivot = pivot;
        }

        self.tile_mut(pp.point).fg(color);

        self.pivot = piv;
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
    /// terminal.put_bg_color([5, 5], color::BLUE);
    /// ```
    #[allow(deprecated)]
    pub fn put_bg_color(&mut self, xy: impl Into<PivotedPoint>, color: impl Into<LinearRgba>) {
        let pp = xy.into();

        let piv = self.pivot;
        if let Some(pivot) = pp.pivot {
            self.pivot = pivot;
        }

        self.tile_mut(pp.point).bg(color);

        self.pivot = piv;
    }

    /// Insert a tile into the terminal.
    #[allow(deprecated)]
    pub fn put_tile(&mut self, xy: impl Into<PivotedPoint>, tile: Tile) {
        let xy = xy.into();
        let piv = self.pivot;
        if let Some(pivot) = xy.pivot {
            self.pivot = pivot;
        }
        let t = self.tile_mut(xy.point);
        *t = tile;

        self.pivot = piv;
    }

    /// Writes title to the top row of the terminal. Leading spaces will offset
    /// the title.
    pub fn put_title<T: AsRef<str>>(&mut self, title: impl Into<TerminalString<T>>) {
        let padding = self.padding;
        self.padding = Padding::ZERO;
        let pivot = self.pivot;
        self.pivot = Pivot::LeftTop;
        let ts = title.into();
        let title = ts.string.as_ref();

        let offset = title.find(|c| c != ' ').unwrap_or(0);
        let title = title.trim_start();
        // TODO: Change
        self.put_string([offset as i32, 0], title);

        self.padding = padding;
        self.pivot = pivot;
    }

    /// Clear the terminal, setting all tiles to the terminal's `clear_tile`.
    pub fn clear(&mut self) {
        self.tiles.fill(self.clear_tile);
    }

    pub fn clear_inner(&mut self) {
        for y in 0..self.inner_height() {
            let i = self.try_transform_point_to_index([0, y as i32]).unwrap();
            let w = self.inner_width();
            self.tiles[i..i + w].fill(self.clear_tile);
        }
    }

    pub fn fill(&mut self, tile: Tile) {
        self.tiles.fill(tile);
    }

    /// Write a border to the terminal using a [BoxStyle]. By default this will
    /// set padding for the terminal - use [BoxStyle::dont_reset_padding] to
    /// prevent this.
    ///
    /// # Example
    /// ```
    /// use bevy_ascii_terminal::*;
    /// let mut term = Terminal::new([10,10]);
    /// term.put_border(BoxStyle::SINGLE_LINE);
    /// ```
    pub fn put_border(&mut self, box_style: BoxStyle) {
        let padding = self.padding;

        self.padding = Padding::ZERO;
        self.put_box([0, 0], self.size(), box_style);

        if box_style.reset_padding {
            let pad_left = box_style.top_left != ' '
                || box_style.center_left != ' '
                || box_style.bottom_left != ' ';
            let pad_right = box_style.top_right != ' '
                || box_style.center_right != ' '
                || box_style.bottom_right != ' ';
            let pad_top = box_style.top_left != ' '
                || box_style.top_center != ' '
                || box_style.top_right != ' ';
            let pad_bottom = box_style.bottom_left != ' '
                || box_style.bottom_center != ' '
                || box_style.bottom_right != ' ';

            self.padding = Padding {
                left: pad_left as usize,
                top: pad_top as usize,
                right: pad_right as usize,
                bottom: pad_bottom as usize,
            };
        } else {
            self.padding = padding;
        }
    }

    pub fn put_box(&mut self, xy: impl Into<IVec2>, size: impl Into<UVec2>, style: BoxStyle) {
        //let xy = xy.into().calculate(self.size);
        let xy = xy.into();

        let size = size.into().as_ivec2();

        let right = ivec2(size.x - 1, 0);
        let up = ivec2(0, size.y - 1);

        let fg = style.fg_color.map(|v| match v {
            ColorWrite::Clear => self.clear_tile.fg_color,
            ColorWrite::Set(col) => col,
        });
        let bg = style.bg_color.map(|v| match v {
            ColorWrite::Clear => self.clear_tile.bg_color,
            ColorWrite::Set(col) => col,
        });
        self.maybe_put(xy, Some(style.bottom_left), fg, bg);
        self.maybe_put(xy + up, Some(style.top_left), fg, bg);
        self.maybe_put(xy + right, Some(style.bottom_right), fg, bg);
        self.maybe_put(xy + right + up, Some(style.top_right), fg, bg);

        for i in 1..size.x - 1 {
            self.maybe_put([xy.x + i, xy.y], Some(style.bottom_center), fg, bg);
            self.maybe_put([xy.x + i, xy.y + up.y], Some(style.top_center), fg, bg);
        }

        for i in 1..size.y - 1 {
            self.maybe_put([xy.x, xy.y + i], Some(style.center_left), fg, bg);
            self.maybe_put([xy.x + right.x, xy.y + i], Some(style.center_right), fg, bg);
        }
    }

    /// Write a formatted string to the terminal.
    ///
    /// # Example
    /// ```
    /// use bevy_ascii_terminal::*;
    /// let mut terminal = Terminal::new([10, 10]);
    ///
    /// terminal.put_string([5, 5], "<fg=blue>Hello</fg>, <fg=#0000FF>World</fg>!");
    /// terminal.put_string([1, 1], "<bg=gray><fg=orange>Beep</fg> <fg=yellow>beep</fg>!");
    /// ```
    #[allow(deprecated)]
    pub fn put_string<T: AsRef<str>>(
        &mut self,
        xy: impl Into<PivotedPoint>,
        string: impl Into<TerminalString<T>>,
    ) {
        let pp = xy.into();
        let ts = string.into();

        let pivot = self.pivot;
        if let Some(p) = pp.pivot {
            self.pivot = p;
        }

        if ts.parse_tags {
            self.put_string_tagged(pp.point, ts);
        } else {
            self.put_string_untagged(pp.point, ts);
        }

        self.pivot = pivot;
    }

    pub fn put_string_tagged<T: AsRef<str>>(
        &mut self,
        xy: impl Into<IVec2>,
        string: impl Into<TerminalString<T>>,
    ) {
        // TODO: Error handling
        let origin = xy.into();

        let ts = string.into();
        let string = ts.string.as_ref();

        let pivot = self.pivot.normalized();

        let mut fg = self.clear_tile.fg_color;
        let mut bg = self.clear_tile.bg_color;

        let max_len = self.inner_width();

        // First line length may be different because of initial x offset
        let first_line_len = max_len - origin.x.unsigned_abs() as usize;

        // TODO: Error handling
        let (mut wrapped, mut char_count, mut rem) =
            wrap_tagged_string(string, first_line_len, ts.word_wrap).unwrap();

        // TODO: Error handling
        let mut xy = self.try_transform_point(origin).unwrap();

        // Count remaining lines after wrapping the first line since all
        // remaining line widths are the same
        // TODO: Error handling
        let lines = 1 + wrap_tagged_line_count(rem, max_len, ts.word_wrap).unwrap();
        let line_offset = (lines.saturating_sub(1) as f32 * (1.0 - pivot.y)) as i32;

        xy.y += line_offset;

        // Newline resets x position based on pivot
        let newline_x = self.try_transform_point(IVec2::ZERO).unwrap().x;

        while !wrapped.is_empty() || !rem.is_empty() {
            let x_offset = (pivot.x * char_count.saturating_sub(1) as f32).floor() as i32;
            xy.x -= x_offset;

            for token in TokenIterator::new(wrapped) {
                // TODO: Error handling
                let token = token.unwrap();
                match token {
                    Token::Text(text) => {
                        for ch in text.chars() {
                            let i = self.tile_to_index(xy);
                            let t = &mut self.tiles[i];
                            t.glyph = ch;
                            t.fg_color = fg;
                            t.bg_color = bg;

                            xy.x += 1;
                        }
                    }
                    Token::Space => {
                        let i = self.tile_to_index(xy);
                        let t = &mut self.tiles[i];
                        t.glyph = ' ';
                        t.fg_color = fg;
                        t.bg_color = bg;
                        xy.x += 1;
                    }
                    Token::Newline => {
                        xy.x = newline_x;
                        xy.y -= 1;
                        if xy.y < 0 {
                            return;
                        }
                    }
                    Token::FgStart(col, _) => fg = col,
                    Token::BgStart(col, _) => bg = col,
                    Token::FgEnd => fg = self.clear_tile.fg_color,
                    Token::BgEnd => bg = self.clear_tile.bg_color,
                    Token::Escaped(ch) => {
                        let i = self.tile_to_index(xy);
                        let t = &mut self.tiles[i];
                        t.glyph = ch;
                        t.fg_color = fg;
                        t.bg_color = bg;

                        xy.x += 1;
                    }
                }
            }

            // TODO: Error handling
            (wrapped, char_count, rem) = wrap_tagged_string(rem, max_len, ts.word_wrap).unwrap();
            xy.x = newline_x;
            xy.y -= 1;
            if xy.y < 0 {
                return;
            }
        }
    }

    fn put_string_untagged<T: AsRef<str>>(
        &mut self,
        xy: impl Into<IVec2>,
        string: impl Into<TerminalString<T>>,
    ) {
        let ts = string.into();
        let s = ts.string.as_ref();
        let pivot = self.pivot.normalized();
        let origin = xy.into();

        let fg = if let Some(fg) = ts.fg_color {
            Some(fg)
        } else if ts.clear_colors {
            Some(self.clear_tile.fg_color)
        } else {
            None
        };

        let bg = if let Some(bg) = ts.bg_color {
            Some(bg)
        } else if ts.clear_colors {
            Some(self.clear_tile.bg_color)
        } else {
            None
        };

        let max_len = self.inner_size().x as usize - self.padding.left - self.padding.right;

        // TODO: Handle negative xy for center pivot
        // First line length may be different because of initial x offset
        let first_line_len = max_len - origin.x as usize;
        let (mut wrapped, mut rem) = wrap_string(s, first_line_len, ts.word_wrap);

        // Count remaining lines after wrapping the first line since all
        // remaining line widths are the same
        let lines = 1 + wrap_line_count(rem, max_len, ts.word_wrap);
        let line_offset = (lines.saturating_sub(1) as f32 * (1.0 - pivot.y)) as i32;

        let mut xy = self.try_transform_point(origin).unwrap();
        xy.y += line_offset;

        // Newline resets x position based on pivot
        let newline_x = self.try_transform_point(IVec2::ZERO).unwrap().x;

        while !wrapped.is_empty() || !rem.is_empty() {
            let charcount = wrapped.chars().count().saturating_sub(1) as f32;

            let x_offset = (pivot.x * charcount).floor() as i32;
            xy.x -= x_offset;

            for ch in wrapped.chars() {
                let i = self.tile_to_index(xy);
                let t = &mut self.tiles[i];
                t.glyph = ch;

                if let Some(fg) = fg {
                    t.fg_color = fg;
                }

                if let Some(bg) = bg {
                    t.bg_color = bg;
                }

                xy.x += 1;
            }

            xy.x = newline_x;
            xy.y -= 1;
            if xy.y < 0 {
                return;
            }

            (wrapped, rem) = wrap_string(rem, max_len, ts.word_wrap);
        }
    }

    /// Read a number of characters from a given line, based on the current pivot.
    pub fn read_line(&self, line: i32, count: usize) -> impl Iterator<Item = char> + '_ {
        let mut p = self
            .try_transform_point([0, line])
            .expect("Index out of bounds");

        let xoff = (count - 1) as f32 * self.pivot.normalized().x;
        p.x -= xoff as i32;

        let i = self.tile_to_index(p);

        self.tiles[i..i + count].iter().map(|t| t.glyph)
    }

    /// Transform a local 2d tile index (bottom left origin) into it's corresponding
    /// 1d index into the terminal tile data. This doesn't account for padding
    /// or pivot - see [Terminal::transform_point] for that.
    #[inline]
    pub fn tile_to_index(&self, xy: impl Into<IVec2>) -> usize {
        let xy = xy.into();
        xy.y as usize * self.width() + xy.x as usize
    }

    /// Convert a 1d index into the terminal tile data into it's corresponding
    /// 2d tile index, ignoring padding.
    #[inline]
    pub fn index_to_tile(&self, i: usize) -> IVec2 {
        let w = self.width() as i32;
        IVec2::new(i as i32 % w, i as i32 / w)
    }

    /// Retrieve a tile from the inner area of the terminal, or None if the
    /// position is out of bounds.
    pub fn try_tile_mut(&mut self, xy: impl Into<IVec2>) -> Option<&mut Tile> {
        let xy = self.try_transform_point(xy.into())?;
        let i = self.tile_to_index(xy);
        Some(&mut self.tiles[i])
    }

    /// Retrieve a tile at the grid position. This will panic if the position is
    /// out of bounds.
    pub fn tile(&mut self, xy: impl Into<IVec2>) -> &Tile {
        let xy = xy.into();
        let xy = self.try_transform_point(xy).unwrap_or_else(|| {
            panic!(
                "Error accessing tile position {} in terminal sized {}",
                xy,
                self.size()
            );
        });
        let i = self.tile_to_index(xy);
        &self.tiles[i]
    }

    /// Retrieve a tile at the grid position. This will panic if the position is
    /// out of bounds.
    pub fn tile_mut(&mut self, xy: impl Into<IVec2>) -> &mut Tile {
        let xy = xy.into();
        let xy = self.try_transform_point(xy).unwrap_or_else(|| {
            panic!(
                "Error accessing tile position {} in terminal sized {}",
                xy,
                self.size()
            );
        });
        let i = self.tile_to_index(xy);
        &mut self.tiles[i]
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

    pub fn inner_size(&self) -> UVec2 {
        UVec2::new(self.inner_width() as u32, self.inner_height() as u32)
    }

    pub fn inner_width(&self) -> usize {
        let w = self.size.x as usize;
        w.saturating_sub(self.padding.left + self.padding.right)
    }

    pub fn inner_height(&self) -> usize {
        let h = self.size.y as usize;
        h.saturating_sub(self.padding.top + self.padding.bottom)
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

    #[allow(deprecated)]
    pub fn resize(&mut self, new_size: impl GridSize) {
        if new_size.to_uvec2() == self.size {
            return;
        }
        let new_size = new_size.to_uvec2().max(UVec2::new(2, 2));
        self.tiles = vec![self.clear_tile; new_size.tile_count()];
        self.size = new_size;
    }

    /// Transform a point from a bottom-left origin to the current pivot origin
    /// within the padded area of the terminal.
    pub fn try_transform_point(&self, xy: impl Into<IVec2>) -> Option<IVec2> {
        let innersize = self.inner_size();
        let mut xy = self.pivot.transform_point(xy, innersize);
        if xy.cmplt(IVec2::ZERO).any() {
            return None;
        }

        let padding_offset = IVec2::new(self.padding.left as i32, self.padding.bottom as i32);
        xy += padding_offset;
        if xy.cmpge(self.size.as_ivec2()).any() {
            return None;
        }

        Some(xy)
    }

    /// Transform a point from a bottom-left origin to it's corresponding tile index
    /// within the padded area of the terminal.
    pub fn try_transform_point_to_index(&self, xy: impl Into<IVec2>) -> Option<usize> {
        let innersize = self.inner_size();
        let mut xy = self.pivot.transform_point(xy, innersize);
        if xy.cmplt(IVec2::ZERO).any() {
            return None;
        }

        let padding_offset = IVec2::new(self.padding.left as i32, self.padding.bottom as i32);
        xy += padding_offset;
        if xy.cmpge(self.size.as_ivec2()).any() {
            return None;
        }
        println!("TRANSFORMED POINT {}", xy);
        Some(xy.y as usize * self.width() + xy.x as usize)
    }

    pub fn progress_bar(&mut self, xy: impl Into<IVec2>, progress_bar: ProgressBar) {
        let pb = &progress_bar;

        let mut xy = self
            .try_transform_point(xy)
            .expect("Progress bar position out of bounds");

        let x_offset = (self.pivot.normalized().x * (pb.len - 1) as f32).floor() as i32;
        xy.x -= x_offset;

        let bar_percent = pb.value.clamp(0.0, 1.0);
        let fill_col = pb.fill_color.unwrap_or(self.clear_tile.fg_color);
        let empty_col = pb.fill_color.unwrap_or(self.clear_tile.fg_color);

        for i in 0..pb.len {
            let t = i as f32 / pb.len as f32;
            let (c, col) = if t < bar_percent {
                (pb.fill_char, fill_col)
            } else {
                (pb.empty_char, empty_col)
            };
            let i = self.tile_to_index(xy);
            self.tiles[i].char(c).fg(col);
            xy.x += 1;
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
                    let y = layer.height - 1 - y;
                    let t = terminal.tile_mut([x as i32, y as i32]);
                    t.glyph = glyph;
                    t.fg_color = fg;
                    t.bg_color = bg;
                }
            }
        }
        Ok(terminal)
    }

    pub fn print_to_console(&self) {
        for y in (0..self.height()).rev() {
            for x in 0..self.width() {
                let i = self.tile_to_index([x as i32, y as i32]);
                let t = self.tiles[i];
                print!("{}", t.glyph);
            }
            println!();
        }
    }
}

pub struct ProgressBar {
    value: f32,
    pub len: usize,
    pub fill_char: char,
    pub empty_char: char,
    pub fill_color: Option<LinearRgba>,
    pub empty_color: Option<LinearRgba>,
}

impl ProgressBar {
    pub fn new(value: f32, len: usize, fill_char: char, empty_char: char) -> Self {
        Self {
            value,
            len,
            fill_char,
            empty_char,
            fill_color: None,
            empty_color: None,
        }
    }

    pub fn with_fill_color(mut self, col: impl Into<LinearRgba>) -> Self {
        self.fill_color = Some(col.into());
        self
    }

    pub fn with_empty_color(mut self, col: impl Into<LinearRgba>) -> Self {
        self.empty_color = Some(col.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::{BoxStyle, TerminalStringBuilder, Tile, terminal::ProgressBar};
    use crate::{Pivot, Terminal, ascii};

    #[test]
    fn put_string_negative() {
        let mut terminal = Terminal::new([10, 10])
            .with_clear_char('.')
            .with_pivot(Pivot::Center);
        terminal.put_string([-2, 2], "Hello");

        terminal.pivot = Pivot::LeftTop;
        assert_eq!(terminal.tile([1, 2]).glyph, 'H');
    }

    #[test]
    fn read_line() {
        let mut terminal = Terminal::new([20, 10]);
        terminal.put_string([2, 2], "Hello, World!");
        let line: String = terminal.read_line(2, 7).collect();
        assert_eq!(line, "  Hello");
    }

    #[test]
    #[ignore]
    fn big_string() {
        let mut term = Terminal::new([16, 16]);
        let string = String::from_iter(ascii::CP_437_ARRAY.iter());
        term.put_string([0, 0], string.dont_parse_tags());
        term.print_to_console();
    }

    #[test]
    #[ignore]
    fn progress() {
        let mut term = Terminal::new([30, 3])
            .with_clear_char('.')
            .with_pivot(Pivot::RightBottom);

        term.progress_bar([0, 0], ProgressBar::new(0.5, 15, '#', '-'));
        term.print_to_console();

        term.clear();

        term.progress_bar([0, 0], ProgressBar::new(1.0, 15, '#', '-'));
        term.print_to_console();
    }

    #[test]
    #[ignore]
    fn clear_inner() {
        let mut t = Terminal::new([20, 10]).with_border(BoxStyle::SINGLE_LINE);
        t.fill(Tile {
            glyph: '.',
            ..Default::default()
        });
        t.clear_inner();
        t.print_to_console();
    }
}
