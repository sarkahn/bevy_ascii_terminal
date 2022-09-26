use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(spawn)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn_bundle(TerminalBundle::new().with_size([5, 5]));
    commands.spawn_bundle(TiledCameraBundle::new().with_tile_count([20, 20]));
}
