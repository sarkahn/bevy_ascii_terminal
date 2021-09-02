use bevy::{prelude::*, render::pipeline::RenderPipeline};

use crate::render::pipeline::TERMINAL_RENDERER_PIPELINE;

use super::{
    renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData,
    TerminalRendererFont,
};

#[derive(Bundle)]
pub struct TerminalRendererBundle {
    pub vert_data: TerminalRendererVertexData,
    pub tile_data: TerminalRendererTileData,
    pub font: TerminalRendererFont,
    pub material: Handle<ColorMaterial>,
    #[bundle]
    pub mesh_bundle: MeshBundle,
}

impl Default for TerminalRendererBundle {
    fn default() -> Self {
        let pipeline = RenderPipelines::from_pipelines(vec![RenderPipeline::new(
           TERMINAL_RENDERER_PIPELINE.typed(),
        )]);
        Self {
            vert_data: Default::default(),
            tile_data: Default::default(),
            font: Default::default(),
            material: Default::default(),
            mesh_bundle: MeshBundle { 
                render_pipelines: pipeline,
                ..Default::default()
            },
        }
    }
}
