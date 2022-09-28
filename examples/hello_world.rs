use bevy::prelude::*;
use bevy_ascii_terminal::{prelude::*, TerminalPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(spawn_terminal)
        .add_system(hello_world)
        .run()
}

fn spawn_terminal(mut commands: Commands) {
    let title = BorderTitle::center("Hello World!").color(Color::YELLOW_GREEN);
    let mut term = Terminal::with_size([20, 1]).with_border(Border::SINGLE_LINE.with_title(title));

    term.put_string([0, 0], "Press spacebar".bg(Color::LIME_GREEN));

    commands
        .spawn_bundle(TerminalBundle::from(term))
        .insert(AutoCamera);
}

fn hello_world(keys: Res<Input<KeyCode>>, mut q: Query<&mut Terminal>) {
    if keys.just_pressed(KeyCode::Space) {
        for mut term in q.iter_mut() {
            term.clear();
            term.put_char([1, 0], 'H'.fg(Color::BLUE).bg(Color::GREEN));
            term.put_char([2, 0], 'e'.fg(Color::BLUE).bg(Color::WHITE));
            term.put_char([3, 0], 'l'.fg(Color::GREEN).bg(Color::BLUE));
            term.put_char([4, 0], 'l'.fg(Color::RED).bg(Color::GREEN));
            term.put_char([5, 0], 'o'.fg(Color::GREEN).bg(Color::GRAY));

            term.put_string([6, 0], " World!");
        }
    }
}
