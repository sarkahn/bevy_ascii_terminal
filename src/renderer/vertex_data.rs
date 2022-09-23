use bevy::{
    math::{vec2, vec3, UVec2, Vec2, Vec3},
    prelude::Component,
};
use sark_grids::{point::Point2d, GridPoint, Size2d};

#[derive(Component, Default)]
pub struct VertexData {
    pub verts: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl VertexData {
    pub fn with_size(size: impl Size2d) -> Self {
        let mut v = Self::default();
        v.terminal_resize([0, 0], size, Vec2::ONE);
        v
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

    pub fn terminal_resize(
        &mut self,
        origin: impl Point2d,
        term_size: impl Size2d,
        tile_size: Vec2,
    ) {
        let len = term_size.len();
        let width = term_size.width();

        self.clear();
        self.reserve(len);

        let mut helper = VertHelper {
            origin: origin.as_vec2(),
            tile_size,
            data: self,
        };

        for i in 0..len {
            let x = i % width;
            let y = i / width;

            helper.tile_at([x, y]);
        }
    }

    pub fn border_resize(&mut self, origin: impl Point2d, term_size: UVec2, tile_size: Vec2) {
        let width = term_size.width() + 2;
        let height = term_size.height() + 2;
        let len = (width * 2) + ((height - 2) * 2);
        let origin = origin.as_vec2();

        self.verts.clear();
        self.verts
            .reserve((len * 4).saturating_sub(self.verts.capacity()));

        self.indices.clear();
        self.indices
            .reserve((len * 6).saturating_sub(self.indices.capacity()));

        let top = height - 1;
        let bottom = 0;
        let left = 0;
        let right = width - 1;

        let mut helper = VertHelper {
            origin,
            tile_size,
            data: self,
        };

        helper.tile_at([left, bottom]);
        helper.tile_at([left, top]);
        helper.tile_at([right, top]);
        helper.tile_at([right, bottom]);

        for x in 1..width - 1 {
            helper.tile_at([x, bottom]);
            helper.tile_at([x, top]);
        }

        for y in 1..height - 1 {
            helper.tile_at([left, y]);
            helper.tile_at([right, y]);
        }
    }
}

struct VertHelper<'a> {
    origin: Vec2,
    tile_size: Vec2,
    data: &'a mut VertexData,
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
