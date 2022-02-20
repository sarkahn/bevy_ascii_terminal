use bevy::prelude::*;

use bevy_ascii_terminal::ldtk::LdtkPlugin;

fn main () {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_plugin(LdtkPlugin)
    .run();
}

fn setup(
    asset_server: Res<AssetServer>
) {
    let handle: Handle<Image> = asset_server.load("ldtk_test.ldtk");
}