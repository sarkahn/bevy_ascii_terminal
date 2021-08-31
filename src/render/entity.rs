use std::fmt::Debug;

use bevy::prelude::*;

use super::{TerminalMesh, renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData};

#[derive(Bundle, Default)]
pub struct TerminalRendererBundle {
    pub vert_data: TerminalRendererVertexData,
    pub tile_data: TerminalRendererTileData,
    pub material: Handle<ColorMaterial>,
    pub mesh: TerminalMesh,
    #[bundle]
    pub mesh_bundle: MeshBundle,
}

impl TerminalRendererBundle {

}

// impl Default for TerminalRendererBundle {
//     fn default() -> Self {
//         Self { 
//             mesh_bundle: MeshBundle {
//                 mesh: Default::default(), // <- I want to generate a new mesh from here
//                 ..Default::default()
//             },
//             ..Default::default()
//         }
//     }
// }