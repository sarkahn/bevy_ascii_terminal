use std::{
    collections::BTreeMap,
    iter,
    slice::{self, IterMut},
};

use bevy::{math::IVec2, render::color::Color};

use crate::{
    border::{BorderState, TerminalBorder},
    string::{NoWordWrapStringIter, StringFormatter, WrappedStringIter, XyStringIter},
    Dir4, FormattedString, GridPoint, GridRect, Pivot, PivotedPoint, Tile,
};

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
    /// color, prevent word wrapping, or prevent colors from being written to
    /// empty characters:
    ///
    /// ```
    /// use bevy_ascii_terminal::*;
    ///
    /// let mut term = Terminal::new([8,4]);
    /// term.put_string([0,0], "Hello".fg(Color::BLUE));
    /// term.put_string([0,1].pivot(Pivot::TopLeft), "A looooong string".no_word_wrap().ignore_spaces());
    /// ```
    pub fn put_string<'a>(
        &'a mut self,
        xy: impl Into<PivotedPoint>,
        string: impl Into<FormattedString<'a>>,
    ) {
        let fmt: FormattedString = string.into();
        // TODO: Change to bottom left default?
        let pivot = xy.into().pivot().unwrap_or(Pivot::TopLeft);
        let iter = match fmt.wrapped {
            true => XyStringIter::Wrapped(WrappedStringIter::new(fmt.string, self.bounds(), pivot)),
            false => XyStringIter::NotWrapped(NoWordWrapStringIter::new(
                fmt.string,
                self.bounds(),
                pivot,
            )),
        };
        for (xy, ch) in iter {
            if fmt.ignore_spaces && ch == ' ' {
                continue;
            }
            let tile = self.tile_mut(xy);
            tile.glyph = ch;
            if let Some(fg) = fmt.fg_color {
                tile.fg_color = fg;
            }
            if let Some(bg) = fmt.bg_color {
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
        self.tiles
            .chunks_mut(w)
            .skip(rect.bottom() as usize)
            .flat_map(move |tiles| tiles[rect.left() as usize..=rect.right() as usize].iter_mut())
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

    pub fn border_mut(&mut self) -> Option<TerminalBorder> {
        let clear_tile = self.clear_tile;
        let term_size = self.size;
        self.border.as_mut().map(|state| TerminalBorder {
            border: state,
            term_size,
            clear_tile,
        })
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
        let step = self.width() - rect.width();
        let iter = self.tiles[start..=end]
            .windows(rect.width())
            .step_by(step)
            .flatten();
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

// pub struct TerminalTile<'a> {
//     index: usize,
//     terminal: &'a mut Terminal,
// }

// impl<'a> TerminalTile<'a> {
//     pub fn glyph(self, glyph: char) -> Self {
//         let i = self.index;
//         self.terminal.tiles[i].glyph = glyph;
//         self
//     }

//     pub fn fg(self, color: Color) -> Self {
//         let i = self.index;
//         self.terminal.tiles[i].fg_color = color;
//         self
//     }

//     pub fn bg(self, color: Color) -> Self {
//         let i = self.index;
//         self.terminal.tiles[i].bg_color = color;
//         self
//     }
// }

// pub struct TilesIter<'a, T>
// where
//     T: Fn(&mut [Tile]) -> slice::IterMut<'_, Tile>,
// {
//     iter: iter::FlatMap<iter::Skip<slice::ChunksMut<'a, Tile>>, IterMut<'a, Tile>, T>,
// }

// impl<'a, T: Fn(&mut [Tile]) -> slice::IterMut<'_, Tile>> Iterator for TilesIter<'a, T> {
//     type Item = &'a mut Tile;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next()
//     }
// }

pub struct TerminalRectIterMut<'a> {
    iter: IterMut<'a, Tile>,
    head: IVec2,
    tail: IVec2,
    rect: GridRect,
    skip: usize,
}

impl<'a> TerminalRectIterMut<'a> {
    pub fn new(term: &'a mut Terminal, rect: GridRect) -> Self {
        let start = rect.min().as_index(term.width());
        let end = rect.max().as_index(term.width());
        let skip = term.width() - rect.width();
        Self {
            iter: term.tiles[start..=end].iter_mut(),
            head: rect.min(),
            tail: rect.max(),
            rect,
            skip,
        }
    }
}

impl<'a> Iterator for TerminalRectIterMut<'a> {
    type Item = &'a mut Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.cmpgt(self.tail).all() {
            return None;
        }
        let ret = self.iter.next();
        if self.head.x > self.rect.right() {
            self.head.x = self.rect.left();
            self.head.y += 1;
            self.iter.nth(self.skip);
        }
        ret
    }
}

impl<'a> DoubleEndedIterator for TerminalRectIterMut<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.tail.cmplt(self.head).all() {
            return None;
        }
        let ret = self.iter.next_back();
        if self.tail.x < self.rect.left() {
            self.tail.x = self.rect.right();
            self.tail.y -= 1;
            self.iter.nth_back(self.skip);
        }
        ret
    }
}

// impl<'a> TerminalTiles<'a> {
//     pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut Tile> {
//         self.terminal.iter_rect_mut(self.rect)
//     }

//     pub fn glyphs(&'a mut self, glyph: char) {
//         self.iter_mut().for_each(|t| t.glyph = glyph);
//     }
// }

// impl<'a> IntoIterator for TerminalTiles<'a> {
//     type Item;

//     type IntoIter;

//     fn into_iter(self) -> Self::IntoIter {
//         self.terminal.iter_rect_mut(self.rect)
//     }
// }

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

    use bevy::render::color::Color;

    use crate::{string::StringFormatter, GridPoint, GridRect, Pivot};

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

        // let string = "whoa".to_string();

        // term.put_string([10, 10], &string);
    }
}
