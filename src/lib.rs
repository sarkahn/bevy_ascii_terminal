mod border;
mod glyph;
mod grid;
pub mod renderer;
mod string;
mod terminal;
mod tile;

use bevy::app::Plugin;
pub use grid::{direction, GridPoint, GridRect, Pivot, PivotedPoint};
pub use renderer::TerminalFont;
pub use string::{FormattedString, StringFormatter};
pub use terminal::Terminal;
pub use tile::Tile;

pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(renderer::TerminalRendererPlugin);
    }
}
