use bevy::prelude::*;

use bevy_ascii_terminal::{terminal::Terminal, TerminalBundle, TerminalPlugin};
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin};

fn spawn_terminal(mut commands: Commands) {
    let (w, h) = (20, 3);
    let mut term_bundle = TerminalBundle::with_size(w, h);

    term_bundle.terminal.draw_border_single();
    term_bundle.terminal.put_string(1, 1, "Press spacebar");

    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(PixelCameraBundle::from_resolution(
        w as i32 * 12,
        h as i32 * 12,
    ));
}

fn hello_world(keys: Res<Input<KeyCode>>, mut q: Query<&mut Terminal>) {
    if keys.just_pressed(KeyCode::Space) {
        for mut term in q.iter_mut() {
            term.clear();
            term.draw_border_single();
            term.put_string(1, 1, "Hello, world!");
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(PixelCameraPlugin)
        .add_startup_system(spawn_terminal.system())
        .add_system(hello_world.system())
        .run()
}
