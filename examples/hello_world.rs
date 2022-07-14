use bevy::prelude::*;
use bevy_ascii_terminal::{
    formatting::{StringWriter, TileModifier},
    ui::BorderGlyphs,
    *,
};

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
    let mut term = Terminal::with_size([20,3]);

    term.draw_border(BorderGlyphs::single_line());
    term.put_string([1, 1], "Press spacebar".bg(Color::LIME_GREEN));

    commands.spawn_bundle(TerminalBundle::from(term))
    .insert(AutoCamera);
}

fn hello_world(keys: Res<Input<KeyCode>>, mut q: Query<&mut Terminal>) {
    if keys.just_pressed(KeyCode::Space) {
        for mut term in q.iter_mut() {
            term.clear();
            term.draw_border(BorderGlyphs::single_line());
            term.put_char([1, 1], 'H'.fg(Color::BLUE).bg(Color::GREEN));
            term.put_char([2, 1], 'e'.fg(Color::BLUE).bg(Color::WHITE));
            term.put_char([3, 1], 'l'.fg(Color::GREEN).bg(Color::BLUE));
            term.put_char([4, 1], 'l'.fg(Color::RED).bg(Color::GREEN));
            term.put_char([5, 1], 'o'.fg(Color::GREEN).bg(Color::GRAY));

            term.put_string([6, 1], " World!");
        }
    }
}
