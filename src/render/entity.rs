use bevy::prelude::*;

use super::{
    renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData,
    TerminalRendererFont,
};

#[derive(Bundle, Default)]
pub struct TerminalRendererBundle {
    pub vert_data: TerminalRendererVertexData,
    pub tile_data: TerminalRendererTileData,
    pub font: TerminalRendererFont,
    pub material: Handle<ColorMaterial>,
    #[bundle]
    pub mesh_bundle: MeshBundle,
}
