use bevy::{
    color::{Color, LinearRgba},
    ecs::component::Component,
    math::{IVec2, UVec2},
    prelude::{Deref, DerefMut},
};

use crate::{
    border::TerminalBorder,
    string::{DecoratedFormattedText, StringIter},
    tile::{FormattedTile, TileFormatter},
    GridPoint, GridRect, GridSize, Pivot, PivotedPoint, Tile,
};

/// A terminal represented as a 2d grid of [Tile]s.
#[derive(Debug, Default, Clone, Component)]
pub struct Terminal {
    tiles: Vec<Tile>,
    size: UVec2,
    /// This tile is used by various functions to represent an "empty" terminal tile.
    clear_tile: Tile,
}

impl Terminal {
    pub fn new(size: impl GridSize) -> Self {
        Self {
            tiles: vec![Tile::DEFAULT; size.tile_count()],
            size: size.as_uvec2(),
            ..Default::default()
        }
    }

    pub fn with_clear_tile(size: impl GridSize, clear_tile: Tile) -> Self {
        Self {
            tiles: vec![clear_tile; size.tile_count()],
            size: size.as_uvec2(),
            clear_tile,
        }
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn width(&self) -> usize {
        self.size.x as usize
    }

    pub fn height(&self) -> usize {
        self.size.y as usize
    }

    pub fn tile_count(&self) -> usize {
        self.tiles.len()
    }

    /// A referebce to a single tile of the [Terminal].
    pub fn tile(&self, xy: impl Into<PivotedPoint>) -> &Tile {
        let xy: IVec2 = xy.into().calc_from_size(self.size());
        let i = xy.as_index(self.size);
        &self.tiles[i]
    }

    /// A mutable reference to a single tile of the [Terminal].
    #[inline]
    pub fn tile_mut(&mut self, xy: impl Into<PivotedPoint>) -> &mut Tile {
        let xy: IVec2 = xy.into().calc_from_size(self.size());
        let i = xy.as_index(self.size);
        &mut self.tiles[i]
    }

    /// Insert a character at the given position.
    ///
    /// # Example:
    /// ```
    /// let mut term = Terminal::new([10,5]);
    /// // Insert at the bottom left corner.
    /// term.put_char([0,0], 'a');
    /// // Insert at the top right corner.
    /// term.put_char([0,0].pivot(Pivot::TopRight), 'b');
    /// // Insert at the bottom left corner.
    /// term.put_char([0,0], 'a'.fg_color(css));
    /// ```
    pub fn put_char(&mut self, xy: impl Into<PivotedPoint>, ch: char) {
        self.tile_mut(xy).glyph = ch;
    }

    pub fn put_fg_color(&mut self, xy: impl Into<PivotedPoint>, color: impl Into<LinearRgba>) {
        self.tile_mut(xy).fg_color = color.into();
    }

    pub fn put_bg_color(&mut self, xy: impl Into<PivotedPoint>, color: impl Into<LinearRgba>) {
        self.tile_mut(xy).bg_color = color.into();
    }

    pub fn put_tile(&mut self, xy: impl Into<PivotedPoint>, tile: Tile) {
        *self.tile_mut(xy) = tile;
    }

    /// Apply selective formatting to a single tile. This can be used to set multiple
    /// properties of a tile at once.
    ///
    /// # Example:
    /// ```
    /// use bevy_ascii_terminal::*;
    /// use bevy::{prelude::*, color::palettes::css};
    ///
    /// let mut term = Terminal::new([5,5]);
    /// // Write a yellow smiley face without affecting the background color.
    /// term.format_tile([1,1], '☺'.fg_color(css::YELLOW.into()));
    /// // Set the background color of the smiley to blue
    /// term.format_tile([1,1], css::BLUE.into().bg());
    /// ```
    pub fn format_tile(&mut self, xy: impl Into<PivotedPoint>, fmt: impl Into<FormattedTile>) {
        let tile = self.tile_mut(xy);
        let fmt = fmt.into();
        if let Some(glyph) = fmt.glyph {
            tile.glyph = glyph;
        }
        if let Some(fg) = fmt.fg_color {
            tile.fg_color = fg;
        }
        if let Some(bg) = fmt.bg_color {
            tile.bg_color = bg;
        }
    }

    /// Write a string to the terminal.
    ///
    /// The [StringFormatter] trait can be used to customize the string before
    /// it gets written to the terminal. You can set a foreground or background
    /// color, prevent word wrapping, or prevent writes on empty characters.
    ///
    /// Note that strings are by default justified to the top left of the terminal.
    /// You can manually set the pivot to override this behavior.
    ///
    /// ```
    /// use bevy_ascii_terminal::*;
    /// use bevy::color::palettes::css;
    ///
    /// let mut term = Terminal::new([13,10]);
    /// // Note that the foreground color of the empty space character is still
    /// // modified, since 'ignore_spaces' was not used.
    /// term.put_string([0,0], "Hello joe".fg(Color::BLUE));
    /// let string = "A looooooooooong string".bg(Color::GREEN.into()).no_word_wrap().ignore_spaces();
    /// term.put_string([0,1].pivot(Pivot::BottomLeft), string);
    /// term.put_string([0,4].pivot(Pivot::Center), "A string\nOver multiple\nlines.");
    /// ```
    pub fn put_string<T: AsRef<str>>(
        &mut self,
        xy: impl Into<PivotedPoint>,
        string: impl Into<DecoratedFormattedText<T>>,
    ) {
        let text: DecoratedFormattedText<T> = string.into();
        let ignore_spaces = text.formatting.ignore_spaces;
        let fg = text.decoration.fg_color;
        let bg = text.decoration.bg_color;
        let clear_colors = text.decoration.clear_colors;
        let wrapped = text.formatting.word_wrap;
        let bounds = self.bounds();
        let clear_tile = self.clear_tile;
        for (xy, ch) in StringIter::new(xy, text.string.as_ref(), bounds, wrapped) {
            if ignore_spaces && ch == ' ' {
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
            if clear_colors {
                tile.fg_color = clear_tile.fg_color;
                tile.bg_color = clear_tile.bg_color;
            }
        }
    }

    /// Set every tile in the terminal to it's [Terminal::clear_tile].
    pub fn clear(&mut self) {
        self.tiles.fill(self.clear_tile)
    }

    /// Change the terminal's `clear_tile`. This will not clear the terminal.
    ///
    /// This tile is used by various functions to represent an "empty" terminal tile.
    pub fn set_clear_tile(&mut self, clear_tile: Tile) {
        self.clear_tile = clear_tile;
    }

    /// Get the terminal's current clear tile.
    ///
    /// This tile is used by various functions to represent an "empty" terminal tile.
    pub fn clear_tile(&self) -> Tile {
        self.clear_tile
    }

    /// Resize the terminal. This will clear the terminal.
    pub fn resize(&mut self, size: impl GridSize) {
        debug_assert!(
            size.as_ivec2().cmpge(IVec2::ONE).all(),
            "Attempting to set terminal size to a value below 1"
        );
        self.size = size.as_uvec2();
        self.tiles.resize(size.tile_count(), Default::default());
        self.clear();
        self.tiles = vec![self.clear_tile; size.tile_count()];
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
            .map(|(i, t)| (self.index_to_xy(i), t))
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

    /// Set the terminal border
    pub fn put_border(&mut self, border: TerminalBorder) {
        //self.draw_box(self.bounds(), border);
    }

    // pub fn draw_box(&mut self, bounds: GridRect, border: Border) {
    //     let mut chars = border.edge_glyphs.iter().cloned();
    //     let mut tile = self.clear_tile;
    //     if let Some(ch) = chars.next().filter(|ch| *ch != ' ') {
    //         self.put_tile(bounds.top_left(), *tile.glyph(ch));
    //     }
    //     if let Some(ch) = chars.next().filter(|ch| *ch != ' ') {
    //         for t in self
    //             .iter_row_mut(bounds.top_index())
    //             .skip(1)
    //             .take(bounds.width() - 2)
    //         {
    //             *t = *tile.glyph(ch);
    //         }
    //     }
    //     if let Some(ch) = chars.next().filter(|ch| *ch != ' ') {
    //         self.put_tile(bounds.top_right(), *tile.glyph(ch));
    //     }
    //     if let Some(ch) = chars.next().filter(|ch| *ch != ' ') {
    //         for t in self
    //             .iter_column_mut(bounds.left_index())
    //             .skip(1)
    //             .take(bounds.height() - 2)
    //         {
    //             *t = *tile.glyph(ch);
    //         }
    //     }
    //     if let Some(ch) = chars.next().filter(|ch| *ch != ' ') {
    //         for t in self
    //             .iter_column_mut(bounds.right_index())
    //             .skip(1)
    //             .take(bounds.height() - 2)
    //         {
    //             *t = *tile.glyph(ch);
    //         }
    //     }
    //     if let Some(ch) = chars.next().filter(|ch| *ch != ' ') {
    //         self.put_tile(bounds.bottom_left(), *tile.glyph(ch));
    //     }
    //     if let Some(ch) = chars.next().filter(|ch| *ch != ' ') {
    //         for t in self
    //             .iter_row_mut(bounds.bottom_index())
    //             .skip(1)
    //             .take(bounds.width() - 2)
    //         {
    //             *t = *tile.glyph(ch);
    //         }
    //     }
    //     if let Some(ch) = chars.next().filter(|ch| *ch != ' ') {
    //         self.put_tile(bounds.bottom_right(), *tile.glyph(ch));
    //     }
    // }

    // pub fn draw_filled_box(
    //     &mut self,
    //     bounds: GridRect,
    //     border: Border,
    //     fill: impl Into<FormattedTile>,
    // ) {
    //     self.draw_box(bounds, border);
    // }

    /// Write a title string to the terminal with some optional formatting.
    ///
    /// # Example:
    /// ```
    /// use bevy_ascii_terminal::*;
    /// use bevy::color::palettes::css;
    /// let mut term = Terminal::new([20,5]);
    /// term.put_title(" The Terminal ".fg(css::BLUE.into()).delimiters("[]"));
    /// ```
    pub fn put_title<T: AsRef<str>>(&mut self, string: impl AsRef<str>) {
        // let title: TitleString<T> = string.into();
        // let string = title.string.as_ref();
        // let mut x = 1;
        // if let Some(delimiters) = title.delimiters.as_ref() {
        //     let mut t = self.clear_tile;
        //     let mut delimiters = delimiters.as_ref().chars();
        //     if let Some(c) = delimiters.next() {
        //         self.format_tile([1, 0].pivot(Pivot::TopLeft), *t.glyph(c));
        //         x += 1;
        //     }
        //     if let Some(c) = delimiters.next() {
        //         self.format_tile(
        //             [2 + string.chars().count(), 0].pivot(Pivot::TopLeft),
        //             *t.glyph(c),
        //         );
        //     }
        // }
        // self.put_string([x, 0], DecoratedText::from(title));
    }

    /// The local grid bounds of the terminal.
    ///
    /// For world space sizes and positions you can use the [crate::TerminalTransform].
    pub fn bounds(&self) -> GridRect {
        GridRect::new([0, 0], self.size())
    }

    /// Transform a 2d grid position to it's corresponding 1d index.
    pub fn xy_to_index(&self, xy: IVec2) -> usize {
        xy.as_index(self.size)
    }

    /// Transform a 1d index to it's corresponding 2d grid position.
    pub fn index_to_xy(&self, i: usize) -> IVec2 {
        let x = (i % self.width()) as i32;
        let y = (i / self.width()) as i32;
        IVec2::new(x, y)
    }
}

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct TerminalSize(pub UVec2);

// #[derive(Debug, Clone)]
// pub struct Border {
//     pub edge_glyphs: [char; 8],
// }

// impl Default for Border {
//     fn default() -> Self {
//         Self {
//             edge_glyphs: [' '; 8],
//         }
//     }
// }

// impl Border {
//     pub fn from_string(string: impl AsRef<str>) -> Self {
//         let mut glyphs = [' '; 8];
//         for (ch, glyph) in string.as_ref().chars().zip(glyphs.iter_mut()) {
//             *glyph = ch;
//         }
//         Self {
//             edge_glyphs: glyphs,
//         }
//     }

//     pub fn single_line() -> Self {
//         Self::from_string("┌─┐││└─┘")
//     }

//     pub fn double_line() -> Self {
//         Self::from_string("╔═╗║║╚═╝")
//     }
// }

#[cfg(test)]
mod tests {

    use bevy::color::palettes::basic;

    use crate::{
        string::{StringDecorator, StringFormatter},
        GridPoint, GridRect, GridSize, Pivot,
    };

    use super::Terminal;

    #[test]
    fn iter_rect_mut() {
        let mut term = Terminal::new([10, 10]);
        let rect = GridRect::from_points([7, 7], [9, 9]);
        let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
        for (ch, t) in chars.iter().zip(term.iter_rect_mut(rect)) {
            t.glyph = *ch;
        }

        assert_eq!('a', term.tile([7, 7]).glyph);
        assert_eq!('i', term.tile([9, 9]).glyph);
    }

    #[allow(dead_code)]
    fn non_static_string_write(string: &str) {
        let mut term = Terminal::new([10, 10]);
        term.put_string([0, 0], string);
    }

    #[test]
    fn string() {
        let mut term = Terminal::new([15, 15]);
        let string = "Hello".word_wrap(false).bg(basic::BLUE);
        term.put_string([1, 1].pivot(Pivot::TopLeft), string);

        term.put_string(
            [1, 1].pivot(Pivot::TopLeft),
            "hi".word_wrap(false).fg(basic::RED),
        );

        term.put_string([1, 1], "Hello");

        let allocated_string = "Hello".to_string();
        term.put_string([1, 1], &allocated_string);
    }

    fn title() {
        let mut term = Terminal::new([10, 10]);
        // term.put_title("Hello");
        // let string = String::from("Hi");
        // term.put_title(&string);
        // term.put_string([0, 0], &string);
    }

    #[test]
    fn border() {
        // let mut term = Terminal::new([15, 15]);
        // term.put_border(Border::single_line())
        //     .put_title("Hello".fg(basic::BLUE.into()));
        // for (_, t) in term.border().iter() {
        //     println!("{}", t.glyph);
        // }
    }
}
