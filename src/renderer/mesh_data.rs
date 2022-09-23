use bevy::prelude::{Vec2, Vec3, Component};
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


pub(crate) struct VertHelper<'a> {
    pub origin: Vec2,
    pub tile_size: Vec2,
    pub data: &'a mut VertexData,
}

impl<'a> VertHelper<'a> {
    pub fn tile_at(&mut self, xy: impl GridPoint) {
        let tile_size = self.tile_size;
        let pos = self.origin;

        let right = Vec3::X * tile_size.x;
        let up = Vec3::Y * tile_size.y;
        let xy = (xy.as_vec2() * tile_size).extend(0.0);
        let pos = pos.extend(0.0) + xy;
        let i = self.data.verts.len();
        // 0---2
        // | / |
        // 1---3
        let verts = &mut self.data.verts;

        verts.extend(&[
            (pos + up).into(),
            pos.into(),
            (pos + right + up).into(),
            (pos + right).into(),
        ]);

        let vi = i as u32;
        let indices = &mut self.data.indices;

        indices.extend(&[vi + 0, vi + 1, vi + 2, vi + 3, vi + 2, vi + 1]);
    }
}
