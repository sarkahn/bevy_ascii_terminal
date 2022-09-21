use bevy::{
    math::{UVec2, Vec2, Vec3, vec3, vec2},
    prelude::Component,
};
use sark_grids::{GridPoint, point::Point2d, Size2d};

use super::{TerminalPivot, TilePivot};

#[derive(Component, Default)]
pub struct TerminalRendererVertexData {
    pub verts: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl TerminalRendererVertexData {
    pub fn with_size(size: impl Size2d) -> Self {
        let mut v = Self::default();
        v.terminal_resize([0,0], size, Vec2::ONE);
        v
    }

    pub fn terminal_resize(&mut self, origin: impl Point2d, term_size: impl Size2d, tile_size: Vec2) {
        let len = term_size.len();
        let width = term_size.width();

        self.verts.clear();
        self.verts.reserve((len * 4).saturating_sub(self.verts.capacity()));

        self.indices.clear();
        self.indices.reserve((len * 6).saturating_sub(self.indices.capacity()));

        let mut builder = TileHelper::new(
            origin.as_vec2(), 
            tile_size, 
            &mut self.verts, &mut self.indices
        );

        for i in 0..len {
            let x = i % width;
            let y = i / width;

            builder.tile_at([x,y]);
        }

    }

    pub fn border_resize(&mut self, origin: impl Point2d, term_size: UVec2, tile_size: Vec2) {
        let width = term_size.width() + 2;
        let height = term_size.height() + 2;
        let len = (width * 2) + ((height - 2) * 2);
        let p = origin.as_vec2();

        self.verts.clear();
        self.verts.reserve((len * 4).saturating_sub(self.verts.capacity()));

        self.indices.clear();
        self.indices.reserve((len * 6).saturating_sub(self.indices.capacity()));

        let top = height - 1;
        let bottom = 0;
        let left = 0;
        let right = width - 1;

        let mut builder = TileHelper::new(
            p, tile_size, &mut self.verts, &mut self.indices,
        );

        builder.tile_at([left,bottom]);
        builder.tile_at([left,top]);
        builder.tile_at([right,top]);
        builder.tile_at([right,bottom]);

        for x in 1..width - 1 {
            builder.tile_at([x, bottom]);
            builder.tile_at([x, top]);
        }

        for y in 1..height - 1 {
            builder.tile_at([left, y] );
            builder.tile_at([right,y] );
        } 
    }
}

struct TileHelper<'a> {
    origin: Vec2,
    tile_size: Vec2,
    verts: &'a mut Vec<[f32;3]>,
    indices: &'a mut Vec<u32>,
} 

impl<'a> TileHelper<'a> {
    pub fn new(
        origin: Vec2,
        tile_size: Vec2,
        verts: &'a mut Vec<[f32;3]>, 
        indices: &'a mut Vec<u32>,
    ) -> Self {
        TileHelper { 
            origin, tile_size, verts, indices 
        }
    }

    pub fn tile_at(&mut self, xy: impl GridPoint) {
        let tile_size = self.tile_size;
        let pos = self.origin;

        let right = Vec3::X * tile_size.x;
        let up = Vec3::Y * tile_size.y;
        let xy = (xy.as_vec2() * tile_size).extend(0.0);
        let pos = pos.extend(0.0) + xy;
        let i = self.verts.len();
        // 0---2
        // | / |
        // 1---3
        let verts = &mut self.verts;

        verts.extend(&[
            (pos + up).into(),
            pos.into(),
            (pos + right + up).into(),
            (pos + right).into(),
        ]);

        let vi = i as u32;
        let indices = &mut self.indices;

        indices.extend(&[
            vi + 0,
            vi + 1,
            vi + 2,
            vi + 3,
            vi + 2,
            vi + 1,
            ]);
    }
}