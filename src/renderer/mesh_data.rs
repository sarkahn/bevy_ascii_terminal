use bevy::prelude::{Component, Vec2, Vec3};
use sark_grids::{GridPoint, Size2d};

#[derive(Component, Default)]
pub struct VertexData {
    pub verts: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl VertexData {
    pub fn with_tile_count(cap: usize) -> Self {
        Self {
            verts: Vec::with_capacity(cap * 4),
            indices: Vec::with_capacity(cap * 6),
        }
    }

    pub fn clear(&mut self) {
        self.verts.clear();
        self.indices.clear();
    }

    pub fn reserve(&mut self, len: usize) {
        self.verts
            .reserve((len * 4).saturating_sub(self.verts.capacity()));
        self.indices
            .reserve((len * 6).saturating_sub(self.indices.capacity()));
    }
}
