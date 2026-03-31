//! A grid of tiles for rendering colorful ascii.
use std::ops::Sub;

use bevy::{
    color::{ColorToPacked, LinearRgba},
    math::{IVec2, UVec2, Vec2, ivec2},
    prelude::{Component, Mesh2d},
    reflect::Reflect,
    sprite_render::MeshMaterial2d,
};

use crate::{
    BoxStyle, Padding, Tile, ascii,
    render::{
        RebuildMeshVerts, TerminalFont, TerminalMaterial, TerminalMeshPivot, UvMappingHandle,
    },
    rexpaint::reader::XpFile,
    strings::{
        //GridStringIterator,
        TerminalString,
        Token,
        TokenIterator,
        wrap_line_count, //Token, next_token
        wrap_string,
        wrap_tagged_line_count,
        wrap_tagged_string,
    },
    transform::TerminalTransform,
};

#[derive(Default, Debug, Copy, Clone, Reflect)]
pub enum ColorWrite {
    /// Set a tile to the terminal's clear tile color.
    #[default]
    Clear,
    Set(LinearRgba),
}

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
    padding: Padding,
    pivot: Pivot,
}

impl Terminal {
    pub fn new(size: impl Into<UVec2>) -> Self {
        let size = size.into();
        Self {
            size,
            tiles: vec![Tile::default(); size.element_product() as usize],
            clear_tile: Tile::default(),
            padding: Padding::ZERO,
            pivot: Pivot::default(),
        }
    }

