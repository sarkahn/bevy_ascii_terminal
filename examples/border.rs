use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(TerminalBundle::new([20, 20]).put_string([0, 0], "Hello"));
    commands.spawn(TerminalCameraBundle::with_tile_resolution([22, 22], [8, 8]));
}
