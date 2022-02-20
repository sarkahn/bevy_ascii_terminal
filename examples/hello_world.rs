use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_plugin(TiledCameraPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(spawn_terminal.system())
        .add_system(hello_world.system())
        .run()
}

fn spawn_terminal(mut commands: Commands) {
    let size = [20, 3];
    let mut term_bundle = TerminalBundle::new().with_size(size);

    term_bundle.terminal.draw_border_single();
    term_bundle.terminal.put_string([1, 1], "Press spacebar");

    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(
        TiledCameraBundle::new()
            .with_centered(true)
            .with_pixels_per_tile(8)
            .with_tile_count(size),
    );
}

fn hello_world(keys: Res<Input<KeyCode>>, mut q: Query<&mut Terminal>) {
    if keys.just_pressed(KeyCode::Space) {
        for mut term in q.iter_mut() {
            term.clear();
            term.draw_border_single();
            term.put_char([1, 1], 'H'.fg(Color::BLUE).bg(Color::GREEN));
            term.put_char([2, 1], 'e'.fg(Color::BLUE).bg(Color::WHITE));
            term.put_char([3, 1], 'l'.fg(Color::GREEN).bg(Color::BLUE));
            term.put_char([4, 1], 'l'.fg(Color::RED).bg(Color::GREEN));
            term.put_char([5, 1], 'o'.fg(Color::GREEN).bg(Color::GRAY));

            term.put_string([6, 1], " World!".fg(Color::TEAL));
        }
    }
}