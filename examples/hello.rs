//! A minimal example with a terminal and camera.

use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Terminal::new([12, 1]).with_string([0, 0], "Hello world!".fg(color::BLUE)),
        TerminalBorder::single_line(),
    ));
    commands.spawn(TerminalCamera::new());
}
