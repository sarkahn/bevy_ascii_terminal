//! Traits for dealing with 2d points on a grid.
use super::{GridSize, IVec2, UVec2, Vec2};

use super::{
    direction::{DIR_4, DIR_8},
    Pivot, PivotedPoint,
};

/// A trait for types representing an integer point on a 2d grid.
///
/// This trait is implemented for most 2d types, such as [IVec2], [UVec2], [i32;2], etc
pub trait GridPoint: Clone + Copy {
    fn xy(&self) -> IVec2;

    fn x(&self) -> i32 {
        self.xy().x
    }
    fn y(&self) -> i32 {
        self.xy().y
    }

    fn width(&self) -> usize {
        self.x() as usize
    }

    fn height(&self) -> usize {
        self.y() as usize
    }

    fn as_ivec2(&self) -> IVec2 {
        IVec2::new(self.x(), self.y())
    }

    fn as_uvec2(&self) -> UVec2 {
        self.as_ivec2().as_uvec2()
    }
    fn as_vec2(&self) -> Vec2 {
        self.as_ivec2().as_vec2()
    }

    fn as_array(&self) -> [i32; 2] {
        self.as_ivec2().to_array()
    }

    fn as_usize_array(&self) -> [usize; 2] {
        [self.x() as usize, self.y() as usize]
    }

    /// Get the grid point's corresponding 1d index.
    #[inline]
    fn as_index(&self, size: impl GridSize) -> usize {
        let p = self.as_ivec2();
        debug_assert!(
            p.cmpge(IVec2::ZERO).all() && p.cmplt(size.as_ivec2()).all(),
            "Attemting to convert an out of bounds grid position {:?} into a 1d index from a grid size of {}", self.as_array(), size.as_ivec2()
        );
        self.y() as usize * size.width() + self.x() as usize
    }

    /// Returns the grid point the given number of spaces above this one.
    fn up(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x(), self.y() + amount)
    }

    /// Returns the grid point the given number of spaces below this one.
    fn down(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x(), self.y() - amount)
    }

    /// Returns the grid point the given number of spaces to the right of
    /// this one.
    fn right(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x() + amount, self.y())
    }

    /// Returns the grid point the given number of spaces to the left of
    /// this one.
    fn left(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x() - amount, self.y())
    }

    /// Returns this grid point offset by the given amount.
    fn offset(&self, xy: impl GridPoint) -> IVec2 {
        self.as_ivec2() + xy.as_ivec2()
    }

    fn min(&self, other: impl GridPoint) -> IVec2 {
        self.as_ivec2().min(other.as_ivec2())
    }

    fn max(&self, other: impl GridPoint) -> IVec2 {
        self.as_ivec2().max(other.as_ivec2())
    }

    /// Applies a [Pivot] to this position, which can be used to calculate a
    /// final pivot adjusted point within a sized rect.
    ///
    /// ## Example:
    ///
    /// ```
    /// let point = [0,0].pivoted(Pivot::TopRight);
    /// assert_eq!([8,8], point.calc_from_size([9,9]));
    /// ```
    fn pivot(self, pivot: Pivot) -> PivotedPoint {
        PivotedPoint::new(self, pivot)
    }

    /// The [taxicab distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
    /// between two grid points.
    #[inline]
    fn taxi_dist(self, other: impl GridPoint) -> usize {
        let d = (self.as_ivec2() - other.as_ivec2()).abs();
        (d.x + d.y) as usize
    }

    /// Linearly interpolate between points a and b by the amount t.
    #[inline]
    fn lerp(self, other: impl GridPoint, t: f32) -> IVec2 {
        self.as_vec2().lerp(other.as_vec2(), t).as_ivec2()
    }

    /// Returns an iterator over the 4 grid points orgthogonally adjacent to
    /// this one.
    #[inline]
    fn adj_4(&self) -> AdjIterator {
        AdjIterator {
            i: 0,
            p: self.as_ivec2(),
            arr: DIR_4,
        }
    }

    /// Returns an iterator over the 8 points adjacent to this one.
    #[inline]
    fn adj_8(&self) -> AdjIterator {
        AdjIterator {
            i: 0,
            p: self.as_ivec2(),
            arr: DIR_8,
        }
    }
}

pub struct AdjIterator<'a> {
    i: usize,
    p: IVec2,
    arr: &'a [IVec2],
}

impl<'a> Iterator for AdjIterator<'a> {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.arr.len() {
            return None;
        };

        let p = self.p + self.arr[self.i];
        self.i += 1;

        Some(p)
    }
}

macro_rules! impl_grid_point {
    ($type:ty) => {
        impl GridPoint for $type {
            fn xy(&self) -> IVec2 {
                IVec2::new(self[0] as i32, self[1] as i32)
            }
        }
    };
}

impl_grid_point!(IVec2);
impl_grid_point!(UVec2);
impl_grid_point!([u32; 2]);
impl_grid_point!([i32; 2]);
impl_grid_point!([usize; 2]);

#[cfg(test)]
mod tests {
    use crate::GridPoint;

    #[test]
    fn taxi() {
        let a = [10, 10];
        let b = [20, 20];

        let dist = GridPoint::taxi_dist(a, b);
        assert_eq!(dist, 20);
    }

    #[test]
    fn adj() {
        let points: Vec<_> = [10, 10].adj_4().map(|p| p.to_array()).collect();
        assert!(points.contains(&[10, 9]));
        assert!(points.contains(&[9, 10]));
        assert!(points.contains(&[11, 10]));
        assert!(points.contains(&[10, 11]));

        let points: Vec<_> = [10, 10].adj_8().map(|p| p.to_array()).collect();
        assert!(points.contains(&[10, 9]));
        assert!(points.contains(&[9, 10]));
        assert!(points.contains(&[11, 10]));
        assert!(points.contains(&[10, 11]));
        assert!(points.contains(&[11, 11]));
        assert!(points.contains(&[9, 9]));
        assert!(points.contains(&[11, 9]));
        assert!(points.contains(&[9, 11]));
    }
}
