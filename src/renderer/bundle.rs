use bevy::{prelude::*, sprite::Mesh2dHandle};

use super::{renderer_vertex_data::TerminalRendererVertexData, renderer_tile_data::TerminalRendererTileData, TerminalFont, TileScaling};





/// A bundle of all the components required to render a terminal.
///
/// Has various functions to help with the construction of a terminal.
#[derive(Bundle)]
pub struct TerminalRendererBundle {
    pub vert_data: TerminalRendererVertexData,
    pub tile_data: TerminalRendererTileData,
    pub font: TerminalFont,
    pub scaling: TileScaling,
    pub mesh: Mesh2dHandle,
    pub material: Handle<TerminalMaterial>,
    pub terminal_pivot: TerminalPivot,
    pub tile_pivot: TilePivot,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for TerminalRendererBundle {
    fn default() -> Self {
        Self {
            vert_data: Default::default(),
            tile_data: Default::default(),
            font: Default::default(),
            scaling: Default::default(),
            mesh: Default::default(),
            material: Default::default(),
            terminal_pivot: Default::default(),
            tile_pivot: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}
