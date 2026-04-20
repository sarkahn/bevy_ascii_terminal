use std::ops::Sub;

use bevy::{
    math::{IVec2, UVec2, Vec2},
    reflect::Reflect,
};

#[derive(Debug, Default, Reflect, Clone, Copy, PartialEq, Eq)]
pub enum Pivot {
    #[default]
    LeftTop, // X right, Y down
    LeftBottom,   // X right, Y up
    LeftCenter,   // X right, Y up
    CenterBottom, // X right, Y up
    Center,       // X right, Y up
    CenterTop,    // X right, Y down,
    RightBottom,  // X left, Y up
    RightCenter,  // X left, Y up
    RightTop,     // X left, Y down
}

impl Pivot {
    pub fn axis(&self) -> IVec2 {
        IVec2::from(match self {
            Pivot::LeftBottom => [1, 1],
            Pivot::LeftCenter => [1, 1],
            Pivot::LeftTop => [1, -1],
            Pivot::CenterTop => [1, -1],
            Pivot::Center => [1, 1],
            Pivot::CenterBottom => [1, 1],
            Pivot::RightTop => [-1, -1],
            Pivot::RightBottom => [-1, 1],
            Pivot::RightCenter => [-1, 1],
        })
    }

    /// Calculate the position of a pivot on a sized grid.
    pub fn pivot_position(&self, grid_size: impl Into<IVec2>) -> IVec2 {
        (grid_size.into().as_vec2().sub(1.0) * self.normalized())
            .round()
            .as_ivec2()
    }

    pub fn normalized(&self) -> Vec2 {
        match self {
            Pivot::LeftTop => Vec2::new(0.0, 1.0),
            Pivot::LeftBottom => Vec2::new(0.0, 0.0),
            Pivot::LeftCenter => Vec2::new(0.0, 0.5),
            Pivot::RightTop => Vec2::new(1.0, 1.0),
            Pivot::RightCenter => Vec2::new(1.0, 0.5),
            Pivot::RightBottom => Vec2::new(1.0, 0.0),
            Pivot::Center => Vec2::new(0.5, 0.5),
            Pivot::CenterTop => Vec2::new(0.5, 1.0),
            Pivot::CenterBottom => Vec2::new(0.5, 0.0),
        }
    }

    /// Transform a point into the pivot's coordinate space.
    pub fn transform_coordinates(&self, grid_point: impl Into<IVec2>) -> IVec2 {
        grid_point.into() * self.axis()
    }

    /// Transform a point into the pivot's coordinate space.
    #[inline]
    pub fn transform_axis(&self, grid_point: impl Into<IVec2>) -> IVec2 {
        grid_point.into() * self.axis()
    }

    /// Transform a point from a bottom-left origin to the pivot origin
    pub fn transform_point(&self, point: impl Into<IVec2>, grid_size: impl Into<UVec2>) -> IVec2 {
        self.pivot_position(grid_size.into().as_ivec2()) + self.transform_coordinates(point)
    }
}

pub trait PivotPoint {
    fn pivot(&self, pivot: Pivot) -> PivotedPoint;
}

impl<T: Into<IVec2> + Copy> PivotPoint for T {
    fn pivot(&self, pivot: Pivot) -> PivotedPoint {
        let p = (*self).into();
        PivotedPoint {
            point: p,
            pivot: Some(pivot),
        }
    }
}

impl From<UVec2> for PivotedPoint {
    fn from(value: UVec2) -> Self {
        PivotedPoint {
            point: value.as_ivec2(),
            pivot: None,
        }
    }
}

impl From<IVec2> for PivotedPoint {
    fn from(value: IVec2) -> Self {
        PivotedPoint {
            point: value,
            pivot: None,
        }
    }
}

impl From<[i32; 2]> for PivotedPoint {
    fn from(value: [i32; 2]) -> Self {
        PivotedPoint {
            point: IVec2::from_array(value),
            pivot: None,
        }
    }
}

impl From<[u32; 2]> for PivotedPoint {
    fn from(value: [u32; 2]) -> Self {
        PivotedPoint {
            point: UVec2::from_array(value).as_ivec2(),
            pivot: None,
        }
    }
}

impl From<[usize; 2]> for PivotedPoint {
    fn from(value: [usize; 2]) -> Self {
        PivotedPoint {
            point: IVec2::new(value[0] as i32, value[1] as i32),
            pivot: None,
        }
    }
}

/// A grid point that may optionally have a pivot applied to it. This can be used
/// to override the terminal pivot for a certain operation.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PivotedPoint {
    pub point: IVec2,
    pub pivot: Option<Pivot>,
}

impl PivotedPoint {
    pub fn new(xy: impl Into<IVec2>, pivot: Pivot) -> Self {
        Self {
            point: xy.into(),
            pivot: Some(pivot),
        }
    }

    /// Calculate the final pivoted position on a sized grid.
    ///
    /// Transforms into the pivot's coordinate space if a pivot is applied,
    /// returns the original point if no pivot is applied.
    pub fn calculate(&self, grid_size: impl Into<UVec2>) -> IVec2 {
        if let Some(pivot) = self.pivot {
            pivot.pivot_position(grid_size.into().as_ivec2()) + pivot.transform_axis(self.point)
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

#[cfg(test)]
mod tests {
    use bevy::math::ivec2;

    use super::*;

    #[test]
    fn center_negative() {
        let pivot = Pivot::Center;
        let p = pivot.transform_point([-2, -2], [10, 10]);
        assert_eq!(ivec2(3, 3), p);
    }

    fn point_taker(xy: impl Into<PivotedPoint>) -> IVec2 {
        let pp = xy.into();
        pp.calculate([10, 10])
    }

    #[test]
    fn pivot_point() {
        let p = point_taker([1, 1].pivot(Pivot::LeftTop));
        assert_eq!([1, 8], p.to_array());
    }
}
