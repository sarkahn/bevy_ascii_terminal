use bevy::{math::{Vec2, Vec3}, prelude::*, render::mesh::VertexAttributeValues};

use crate::{Terminal, TerminalSize};

#[derive(Default, Clone)]
struct TileRenderData {
    tile_index: [usize;2],
    fg_color: [f32;4],
    bg_color: [f32;4],
}

struct TerminalRenderer {
    mesh: Handle<Mesh>,
    render_tiles: Vec<TileRenderData>,
    verts: Vec<Vec3>,
    uvs: Vec<Vec2>,
    indices: Vec<usize>,
    size: (usize, usize)
}

impl TerminalRenderer {
    pub fn resize(&mut self, size: (usize,usize)) {
        self.size = size;
        let (width, height) = size;
        let len = width * height;
        self.render_tiles.resize(len, TileRenderData::default());
        self.verts.resize(len * 4, Vec3::ZERO);
        self.uvs.resize(len * 4, Vec2::ZERO);
        self.indices.resize(len * 6, 0);
    }
}

impl Plugin for TerminalRenderer {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        todo!()
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}


fn terminal_mesh_data_resize( mut q: Query<(&TerminalSize, &mut TerminalRenderer), Changed<TerminalSize>> ) {

    for(size, mut renderer) in  q.iter_mut() {


        if renderer.size != size.into() {
            renderer.resize(size.into());
        }

        let (width,height) = size.into();

        for x in 0..width {
            for y in 0..height {
                let i = y * width + x;

                let origin = Vec3::new(x as f32, y as f32, 0.0);
                let right = Vec3::X;
                let up = Vec3::Y;

                let vi = i * 4;
                // 0---1
                // | / | 
                // 2---3
                let verts = &mut renderer.verts;
                verts[vi + 0] = origin + up;
                verts[vi + 1] = origin + right + up;
                verts[vi + 2] = origin;
                verts[vi + 3] = origin + right;

                let ii = i * 6;
                let indices = &mut renderer.indices;
                indices[ii + 0] = vi + 0;
                indices[ii + 1] = vi + 1;
                indices[ii + 2] = vi + 2;
                indices[ii + 3] = vi + 3;
                indices[ii + 4] = vi + 2;
                indices[ii + 5] = vi + 1;
            }
        }
    }
}

fn terminal_mesh_data_update(mut q: Query<(&Terminal, &mut TerminalRenderer), Changed<Terminal>>) {
    for (term, mut renderer) in q.iter_mut() {
        let (width, _) = term.size();
        for (i, tile) in term.iter().enumerate() {
            let x = i % width;
            let y = i / width;

            let glyph = tile.glyph as usize;
            // y is flipped
            let glyph_index = [glyph % 16, 16 - 1 - (glyph / 16)];
            
            renderer.render_tiles[i] = TileRenderData {
                tile_index: glyph_index,
                fg_color: tile.fg_color.into(),
                bg_color: tile.bg_color.into(),
            };

            let uv_size = Vec2::new(1.0 / 16.0, 1.0 / 16.0);
            let right = Vec2::new(uv_size.x, 0.0);
            let up = Vec2::new(0.0, uv_size.y);
            let origin = Vec2::new(glyph_index[0] as f32 * uv_size.x, glyph_index[1] as f32 * uv_size.y);

            let vi = i * 4;
            let uvs = &mut renderer.uvs;
            uvs[vi + 0] = origin + up;
            uvs[vi + 1] = origin + up + right;
            uvs[vi + 2] = origin;
            uvs[vi + 3] = origin + right;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blah() {

    }
}