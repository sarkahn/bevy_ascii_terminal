mod border;
mod glyph;
mod grid;
pub mod renderer;
mod string;
mod terminal;
mod tile;
mod transform;

use bevy::{app::Plugin, ecs::bundle::Bundle};
pub use grid::{direction, GridPoint, GridRect, Pivot, PivotedPoint};
pub use renderer::TerminalFont;
use renderer::TerminalRenderBundle;
pub use string::{FormattedString, StringFormatter};
pub use terminal::Terminal;
pub use tile::Tile;

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(renderer::TerminalRendererPlugin);
    }
}

#[derive(Bundle)]
pub struct TerminalBundle {
    terminal: Terminal,
    renderer: TerminalRenderBundle,
}

impl TerminalBundle {
    // pub fn with_string(mut self, xy: impl GridPoint, string: impl Into<FormattedString>) -> Self {
    //     self.terminal.put_string(xy, string);

    //     self
    // }
}