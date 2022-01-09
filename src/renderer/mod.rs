//! Handles mesh construction and rendering for the terminal.

pub mod material;
pub mod plugin;
pub mod entity;

pub mod code_page_437;
pub mod uv_mapping;

pub(crate) mod renderer_tile_data;
pub(crate) mod renderer_vertex_data;

use self::{
    material::TerminalMaterial, renderer_tile_data::TerminalRendererTileData,
    renderer_vertex_data::TerminalRendererVertexData,
};
use crate::terminal::Terminal;

/// System label for the terminal mesh initialization function.
pub const TERMINAL_INIT: &str = "terminal_init_mesh";
/// System label for the terminal mesh size update function.
pub const TERMINAL_UPDATE_SIZE: &str = "terminal_update_size";
/// System label for the terminal mesh tile update function.
pub const TERMINAL_UPDATE_TILE_DATA: &str = "terminal_update_tile_data";
/// System label for the terminal mesh update function.
pub const TERMINAL_UPDATE_MESH: &str = "terminal_update_mesh";

pub use plugin::TerminalRendererPlugin;
pub use entity::*;
