//! A pivot point on a 2d grid.
use std::ops::Sub;

use bevy::reflect::Reflect;

use bevy::prelude::{IVec2, Vec2};
use enum_ordinalize::Ordinalize;

use crate::GridPoint;

use super::GridSize;

/// A pivot on a 2d sized grid. Can be used to set positions relative to a given
/// pivot. Each pivot has it's own coordinate space it uses to calculate
/// the final adjusted position.
#[derive(Eq, PartialEq, Clone, Copy, Debug, Reflect, Ordinalize)]
pub enum Pivot {
    /// Coordinate space: X increases to the right, Y increases downwards.
    TopLeft,
    /// Coordinate space: X increases to the right, Y increases downwards.
    TopCenter,
    /// Coordinate space: X increases to the left, Y increases downwards.
    TopRight,
    /// Coordinate space: X increases to the right, Y increases upwards.
    LeftCenter,
    /// Coordinate space: X increases to the left, Y increases upwards.
    RightCenter,
    /// Coordinate space: X increases to the right, Y increases upwards.
    BottomLeft,
    /// Coordinate space: X increases to the right, Y increases upwards.
    BottomCenter,
    /// Coordinate space: X increases to the left, Y increases upwards.
    BottomRight,
    /// Coordinate space: X increases to the right, Y increases upwards.
    Center,
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

    /// The normalized value of this pivot in default coordinate space where
    /// `[0.0, 0.0]` is the bottom left and `[1.0, 1.0]` is the top right.
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

    /// Transform a point into the pivot's coordinate space.
    #[inline]
    pub fn transform_point(&self, grid_point: impl GridPoint) -> IVec2 {
        grid_point.to_ivec2() * self.axis()
    }

    /// Calculate the position of a pivot on a sized grid.
    #[inline]
    pub fn pivot_position(&self, grid_size: impl GridSize) -> IVec2 {
        (grid_size.to_vec2().sub(1.0) * self.normalized())
            .round()
            .as_ivec2()
    }
}

/// A grid point that may optionally have a pivot applied to it.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PivotedPoint {
    pub point: IVec2,
    pub pivot: Option<Pivot>,
}

impl PivotedPoint {
    pub fn new(xy: impl GridPoint, pivot: Pivot) -> Self {
        Self {
            point: xy.to_ivec2(),
            pivot: Some(pivot),
        }
    }

    /// Calculate the final pivoted position on a sized grid.
    ///
    /// Transforms into the pivot's coordinate space if a pivot is applied,
    /// returns the original point if no pivot is applied.
    pub fn calculate(&self, grid_size: impl GridSize) -> IVec2 {
        if let Some(pivot) = self.pivot {
            pivot.pivot_position(grid_size) + pivot.transform_point(self.point)
        } else {
            self.point
        }
    }

    /// Returns a new PivotedPoint with this point's pivot or a default applied
    /// to it if this point doesn't have one.
    pub fn with_default_pivot(&self, default_pivot: Pivot) -> PivotedPoint {
        Self {
            point: self.point,
            pivot: Some(self.pivot.unwrap_or(default_pivot)),
        }
    }
}

impl<T: GridPoint> From<T> for PivotedPoint {
    fn from(value: T) -> Self {
        Self {
            point: value.to_ivec2(),
            pivot: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_pivot_size_offset() {
        assert_eq!([4, 4], Pivot::TopRight.pivot_position([5, 5]).to_array());
        assert_eq!([2, 2], Pivot::Center.pivot_position([5, 5]).to_array());
        assert_eq!([3, 3], Pivot::TopRight.pivot_position([4, 4]).to_array());
        assert_eq!([2, 2], Pivot::Center.pivot_position([4, 4]).to_array());
    }

    #[test]
    fn pivoted_point() {
        let pp = [1, 1].pivot(Pivot::TopLeft);
        assert_eq!([1, 3], pp.calculate([5, 5]).to_array());

        let pp = [1, 1].pivot(Pivot::TopRight);
        assert_eq!([3, 3], pp.calculate([5, 5]).to_array());

        let pp = [1, 1].pivot(Pivot::TopRight);
        assert_eq!([4, 4], pp.calculate([6, 6]).to_array());

        let pp = [1, 1].pivot(Pivot::Center);
        assert_eq!([4, 4], pp.calculate([6, 6]).to_array());

        let pp = [1, 1].pivot(Pivot::Center);
        assert_eq!([3, 3], pp.calculate([5, 5]).to_array());

        let pp = [0, 0].pivot(Pivot::BottomRight);
        assert_eq!([8, 0], pp.calculate([9, 9]).to_array());
    }
}
