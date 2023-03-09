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

use bevy::prelude::{App, CoreSet, IntoSystemConfig, Plugin};
pub(crate) use font::BuiltInFontHandles;

pub use entity::*;

pub use font::TerminalFont;

pub use layout::TerminalLayout;
pub use material::TerminalMaterial;

#[cfg(feature = "camera")]
pub use camera::{AutoCamera, TiledCamera, TiledCameraBundle};

/// System label for the terminal mesh initialization system.
pub const TERMINAL_INIT: &str = "terminal_init";
pub const MESH_INIT: &str = "mesh_init";
/// System label for the terminal material change system.
pub const TERMINAL_MATERIAL_CHANGE: &str = "terminal_material_change";
/// System label for the terminal layout update system.
pub const TERMINAL_LAYOUT_UPDATE: &str = "terminal_layout_update";
/// System label for the terminal mesh size update function.
pub const TERMINAL_LAYOUT_CHANGE: &str = "terminal_layout_change";
/// System label for the terminal font changing function.
pub const TERMINAL_CHANGE_FONT: &str = "terminal_change_font";
pub const TERMINAL_UPDATE_TILES: &str = "terminal_update_tile_data";
/// System label for the terminal render system. This is the last terminal
/// system to run and runs at the end of the frame.
pub const TERMINAL_RENDER: &str = "terminal_update_mesh";

pub(crate) struct TerminalRendererPlugin;

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(mesh::init_mesh.in_set(MESH_INIT).in_base_set(CoreSet::Last))
            .add_system(
                material::material_change
                    .in_set(TERMINAL_MATERIAL_CHANGE)
                    .after(MESH_INIT)
                    .in_base_set(CoreSet::Last),
            )
            .add_system(
                terminal_mesh::update_layout
                    .in_set(TERMINAL_LAYOUT_CHANGE)
                    .after(TERMINAL_MATERIAL_CHANGE)
                    .in_base_set(CoreSet::Last),
            )
            .add_system(
                terminal_mesh::update_vert_data
                    .in_set(TERMINAL_UPDATE_TILES)
                    .after(TERMINAL_LAYOUT_CHANGE)
                    .in_base_set(CoreSet::Last),
            )
            .add_system(
                terminal_mesh::update_tile_data
                    .in_set(TERMINAL_UPDATE_TILES)
                    .after(TERMINAL_LAYOUT_CHANGE)
                    .in_base_set(CoreSet::Last),
            )
            .add_system(
                mesh::update_mesh_verts
                    .in_set(TERMINAL_RENDER)
                    .after(TERMINAL_UPDATE_TILES)
                    .in_base_set(CoreSet::Last),
            )
            .add_system(
                mesh::update_mesh_tiles
                    .in_set(TERMINAL_RENDER)
                    .after(TERMINAL_UPDATE_TILES)
                    .in_base_set(CoreSet::Last),
            );
        app.add_plugin(material::TerminalMaterialPlugin);
        app.add_plugin(camera::TerminalCameraPlugin);
        app.add_plugin(uv_mapping::UvMappingPlugin);
        app.add_plugin(border_mesh::BorderMeshPlugin);
    }
}
