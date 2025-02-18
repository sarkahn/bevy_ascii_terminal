use std::ops::Sub;

use bevy::{math::IVec2, prelude::Component, reflect::Reflect, utils::HashMap};
use enum_ordinalize::Ordinalize;
use sark_grids::{GridPoint, GridRect, GridSize, Pivot};

use crate::{
    string::{DecoratedString, StringDecoration, StringIter},
    Tile,
};

/// A component for drawing a border around a terminal.
///
/// Along with a 9 slice string to represent the border, aligned and formatted
/// strings can be written can be four sides of the border.
///
/// The border can have "empty" sides which will be ignored when building the
/// terminal mesh.
#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct TerminalBorder {
    pub edge_glyphs: [Option<char>; 8],
    pub border_strings: Vec<BorderString>,
    tiles: HashMap<IVec2, Tile>,
}

impl TerminalBorder {
    /// Create a [Border] from a 9 slice string.
    ///
    /// The string will be read line by line, with the last 3 characters on
    /// each line being used to set the border glyphs. The center of the
    /// 9 slice being ignored.
    ///
    /// 'Space' characters will count as an empty tile for that edge.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_ascii_terminal::*;
    ///
    /// // Create a single-line border with no tiles on the top and bottom edges.
    /// let border = TerminalBorder::from_string(
    /// "┌ ┐
    ///  │ │
    ///  └ ┘");
    /// ```
    pub fn from_string(string: impl AsRef<str>) -> Self {
        let mut glyphs = [None; 8];
        let mut lines = string.as_ref().lines();
        if let Some(mut line) = lines.next().map(|l| l.chars().rev().take(3)) {
            glyphs[2] = line.next();
            glyphs[1] = line.next();
            glyphs[0] = line.next();
        }

        if let Some(mut line) = lines.next().map(|l| l.chars().rev().take(3)) {
            glyphs[4] = line.next();
            line.next();
            glyphs[3] = line.next();
        }
        if let Some(mut line) = lines.next().map(|l| l.chars().rev().take(3)) {
            glyphs[7] = line.next();
            glyphs[6] = line.next();
            glyphs[5] = line.next();
        }

        Self {
            edge_glyphs: glyphs,
            ..Default::default()
        }
    }

    pub fn single_line() -> Self {
        Self::from_string("┌─┐\n│ │\n└─┘")
    }

    pub fn double_line() -> Self {
        Self::from_string("╔═╗\n║ ║\n╚═╝")
    }

    pub fn with_title(mut self, title: impl AsRef<str>) -> Self {
        self.put_title(title);
        self
    }

    pub fn top_left_glyph(&self) -> Option<char> {
        self.edge_glyphs[0]
    }

    pub fn top_glyph(&self) -> Option<char> {
        self.edge_glyphs[1]
    }

    pub fn top_right_glyph(&self) -> Option<char> {
        self.edge_glyphs[2]
    }

    pub fn left_glyph(&self) -> Option<char> {
        self.edge_glyphs[3]
    }

    pub fn right_glyph(&self) -> Option<char> {
        self.edge_glyphs[4]
    }

    pub fn bottom_left_glyph(&self) -> Option<char> {
        self.edge_glyphs[5]
    }

    pub fn bottom_glyph(&self) -> Option<char> {
        self.edge_glyphs[6]
    }

    pub fn bottom_right_glyph(&self) -> Option<char> {
        self.edge_glyphs[7]
    }

    /// True if any glyph on the entire left side of the border is set.
    pub fn has_left_side(&self) -> bool {
        self.left_glyph().is_some()
            || self.top_left_glyph().is_some()
            || self.bottom_left_glyph().is_some()
            || self
                .border_strings
                .iter()
                .any(|bs| bs.edge == BorderSide::Left)
    }

    /// True if any glyph on the entire right side of the border is set.
    pub fn has_right_side(&self) -> bool {
        self.right_glyph().is_some()
            || self.top_right_glyph().is_some()
            || self.bottom_right_glyph().is_some()
            || self
                .border_strings
                .iter()
                .any(|bs| bs.edge == BorderSide::Right)
    }

