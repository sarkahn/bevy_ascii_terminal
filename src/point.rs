use bevy::math::{IVec2, UVec2, Vec2};

/// A trait for easier mixing of the different types representing a 2d point.
pub trait Point2d {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn xy(&self) -> IVec2;
}

impl Point2d for IVec2 {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn xy(&self) -> IVec2 {
        *self
    }
}

impl Point2d for [i32; 2] {
    fn x(&self) -> i32 {
        self[0]
    }

    fn y(&self) -> i32 {
        self[1]
    }

    fn xy(&self) -> IVec2 {
        IVec2::from(*self)
    }
}

/// A trait for aligning a 2d point in a terminal.
pub trait Point2dFormatter {
    /// Return the pivot-adjusted point.
    fn point(&self, size: impl Size2d) -> IVec2;
    /// Set the pivot for this point.
    fn pivot(self, pivot: Pivot) -> FormattedPoint;
    /// Retrieve a point relative to this one given the current pivot.
    fn relative_point(&self, xy: impl Point2d) -> IVec2;
}

/// A point with optional formatting.
#[derive(Default, Clone, Copy)]
pub struct FormattedPoint {
    pub point: IVec2,
    pub pivot: Option<Pivot>,
}

impl Point2dFormatter for IVec2 {
    /// Set the pivot for this point.
    fn pivot(self, pivot: Pivot) -> FormattedPoint {
        FormattedPoint {
            point: self,
            pivot: Some(pivot),
        }
    }

    fn relative_point(&self, xy: impl Point2d) -> IVec2 {
        xy.xy()
    }

    fn point(&self, _size: impl Size2d) -> IVec2 {
        *self
    }
}

impl Point2dFormatter for [i32; 2] {
    fn pivot(self, pivot: Pivot) -> FormattedPoint {
        FormattedPoint {
            point: IVec2::from(self),
            pivot: Some(pivot),
        }
    }

    fn relative_point(&self, xy: impl Point2d) -> IVec2 {
        xy.xy()
    }

    fn point(&self, _size: impl Size2d) -> IVec2 {
        IVec2::from(*self)
    }
}

impl Point2dFormatter for FormattedPoint {
    fn pivot(mut self, pivot: Pivot) -> FormattedPoint {
        self.pivot = Some(pivot);
        self
    }

    fn relative_point(&self, xy: impl Point2d) -> IVec2 {
        match self.pivot {
            Some(p) => p.axis() * xy.xy(),
            None => xy.xy(),
        }
    }

    fn point(&self, size: impl Size2d) -> IVec2 {
        match self.pivot {
            Some(p) => p.pivot_aligned_point(self.point, size),
            None => self.point,
        }
    }
}

#[allow(clippy::len_without_is_empty)]
/// A trait for mixing of the different types representing a 2d size.
pub trait Size2d {
    fn width(&self) -> usize {
        self.size().x as usize
    }
    fn height(&self) -> usize {
        self.size().y as usize
    }
    fn size(&self) -> UVec2;
    fn as_point(&self) -> IVec2;
    fn len(&self) -> usize {
        self.width() * self.height()
    }
}

impl Size2d for [u32; 2] {
    fn size(&self) -> UVec2 {
        UVec2::new(self[0] as u32, self[1] as u32)
    }

    fn as_point(&self) -> IVec2 {
        UVec2::from(*self).as_ivec2()
    }
}

impl Size2d for UVec2 {
    fn size(&self) -> UVec2 {
        *self
    }

    fn as_point(&self) -> IVec2 {
        self.as_ivec2()
    }
}

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
    pub fn pivot_aligned_point(&self, point: impl Point2d, size: impl Size2d) -> IVec2 {
        let point = point.xy();
        let align_offset = size.as_point().as_vec2() - Vec2::ONE;
        let align_offset = (align_offset * self.pivot()).as_ivec2();

        point * self.axis() + align_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axis() {
        let xy = [0, 9].pivot(Pivot::TopLeft).point([10, 10]).to_array();
        assert_eq!([0, 0], xy);

        let xy = [9, 9].pivot(Pivot::TopRight).point([10, 10]).to_array();
        assert_eq!([0, 0], xy);

        let xy = [0, 0].pivot(Pivot::BottomLeft).point([10, 10]).to_array();
        assert_eq!([0, 0], xy);

        let xy = [9, 9].pivot(Pivot::TopRight).point([10, 10]).to_array();
        assert_eq!([0, 0], xy);

        let xy = [-1, 10].pivot(Pivot::TopLeft).point([10, 10]).to_array();
        assert_eq!([-1, -1], xy);

        let xy = [10, 10].pivot(Pivot::TopRight).point([10, 10]).to_array();
        assert_eq!([-1, -1], xy);

        let xy = [-1, -1].pivot(Pivot::BottomLeft).point([10, 10]).to_array();
        assert_eq!([-1, -1], xy);

        let xy = [10, -1]
            .pivot(Pivot::BottomRight)
            .point([10, 10])
            .to_array();
        assert_eq!([-1, -1], xy);

        let xy = [0, 0].pivot(Pivot::Center).point([10, 10]).to_array();
        assert_eq!([4, 4], xy);

        let xy = [-1, -1].pivot(Pivot::Center).point([10, 10]).to_array();
        assert_eq!([3, 3], xy);

        let xy = [1, 1].pivot(Pivot::Center).point([10, 10]).to_array();
        assert_eq!([5, 5], xy);
    }
}
