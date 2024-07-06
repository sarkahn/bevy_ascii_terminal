use bevy::{color::Color, ecs::component::Component, math::IVec2};

use crate::{
    border::{Border, TerminalBorderMut},
    string::StringIter,
    GridPoint, GridRect, PivotedPoint, StringFormatter, Tile,
};

#[derive(Debug, Default, Clone, Component)]
pub struct Terminal {
    tiles: Vec<Tile>,
    size: IVec2,
    border: Option<Border>,
    clear_tile: Tile,
}

impl Terminal {
    pub fn new(size: impl GridPoint) -> Self {
        Self {
            tiles: vec![Tile::DEFAULT; size.len()],
            size: size.as_ivec2(),
            border: None,
            clear_tile: Tile::DEFAULT,
        }
    }

    pub fn with_clear_tile(size: impl GridPoint, clear_tile: Tile) -> Self {
        Self {
            tiles: vec![clear_tile; size.len()],
            size: size.as_ivec2(),
            border: None,
            clear_tile,
        }
    }

    pub fn size(&self) -> IVec2 {
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

    pub fn tile(&self, xy: impl Into<PivotedPoint>) -> &Tile {
        let xy: IVec2 = xy.into().calc_from_size(self.size());
        let i = xy.as_index(self.width());
        &self.tiles[i]
    }

    #[inline]
    pub fn tile_mut(&mut self, xy: impl Into<PivotedPoint>) -> &mut Tile {
        let xy: IVec2 = xy.into().calc_from_size(self.size());
        let i = xy.as_index(self.width());
        &mut self.tiles[i]
    }

    /// Insert a character at the given position.
    ///
    /// A pivot can be applied to the position, changing how the coordinates are
    /// interpreted.
    ///
    /// # Example:
    /// ```
    /// let mut term = Terminal::new([10,5]);
    /// // Insert at the bottom left corner.
    /// term.put_char([0,0], 'a');
    /// // Insert at the top right corner.
    /// term.put_char([0,0].pivot(Pivot::TopRight), 'b');
    /// ```
    pub fn put_char(&mut self, xy: impl Into<PivotedPoint>, ch: char) -> &mut Tile {
        self.tile_mut(xy).glyph(ch)
    }

    pub fn put_fg_color(&mut self, xy: impl Into<PivotedPoint>, color: Color) -> &mut Tile {
        self.tile_mut(xy).fg(color)
    }

    pub fn put_bg_color(&mut self, xy: impl Into<PivotedPoint>, color: Color) -> &mut Tile {
        self.tile_mut(xy).bg(color)
    }

    /// Write a string to the terminal.
    ///
    /// The [StringFormatter] trait can be used to customize the string before
    /// it gets written to the terminal. You can set a foreground or background
    /// color, prevent word wrapping, or prevent writes on empty characters.
    ///
    /// Note that unlike [Terminal::put_char], strings are by default justified
    /// to the top left of the terminal. You can manually set the pivot to
    /// override this behavior.
    ///
    /// ```
    /// use bevy_ascii_terminal::*;
    ///
    /// let mut term = Terminal::new([13,10]);
    /// // Note that the foreground color of the empty space character is still
    /// // modified, since 'ignore_spaces' was not used.
    /// term.put_string([0,0], "Hello joe".fg(Color::BLUE));
    /// let string = "A looooooooooong string".bg(Color::GREEN).no_word_wrap().ignore_spaces();
    /// term.put_string([0,1].pivot(Pivot::BottomLeft), string);
    /// term.put_string([0,4].pivot(Pivot::Center), "A string\nOver multiple\nlines.");
    /// ```
    pub fn put_string<'a>(
        &mut self,
        xy: impl Into<PivotedPoint>,
        string: impl StringFormatter<'a>,
    ) {
        let fmt = string.formatting();
        let ignore_spaces = fmt.ignore_spaces;
        let fg = fmt.fg_color;
        let bg = fmt.bg_color;
        for (xy, ch) in StringIter::new(xy, string.string(), self.bounds(), fmt.word_wrapped) {
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
        }
    }

