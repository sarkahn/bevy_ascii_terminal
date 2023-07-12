//! Handles mesh construction and rendering for the terminal.

mod border_mesh;
mod entity;
mod layout;
mod material;
mod mesh;
mod mesh_data;
mod terminal_mesh;

mod font;
mod uv_mapping;

#[cfg(feature = "camera")]
mod camera;

pub mod code_page_437;

use bevy::prelude::{App, IntoSystemConfigs, Last, Plugin, SystemSet};
pub(crate) use font::BuiltInFontHandles;

pub use entity::*;

pub use font::TerminalFont;

pub use layout::TerminalLayout;
pub use material::TerminalMaterial;

#[cfg(feature = "camera")]
pub use camera::{AutoCamera, TiledCamera, TiledCameraBundle};

/// System set for the terminal mesh initialization system.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TerminalInit;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MeshInit;

/// System set for the terminal material change system.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TerminalMaterialChange;

/// System set for the terminal layout update system.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TerminalLayoutUpdate;

/// System set for the terminal mesh size update function.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TerminalLayoutChange;

/// System set for the terminal font changing function.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TerminalChangeFont;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TerminalUpdateTiles;

/// System set for the terminal render system. This is the last terminal
/// system to run and runs at the end of the frame.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TerminalRender;

pub(crate) struct TerminalRendererPlugin;

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Last,
            (
                mesh::init_mesh.in_set(MeshInit),
                material::material_change.in_set(TerminalMaterialChange),
                terminal_mesh::update_layout.in_set(TerminalLayoutChange),
                terminal_mesh::update_vert_data.in_set(TerminalUpdateTiles),
                terminal_mesh::update_tile_data.in_set(TerminalUpdateTiles),
                mesh::update_mesh_verts.in_set(TerminalRender),
                mesh::update_mesh_tiles.in_set(TerminalRender),
            )
                .chain(),
        );
        app.add_plugins((
            material::TerminalMaterialPlugin,
            #[cfg(feature = "camera")]
            camera::TerminalCameraPlugin,
            uv_mapping::UvMappingPlugin,
            border_mesh::BorderMeshPlugin,
        ));
    }
}
