use std::ops::Sub;

use bevy::{
    math::{IVec2, UVec2, Vec2},
    reflect::Reflect,
};

#[derive(Debug, Default, Reflect, Clone, Copy, PartialEq, Eq)]
pub enum Pivot {
    LeftBottom, // X right, Y up
    LeftCenter, // X right, Y up
    #[default]
    LeftTop, // X right, Y down
    CenterBottom, // X right, Y up
    Center,     // X right, Y up
    CenterTop,  // X right, Y down,
    RightBottom, // X left, Y up
    RightCenter, // X left, Y up
    RightTop,   // X left, Y down
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

    /// Transform a point from a bottom-left origin to the pivot origin
    pub fn transform_point(&self, point: impl Into<IVec2>, grid_size: impl Into<UVec2>) -> IVec2 {
        self.pivot_position(grid_size.into().as_ivec2()) + self.transform_coordinates(point)
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
}
