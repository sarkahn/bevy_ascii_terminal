//! Utilities for formatting strings and chars written to the terminal.

use crate::Tile;
use bevy::prelude::*;

/// A pivot point on a 2d rect.
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Pivot {
    /// +X Right, +Y Down.
    TopLeft,
    /// +X Left, +Y Down.
    TopRight,
    /// +X Right, +Y Up.
    Center,
    /// +X Right, +Y Up.
    BottomLeft,
    /// +X Left, +Y Up
    BottomRight,
}

impl Pivot {
    /// A Vec2 used to derive a pivot offset from the size of a
    /// 2d rect.
    pub fn pivot(&self) -> Vec2 {
        match self {
            Pivot::TopLeft => Vec2::new(0.0, 1.0),
            Pivot::TopRight => Vec2::new(1.0, 1.0),
            Pivot::Center => Vec2::new(0.5, 0.5),
            Pivot::BottomLeft => Vec2::new(0.0, 0.0),
            Pivot::BottomRight => Vec2::new(1.0, 0.0),
        }
    }

    /// Coordinate axis for adjusting an aligned position on a 2d rect.
    pub fn axis(&self) -> IVec2 {
        match self {
            Pivot::TopLeft => IVec2::new(1, -1),
            Pivot::TopRight => IVec2::new(-1, -1),
            Pivot::Center => IVec2::new(1, 1),
            Pivot::BottomLeft => IVec2::new(1, 1),
            Pivot::BottomRight => IVec2::new(-1, 1),
        }
    }

    /// Transform a point to it's equivalent from the perspective of
    /// a pivot on a 2d rect.
    pub fn pivot_aligned_point(&self, point: [i32; 2], size: [u32; 2]) -> IVec2 {
        let point = IVec2::from(point);
        let align_offset = UVec2::from(size).as_vec2() - Vec2::ONE;
        let align_offset = (align_offset * self.pivot()).as_ivec2();
        
        point * self.axis() + align_offset
    }
}

/// Describes formatting for a char written to the terminal.
#[derive(Copy, Clone)]
pub struct CharFormat {
    pub fg_color: Color,
    pub bg_color: Color,
    pub pivot: Pivot,
}

impl Default for CharFormat {
    fn default() -> Self {
        Self {
            fg_color: Color::WHITE,
            bg_color: Color::BLACK,
            pivot: Pivot::BottomLeft,
        }
    }
}

impl CharFormat {
    pub fn new(fg_color: Color, bg_color: Color) -> Self {
        Self {
            fg_color,
            bg_color,
            ..Default::default()
        }
    }

    pub fn with_fg_color(mut self, color: Color) -> Self {
        self.fg_color = color;
        self
    }

    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.fg_color = color;
        self
    }

    pub fn with_pivot(mut self, pivot: Pivot) -> Self {
        self.pivot = pivot;
        self
    }

    pub(crate) fn tile(&self, glyph: char) -> Tile {
        Tile {
            glyph,
            fg_color: self.fg_color,
            bg_color: self.bg_color,
        }
    }
}

/// Describes formatting for a string written to the terminal.
#[derive(Copy, Clone)]
pub struct StringFormat {
    pub pivot: Pivot,
    pub fg_color: Color,
    pub bg_color: Color,
}

impl Default for StringFormat {
    fn default() -> Self {
        Self {
            pivot: Pivot::BottomLeft,
            fg_color: Color::WHITE,
            bg_color: Color::BLACK,
        }
    }
}

impl StringFormat {
    pub fn new(pivot: Pivot, fg_color: Color, bg_color: Color) -> Self {
        Self {
            pivot,
            fg_color,
            bg_color,
        }
    }

    pub fn colors(fg_color: Color, bg_color: Color) -> Self {
        Self {
            fg_color,
            bg_color,
            ..Default::default()
        }
    }

    pub fn pivot(pivot: Pivot) -> Self {
        pivot.into()
    }

    pub fn with_pivot(mut self, alignment: Pivot) -> Self {
        self.pivot = alignment;
        self
    }

    pub fn with_fg_color(mut self, color: Color) -> Self {
        self.fg_color = color;
        self
    }

    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    pub(crate) fn get_string_position(&self, pos: [i32; 2], size: [u32; 2], string: &str) -> IVec2 {
        let len = string.len() - 1;
        let origin = self.pivot.pivot_aligned_point(pos, size);
        let string_offset = (len as f32 * -self.pivot.pivot().x) as i32;
        IVec2::new(origin.x + string_offset, origin.y)
    }

    pub(crate) fn tile(&self, glyph: char) -> Tile {
        Tile {
            glyph,
            fg_color: self.fg_color,
            bg_color: self.bg_color,
        }
    }
}

impl From<Pivot> for StringFormat {
    fn from(pivot: Pivot) -> Self {
        Self {
            pivot,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis() {
        let xy: [i32; 2] = Pivot::TopLeft.pivot_aligned_point([0, 9], [10, 10]).into();
        assert_eq!([0, 0], xy);
        let xy: [i32; 2] = Pivot::TopRight.pivot_aligned_point([9, 9], [10, 10]).into();
        assert_eq!([0, 0], xy);
        let xy: [i32; 2] = Pivot::BottomLeft
            .pivot_aligned_point([0, 0], [10, 10])
            .into();
        assert_eq!([0, 0], xy);
        let xy: [i32; 2] = Pivot::BottomRight
            .pivot_aligned_point([9, 0], [10, 10])
            .into();
        assert_eq!([0, 0], xy);

        let xy: [i32; 2] = Pivot::TopLeft
            .pivot_aligned_point([-1, 10], [10, 10])
            .into();
        assert_eq!([-1, -1], xy);
        let xy: [i32; 2] = Pivot::TopRight
            .pivot_aligned_point([10, 10], [10, 10])
            .into();
        assert_eq!([-1, -1], xy);
        let xy: [i32; 2] = Pivot::BottomLeft
            .pivot_aligned_point([-1, -1], [10, 10])
            .into();
        assert_eq!([-1, -1], xy);
        let xy: [i32; 2] = Pivot::BottomRight
            .pivot_aligned_point([10, -1], [10, 10])
            .into();
        assert_eq!([-1, -1], xy);

        let xy: [i32; 2] = Pivot::Center.pivot_aligned_point([0, 0], [10, 10]).into();
        assert_eq!([4, 4], xy);
        let xy: [i32; 2] = Pivot::Center.pivot_aligned_point([-1, -1], [10, 10]).into();
        assert_eq!([3, 3], xy);
        let xy: [i32; 2] = Pivot::Center.pivot_aligned_point([1, 1], [10, 10]).into();
        assert_eq!([5, 5], xy);
    }

    #[test]
    fn string_positioning() {
        let formatter = StringFormat::default().with_pivot(Pivot::TopRight);

        let p = formatter.get_string_position([0, 0], [10, 10], "Hello");
        assert_eq!([5, 9], p.to_array());

        let p = formatter.get_string_position([1, 2], [10, 10], "Hello");
        assert_eq!([4, 7], p.to_array());

        let formatter = StringFormat::default().with_pivot(Pivot::BottomRight);

        let p = formatter.get_string_position([0, 0], [10, 10], "Hello");
        assert_eq!([5, 0], p.to_array());

        let p = formatter.get_string_position([1, 2], [10, 10], "Hello");
        assert_eq!([4, 2], p.to_array());
    }
}