    /// True if any glyph on the entire bottom side of the border is set.
    pub fn has_bottom_side(&self) -> bool {
        self.bottom_left_glyph().is_some()
            || self.bottom_glyph().is_some()
            || self.bottom_right_glyph().is_some()
            || self
                .border_strings
                .iter()
                .any(|bs| bs.edge == BorderSide::Bottom)
    }

    /// True if any glyph on the entire top side of the border is set.
    pub fn has_top_side(&self) -> bool {
        self.top_left_glyph().is_some()
            || self.top_glyph().is_some()
            || self.top_right_glyph().is_some()
            || self
                .border_strings
                .iter()
                .any(|bs| bs.edge == BorderSide::Top)
    }

    pub fn set_edge_glyph(&mut self, pivot: Pivot, glyph: Option<char>) {
        let pivot = if pivot == Pivot::Center {
            Pivot::TopCenter
        } else {
            pivot
        };
        self.edge_glyphs[pivot.ordinal() as usize] = glyph;
    }

    pub fn clear_strings(&mut self) {
        self.border_strings.clear();
    }

    pub fn clear_edge_glyphs(&mut self) {
        self.edge_glyphs.fill(None);
    }

    /// Write a string to a side of the border.
    ///
    /// # Arguments
    /// * `edge` - Which edge of the border to write to. Multiple strings
    ///   can be written to a single edge, and they will be drawn in order
    ///   of insertion.
    ///
    /// * `alignment` - Determines the starting position of the string, where 0.0
    ///   is the bottom/left and 1.0 is the top/right.
    ///
    /// * `offset` - Offset the string by the given number of tiles from it's
    ///   aligned position. Positive values adjust up/right, negative values adjust
    ///   down/left.
    pub fn put_string<T: AsRef<str>>(
        &mut self,
        edge: BorderSide,
        alignment: f32,
        offset: i32,
        string: impl Into<DecoratedString<T>>,
    ) {
        let ds: DecoratedString<T> = string.into();
        let bs = BorderString {
            edge,
            string: String::from(ds.string.as_ref()),
            decoration: ds.decoration,
            offset,
            alignment,
        };
        self.border_strings.push(bs);
    }

    /// Write a title to the top left of the border.
    pub fn put_title<T: AsRef<str>>(&mut self, string: impl Into<DecoratedString<T>>) {
        self.put_string(BorderSide::Top, 0.0, 0, string);
    }

    pub fn bounds(&self, size: impl GridSize) -> GridRect {
        let mut bounds = GridRect::new([0, 0], size);
        bounds.resize_from_pivot(Pivot::LeftCenter, self.has_left_side() as i32);
        bounds.resize_from_pivot(Pivot::RightCenter, self.has_right_side() as i32);
        bounds.resize_from_pivot(Pivot::TopCenter, self.has_top_side() as i32);
        bounds.resize_from_pivot(Pivot::BottomCenter, self.has_bottom_side() as i32);
        bounds
    }

