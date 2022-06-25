use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugin(TiledCameraPlugin)
    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .add_startup_system(setup)
    .run();
}

fn setup(
    mut commands: Commands,
) {
    let size = [20,20];

    commands.spawn_bundle(TiledCameraBundle::new()
        .with_tile_count(size));

    let mut term = Terminal::with_size(size);
    term.draw_border(BorderGlyphs::single_line());

    commands.spawn_bundle(TerminalBundle::from(term));
}