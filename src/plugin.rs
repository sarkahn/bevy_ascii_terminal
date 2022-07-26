use bevy::prelude::{default, App, Plugin};

use crate::{entity::TerminalBundle, renderer, to_world::ToWorldPlugin, Terminal};

impl From<Terminal> for TerminalBundle {
    fn from(terminal: Terminal) -> Self {
        TerminalBundle {
            terminal,
            ..default()
        }
    }
}

/// Plugin for terminal rendering and related components and systems.
pub struct TerminalPlugin;
impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(renderer::TerminalRendererPlugin)
            .add_plugin(ToWorldPlugin);
    }
}
