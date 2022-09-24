use bevy::prelude::{Changed, Component, Query};

use crate::{Terminal, Tile};

use super::uv_mapping::UvMapping;

#[derive(Component, Default)]
pub struct TerminalRendererTileData {
    pub fg_colors: Vec<[f32; 4]>,
    pub bg_colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
}

#[derive(Component, Default)]
pub struct TerminalRendererVertexData {
    pub verts: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

fn terminal_renderer_update_tile_data(
    mut q: Query<(&Terminal, &mut TerminalRendererTileData, &UvMapping), Changed<Terminal>>,
) {
    for (term, mut data, uv_mapping) in q.iter_mut() {
        //info!("Renderer update tile data (colors)!");
        //info!("First tiles: {:?}", &term.tiles[0..4]);
        for (i, tile) in term.iter().enumerate() {
            let glyph = tile.glyph;

            let vi = i * 4;
            let uvs = &mut data.uvs;

            let glyph_uvs = uv_mapping.uvs_from_glyph(glyph);

            for (a, b) in uvs[vi..vi + 4].iter_mut().zip(glyph_uvs) {
                *a = *b;
            }

            for j in vi..vi + 4 {
                data.fg_colors[j] = tile.fg_color.as_linear_rgba_f32();
                data.bg_colors[j] = tile.bg_color.as_linear_rgba_f32();
            }
        }
    }
}
