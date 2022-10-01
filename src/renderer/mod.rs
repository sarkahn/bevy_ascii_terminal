//! Handles mesh construction and rendering for the terminal.

mod entity;
mod material;
mod mesh;
mod mesh_data;

mod font;
mod uv_mapping;

#[cfg(feature = "camera")]
mod camera;

pub mod code_page_437;

use bevy::prelude::{App, CoreStage, Plugin, IntoSystemDescriptor};
pub(crate) use font::BuiltInFontHandles;

pub use entity::*;

pub use font::TerminalFont;

pub use material::TerminalMaterial;

#[cfg(feature = "camera")]
pub use camera::{AutoCamera, TiledCamera, TiledCameraBundle};

use self::mesh::*;

/// System label for the terminal mesh initialization function.
pub const TERMINAL_INIT: &str = "terminal_init_mesh";
/// System label for the terminal mesh size update function.
pub const TERMINAL_UPDATE_SIZE: &str = "terminal_update_size";
/// System label for the terminal mesh tile update function.
pub const TERMINAL_UPDATE_TILE_DATA: &str = "terminal_update_tile_data";
/// System label for the terminal mesh update function.
pub const TERMINAL_UPDATE_MESH: &str = "terminal_update_mesh";
/// System label for the terminal font changing function.
pub const TERMINAL_CHANGE_FONT: &str = "terminal_change_font";

pub(crate) struct TerminalRendererPlugin;

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(material::TerminalMaterialPlugin);
        app.add_plugin(camera::TerminalCameraPlugin);

        app.add_system_to_stage(CoreStage::PostUpdate, init_terminal)
            .add_system_to_stage(CoreStage::Last, material_change)
            .add_system_to_stage(CoreStage::Last, update_layout.after(material_change))
            .add_system_to_stage(CoreStage::Last, layout_changed.after(update_layout))
            .add_system_to_stage(CoreStage::Last, update_tiles.after(layout_changed))
            ;
    }
}
