pub mod render;
pub mod terminal;

use bevy::prelude::*;
use render::{
    entity::TerminalRendererBundle, plugin::TerminalRendererPlugin,
    renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData,
};
pub use terminal::{Terminal, TerminalSize};

#[derive(Bundle, Default)]
pub struct TerminalBundle {
    pub terminal: Terminal,
    pub size: TerminalSize,
    pub transform: Transform,
    pub global_transform: GlobalTransform,

    #[bundle]
    pub renderer: TerminalRendererBundle,
}

impl TerminalBundle {
    pub fn with_size(width: usize, height: usize) -> Self {
        let size = UVec2::new(width as u32, height as u32);
        Self {
            terminal: Terminal::new(width, height),
            size: TerminalSize { value: size },
            renderer: TerminalRendererBundle {
                vert_data: TerminalRendererVertexData::with_size(size),
                tile_data: TerminalRendererTileData::with_size(size),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub struct TerminalPlugin;
impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TerminalRendererPlugin);
    }
}
