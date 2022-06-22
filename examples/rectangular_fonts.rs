use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugin(TiledCameraPlugin)
    .add_plugin(TerminalPlugin)
    .add_plugins(DefaultPlugins)
    .run();
}

fn setup(
    mut commands: Commands,
) {
    
}