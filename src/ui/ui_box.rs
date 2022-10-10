use bevy::math::Vec2;
use bevy::prelude::Color;
use sark_grids::GridPoint;
use sark_grids::Size2d;

use crate::border::Border;
//use crate::border::Edge;

use crate::fmt_tile::FormattedTile;
use crate::formatting::TileFormatter;
use crate::Terminal;

#[derive(Debug, Default, Clone)]
/// Specifies the glyphs and colors to use when drawing a box on the terminal.
pub struct UiBox {
    /// Optional glyphs to be draw on the border of the box.
    ///
    /// Foreground and/or background color can optionally be provided with the `fg(Color)` and
    /// `bg(Color)` functions.
    pub border_glyphs: Option<Border>,

    /// Optional tile to fill the box with.
    pub fill_tile: Option<FormattedTile>,
}

impl UiBox {
    pub fn new() -> Self {
        UiBox::default()
    }

    /// Create a hollow UiBox with a single line border.
    pub fn single_line() -> Self {
        todo!()
        // let border = Border::SINGLE_LINE;
        // UiBox::new().with_border(border)
    }

    /// Create a hollow UiBox with a double line border.
    pub fn double_line() -> Self {
        todo!();
        // let border = Border::DOUBLE_LINE;
        // UiBox::new().with_border(border)
    }

    /// Specify the [BorderGlyphs] for the box.
    pub fn with_border(mut self, glyphs: Border) -> Self {
        self.border_glyphs = Some(glyphs);
        self
    }

    /// Specify the optional fill tile for the box.
    ///
    /// If specified, the fill tile will be drawn to the whole area of the box before
    /// the border is drawn. If no border is specified the entire box will be filled.
    pub fn filled(mut self, fill_tile: impl TileFormatter) -> UiBox {
        self.fill_tile = Some(fill_tile.format());
        self
    }

    /// A box which will apply the given foreground and background colors, without affecting
    /// existing glyphs.
    pub fn color_fill(self, fg: Color, bg: Color) -> UiBox {
        self.filled(FormattedTile::new().fg(fg).bg(bg))
    }

    /// Clear the entire area of the box to default tiles before drawing the border.
    pub fn cleared(self) -> Self {
        self.filled(FormattedTile::clear())
    }

    /// Draw the box to a terminal.
    pub(crate) fn draw(&self, xy: impl GridPoint, size: impl Size2d, term: &mut Terminal) {
        let pivoted_point = xy.get_pivot();
        let xy = xy.get_aligned_point(term.size());
        let pivot = pivoted_point.pivot;

        let pivot = Vec2::from(pivot);
        let align_offset = ((size.as_vec2() - Vec2::ONE) * pivot).as_ivec2();
        let xy = xy - align_offset;

        let [x, y] = xy.as_usize_array();
        let [width, height] = size.as_usize_array();
        let left = x;
        let right = x + width - 1;
        let bottom = y;
        let top = y + height - 1;

        if let Some(fill) = &self.fill_tile {
            let [width, height] = size.as_array();
            let [x, y] = xy.to_array();
            for y in y..y + height as i32 {
                for x in x..x + width as i32 {
                    fill.draw([x, y], term);
                }
            }
        }

        if let Some(glyphs) = &self.border_glyphs {
            // for t in term.iter_row_mut(top).skip(left).take(width) {
            //     glyphs.edge_tile(Edge::Top).apply(t);
            // }
            // for t in term.iter_row_mut(bottom).skip(left).take(width) {
            //     glyphs.edge_tile(Edge::Bottom).apply(t);
            // }
            // for t in term.iter_column_mut(left).skip(bottom).take(height) {
            //     glyphs.edge_tile(Edge::Left).apply(t);
            // }
            // for t in term.iter_column_mut(right).skip(bottom).take(height) {
            //     glyphs.edge_tile(Edge::Right).apply(t);
            // }

            // let left = left as i32;
            // let right = right as i32;
            // let top = top as i32;
            // let bottom = bottom as i32;

            // term.put_char([left, bottom], glyphs.edge_tile(Edge::BottomLeft));
            // term.put_char([left, top], glyphs.edge_tile(Edge::TopLeft));
            // term.put_char([right, top], glyphs.edge_tile(Edge::TopRight));
            // term.put_char([right, bottom], glyphs.edge_tile(Edge::BottomRight));
        }
    }
}
