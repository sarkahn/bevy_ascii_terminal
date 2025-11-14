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
        Terminal::new([14, 4]).with_string([1, 2], "Hello world!".fg(color::BLUE)),
        TerminalBorder::double_line(),
    ));
    commands.spawn(TerminalCamera::new());
}
