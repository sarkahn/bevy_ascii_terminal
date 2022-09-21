//! Handles mesh construction and rendering for the terminal.

mod entity;
mod material;
mod plugin;
mod border;
mod util;

mod font;
mod uv_mapping;

#[cfg(feature = "camera")]
mod camera;

pub(crate) mod renderer_tile_data;
pub(crate) mod renderer_vertex_data;

pub mod code_page_437;

use crate::terminal::Terminal;
use {
    renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData,
};

pub(crate) use font::BuiltInFontHandles;
pub(crate) use plugin::TerminalRendererPlugin;

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
