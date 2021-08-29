use bevy::{math::{Vec2, Vec3}, prelude::*, render::mesh::{Indices}};

use crate::{Terminal, TerminalSize, Tile};

#[derive(Default)]
struct TerminalRenderer {
    mesh: Handle<Mesh>,
    verts: Vec<[f32;3]>,
    uvs: Vec<[f32;2]>,
    fg_colors: Vec<[f32;3]>,
    bg_colors: Vec<[f32;3]>,
    indices: Vec<u32>,
    size: (usize, usize)
}

impl TerminalRenderer {
    pub fn with_size(width: usize, height: usize) -> Self {
        let mut v = TerminalRenderer::default();
        v.resize(width,height);
        v
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.size = (width,height);
        let len = width * height;

        self.verts.resize(len * 4, Vec3::ZERO.into());
        self.uvs.resize(len * 4, Vec2::ZERO.into());
        self.fg_colors.resize(len * 4, Default::default());
        self.bg_colors.resize(len * 4, Default::default());
        self.indices.resize(len * 6, 0);

        for i in 0..len {
            let x = i % width;
            let y = i / width;
            let origin = Vec3::new(x as f32, y as f32, 0.0);
            let right = Vec3::X;
            let up = Vec3::Y;

            let vi = i * 4;
            // 0---1
            // | / | 
            // 2---3
            let verts = &mut self.verts;
            verts[vi + 0] = (origin + up).into();
            verts[vi + 1] = (origin + right + up).into();
            verts[vi + 2] = origin.into();
            verts[vi + 3] = (origin + right).into();

            let ii = i * 6;
            let indices = &mut self.indices;
            indices[ii + 0] = (vi + 0) as u32;
            indices[ii + 1] = (vi + 1) as u32;
            indices[ii + 2] = (vi + 2) as u32;
            indices[ii + 3] = (vi + 3) as u32;
            indices[ii + 4] = (vi + 2) as u32;
            indices[ii + 5] = (vi + 1) as u32;
        }
    }

    pub fn update_uvs_from_tiles(&mut self, tiles: &Vec<Tile>) {
        let uv_size = Vec2::new(1.0 / 16.0, 1.0 / 16.0);
        let right = Vec2::new(uv_size.x, 0.0);
        let up = Vec2::new(0.0, uv_size.y);

        for (i, tile) in tiles.iter().enumerate() {
            let glyph = tile.glyph as usize;
            // y is flipped
            let glyph_index = [glyph % 16, 16 - 1 - (glyph / 16)];
            
            let origin = Vec2::new(glyph_index[0] as f32 * uv_size.x, glyph_index[1] as f32 * uv_size.y);

            let vi = i * 4;
            let uvs = &mut self.uvs;
            uvs[vi + 0] = (origin + up).into();
            uvs[vi + 1] = (origin + up + right).into();
            uvs[vi + 2] = origin.into();
            uvs[vi + 3] = (origin + right).into();


            let rgb = &tile.fg_color;
            let rgb = [rgb.r(), rgb.g(), rgb.b()];

            let colors = &mut self.fg_colors;
            colors[vi + 0] = rgb;
            colors[vi + 1] = rgb;
            colors[vi + 2] = rgb;
            colors[vi + 3] = rgb;

            let rgb = &tile.bg_color;
            let rgb = [rgb.r(), rgb.g(), rgb.b()];

            let colors = &mut self.bg_colors;
            colors[vi + 0] = rgb;
            colors[vi + 1] = rgb;
            colors[vi + 2] = rgb;
            colors[vi + 3] = rgb;
        }
    }
}

fn terminal_mesh_data_resize( 
    meshes: &mut ResMut<Assets<Mesh>>,
    mut q: Query<(&TerminalSize, &mut TerminalRenderer), Changed<TerminalSize>> ) {
    for(size, mut renderer) in  q.iter_mut() {

        let size = size.into();
        if renderer.size != size {
            renderer.resize(size.0, size.1);

            // TODO: Should update all mesh data at once at the end of the frame (if the tile data changed). 
            // We don't want different length attributes in the mesh.
            let mesh = meshes.get_mut(renderer.mesh.clone()).unwrap();
            mesh.set_indices(Some(Indices::U32(renderer.indices.clone())));
            mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, renderer.verts.clone());
        }
    }
}

fn terminal_mesh_data_update(
    meshes: &mut ResMut<Assets<Mesh>>,
    mut q: Query<(&Terminal, &mut TerminalRenderer), Changed<Terminal>>) {
    for (term, mut renderer) in q.iter_mut() {
        renderer.update_uvs_from_tiles(&term.data);
    }
}

fn update_terminal_mesh_verts(
    meshes: &mut ResMut<Assets<Mesh>>,
    q: Query<&TerminalRenderer, Changed<TerminalSize>>
) {
    for renderer in q.iter() {
        let mesh = meshes.get_mut(renderer.mesh.clone()).unwrap();
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, renderer.uvs.clone());
        mesh.set_attribute("FG_Color", renderer.fg_colors.clone());
        mesh.set_attribute("BG_Color", renderer.bg_colors.clone());
        // TODO: Need to force data to update here as well (or later in the frame, maybe
        // set <Terminal> changed somehow?) 
    }

}

fn update_terminal_mesh_data(
    meshes: &mut ResMut<Assets<Mesh>>,
    q: Query<&TerminalRenderer, Changed<Terminal>> 
) {
    for renderer in q.iter() {
        let mesh = meshes.get_mut(renderer.mesh.clone()).unwrap();
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, renderer.uvs.clone());
        mesh.set_attribute("FG_Color", renderer.fg_colors.clone());
        mesh.set_attribute("BG_Color", renderer.bg_colors.clone());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize_test() {
        let renderer = TerminalRenderer::with_size(5,5);

        assert_eq!(renderer.verts.len(), 5*5*4);
        assert_eq!(renderer.uvs.len(), 5*5*4);
        assert_eq!(renderer.fg_colors.len(), 5*5*4);
        assert_eq!(renderer.bg_colors.len(), 5*5*4);
        assert_eq!(renderer.indices.len(), 5*5*6);
    }

    #[test]
    fn update_data_test() {
        let tile = Tile {
            glyph: 'a',
            ..Default::default()
        };

        let tiles = vec![tile; 4];
        let mut renderer = TerminalRenderer::with_size(2,2);
        renderer.update_uvs_from_tiles(&tiles);

        assert_eq!(renderer.fg_colors[0], [1.0, 1.0, 1.0]);
        assert_ne!(renderer.uvs[0], [0.0,0.0]);
    }
}