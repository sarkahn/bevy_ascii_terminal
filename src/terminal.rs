use std::{
    collections::BTreeMap,
    iter,
    slice::{self, IterMut},
};

use bevy::{log::tracing_subscriber::filter::targets::IntoIter, math::IVec2, render::color::Color};
use itertools::Itertools;

use crate::{border::BorderState, Dir4, GridPoint, GridRect, Pivot, PivotedPoint, Tile};

pub struct Terminal {
    tiles: Vec<Tile>,
    size: IVec2,
    border: Option<BorderState>,
    clear_tile: Tile,
}

impl Terminal {
    pub fn new(size: impl Into<IVec2>) -> Self {
        let size = size.into();
        Self {
            tiles: vec![Tile::DEFAULT; size.len()],
            size,
            border: None,
            clear_tile: Tile::DEFAULT,
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

    pub fn tile(&self, xy: impl Into<PivotedPoint>) -> &Tile {
        let xy: IVec2 = xy.into().calc_from_size(self.size());
        let i = xy.as_index(self.width());
        &self.tiles[i]
    }

    pub fn tile_mut(&mut self, xy: impl Into<PivotedPoint>) -> &mut Tile {
        let xy: IVec2 = xy.into().calc_from_size(self.size());
        let i = xy.as_index(self.width());
        &mut self.tiles[i]
    }

    #[inline]
    fn terminal_tile(&mut self, xy: impl Into<PivotedPoint>) -> TerminalTile {
        let xy: IVec2 = xy.into().calc_from_size(self.size());
        let index = xy.as_index(self.width());
        TerminalTile {
            index,
            terminal: self,
        }
    }

    pub fn put_char(&mut self, xy: impl Into<PivotedPoint>, ch: char) -> TerminalTile {
        self.terminal_tile(xy).glyph(ch)
    }

    pub fn put_fg_color(&mut self, xy: impl Into<PivotedPoint>, color: Color) -> TerminalTile {
        self.terminal_tile(xy).fg(color)
    }

    pub fn put_bg_color(&mut self, xy: impl Into<PivotedPoint>, color: Color) -> TerminalTile {
        self.terminal_tile(xy).bg(color)
    }

    pub fn clear(&mut self) {
        self.tiles.fill(self.clear_tile)
    }

    /// Change the terminal's `clear_tile`. This will not clear the terminal.
    pub fn set_clear_tile(&mut self, clear_tile: Tile) {
        self.clear_tile = clear_tile;
    }

    pub fn tiles(&self) -> &[Tile] {
        self.tiles.as_slice()
    }

    pub fn tiles_mut(&mut self) -> &mut [Tile] {
        self.tiles.as_mut_slice()
    }

    pub fn iter_row(&self, row: usize) -> impl DoubleEndedIterator<Item = &Tile> {
        let start = self.width() * row;
        let end = start + self.width();
        self.tiles[start..end].iter()
    }

    pub fn iter_row_mut(&mut self, row: usize) -> impl DoubleEndedIterator<Item = &mut Tile> {
        let start = self.width() * row;
        let end = start + self.width();
        self.tiles[start..end].iter_mut()
    }

    pub fn iter_column(&self, column: usize) -> impl DoubleEndedIterator<Item = &Tile> {
        self.tiles.iter().skip(column).step_by(self.width())
    }

    pub fn iter_column_mut(&mut self, column: usize) -> impl DoubleEndedIterator<Item = &mut Tile> {
        let w = self.width();
        self.tiles.iter_mut().skip(column).step_by(w)
    }

    pub fn iter_rect(&self, rect: GridRect) -> impl DoubleEndedIterator<Item = &Tile> {
        let iter = self
            .tiles
            .chunks(self.width())
            .skip(rect.bottom() as usize)
            .flat_map(move |tiles| tiles[rect.left() as usize..=rect.right() as usize].iter());

        iter
    }

    pub fn iter_rect_mut(&mut self, rect: GridRect) -> impl DoubleEndedIterator<Item = &mut Tile> {
        let w = self.width();
        let iter = self
            .tiles
            .chunks_mut(w)
            .skip(rect.bottom() as usize)
            .flat_map(move |tiles| tiles[rect.left() as usize..=rect.right() as usize].iter_mut());
        iter
    }

    pub fn set_border(&mut self, border: Option<impl Into<BorderState>>) {
        if let Some(border) = border.map(|b| b.into()) {
            // border.clear_tile = self.clear_tile;
            // border.terminal_size = self.size;
            self.border = Some(border);
        } else {
            self.border = None
        }
    }

    pub fn border(&self) -> Option<&BorderState> {
        self.border.as_ref()
    }

    pub fn border_mut(&mut self) -> Option<&mut BorderState> {
        self.border.as_mut()
    }

    pub fn bounds(&self) -> GridRect {
        GridRect::new([0, 0], self.size())
    }

    // T: Fn(&mut [Tile]) -> slice::IterMut<'_, Tile>
    // iter::FlatMap<iter::Skip<slice::ChunksMut<'a, Tile>>, IterMut<'a, Tile>, T>
    fn iter_test(&mut self, rect: GridRect) {
        // -> impl Iterator<Item = &mut Tile> {
        let start = rect.min().as_index(self.width());
        let end = rect.max().as_index(self.width());
        // //let row_width = self.width();
        // let w = self.width();
        // self.tiles
        //     .chunks_mut(w)
        //     .skip(rect.bottom() as usize)
        //     .flat_map(move |tiles: &'_ mut [Tile]| {
        //         tiles[rect.left() as usize..=rect.right() as usize].iter_mut()
        //     })
    }
}

pub struct TerminalTile<'a> {
    index: usize,
    terminal: &'a mut Terminal,
}

impl<'a> TerminalTile<'a> {
    pub fn glyph(self, glyph: char) -> Self {
        let i = self.index;
        self.terminal.tiles[i].glyph = glyph;
        self
    }

