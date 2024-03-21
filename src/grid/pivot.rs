//! A pivot point on a 2d grid.
use std::ops::Sub;

use bevy::reflect::Reflect;

use super::{IVec2, Vec2};

use crate::GridPoint;

/// A pivot point on a 2d grid rect.
#[derive(Eq, PartialEq, Clone, Copy, Debug, Reflect)]
pub enum Pivot {
    /// X axis increases to the right, Y axis increases downwards.
    TopLeft,
    /// X axis increases to the right, Y axis increases downwards.
    TopCenter,
    /// X axis increases to the left, Y axis increases downwards.
    TopRight,
    /// X axis increases to the right, Y axis increases upwards.
    LeftCenter,
    /// X axis increases to the right, Y axis increases upwards.
    Center,
    /// X axis increases to the left, Y axis increases upwards.
    RightCenter,
    /// X axis increases to the right, Y axis increases upwards.
    BottomLeft,
    /// X axis increases to the right, Y axis increases upwards.
    BottomCenter,
    /// X axis increases to the left, Y axis increases upwards.
    BottomRight,
}

impl Pivot {
    /// Coordinate axis for each pivot, used when transforming a point into
    /// the pivot's coordinate space.
    #[inline]
    pub fn axis(&self) -> IVec2 {
        match self {
            Pivot::TopLeft => IVec2::new(1, -1),
            Pivot::TopRight => IVec2::new(-1, -1),
            Pivot::Center => IVec2::new(1, 1),
            Pivot::BottomLeft => IVec2::new(1, 1),
            Pivot::BottomRight => IVec2::new(-1, 1),
            Pivot::TopCenter => IVec2::new(1, -1),
            Pivot::LeftCenter => IVec2::new(1, 1),
            Pivot::RightCenter => IVec2::new(-1, 1),
            Pivot::BottomCenter => IVec2::new(1, 1),
        }
    }

    /// The normalized value of this pivot, in the `[0.0, 0.0]..[1.0, 1.0]`
    /// range, where `[0.0, 0.0]` is the bottom left and `[1.0, 1.0]` is the top
    /// right.
    #[inline]
    pub fn normalized(&self) -> Vec2 {
        match self {
            Pivot::TopLeft => Vec2::new(0.0, 1.0),
            Pivot::TopRight => Vec2::new(1.0, 1.0),
            Pivot::Center => Vec2::new(0.5, 0.5),
            Pivot::BottomLeft => Vec2::new(0.0, 0.0),
            Pivot::BottomRight => Vec2::new(1.0, 0.0),
            Pivot::TopCenter => Vec2::new(0.5, 1.0),
            Pivot::LeftCenter => Vec2::new(0.0, 0.5),
            Pivot::RightCenter => Vec2::new(1.0, 0.5),
            Pivot::BottomCenter => Vec2::new(0.5, 0.0),
        }
    }

    /// Transform a "world" grid position (bottom left origin) to it's equivalent
    /// position from the perspective of this pivot.
    ///
    /// This assumes a world origin of `[0,0]`.
    #[inline]
    pub fn transform_point(&self, point: impl GridPoint) -> IVec2 {
        point.as_ivec2() * self.axis()
    }

    /// Transform a point from the perspective of this pivot back to it's
    /// equivalent "world" grid position.
    ///
    /// This assumes a world origin of `[0,0]`.
    #[inline]
    pub fn inverse_transform_point(&self, point: impl GridPoint) -> IVec2 {
        point.as_ivec2() * -self.axis()
    }

    /// Retrieve the pivot offset for a rect of the given size.
    #[inline]
    pub fn size_offset(&self, size: impl GridPoint) -> IVec2 {
        (size.as_vec2().sub(1.0) * self.normalized())
            .round()
            .as_ivec2()
    }
}

/// A grid point that may optionally have a pivot applied to it.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PivotedPoint {
    point: IVec2,
    pivot: Option<Pivot>,
}

impl PivotedPoint {
    pub fn new(xy: impl GridPoint, pivot: Pivot) -> Self {
        Self {
            point: xy.as_ivec2(),
            pivot: Some(pivot),
        }
    }

    /// Calculate the pivot adjusted position within a sized rect.
    ///
    /// If no pivot is applied, this will return the position.
    #[inline]
    pub fn calc_from_size(&self, size: impl GridPoint) -> IVec2 {
        if let Some(pivot) = self.pivot {
            pivot.size_offset(size) + self.point * pivot.axis()
        } else {
            self.point
        }
    }

    pub fn pivot(&self) -> Option<Pivot> {
        self.pivot
    }
}

impl<T: GridPoint> From<T> for PivotedPoint {
    fn from(value: T) -> Self {
        Self {
            point: value.as_ivec2(),
            pivot: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_offset() {
        assert_eq!([4, 4], Pivot::TopRight.size_offset([5, 5]).to_array());
        assert_eq!([2, 2], Pivot::Center.size_offset([5, 5]).to_array());
        assert_eq!([3, 3], Pivot::TopRight.size_offset([4, 4]).to_array());
        assert_eq!([2, 2], Pivot::Center.size_offset([4, 4]).to_array());
    }

    #[test]
    fn pivoted_point() {
        let pp = [1, 1].pivot(Pivot::TopLeft);
        assert_eq!([1, 3], pp.calc_from_size([5, 5]).to_array());

        let pp = [1, 1].pivot(Pivot::TopRight);
        assert_eq!([3, 3], pp.calc_from_size([5, 5]).to_array());

        let pp = [1, 1].pivot(Pivot::TopRight);
        assert_eq!([4, 4], pp.calc_from_size([6, 6]).to_array());

        let pp = [1, 1].pivot(Pivot::Center);
        assert_eq!([4, 4], pp.calc_from_size([6, 6]).to_array());

        let pp = [1, 1].pivot(Pivot::Center);
        assert_eq!([3, 3], pp.calc_from_size([5, 5]).to_array());

        let pp = [0, 0].pivot(Pivot::BottomRight);
        assert_eq!([8, 0], pp.calc_from_size([9, 9]).to_array());
    }
}
