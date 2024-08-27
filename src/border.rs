use bevy::{color::LinearRgba, math::UVec2, prelude::Component};
use enum_ordinalize::Ordinalize;

use crate::{
    string::{DecoratedString, StringDecoration},
    GridRect, Pivot,
};

#[derive(Debug, Default, Clone, Component)]
pub struct TerminalBorder {
    pub edge_glyphs: [Option<char>; 8],
    pub border_strings: Vec<BorderString>,
    size: UVec2,
}

impl TerminalBorder {
    /// Create a [Border] from a string.
    ///
    /// The string will be read as 9 tiles with line returns and the center
    /// space being ignored. Any other spaces will be treated as a clear tile
    /// for that edge.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_ascii_terminal::*;
    ///
    /// // Create a single-line border with a blank edge on the top and bottom.
    /// let border = Border::from_string(
    /// "┌ ┐
    /// │ │
    /// └ ┘"
    /// );
    /// ```
    #[allow(clippy::needless_range_loop)]
    pub fn from_string(string: impl AsRef<str>) -> Self {
        let mut edge_glyphs = [None; 8];
        let mut chars = string.as_ref().chars().filter(|c| *c != '\n');
        for i in 0..4 {
            if let Some(ch) = chars.next().filter(|ch| *ch != ' ') {
                edge_glyphs[i] = Some(ch);
            }
        }
        chars.next();
        for i in 4..8 {
            if let Some(ch) = chars.next().filter(|ch| *ch != ' ') {
                edge_glyphs[i] = Some(ch);
            }
        }
        Self {
            edge_glyphs,
            ..Default::default()
        }
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

    pub fn edge_glyph(&self, pivot: Pivot) -> Option<char> {
        let i = if pivot == Pivot::Center {
            Pivot::TopCenter.ordinal() as usize
        } else {
            pivot.ordinal() as usize
        };
        self.edge_glyphs[i]
    }

    pub fn edge_glyph_mut(&mut self, pivot: Pivot) -> Option<&mut char> {
        let i = if pivot == Pivot::Center {
            Pivot::TopCenter.ordinal() as usize
        } else {
            pivot.ordinal() as usize
        };
        self.edge_glyphs[i].as_mut()
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
    ///   can be written to a single edge, and they will be drawn in order.
    ///
    /// * `alignment` - Determines the relative origin of the string, where 0.0
    ///   is the bottom/left and 1.0 is the top/right.
    ///
    /// * `offset` - Offset the string by the given number of tiles from it's the
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

    /// The number of visible tiles on the border, taking into account the
    /// border glyphs and any border strings.
    pub(crate) fn tile_count(&self) -> usize {
        let mut count = 0;
        count += self.top_left_glyph().is_some() as usize;
        count += self.top_right_glyph().is_some() as usize;
        count += self.bottom_left_glyph().is_some() as usize;
        count += self.bottom_right_glyph().is_some() as usize;
        if self.top_glyph().is_some()
            || self
                .border_strings
                .iter()
                .any(|bs| bs.edge == BorderSide::Top)
        {
            count += self.width() - 2;
        }
        if self.bottom_glyph().is_some()
            || self
                .border_strings
                .iter()
                .any(|bs| bs.edge == BorderSide::Bottom)
        {
            count += self.width() - 2;
        }
        if self.left_glyph().is_some()
            || self
                .border_strings
                .iter()
                .any(|bs| bs.edge == BorderSide::Left)
        {
            count += self.height() - 2;
        }
        if self.right_glyph().is_some()
            || self
                .border_strings
                .iter()
                .any(|bs| bs.edge == BorderSide::Right)
        {
            count += self.height() - 2;
        }
        count
    }

    pub fn pivot_tile_count(&self, pivot: Pivot) -> usize {
        match pivot {
            Pivot::TopLeft => self.top_left_glyph().is_some() as usize,
            Pivot::TopRight => self.top_right_glyph().is_some() as usize,
            Pivot::BottomLeft => self.bottom_left_glyph().is_some() as usize,
            Pivot::BottomRight => self.bottom_right_glyph().is_some() as usize,
            Pivot::TopCenter => {
                if self.top_glyph().is_some()
                    || self
                        .border_strings
                        .iter()
                        .any(|bs| bs.edge == BorderSide::Top)
                {
                    self.width() - 2
                } else {
                    0
                }
            }
            Pivot::LeftCenter => {
                if self.left_glyph().is_some()
                    || self
                        .border_strings
                        .iter()
                        .any(|bs| bs.edge == BorderSide::Left)
                {
                    self.height() - 2
                } else {
                    0
                }
            }
            Pivot::RightCenter => {
                if self.right_glyph().is_some()
                    || self
                        .border_strings
                        .iter()
                        .any(|bs| bs.edge == BorderSide::Right)
                {
                    self.height() - 2
                } else {
                    0
                }
            }
            Pivot::BottomCenter => {
                if self.bottom_glyph().is_some()
                    || self
                        .border_strings
                        .iter()
                        .any(|bs| bs.edge == BorderSide::Bottom)
                {
                    self.width() - 2
                } else {
                    0
                }
            }
            Pivot::Center => todo!(),
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

    pub fn bounds(&self) -> GridRect {
        GridRect::new([0, 0], self.size)
    }
}

/// One of four sides of a border.
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ordinalize)]
pub enum BorderSide {
    Top,
    Left,
    Right,
    Bottom,
}

impl BorderSide {
    pub fn from_pivot(pivot: Pivot) -> Option<BorderSide> {
        match pivot {
            Pivot::TopCenter => Some(BorderSide::Top),
            Pivot::LeftCenter => Some(BorderSide::Left),
            Pivot::RightCenter => Some(BorderSide::Right),
            Pivot::BottomCenter => Some(BorderSide::Bottom),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BorderString {
    pub edge: BorderSide,
    pub string: String,
    pub decoration: StringDecoration,
    pub offset: i32,
    pub alignment: f32,
}

pub trait BorderStringWriter {
    fn delimiters(self, delimiters: impl AsRef<str>) -> BorderString;
    fn fg(self, color: impl Into<LinearRgba>) -> BorderString;
    fn bg(self, color: impl Into<LinearRgba>) -> BorderString;
}

#[cfg(test)]
mod tests {
    use bevy::color::palettes::basic;

    use crate::string::StringDecorator;

    use super::*;

    #[test]
    fn a() {
        let mut border = TerminalBorder::default();
        border.put_string(BorderSide::Bottom, 0.0, 1, "hi".fg(basic::BLACK));
        border.put_title("hi".fg(basic::BLUE).delimiters("[]"));
    }

    #[test]
    fn from_string() {
        let border = TerminalBorder::from_string(
            "┌ ┐
│ │
└ ┘",
        );
        println!("{:?}", border);
    }
}
