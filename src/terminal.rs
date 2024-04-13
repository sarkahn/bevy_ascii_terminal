use bevy::{ecs::component::Component, math::IVec2, render::color::Color};

use crate::{
    border::{Border, TerminalBorderMut},
    string::{StringFormatter, StringIter},
    FormattedString, GridPoint, GridRect, Pivot, PivotedPoint, Tile,
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
    /// Note that by default strings get written to the
    /// top left of the terminal. You can manually set the pivot to override
    /// this behavior.
    ///
    /// ```
    /// use bevy_ascii_terminal::*;
    ///
    /// let mut term = Terminal::new([8,4]);
    /// term.put_string([0,0], "Hello".fg(Color::BLUE));
    /// let string = "A looooong string".bg(Color::GREEN).no_word_wrap().ignore_spaces();
    /// term.put_string([0,1], string);
    /// ```
    pub fn put_string<'a>(
        &'a mut self,
        xy: impl Into<PivotedPoint>,
        string: impl Into<FormattedString<'a>>,
    ) {
        let string: FormattedString = string.into();
        let ignore_spaces = string.ignore_spaces;
        let fg = string.fg_color;
        let bg = string.bg_color;
        for (xy, ch) in StringIter::new(xy, string, self.bounds()) {
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

    pub fn clear(&mut self) {
        self.tiles.fill(self.clear_tile)
    }

    /// Change the terminal's `clear_tile`. This will not clear the terminal.
    pub fn set_clear_tile(&mut self, clear_tile: Tile) {
        self.clear_tile = clear_tile;
    }

    pub fn clear_tile(&self) -> Tile {
        self.clear_tile
    }

    /// The terminal tiles as a slice
    pub fn tiles(&self) -> &[Tile] {
        self.tiles.as_slice()
    }

    /// The terminal tiles as a slice
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

    pub fn set_border(&mut self, border: Option<Border>) {
        if let Some(border) = border {
            self.border = Some(border);
            self.border_mut().clear();
        } else {
            self.border = None
        }
    }

    pub fn border(&self) -> &Border {
        self.border.as_ref().unwrap()
    }

    pub fn border_mut(&mut self) -> TerminalBorderMut {
        self.get_border_mut().unwrap()
    }

    pub fn get_border(&self) -> Option<&Border> {
        self.border.as_ref()
    }

    pub fn get_border_mut(&mut self) -> Option<TerminalBorderMut> {
        let clear_tile = self.clear_tile;
        let term_size = self.size;
        self.border
            .as_mut()
            .map(|state| TerminalBorderMut::new(state, term_size, clear_tile))
    }

    pub fn bounds(&self) -> GridRect {
        GridRect::new([0, 0], self.size())
    }

    pub fn xy_to_index(&self, xy: IVec2) -> usize {
        xy.as_index(self.width())
    }

    pub fn index_to_xy(&self, i: usize) -> IVec2 {
        let x = (i % self.width()) as i32;
        let y = (i / self.width()) as i32;
        IVec2::new(x, y)
    }
}

#[cfg(test)]
mod tests {

    use bevy::render::color::Color;

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

    #[test]
    fn string() {
        let mut term = Terminal::new([15, 15]);
        let string = "Hello".no_word_wrap().fg(Color::BLUE);
        term.put_string([1, 1].pivot(Pivot::TopLeft), string);

        term.put_string(
            [1, 1].pivot(Pivot::TopLeft),
            "hi".no_word_wrap().fg(Color::RED),
        );

        term.put_string([1, 1], "Hello");
    }

    #[test]
    fn border() {
        let mut term = Terminal::new([15, 15]);
        term.put_border(Border::single_line())
            .put_title("Hello".fg(Color::BLUE));
        for (_, t) in term.border().iter() {
            println!("{}", t.glyph);
        }
    }
}
