use std::fmt;

use bevy::math::{IVec2, UVec2, XY};
use itertools::{Itertools, IntoChunks};

use crate::formatting::Pivot;



pub struct GridRect {
    pub min: IVec2,
    // Inclusive
    pub max: IVec2,
}

impl GridRect {
    pub fn new(pos: [i32;2], size: [u32;2]) -> Self {
        debug_assert!(size[0] > 0 && size[1] > 0, "GridRect size must be greater than 0" );
        let min = IVec2::from(pos);
        let max = min + UVec2::from(size).as_ivec2() - IVec2::ONE;
        Self {
            min,
            max
        }
    }

    pub fn from_min_max(min: [i32;2], max: [i32;2]) -> Self {
        let min = IVec2::from(min);
        let max = IVec2::from(max);
        let low = IVec2::min(min, max);
        let high = IVec2::max(min, max);
        Self {
            min: low,
            max: high
        }
    }

    pub fn size(&self) -> UVec2 {
        (self.max - self.min).as_uvec2() + UVec2::ONE
    }

    pub fn position(&self) -> IVec2 {
        self.min
    }

    pub fn width(&self) -> usize {
        (self.max.x - self.min.x) as usize
    }

    pub fn height(&self) -> usize {
        (self.max.y - self.min.y) as usize
    }

    pub fn set_position(&mut self, pos: [i32;2]) {
        let size = self.size();
        self.min = IVec2::from(pos);
        self.max = self.min + size.as_ivec2();
    }

    pub fn rows_iter(&self) -> IntoChunks<impl Iterator<Item=IVec2>> {
        self.iter().chunks(self.width())
    }

    pub fn iter_cols(&self) -> IntoChunks<impl Iterator<Item=IVec2>> {
        self.iter_vert().chunks(self.height())
    }

    pub fn iter_vert(&self) ->impl Iterator<Item=IVec2> {
        (self.min.x..self.max.x).cartesian_product(self.min.y..self.max.y).map(
            |(x,y)| IVec2::new(x,y)
        )
    }

    pub fn iter(&self) -> impl Iterator<Item=IVec2> {
        (self.min.x..=self.max.x).cartesian_product(self.min.y..=self.max.y).map(
            |(y,x)| IVec2::new(x,y)
        )
    }

    pub fn iter_from_pivot(&self, pivot: Pivot) -> impl Iterator<Item=IVec2> {
        let origin = pivot.pivot_aligned_point([0,0], self.size().into());

        let axis = pivot.axis();

        let box_size = self.size().as_ivec2();

        let xy_iter = (0..box_size.y).cartesian_product(0..box_size.x);
        
        let origin = self.min + origin;

        xy_iter.map(move |(y,x)| origin + IVec2::new(x * axis.x, y * axis.y))
    }

    pub fn rect_from_pivot(&self, pivot: Pivot, pos: [i32;2], size: [u32; 2]) -> GridRect {
        let pos = IVec2::from(pos);
        let size = UVec2::from(size);
        let a = pivot.pivot_aligned_point(pos.into(), self.size().into());
        let b = pivot.pivot_aligned_point((pos + size.as_ivec2() - IVec2::ONE).into(), self.size().into());

        GridRect::from_min_max(a.into(),b.into())
    }
}

impl fmt::Display for GridRect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Min {}, Max {}, Size {})", self.min, self.max, self.size())
    }
}

#[cfg(test)]
mod test {
    use bevy::math::IVec2;
    use itertools::Itertools;

    use crate::formatting::Pivot;

    use super::GridRect;

    #[test]
    fn iter() {
        let rect = GridRect::from_min_max([1,1], [5,5]);
        let vec = rect.iter().collect_vec();
        assert_eq!(vec[0], IVec2::new(1,1));
        assert_eq!(*vec.last().unwrap(), IVec2::new(5,5));
    }

    #[test]
    fn iter_from_pivot() {
        let rect = GridRect::new([2,2], [3,3]);
        let mut iter = rect.iter_from_pivot(Pivot::TopRight);
        assert_eq!(iter.next().unwrap().to_array(), [4,4]);
        assert_eq!(iter.last().unwrap().to_array(), [2,2]);
    }

    #[test]
    fn cols_iter() {
        let rect = GridRect::new([3,3], [2,3]);

        let iter = rect.iter_cols();
        let mut chunk_iter = iter.into_iter();

        let mut chunk = chunk_iter.next().unwrap();
        assert_eq!(chunk.next().unwrap().to_array(), [3,3]);
        assert_eq!(chunk.last().unwrap().to_array(), [3,5]);
        
        let mut chunk = chunk_iter.next().unwrap();
        assert_eq!(chunk.next().unwrap().to_array(), [4,3]);
        assert_eq!(chunk.last().unwrap().to_array(), [4,5]);
    }

    #[test]
    fn rect_from_pivot() {
        let rect = GridRect::new([1,1], [10,5]);

        println!("{}", rect);

        let other = rect.rect_from_pivot(Pivot::TopRight, [1,1], [3,3]);

        println!("Other {}", other);

        //assert_eq!(other.max.to_array(), [9,4]);
        //assert_eq!(other.min.to_array(), [7,2]);
    }
}