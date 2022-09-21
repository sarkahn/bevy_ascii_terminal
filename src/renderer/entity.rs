//! Terminal components
use bevy::{
    math::{vec2},
    prelude::{Bundle, Component, Vec2, UVec2, Deref},
    sprite::MaterialMesh2dBundle,
};

use crate::TerminalMaterial;

use super::{
    renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData,
    uv_mapping::UvMapping, TerminalFont,
};

/// Terminal component specifying the origin of the terminal mesh.
///
/// (0,0) is the bottom left. Defaults to (0.5,0.5).
#[derive(Component)]
pub struct TerminalPivot(pub Vec2);
impl Default for TerminalPivot {
    fn default() -> Self {
        Self(vec2(0.5, 0.5))
    }
}

/// Terminal component specifying the origin of each tile of the terminal mesh.
///
/// (0,0) is the bottom left. Defaults to (0,0).
#[derive(Component, Default)]
pub struct TilePivot(pub Vec2);

/// Terminal component specifying how terminal mesh tiles will be scaled.
#[derive(Component, Clone, Copy)]
pub enum TileScaling {
    /// Each tile will take up 1 unit of world space vertically. This is the default setting.
    World,
    /// Scale terminal tiles based on the size of their texture.
    ///
    /// With this setting, 1 pixel == 1 world unit. This matches the expected
    /// defaults for bevy's orthographic camera.
    Pixels,
}

impl Default for TileScaling {
    fn default() -> Self {
        TileScaling::World
    }
}

#[derive(Component, Default, Deref)]
pub struct PixelsPerTile(pub(crate) UVec2);

/// A bundle of all the components required to render a terminal.
///
/// Has various functions to help with the construction of a terminal.
#[derive(Default, Bundle)]
pub struct TerminalRendererBundle {
    pub vert_data: TerminalRendererVertexData,
    pub tile_data: TerminalRendererTileData,
    pub scaling: TileScaling,
    pub uv_mapping: UvMapping,
    pub terminal_pivot: TerminalPivot,
    pub tile_pivot: TilePivot,
    pub pixels_per_tile: PixelsPerTile,
    pub font: TerminalFont,
    pub bundle: MaterialMesh2dBundle<TerminalMaterial>,
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

    /// Sets the [TileScaling] for the terminal.
    pub fn with_tile_scaling(mut self, scaling: TileScaling) -> Self {
        self.scaling = scaling;
        self
    }
}