    pub fn fg(self, color: Color) -> Self {
        let i = self.index;
        self.terminal.tiles[i].fg_color = color;
        self
    }

    pub fn bg(self, color: Color) -> Self {
        let i = self.index;
        self.terminal.tiles[i].bg_color = color;
        self
    }
}

pub struct TerminalBorder<'a> {
    border: &'a mut BorderState,
    terminal: &'a Terminal,
}

impl<'a> TerminalBorder<'a> {
    pub fn put_string(
        &'a mut self,
        edge: Pivot,
        direction: Dir4,
        offset: i32,
        string: impl AsRef<str>,
    ) -> &'a mut Self {
        let xy = edge.size_offset(self.terminal.size());

        self
    }

    // pub fn put_title(&'a mut self, string: impl AsRef<str>) -> TerminalTiles {
    //     self.put_string(Pivot::TopLeft, Dir4::Right, 1, string);
    // }

    pub fn clear_colors(&'a mut self) -> &'a mut Self {
        let clear = self.terminal.clear_tile;
        for tile in self.border.tiles.values_mut() {
            tile.fg_color = clear.fg_color;
            tile.bg_color = clear.bg_color;
        }
        self
    }
}

pub struct TilesIter<'a, T>
where
    T: Fn(&mut [Tile]) -> slice::IterMut<'_, Tile>,
{
    iter: iter::FlatMap<iter::Skip<slice::ChunksMut<'a, Tile>>, IterMut<'a, Tile>, T>,
}

impl<'a, T: Fn(&mut [Tile]) -> slice::IterMut<'_, Tile>> Iterator for TilesIter<'a, T> {
    type Item = &'a mut Tile;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct TerminalTiles<'a> {
    terminal: &'a mut Terminal,
    rect: GridRect,
}

impl<'a> TerminalTiles<'a> {
    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut Tile> {
        self.terminal.iter_rect_mut(self.rect)
    }

    pub fn glyphs(&'a mut self, glyph: char) {
        self.iter_mut().for_each(|t| t.glyph = glyph);
    }
}

impl<'a> IntoIterator for TerminalTiles<'a> {
    type Item;

    type IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.terminal.iter_rect_mut(self.rect)
    }
}

// impl<'a> TerminalTiles<'a> {
//     pub fn iter(&'a mut self) -> impl Iterator<Item = &'a mut Tile> {
//         self.terminal.iter_rect_mut(self.rect)
//     }
// }

// impl<'a, T> IntoIterator for TerminalTiles<'a>
// where
//     T: Fn(&mut [Tile]) -> slice::IterMut<'_, Tile>,
// {
//     type Item = &'a mut Tile;

//     type IntoIter = iter::FlatMap<iter::Skip<slice::ChunksMut<'a, Tile>>, IterMut<'a, Tile>, T>;

//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }
// }

#[cfg(test)]
mod tests {

    use crate::GridRect;

    use super::{BorderState, Terminal};

    #[test]
    fn border() {
        let mut term = Terminal::new([10, 10]);
        let rect = GridRect::from_points([7, 7], [9, 9]);
        let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
        for (ch, t) in chars.iter().zip(term.iter_rect_mut(rect)) {
            t.glyph = *ch;
        }

        assert_eq!('a', term.tile([7, 7]).glyph);
        assert_eq!('i', term.tile([9, 9]).glyph);

        // for t in term.iter_test(rect) {
        //     println!("{}", t.glyph);
        // }

        //let border = TerminalBorder::from_string("abcdefgh").put_title("Hello");
        //term.set_border(Some(border)).put_title("Hello");
    }
}
