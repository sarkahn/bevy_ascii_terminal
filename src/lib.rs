use bevy::prelude::*;
use render::{TerminalRendererPlugin, entity::TerminalRendererBundle, renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData};
use terminal::{Terminal, TerminalSize};

pub mod render;
pub mod terminal;

#[derive(Bundle, Default)]
pub struct TerminalBundle {
    pub terminal: Terminal,
    size: TerminalSize,

    #[bundle]
    renderer_bundle: TerminalRendererBundle,
}

impl TerminalBundle {
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            terminal: Terminal::new(width, height),
            size: TerminalSize {
                size: (width, height)
            },
            renderer_bundle: TerminalRendererBundle {
                vert_data: TerminalRendererVertexData::with_size(width, height),
                tile_data: TerminalRendererTileData::with_size(width, height),
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