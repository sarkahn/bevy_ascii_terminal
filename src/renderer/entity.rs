//! Terminal components
use bevy::{
    math::{uvec2, vec2},
    prelude::{Bundle, Component, Deref, Entity, UVec2, Vec2, Handle},
    sprite::MaterialMesh2dBundle,
};
use sark_grids::Pivot;

use crate::{Border, Terminal, TerminalMaterial};

use super::{uv_mapping::UvMapping, TerminalFont, mesh_data::{TileData, VertData}};

#[derive(Component, Deref)]
pub struct TerminalSize(pub UVec2);

/// Layout settings for the terminal renderer.
///
/// You can modify the `scaling`, `term_pivot`, or `tile_pivot` of the layout
/// to change how the terminal is rendered.
#[derive(Debug, Component, Clone)]
pub struct TerminalLayout {
    pub scaling: TileScaling,
    pub pivot: Pivot,
    //pub(crate) border_entity: Option<Entity>,
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
            pivot: Pivot::Center,
            //border_entity: None,
            border: None,
        }
    }
}

impl TerminalLayout {
    pub fn origin(&self) -> Vec2 {
        let pivot = Vec2::from(self.pivot);
        -(self.term_size.as_vec2() * self.tile_size * pivot)
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

    pub fn width(&self) -> usize {
        self.term_size.x as usize
    }

    pub fn height(&self) -> usize {
        self.term_size.y as usize
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
    pub uv_mapping: Handle<UvMapping>,
    pub tile_data: TileData,
    pub vert_data: VertData,
}

impl TerminalRenderBundle {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Component)]
pub(crate) struct TerminalBorder;
