use bevy::math::{IVec2, UVec2, Vec2};

/// A trait for types representing a 2d size on a grid.
pub trait GridSize: Clone + Copy {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn tile_count(&self) -> usize {
        self.width() * self.height()
    }
    fn as_ivec2(&self) -> IVec2 {
        IVec2 {
            x: self.width() as i32,
            y: self.height() as i32,
        }
    }
    fn as_uvec2(&self) -> UVec2 {
        UVec2 {
            x: self.width() as u32,
            y: self.height() as u32,
        }
    }
    fn as_vec2(&self) -> Vec2 {
        Vec2 {
            x: self.width() as f32,
            y: self.height() as f32,
        }
    }
    fn as_array(&self) -> [usize; 2] {
        [self.width(), self.height()]
    }
}

macro_rules! impl_grid_size {
    ($type:ty) => {
        impl GridSize for $type {
            fn width(&self) -> usize {
                self[0] as usize
            }

            fn height(&self) -> usize {
                self[1] as usize
            }
        }
    };
}

impl_grid_size!(UVec2);
impl_grid_size!([u32; 2]);
impl_grid_size!([i32; 2]);
impl_grid_size!([usize; 2]);
