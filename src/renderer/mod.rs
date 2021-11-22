//! Handles mesh construction and rendering for the terminal.

pub mod font;
pub mod plugin;

pub use font::{TerminalFont, TerminalFonts};
pub use plugin::TerminalRendererPlugin;

pub(crate) mod renderer_tile_data;
pub(crate) mod renderer_vertex_data;

mod glyph_mapping;

use self::{
    renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData,
};
use crate::{renderer::plugin::TERMINAL_RENDERER_PIPELINE, terminal::Terminal};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::Indices, pipeline::PrimitiveTopology, pipeline::RenderPipeline,
        render_graph::base::MainPass, renderer::RenderResources, shader::ShaderDefs,
    },
};

/// Terminal component specifying the origin of the terminal mesh.
///
/// (0,0) is the bottom left. Defaults to (0.5,0.5).
pub struct TerminalPivot(pub Vec2);
impl Default for TerminalPivot {
    fn default() -> Self {
        Self(Vec2::new(0.5, 0.5))
    }
}

/// Terminal component specifying the origin of each tile of the terminal mesh will be.
///
/// (0,0) is the bottom left. Defaults to (0,0).
#[derive(Default)]
pub struct TilePivot(Vec2);

/// Terminal component specifying how terminal mesh tiles will be scaled.
#[derive(Clone, Copy)]
pub enum TerminalTileScaling {
    /// Each tile will take up 1 unit of world space.
    ///
    /// This matches how [TiledCamera](https://crates.io/crates/bevy_tiled_camera) is set up.
    World,
    /// Scale terminal tiles based on the size of their texture.
    ///
    /// With this setting, 1 pixel == 1 world unit. This matches the expected
    /// defaults for bevy's orthographic camera.
    Pixels,
}

impl Default for TerminalTileScaling {
    fn default() -> Self {
        TerminalTileScaling::World
    }
}

/// The material for the terminal renderer.
#[derive(Debug, RenderResources, ShaderDefs, Default, TypeUuid)]
#[uuid = "1e01121c-0b4a-315e-1bca-36733b11127e"]
pub struct TerminalMaterial {
    pub color: Color,
    pub clip_color: Color,
    #[shader_def] // This doesn't work for some reason...
    pub texture: Option<Handle<Texture>>,
}

impl TerminalMaterial {
    pub fn from_texture(tex: Handle<Texture>, clip_color: Color) -> Self {
        TerminalMaterial {
            color: Color::WHITE,
            clip_color,
            texture: Some(tex),
        }
    }
}

/// A bundle of all the components required to render a terminal.
#[derive(Bundle)]
pub struct TerminalRendererBundle {
    pub vert_data: TerminalRendererVertexData,
    pub tile_data: TerminalRendererTileData,
    pub font: TerminalFont,
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

impl TerminalRendererBundle {
    pub fn new() -> Self {
        TerminalRendererBundle::default()
    }

    /// Set the terminal pivot value.
    ///
    /// Terminal pivot determines where the origin of the terminal mesh sits, where
    /// (0,0) is the bottom left. Defaults to centered (0.5,0.5).
    pub fn with_terminal_pivot(mut self, x: f32, y: f32) -> Self {
        self.terminal_pivot.0 = (x, y).into();
        self
    }

    /// Set the tile pivot value.
    ///
    /// Tile pivot determines where the origin of a tile sits within the mesh, where
    /// (0,0) is the bottom left. Defaults to bottom left (0,0).
    pub fn with_tile_pivot(mut self, x: f32, y: f32) -> Self {
        self.tile_pivot.0 = (x, y).into();
        self
    }
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