    /// Create a terminal from a REXPaint file. Note this writes all layers to the
    /// same terminal, so it won't preserve the transparent layering aspect of
    /// actual rexpaint files.
    pub fn from_rexpaint_file(file_path: impl AsRef<str>) -> std::io::Result<Self> {
        let mut file = std::fs::File::open(file_path.as_ref())?;
        let xp = XpFile::read(&mut file)?;
        let Some((w, h)) = xp.layers.first().map(|l| (l.width as i32, l.height as i32)) else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No layers found in REXPaint file",
            ));
        };
        let mut terminal = Self::new([w as u32, h as u32]).with_pivot(Pivot::LeftBottom);
        for layer in &xp.layers {
            for y in 0..layer.height as i32 {
                for x in 0..layer.width as i32 {
                    let cell = layer.get(x as usize, y as usize).unwrap();
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
        let mut terminal = Self::new([width as u32, height as u32]);
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

    /// Set the padding for the terminal.
    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Draw a border on the terminal.
    pub fn with_border(mut self, style: BoxStyle) -> Self {
        self.put_border(style);
        self
    }

    /// Set a title
    pub fn with_title(mut self, title: &str) -> Self {
        self.put_title(title);
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

    /// Set the clear colors
    pub fn with_clear_colors(mut self, fg: LinearRgba, bg: LinearRgba) -> Self {
        self.maybe_fill(None, Some(fg), Some(bg));
        self
    }

    /// A utility function to add a string to the terminal during creation.
    pub fn with_string<T: AsRef<str>>(
        mut self,
        xy: impl Into<IVec2>,
        string: impl Into<TerminalString<T>>,
    ) -> Self {
        self.put_string(xy, string);
        self
    }

    pub fn with_pivot(mut self, pivot: Pivot) -> Self {
        self.pivot = pivot;
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
    pub fn put_char(&mut self, xy: impl Into<IVec2>, ch: char) -> &mut Tile {
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
        xy: impl Into<IVec2>,
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
        xy: impl Into<IVec2>,
        color: impl Into<LinearRgba>,
    ) -> &mut Tile {
        self.tile_mut(xy).bg(color)
    }

    /// Insert a tile into the terminal.
    pub fn put_tile(&mut self, xy: impl Into<IVec2>, tile: Tile) -> &mut Tile {
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

    /// Write a string within the padded area of the terminal.
    ///
    /// You can modify the output with the [TerminalString] trait.
    pub fn put_string<T: AsRef<str>>(
        &mut self,
        xy: impl Into<IVec2>,
        string: impl Into<TerminalString<T>>,
    ) {
        let ts = string.into();
        if ts.parse_tags {
            self.put_string_tagged(xy, ts);
        } else {
            self.put_string_untagged(xy, ts);
        }
    }

    fn put_string_tagged<T: AsRef<str>>(
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

        let max_len = self.width() - self.padding.right - self.padding.left;

        // First line length may be different because of initial x offset
        let first_line_len = max_len - origin.x as usize;
        // TODO: Error handling
        let (mut wrapped, mut char_count, mut rem) =
            wrap_tagged_string(string, first_line_len, ts.word_wrap).unwrap();

        // Count remaining lines after wrapping the first line since all
        // remaining line widths are the same
        // TODO: Error handling
        let lines = 1 + wrap_tagged_line_count(rem, max_len, ts.word_wrap).unwrap();
        let line_offset = (lines.saturating_sub(1) as f32 * (1.0 - pivot.y)) as i32;

        // TODO: Error handling
        let mut xy = self.transform_point(origin).unwrap();
        xy.y += line_offset;

        // Newline resets x position based on pivot
        let newline_x = self.transform_point(IVec2::ZERO).unwrap().x;

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

        // TODO: Handle TerminalString color overrides
        let fg = self.clear_tile.fg_color;
        let bg = self.clear_tile.bg_color;

        let max_len = self.width() - self.padding.left - self.padding.right;

        // TODO: Handle negative xy for center pivot
        // First line length may be different because of initial x offset
        let first_line_len = max_len - origin.x as usize;
        let (mut wrapped, mut rem) = wrap_string(s, first_line_len, ts.word_wrap);

        // Count remaining lines after wrapping the first line since all
        // remaining line widths are the same
        let lines = 1 + wrap_line_count(rem, max_len, ts.word_wrap);
        let line_offset = (lines.saturating_sub(1) as f32 * (1.0 - pivot.y)) as i32;

        let mut xy = self.transform_point(origin).unwrap();
        xy.y += line_offset;

        // Newline resets x position based on pivot
        let newline_x = self.transform_point(IVec2::ZERO).unwrap().x;

        while !wrapped.is_empty() || !rem.is_empty() {
            let charcount = wrapped.chars().count().saturating_sub(1) as f32;

            let x_offset = (pivot.x * charcount).floor() as i32;
            xy.x -= x_offset;

            for ch in wrapped.chars() {
                let i = self.tile_to_index(xy);
                let t = &mut self.tiles[i];
                t.glyph = ch;
                t.fg_color = fg;
                t.bg_color = bg;

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

    /// Write a border to the terminal. By default this will set padding for the terminal
    /// - use [BoxStyle::dont_reset_padding] to prevent this.
    pub fn put_border(&mut self, style: BoxStyle) {
        let padding = self.padding;

        self.padding = Padding::ZERO;
        self.put_box([0, 0], self.size(), style);

        if style.reset_padding {
            let pad_left =
                style.top_left != ' ' || style.center_left != ' ' || style.bottom_left != ' ';
            let pad_right =
                style.top_right != ' ' || style.center_right != ' ' || style.bottom_right != ' ';
            let pad_top =
                style.top_left != ' ' || style.top_center != ' ' || style.top_right != ' ';
            let pad_bottom =
                style.bottom_left != ' ' || style.bottom_center != ' ' || style.bottom_right != ' ';

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

    /// Sets a title for the terminal, writing it to the top row. Leading
    /// spaces will offset the title.
    pub fn put_title(&mut self, title: &str) {
        let padding = self.padding;
        self.padding = Padding::ZERO;

        let offset = title.find(|c| c != ' ').unwrap_or(0);
        let title = title.trim_start();
        self.put_string([offset as i32, 0], title);

        self.padding = padding;
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

    /// Read a number of characters from the terminal.
    pub fn read_chars(
        &self,
        xy: impl Into<IVec2>,
        count: usize,
    ) -> impl Iterator<Item = char> + '_ {
        let xy = self.transform_point(xy.into()).unwrap();

        self.tiles
            .chunks(self.width())
            .rev()
            .skip(self.height() - 1 - xy.y as usize)
            .flat_map(|c| c.iter())
            .skip(xy.x as usize)
            .map(|t| t.glyph)
            .take(count)
    }

    pub fn read_to_string(&self, xy: impl Into<IVec2>, count: usize) -> String {
        String::from_iter(self.read_chars(xy, count))
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
    /// 2d tile index.
    #[inline]
    pub fn index_to_tile(&self, i: usize) -> IVec2 {
        let w = self.width() as i32;
        IVec2::new(i as i32 % w, i as i32 / w)
    }

    /// Retrieve a tile at the grid position. This will panic if the position is
    /// out of bounds.
    pub fn tile_mut(&mut self, xy: impl Into<IVec2>) -> &mut Tile {
        let xy = self.transform_point(xy.into()).unwrap();
        debug_assert!(
            xy.cmplt(self.size.as_ivec2()).all(),
            "Attempting to access a tile at an out of bounds grid position {:?} 
        from a terminal of size {}",
            xy,
            self.size
        );
        let i = self.tile_to_index(xy);
        &mut self.tiles[i]
    }

    /// Retrieve a tile at the grid position from within the padded
    /// area of the terminal. This will panic if the position is
    /// out of bounds.
    pub fn tile(&self, xy: impl Into<IVec2>) -> &Tile {
        let xy = self.transform_point(xy.into()).unwrap();

        debug_assert!(
            //self.size.contains_point(xy.calculate(self.size)),
            xy.cmpge(IVec2::ZERO).all() && xy.cmplt(self.size.as_ivec2()).all(),
            "Attempting to access a tile at an out of bounds grid position {:?}
        from a terminal of size {}",
            xy,
            self.size
        );
        let i = self.tile_to_index(xy);
        &self.tiles[i]
    }

    /// Apply padding the edges of the terminal, this will cause get/set
    /// functions to be adjusted according to the padding.
    pub fn set_padding(&mut self, padding: Padding) {
        self.padding = padding;
    }

    pub fn set_pivot(&mut self, pivot: Pivot) {
        self.pivot = pivot;
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

    /// Iterate over a rectangular section of terminal tiles - bottom left origin.
    pub fn iter_rect(
        &self,
        pos: impl Into<IVec2>,
        size: impl Into<UVec2>,
    ) -> impl DoubleEndedIterator<Item = &Tile> {
        let pos = pos.into();
        let size = size.into().as_ivec2();

        let left = pos.x;
        let bottom = pos.y;
        let right = pos.x + size.x - 1;

        self.tiles
            .chunks(self.width())
            .skip(bottom as usize)
            .flat_map(move |tiles| tiles[left as usize..=right as usize].iter())
    }

    /// Iterate over a rectangular section of terminal tiles - bottom left origin.
    pub fn iter_rect_mut(
        &mut self,
        pos: impl Into<IVec2>,
        size: impl Into<UVec2>,
    ) -> impl DoubleEndedIterator<Item = &mut Tile> {
        let pos = pos.into();
        let size = size.into().as_ivec2();

        let left = pos.x;
        let bottom = pos.y;
        let right = pos.x + size.x - 1;
        let w = self.width();
        self.tiles
            .chunks_mut(w)
            .skip(bottom as usize)
            .flat_map(move |tiles| tiles[left as usize..=right as usize].iter_mut())
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

    /// Iterate through the terminal tiles, starting from the bottom left, moving
    /// right then up
    pub fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Tile> {
        self.tiles.iter_mut()
    }

    // /// The local grid bounds of the terminal. For world bounds see [TerminalTransform].
    // pub fn bounds(&self) -> GridRect {
    //     GridRect::new([0, 0], self.size)
    // }

    pub fn clear_tile(&self) -> Tile {
        self.clear_tile
    }

    pub fn resize(&mut self, new_size: impl Into<UVec2>) {
        let new_size = new_size.into().max(UVec2::new(2, 2));
        self.tiles = vec![self.clear_tile; new_size.element_product() as usize];
        self.size = new_size;
    }

    pub fn print_to_console(self) {
        for y in (0..self.height()).rev() {
            for x in 0..self.width() {
                let t = &self.tiles[y * self.width() + x];
                print!("{}", t.glyph);
            }
            println!();
        }
    }

    /// Transform a point from a bottom-left origin to the current pivot origin
    /// within the padded area of the terminal.
    fn transform_point(&self, xy: impl Into<IVec2>) -> Option<IVec2> {
        let padding_offset = IVec2::new(self.padding.left as i32, self.padding.bottom as i32);
        let xy = xy.into();

        let inner = UVec2::new(
            self.size
                .x
                .saturating_sub((self.padding.left + self.padding.right) as u32),
            self.size
                .y
                .saturating_sub((self.padding.top + self.padding.bottom) as u32),
        )
        .as_ivec2();

        let max = IVec2::new(
            (self.width() - 1 - self.padding.right) as i32,
            (self.height() - 1 - self.padding.top) as i32,
        );

        let xy = padding_offset + self.pivot.transform_point(xy, inner);
        if xy.cmplt(IVec2::ZERO).any() || xy.cmpgt(max).any() {
            return None;
        }

        Some(xy)
    }
}

#[derive(Debug, Default, Reflect, Clone, Copy, PartialEq, Eq)]
pub enum Pivot {
    LeftBottom, // X right, Y up
    LeftCenter, // X right, Y up
    #[default]
    LeftTop, // X right, Y down
    CenterTop,  // X right, Y down,
    Center,     // X right, Y up
    CenterBottom, // X right, Y up
    RightTop,   // X left, Y down
    RightCenter, // X left, Y up
    RightBottom, // X left, Y up
}

impl Pivot {
    pub fn axis(&self) -> IVec2 {
        IVec2::from(match self {
            Pivot::LeftBottom => [1, 1],
            Pivot::LeftCenter => [1, 1],
            Pivot::LeftTop => [1, -1],
            Pivot::CenterTop => [1, -1],
            Pivot::Center => [1, 1],
            Pivot::CenterBottom => [1, 1],
            Pivot::RightTop => [-1, -1],
            Pivot::RightBottom => [-1, 1],
            Pivot::RightCenter => [-1, 1],
        })
    }

    /// Calculate the position of a pivot on a sized grid.
    pub fn pivot_position(&self, grid_size: impl Into<IVec2>) -> IVec2 {
        (grid_size.into().as_vec2().sub(1.0) * self.normalized())
            .round()
            .as_ivec2()
    }

    pub fn normalized(&self) -> Vec2 {
        match self {
            Pivot::LeftTop => Vec2::new(0.0, 1.0),
            Pivot::LeftBottom => Vec2::new(0.0, 0.0),
            Pivot::LeftCenter => Vec2::new(0.0, 0.5),
            Pivot::RightTop => Vec2::new(1.0, 1.0),
            Pivot::RightCenter => Vec2::new(1.0, 0.5),
            Pivot::RightBottom => Vec2::new(1.0, 0.0),
            Pivot::Center => Vec2::new(0.5, 0.5),
            Pivot::CenterTop => Vec2::new(0.5, 1.0),
            Pivot::CenterBottom => Vec2::new(0.5, 0.0),
        }
    }

    /// Transform a point into the pivot's coordinate space.
    pub fn transform_coordinates(&self, grid_point: impl Into<IVec2>) -> IVec2 {
        grid_point.into() * self.axis()
    }

    /// Transform a point from a bottom-left origin to the pivot origin
    pub fn transform_point(&self, point: impl Into<IVec2>, grid_size: impl Into<IVec2>) -> IVec2 {
        self.pivot_position(grid_size) + self.transform_coordinates(point)
    }
}

/// A grid point that may optionally have a pivot applied to it.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PivotedPoint {
    pub point: IVec2,
    pub pivot: Option<Pivot>,
}

impl PivotedPoint {
    pub fn new(xy: impl Into<IVec2>, pivot: Pivot) -> Self {
        Self {
            point: xy.into(),
            pivot: Some(pivot),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Terminal, TerminalStringBuilder, ascii};

    #[test]
    fn printing() {
        let mut term = Terminal::new([10, 6]).with_pivot(Pivot::LeftTop);
        term.put_string([0, 0], "Hello");

        term.put_box([0, 1], [5, 4], BoxStyle::DOUBLE_LINE);

        term.print_to_console();
    }

    #[test]
    fn put_string_negative() {
        let mut terminal = Terminal::new([10, 10]).with_pivot(Pivot::Center);
        terminal.put_string([-2, -2], "Hello");
        assert_eq!(terminal.tile([-2, -2]).glyph, 'H');
    }

    #[test]
    fn big_string() {
        let mut term = Terminal::new([16, 16]);
        let string = String::from_iter(ascii::CP_437_ARRAY.iter());
        term.put_string([0, 0], string);
    }

    #[test]
    fn tokenstest() {
        let mut term = Terminal::new([15, 3]);
        term.put_border(BoxStyle::SINGLE_LINE);
        term.put_string([0, 0], "<bg=gray><fg=blue>Hello</fg> <fg=red>World</fg>!");
        assert_eq!("Hello World!", term.read_to_string([0, 0], 12).as_str());
    }

    const INPUT: &str =
        "HELLO WORLD!\nHow the heck are you doing today?\nI'm good.\nHow about you?\nCOOL BEANS";

    #[test]
    fn put_string_left_bottom() {
        let mut term = Terminal::new([35, 10]).with_pivot(Pivot::LeftBottom);
        term.put_border(BoxStyle::SINGLE_LINE);

        term.put_string([0, 0], INPUT);
        term.print_to_console();
        println!()
    }

    #[test]
    fn put_string_left_top() {
        let mut term = Terminal::new([35, 10]);
        term.put_border(BoxStyle::SINGLE_LINE);

        term.pivot = Pivot::LeftTop;
        term.put_string([0, 0], INPUT);
        term.print_to_console();
        println!()
    }

    #[test]
    fn put_string_center() {
        let mut term = Terminal::new([35, 10]).with_pivot(Pivot::Center);
        term.put_border(BoxStyle::SINGLE_LINE);

        term.put_string([0, 0], INPUT);
        term.print_to_console();
        println!()
    }

    #[test]
    fn put_string_right_top() {
        let mut term = Terminal::new([35, 10]);
        term.put_border(BoxStyle::SINGLE_LINE);

        term.pivot = Pivot::RightTop;
        term.put_string([0, 0], INPUT);
        term.print_to_console();
        println!()
    }

    #[test]
    fn put_string_right_bottom() {
        let mut term = Terminal::new([35, 10]).with_pivot(Pivot::RightBottom);
        term.put_border(BoxStyle::SINGLE_LINE);

        term.put_string([0, 0], INPUT);
        term.print_to_console();
        println!()
    }

    #[test]
    fn put_with_no_padding_and_no_word_wrap_wraps_correctly() {
        let mut term = Terminal::new([10, 5]).with_pivot(Pivot::LeftTop);
        let string = "Hello, how are";

        term.put_string([1, 1], string.dont_word_wrap());
        assert_eq!(string, term.read_to_string([1, 1], string.len()).as_str());
    }

    #[test]
    fn put_with_no_padding_and_word_wrap_wraps_correctly() {
        let mut term = Terminal::new([10, 5])
            .with_pivot(Pivot::LeftTop)
            .with_clear_char('.');
        let string = "Hello, how are";

        term.put_string([1, 1], string);
        assert_eq!("Hello,", term.read_to_string([1, 1], 6).as_str());
        assert_eq!("how are", term.read_to_string([0, 2], 7).as_str());
    }

    #[test]
    fn put_string_without_border() {
        let mut term = Terminal::new([15, 5]).with_clear_char('.');
        term.put_string([0, 1], "Hello. How are you doing?".dont_word_wrap());

        assert_eq!("Hello. How are", term.read_to_string([0, 2], 14));
        assert_eq!("you doing?", term.read_to_string([0, 1], 10));
    }
}
