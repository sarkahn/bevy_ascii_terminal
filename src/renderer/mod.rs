//! Handles mesh construction and rendering for the terminal.

mod border;
mod entity;
mod material;
mod mesh;
mod mesh_data;
mod tiles;

mod font;
mod uv_mapping;

#[cfg(feature = "camera")]
mod camera;

pub(crate) mod tile_data;
pub(crate) mod vertex_data;

pub mod code_page_437;

use crate::terminal::Terminal;
use tile_data::TileData;

use bevy::prelude::{App, Plugin};
pub(crate) use font::BuiltInFontHandles;

pub use entity::*;

pub use font::TerminalFont;

pub use material::TerminalMaterial;

#[cfg(feature = "camera")]
pub use camera::{AutoCamera, TiledCamera, TiledCameraBundle};

pub use border::TerminalBorderBundle;

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
        //app.add_plugin(border::BorderPlugin);
    }
}

//         //app.add_system(terminal_renderer_init.label(TERMINAL_INIT))
//             // .add_system(
//             //     terminal_renderer_update_size
//             //         .after(TERMINAL_CHANGE_FONT)
//             //         .label(TERMINAL_UPDATE_SIZE),
//             // )
//             // .add_system(
//             //     terminal_renderer_update_tile_data
//             //         .after(TERMINAL_UPDATE_SIZE)
//             //         .label(TERMINAL_UPDATE_TILE_DATA),
//             // )
//             // .add_system(
//             //     terminal_renderer_update_mesh
//             //         .after(TERMINAL_UPDATE_TILE_DATA)
//             //         .label(TERMINAL_UPDATE_MESH),
//             // )
//             //;

/*
Order of operations:
Start:
Spawn terminal entity with terminal mesh bundle
SPawn border entity as child of terminal eneity with it's own terminal mesh bundle
Meshes get initialized by system

On Terminal Layout change:
Update term mesh size
Update term mesh tiles
Update border mesh size
Update border mesh tiles

On Term tiles change:
Update term mesh tiles

*/
