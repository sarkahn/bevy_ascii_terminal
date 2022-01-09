use bevy::{math::{UVec2, Vec2, Vec3}, prelude::Component};

use super::{TerminalPivot, TilePivot};

#[derive(Component, Default)]
pub struct TerminalRendererVertexData {
    pub verts: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl TerminalRendererVertexData {
    pub fn with_size(size: UVec2) -> Self {
        let mut v = Self::default();
        let term_pivot = TerminalPivot::default();
        let tile_pivot = TilePivot::default();
        v.resize(size, term_pivot.0, tile_pivot.0, UVec2::ONE);
        v
    }

    pub fn resize(
        &mut self,
        term_size: UVec2,
        term_pivot: Vec2,
        tile_pivot: Vec2,
        tile_size: UVec2,
    ) {
        let len = (term_size.x * term_size.y) as usize;

        let size = term_size.as_vec2();
        let tile_size = tile_size.as_vec2();

        let world_size = size * tile_size;

        let term_pivot = world_size * term_pivot;
        let tile_pivot = tile_size * tile_pivot;

        let term_pivot = -term_pivot.extend(0.0);
        let tile_pivot = -tile_pivot.extend(0.0);

        self.verts.resize(len * 4, Default::default());
        self.indices.resize(len * 6, 0);

        let (tile_width, tile_height) = tile_size.into();

        for i in 0..len {
            let x = (i % term_size.x as usize) as f32 * tile_width;
            // Flip y: y0 == the top of the terminal
            let y = size.y as usize - 1 - (i / term_size.x as usize);
            let y = y as f32 * tile_height;
            //let y = (i / term_size.x as usize) as f32 * tile_height;
            let origin = Vec3::new(x, y, 0.0) + term_pivot + tile_pivot;
            let right = Vec3::X * tile_width;
            let up = Vec3::Y * tile_height;

            let vi = i * 4;
            // 0---2
            // | / |
            // 1---3
            let verts = &mut self.verts;
            verts[vi] = (origin + up).into();
            verts[vi + 1] = origin.into();
            verts[vi + 2] = (origin + right + up).into();
            verts[vi + 3] = (origin + right).into();

            let ii = i * 6;
            let vi = vi as u32;
            let indices = &mut self.indices;
            indices[ii] =     vi;
            indices[ii + 1] = vi + 1;
            indices[ii + 2] = vi + 2;
            indices[ii + 3] = vi + 3;
            indices[ii + 4] = vi + 2;
            indices[ii + 5] = vi + 1;
        }
    }
}