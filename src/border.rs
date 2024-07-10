use bevy::color::LinearRgba;
use enum_ordinalize::Ordinalize;

use crate::string::{DecoratedString, StringDecoration};

#[derive(Debug, Default, Clone)]
pub struct Border {
    pub edge_glyphs: [Option<char>; 8],
    pub edge_strings: [EdgeString; 4],
}

impl Border {
    // pub fn from_string(string: impl AsRef<str>) -> Self {
    //     let mut chars = string.as_ref().chars();
    // }

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

    pub fn has_left_side(&self) -> bool {
        self.left_glyph().is_some()
            || self.top_left_glyph().is_some()
            || self.bottom_left_glyph().is_some()
    }

    pub fn has_bottom_side(&self) -> bool {
        self.bottom_left_glyph().is_some()
            || self.bottom_glyph().is_some()
            || self.bottom_right_glyph().is_some()
    }

    pub fn has_top_side(&self) -> bool {
        self.top_left_glyph().is_some()
            || self.top_glyph().is_some()
            || self.top_right_glyph().is_some()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ordinalize)]
pub enum BorderEdge {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Debug, Default, Clone)]
pub struct EdgeString {
    pub string: String,
    pub decoration: StringDecoration,
    pub offset: i32,
    pub alignment: f32,
}

pub trait BorderStringWriter {
    fn delimiters(self, delimiters: impl AsRef<str>) -> EdgeString;
    fn fg(self, color: impl Into<LinearRgba>) -> EdgeString;
}

impl Border {
    pub fn put_string<T: AsRef<str>>(
        &mut self,
        edge: BorderEdge,
        // The string alignment, where 0.0 is the bottom/left and 1.0 is the top/right.
        alignment: f32,
        // Offset the string by the given number of tiles from it's the aligned position.
        offset: i32,
        string: impl Into<DecoratedString<T>>,
    ) {
        let ds: DecoratedString<T> = string.into();
        let es = EdgeString {
            string: String::from(ds.string.as_ref()),
            decoration: ds.decoration,
            offset,
            alignment,
        };
        self.edge_strings[edge.ordinal() as usize] = es;
    }
}

#[cfg(test)]
mod tests {
    use bevy::color::palettes::{basic, css};

    use crate::string::StringDecorator;

    use super::*;

    fn put_color(color: impl Into<LinearRgba>) {}

    #[test]
    fn a() {
        put_color(css::ALICE_BLUE);
        let mut border = Border::default();
        border.put_string(BorderEdge::Bottom, 0.0, 1, "hi".fg(basic::BLACK));
    }
}
