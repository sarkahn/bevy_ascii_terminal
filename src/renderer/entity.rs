//! Terminal components
use bevy::{
    math::{uvec2, vec2},
    prelude::{Bundle, Component, Deref, UVec2, Vec2},
    sprite::MaterialMesh2dBundle,
};

use crate::{Terminal, TerminalMaterial};

use super::{mesh_data::VertexData, tile_data::TileData, uv_mapping::UvMapping, TerminalFont};

#[derive(Component, Deref)]
pub struct TerminalSize(pub UVec2);

#[derive(Debug, Component, Clone, Copy)]
pub struct TerminalLayout {
    pub tile_size: Vec2,
    pub scaling: TileScaling,
    pub pixels_per_tile: UVec2,
    pub term_pivot: Vec2,
    pub tile_pivot: Vec2,
    term_size: UVec2,
    has_border: bool,
}

impl Default for TerminalLayout {
    fn default() -> Self {
        Self {
            term_size: UVec2::ONE,
            tile_size: Vec2::ONE,
            scaling: TileScaling::World,
            pixels_per_tile: uvec2(8, 8),
            term_pivot: vec2(0.5, 0.5),
            tile_pivot: Vec2::ZERO,
            has_border: false,
        }
    }
}

impl TerminalLayout {
    pub fn origin(&self) -> Vec2 {
        let term_size = self.term_size.as_vec2();
        let term_offset = -(self.term_size.as_vec2() * self.tile_size * self.term_pivot);
        let tile_offset = -(self.tile_size * self.tile_pivot);
        term_offset + tile_offset
    }

    pub fn term_size(&self) -> UVec2 {
        self.term_size
    }

    pub fn has_border(&self) -> bool {
        self.has_border
    }

    pub(crate) fn update_state(&mut self, term: &Terminal) {
        self.has_border = term.border.is_some();
        self.term_size = term.size();
    }
}

// /// Terminal component specifying the origin of the terminal mesh.
// ///
// /// (0,0) is the bottom left. Defaults to (0.5,0.5).
// #[derive(Component)]
// pub struct TerminalPivot(pub Vec2);
// impl Default for TerminalPivot {
//     fn default() -> Self {
//         Self(vec2(0.5, 0.5))
//     }
// }

// /// Terminal component specifying the origin of each tile of the terminal mesh.
// ///
// /// (0,0) is the bottom left. Defaults to (0,0).
// #[derive(Component, Default)]
// pub struct TilePivot(pub Vec2);

/// Terminal component specifying how terminal mesh tiles will be scaled.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileScaling {
    /// Each tile will take up 1 unit of world space vertically. This is the default setting.
    World,
    /// Scale terminal tiles based on the size of their texture.
    ///
    /// With this setting, 1 pixel == 1 world unit. This matches the expected
    /// defaults for bevy's orthographic camera.
    Pixels,
}

// #[derive(Component, Default, Deref)]
// pub struct PixelsPerTile(pub(crate) UVec2);

/// A bundle of all the components required to render a terminal.
///
/// Has various functions to help with the construction of a terminal.
#[derive(Default, Bundle)]
pub struct TerminalRendererBundle {
    pub mesh_bundle: MaterialMesh2dBundle<TerminalMaterial>,
    pub vert_data: VertexData,
    pub tile_data: TileData,
    pub uv_mapping: UvMapping,
    pub layout: TerminalLayout,
    pub font: TerminalFont,
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
        self.layout.term_pivot = (x, y).into();
        self
    }

    /// Set the tile pivot value.
    ///
    /// Tile pivot determines where the origin of a tile sits within the mesh, where
    /// (0,0) is the bottom left. Defaults to bottom left (0,0).
    pub fn with_tile_pivot(mut self, x: f32, y: f32) -> Self {
        self.layout.tile_pivot = (x, y).into();
        self
    }

    /// Sets the [TileScaling] for the terminal.
    pub fn with_tile_scaling(mut self, scaling: TileScaling) -> Self {
        self.layout.scaling = scaling;
        self
    }
}
