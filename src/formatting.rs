use bevy::prelude::*;
use itertools::{Itertools, Chunk, IntoChunks};

use crate::Tile;


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
    pub fn pivot(&self) -> Vec2 {
        match self {
            Pivot::TopLeft => Vec2::new(0.0,1.0),
            Pivot::TopRight => Vec2::new(1.0,1.0),
            Pivot::Center => Vec2::new(0.5,0.5),
            Pivot::BottomLeft => Vec2::new(0.0,0.0),
            Pivot::BottomRight => Vec2::new(1.0,0.0),
        }
    }

    /// Coridnate axis for adjusting an aligned position.
    pub fn axis(&self) -> IVec2 {
        match self {
            Pivot::TopLeft => IVec2::new(1,-1),
            Pivot::TopRight => IVec2::new(-1,-1),
            Pivot::Center => IVec2::new(1,1),
            Pivot::BottomLeft => IVec2::new(1,1),
            Pivot::BottomRight => IVec2::new(-1,1),
        }
    }

    /// Transform a point to it's equivalent from the perspective of this pivot.
    pub fn pivot_aligned_point(&self, point: [i32;2], size: [u32;2]) -> IVec2 {
        let point = IVec2::from(point);
        let align_offset = UVec2::from(size).as_vec2() - Vec2::ONE;
        let align_offset = (align_offset * self.pivot()).as_ivec2();
        let point = point * self.axis() + align_offset;
        point
    }

    pub fn pivot_aligned_rect_iter(&self, point: [i32;2], total_size: [u32;2], rect_size: [u32;2]) 
    -> IntoChunks<impl Iterator<Item=IVec2>>
    {

        let origin = self.pivot_aligned_point(point, total_size);

        let axis = self.axis();

        let box_size = UVec2::from(rect_size).as_ivec2();

        let xy_iter = (0..box_size.x).cartesian_product(0..box_size.y);

        xy_iter.map(move |(y,x)| origin + IVec2::new(x * axis.x, y * axis.y)).chunks(box_size.x as usize)
    }

    pub fn rect_iter(&self, point: [i32;2], rect_size: [u32;2], total_size: [u32;2]) {
        let origin = self.pivot_aligned_point(point, total_size);

        let axis = self.axis();

        let box_size = UVec2::from(rect_size).as_ivec2();

        let bottom_left = origin + (box_size * axis);


    }
}

pub struct CharFormatter {
    tile: Tile,
    alignment: Pivot,
}

impl CharFormatter {
    pub fn with_fg_color(mut self, color: Color) -> Self {
        self.tile.fg_color = color.into();
        self
    }

    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.tile.fg_color = color.into();
        self
    }

    pub fn with_glyph(mut self, glyph: char) -> Self {
        self.tile.glyph = glyph;
        self
    }

    pub fn with_alignment(mut self, alignment: Pivot) -> Self {
        self.alignment = alignment;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn axis() {
        let xy: [i32;2] = Pivot::TopLeft.pivot_aligned_point([0,9], [10,10]).into();
        assert_eq!([0,0], xy);
        let xy: [i32;2] = Pivot::TopRight.pivot_aligned_point([9,9], [10,10]).into();
        assert_eq!([0,0], xy);
        let xy: [i32;2] = Pivot::BottomLeft.pivot_aligned_point([0,0], [10,10]).into();
        assert_eq!([0,0], xy);
        let xy: [i32;2] = Pivot::BottomRight.pivot_aligned_point([9,0], [10,10]).into();
        assert_eq!([0,0], xy);
    
        let xy: [i32;2] = Pivot::TopLeft.pivot_aligned_point([-1,10], [10,10]).into();
        assert_eq!([-1,-1], xy);
        let xy: [i32;2] = Pivot::TopRight.pivot_aligned_point([10,10], [10,10]).into();
        assert_eq!([-1,-1], xy);
        let xy: [i32;2] = Pivot::BottomLeft.pivot_aligned_point([-1,-1], [10,10]).into();
        assert_eq!([-1,-1], xy);
        let xy: [i32;2] = Pivot::BottomRight.pivot_aligned_point([10,-1], [10,10]).into();
        assert_eq!([-1,-1], xy);
    
        let xy: [i32;2] = Pivot::Center.pivot_aligned_point([0,0], [10,10]).into();
        assert_eq!([4,4], xy);
        let xy: [i32;2] = Pivot::Center.pivot_aligned_point([-1,-1], [10,10]).into();
        assert_eq!([3,3], xy);
        let xy: [i32;2] = Pivot::Center.pivot_aligned_point([1,1], [10,10]).into();
        assert_eq!([5,5], xy);
    }
    
    #[test]
    fn rect_iter() {
        let iter = Pivot::TopRight.pivot_aligned_rect_iter([0,0], [15,7], [3,3]);
        
        for chunk in &iter {
            chunk.skip(1).take(3).for_each(|v|println!("{}",v));
        }
    }
    
}