    /// Rebuild border tiles. This is called by terminal systems to update the
    /// border tiles before building them into the terminal mesh.
    pub(crate) fn rebuild(&mut self, size: impl GridSize, clear_tile: Tile) {
        self.tiles.clear();
        let bounds = self.bounds(size);
        if let Some(tl) = self.top_left_glyph() {
            self.tiles
                .insert(bounds.top_left(), clear_tile.with_char(tl));
        }
        if let Some(tr) = self.top_right_glyph() {
            self.tiles
                .insert(bounds.top_right(), clear_tile.with_char(tr));
        }
        if let Some(bl) = self.bottom_left_glyph() {
            self.tiles
                .insert(bounds.bottom_left(), clear_tile.with_char(bl));
        }
        if let Some(br) = self.bottom_right_glyph() {
            self.tiles
                .insert(bounds.bottom_right(), clear_tile.with_char(br));
        }
        if let Some(t) = self.top_glyph() {
            for xy in bounds
                .iter_row(bounds.top_index())
                .skip(1)
                .take(bounds.width() - 2)
            {
                self.tiles.insert(xy, clear_tile.with_char(t));
            }
        }

        if let Some(b) = self.bottom_glyph() {
            for xy in bounds
                .iter_row(bounds.bottom_index())
                .skip(1)
                .take(bounds.width() - 2)
            {
                self.tiles.insert(xy, clear_tile.with_char(b));
            }
        }

        if let Some(l) = self.left_glyph() {
            for xy in bounds
                .iter_column(bounds.left_index())
                .skip(1)
                .take(bounds.height() - 2)
            {
                self.tiles.insert(xy, clear_tile.with_char(l));
            }
        }

        if let Some(r) = self.right_glyph() {
            for xy in bounds
                .iter_column(bounds.right_index())
                .skip(1)
                .take(bounds.height() - 2)
            {
                self.tiles.insert(xy, clear_tile.with_char(r));
            }
        }

        for s in self.border_strings.iter() {
            let dir = match s.edge {
                BorderSide::Top | BorderSide::Bottom => IVec2::new(1, 0),
                BorderSide::Left | BorderSide::Right => IVec2::new(0, -1),
            };
            let char_count = s.string.chars().count();
            let offset = match s.edge {
                BorderSide::Top | BorderSide::Bottom => {
                    let align_off = ((bounds.width() - 2) as f32 * s.alignment).round() as i32;
                    let size_off = (char_count as f32 * s.alignment).round() as i32;
                    dir * (align_off - size_off)
                }
                BorderSide::Left | BorderSide::Right => {
                    let align_off = ((bounds.height() - 3) as f32 * s.alignment).floor() as i32;
                    let size_off = (char_count.sub(1) as f32 * (1.0 - s.alignment)).floor() as i32;
                    -dir * (align_off + size_off)
                }
            };

            let side_rect = match s.edge {
                BorderSide::Top => {
                    GridRect::new(bounds.top_left().right(1), [bounds.width() - 2, 1])
                }
                BorderSide::Bottom => {
                    GridRect::new(bounds.bottom_left().right(1), [bounds.width() - 2, 1])
                }
                BorderSide::Left => {
                    GridRect::new(bounds.bottom_left().up(1), [1, bounds.height() - 2])
                }
                BorderSide::Right => {
                    GridRect::new(bounds.bottom_right().up(1), [1, bounds.height() - 2])
                }
            };

            for (p, (ch, fg, bg)) in
                StringIter::new(&s.string, side_rect, offset, None, Some(s.decoration))
            {
                // decoration.clear_colors is ignored in borders since we don't have
                // an existing tile to work from.
                self.tiles.insert(
                    p,
                    Tile {
                        glyph: ch,
                        fg_color: fg.unwrap_or(clear_tile.fg_color),
                        bg_color: bg.unwrap_or(clear_tile.bg_color),
                    },
                );
            }
        }
    }

    pub fn tiles(&self) -> &HashMap<IVec2, Tile> {
        &self.tiles
    }
}

/// One of four sides of a border.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ordinalize, Reflect)]
pub enum BorderSide {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Debug, Clone, Reflect)]
pub struct BorderString {
    pub edge: BorderSide,
    pub string: String,
    pub decoration: StringDecoration,
    pub offset: i32,
    pub alignment: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_border_strings() {
        let mut border = TerminalBorder::single_line();
        border.put_string(BorderSide::Top, 0.0, 0, "TopLef");
        border.put_string(BorderSide::Top, 1.0, 0, "TopRig");
        border.put_string(BorderSide::Bottom, 0.0, 0, "BotLef");
        border.put_string(BorderSide::Bottom, 1.0, 0, "BotRig");
        border.put_string(BorderSide::Left, 0.0, 0, "LeftBot");
        border.put_string(BorderSide::Left, 1.0, 0, "LeftTop");
        border.put_string(BorderSide::Right, 0.0, 0, "RightBot");
        border.put_string(BorderSide::Right, 1.0, 0, "RightTop");
        border.rebuild([40, 40], Tile::default());

        for (p, t) in border.tiles() {
            println!("{:?} {:?}", p, t);
        }
    }
}