    /// Set every tile in the terminal to it's `clear_tile`.
    pub fn clear(&mut self) {
        self.tiles.fill(self.clear_tile)
    }

    /// Change the terminal's `clear_tile`. This will not clear the terminal.
    pub fn set_clear_tile(&mut self, clear_tile: Tile) {
        self.clear_tile = clear_tile;
    }

    /// Get the terminal's current clear tile.
    pub fn clear_tile(&self) -> Tile {
        self.clear_tile
    }

    /// Resize the terminal. This will clear the terminal.
    pub fn resize(&mut self, size: impl GridPoint) {
        debug_assert!(
            size.as_ivec2().cmpge(IVec2::ONE).all(),
            "Attempting to set terminal size to a value below 1"
        );
        self.size = size.as_ivec2();
        self.tiles.resize(size.len(), Default::default());
        self.clear();
        self.tiles = vec![self.clear_tile; size.len()];
        if let Some(mut border) = self.get_border_mut() {
            border.clear();
        }
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
    pub fn put_border(&mut self, border: Border) -> TerminalBorderMut {
        self.set_border(Some(border));
        self.border_mut()
    }

    /// Set the terminal border. If `None` is passed then the border will be
    /// removed.
    pub fn set_border(&mut self, border: Option<Border>) {
        if let Some(border) = border {
            self.border = Some(border);
            self.border_mut().clear();
        } else {
            self.border = None
        }
    }

    /// Get the terminal border. Panics if no border was set.
    pub fn border(&self) -> &Border {
        self.border.as_ref().unwrap()
    }

    /// Get the terminal border. Panics if no border was set.
    pub fn border_mut(&mut self) -> TerminalBorderMut {
        self.get_border_mut().unwrap()
    }

    /// Attempt to retrieve the terminal border.
    pub fn get_border(&self) -> Option<&Border> {
        self.border.as_ref()
    }

    /// Attempt to retrieve the terminal border.
    pub fn get_border_mut(&mut self) -> Option<TerminalBorderMut> {
        let clear_tile = self.clear_tile;
        let term_size = self.size;
        self.border
            .as_mut()
            .map(|state| TerminalBorderMut::new(state, term_size, clear_tile))
    }

    /// The grid bounds of the terminal.
    ///
    /// This has no relation to world space, for world space you can use the
    /// [crate::TerminalTransform].
    pub fn bounds(&self) -> GridRect {
        GridRect::new([0, 0], self.size())
    }

    /// Transform a 2d grid position to it's corresponding 1d index.
    pub fn xy_to_index(&self, xy: IVec2) -> usize {
        xy.as_index(self.width())
    }

    /// Transform a 1d index to it's corresponding 2d grid position.
    pub fn index_to_xy(&self, i: usize) -> IVec2 {
        let x = (i % self.width()) as i32;
        let y = (i / self.width()) as i32;
        IVec2::new(x, y)
    }
}

#[cfg(test)]
mod tests {

    use bevy::color::palettes::basic;

    use crate::{border::Border, string::StringFormatter, GridPoint, GridRect, Pivot};

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
        let string = "Hello".no_word_wrap().fg(basic::BLUE.into());
        term.put_string([1, 1].pivot(Pivot::TopLeft), string);

        term.put_string(
            [1, 1].pivot(Pivot::TopLeft),
            "hi".no_word_wrap().fg(basic::RED.into()),
        );

        term.put_string([1, 1], "Hello");

        let allocated_string = "Hello".to_string();
        term.put_string([1, 1], &allocated_string);
    }

    #[test]
    fn border() {
        let mut term = Terminal::new([15, 15]);
        term.put_border(Border::single_line())
            .put_title("Hello".fg(basic::BLUE.into()));
        for (_, t) in term.border().iter() {
            println!("{}", t.glyph);
        }
    }
}
