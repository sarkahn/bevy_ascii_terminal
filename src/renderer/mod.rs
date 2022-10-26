//! Handles mesh construction and rendering for the terminal.

mod entity;
mod material;
mod mesh;
mod mesh_data;
mod border_mesh;
mod terminal_mesh;
mod layout;

mod font;
mod uv_mapping;

#[cfg(feature = "camera")]
mod camera;

pub mod code_page_437;

use bevy::prelude::{App, CoreStage, IntoSystemDescriptor, Plugin};
pub(crate) use font::BuiltInFontHandles;

pub use entity::*;

pub use font::TerminalFont;

pub use material::TerminalMaterial;
pub use layout::TerminalLayout;

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
        app
            .add_system_to_stage(CoreStage::PostUpdate, 
                terminal_mesh::init_terminal
                    .label(TERMINAL_INIT)
            )
            .add_system_to_stage(CoreStage::Last,
                mesh::init_mesh
                    .label(MESH_INIT)
            )
            .add_system_to_stage(CoreStage::Last,
                material::material_change
                    .label(TERMINAL_MATERIAL_CHANGE)
                    .after(MESH_INIT),
            )
            .add_system_to_stage(CoreStage::Last,
                terminal_mesh::update_layout
                    .label(TERMINAL_LAYOUT_CHANGE)
                    .after(TERMINAL_MATERIAL_CHANGE),
            )
            .add_system_to_stage(CoreStage::Last,
                terminal_mesh::update_vert_data
                    .label(TERMINAL_UPDATE_TILES)
                    .after(TERMINAL_LAYOUT_CHANGE)
            )
            .add_system_to_stage(CoreStage::Last,
                terminal_mesh::update_tile_data
                    .label(TERMINAL_UPDATE_TILES)
                    .after(TERMINAL_LAYOUT_CHANGE)
            )
            .add_system_to_stage(CoreStage::Last,
                mesh::update_mesh_verts
                    .label(TERMINAL_RENDER)
                    .after(TERMINAL_UPDATE_TILES),
            )
            .add_system_to_stage(CoreStage::Last,
                mesh::update_mesh_tiles
                    .label(TERMINAL_RENDER)
                    .after(TERMINAL_UPDATE_TILES),
            )
            ;
        app.add_plugin(material::TerminalMaterialPlugin);
        app.add_plugin(camera::TerminalCameraPlugin);
        app.add_plugin(uv_mapping::UvMappingPlugin);
        app.add_plugin(border_mesh::BorderMeshPlugin);
    }
}
