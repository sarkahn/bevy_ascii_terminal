pub mod bundle;
pub mod font;
pub mod plugin;

mod glyph_mapping;

pub(crate) mod renderer_tile_data;
pub(crate) mod renderer_vertex_data;

use self::{
    renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData,
};
use crate::terminal::{Terminal, TerminalSize};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::Indices, pipeline::PrimitiveTopology, renderer::RenderResources, shader::ShaderDefs,
    },
};

pub struct TerminalPivot(pub Vec2);
impl Default for TerminalPivot {
    fn default() -> Self {
        Self(Vec2::new(0.5, 0.5))
    }
}

#[derive(Default)]
pub struct TilePivot(Vec2);

#[derive(Clone, Copy)]
pub enum TerminalTileScaling {
    /// Scale terminal tiles based on the size of their texture, such that 1 pixel == 1 world unit.
    /// This behavior matches the expected defaults for bevy's orthographic camera.
    Pixels,
    /// Each tile will take up 1 unit of world space
    World,
}

impl Default for TerminalTileScaling {
    fn default() -> Self {
        TerminalTileScaling::Pixels
    }
}

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
