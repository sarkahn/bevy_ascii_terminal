use bevy::prelude::*;
use terminal::{Terminal, TerminalSize};

mod glyph_mapping;
pub mod render;
mod terminal;

#[derive(Bundle)]
pub struct TerminalBundle {
    terminal: Terminal,
    size: TerminalSize,
}

impl Plugin for Terminal {
    fn build(&self, app: &mut AppBuilder) {
        todo!()
    }
}