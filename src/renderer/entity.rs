//! Terminal components
use bevy::{
    math::{uvec2, vec2},
    prelude::{Bundle, Component, Deref, Entity, UVec2, Vec2},
    sprite::MaterialMesh2dBundle,
};

use crate::{Border, Terminal, TerminalMaterial};

use super::{uv_mapping::UvMapping, TerminalFont};

#[derive(Component, Deref)]
pub struct TerminalSize(pub UVec2);

/// Layout settings for the terminal renderer.
///
/// You can modify the `scaling`, `term_pivot`, or `tile_pivot` of the layout
/// to change how the terminal is rendered.
#[derive(Debug, Component, Clone)]
pub struct TerminalLayout {
    pub scaling: TileScaling,
    pub term_pivot: Vec2,
    pub tile_pivot: Vec2,
    pub(crate) border_entity: Option<Entity>,
    pub(crate) border: Option<Border>,
    pub(crate) pixels_per_tile: UVec2,
    pub(crate) term_size: UVec2,
    pub(crate) tile_size: Vec2,
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
            border_entity: None,
            border: None,
        }
    }
}

impl TerminalLayout {
    pub fn origin(&self) -> Vec2 {
        let term_offset = -(self.term_size.as_vec2() * self.tile_size * self.term_pivot);
        let tile_offset = -(self.tile_size * self.tile_pivot);
        term_offset + tile_offset
    }

    pub fn term_size(&self) -> UVec2 {
        self.term_size
    }

    pub fn pixels_per_tile(&self) -> UVec2 {
        self.pixels_per_tile
    }

    pub(crate) fn update_state(&mut self, term: &Terminal) {
        self.border = term.border().cloned();
        self.term_size = term.size();
    }
}

/// Terminal component specifying how terminal mesh tiles will be scaled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileScaling {
    /// Each tile will take up 1 unit of world space vertically. This is the default setting.
    World,
    /// Scale terminal tiles based on the size of their texture.
    ///
    /// With this setting, 1 pixel == 1 world unit. This matches the expected
    /// defaults for bevy's orthographic camera.
    Pixels,
}

/// Bundle for a rendering a terminal.
/// Has various functions to help with the construction of a terminal.
#[derive(Default, Bundle)]
pub struct TerminalRenderBundle {
    pub render_bundle: MaterialMesh2dBundle<TerminalMaterial>,
    pub uv_mapping: UvMapping,
    pub layout: TerminalLayout,
    pub font: TerminalFont,
}

impl TerminalRenderBundle {
    pub fn new() -> Self {
        Self::default()
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

#[derive(Component)]
pub(crate) struct TerminalBorder;
