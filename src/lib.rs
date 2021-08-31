use bevy::prelude::*;
use render::{TerminalRendererPlugin, entity::TerminalRendererBundle, renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData};
use terminal::{Terminal, TerminalSize};

mod glyph_mapping;
pub mod render;
mod terminal;

#[derive(Bundle, Default)]
pub struct TerminalBundle {
    terminal: Terminal,
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

// impl Default for TerminalBundle {
//     fn default() -> Self {
//         Self { 
//             terminal: Default::default(), 
//             size: Default::default(), 
//             renderer_bundle: Default::default() 
//         }
//     }
// }

pub struct TerminalPlugin;
impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TerminalRendererPlugin);
    }
}