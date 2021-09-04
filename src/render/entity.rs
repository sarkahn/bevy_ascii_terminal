use bevy::{
    prelude::*,
    render::{pipeline::RenderPipeline, render_graph::base::MainPass},
};


use crate::render::plugin::TERMINAL_RENDERER_PIPELINE;

use super::{TerminalPivot, TerminalRendererFont, TerminalMaterial, TerminalTileScaling, TilePivot, renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData};

#[derive(Bundle)]
pub struct TerminalRendererBundle {
    pub vert_data: TerminalRendererVertexData,
    pub tile_data: TerminalRendererTileData,
    pub font: TerminalRendererFont,
    //pub material: Handle<ColorMaterial>,
    pub terminal_mat: Handle<TerminalMaterial>,
    pub scaling: TerminalTileScaling,
    pub mesh: Handle<Mesh>,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub main_pass: MainPass,
    pub terminal_pivot: TerminalPivot,
    pub tile_pivot: TilePivot,
}

impl Default for TerminalRendererBundle {
    fn default() -> Self {
        let pipeline = RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            TERMINAL_RENDERER_PIPELINE.typed(),
        )]);
        Self {
            render_pipelines: pipeline,
            visible: Default::default(),
            vert_data: Default::default(),
            tile_data: Default::default(),
            font: Default::default(),
            //material: Default::default(),
            terminal_mat: Default::default(),
            scaling: Default::default(),
            mesh: Default::default(),
            draw: Default::default(),
            main_pass: Default::default(),
            terminal_pivot: Default::default(),
            tile_pivot: Default::default(),
        }
    }
}
