use bevy::prelude::*;
use bevy_ascii_terminal::*;

const BORDERS: &[Option<Border>] = &[Some(Border::SINGLE_LINE), Some(Border::DOUBLE_LINE), None];

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(spawn)
        .add_system(input)
        .run();
}

const INFO_STRING: &str = "Arrow keys to resize\nSpacebar to change border";

fn spawn(mut commands: Commands) {
    let mut term = Terminal::with_size([25, 2]).with_border(Border::SINGLE_LINE);
    term.put_string([0, 0].pivot(Pivot::TopLeft), INFO_STRING);
    commands.spawn((TerminalBundle::from(term), AutoCamera));
}

fn input(input: Res<Input<KeyCode>>, mut q_term: Query<&mut Terminal>, mut index: Local<usize>) {
    let mut cleared = false;
    if input.just_pressed(KeyCode::Space) {
        let mut term = q_term.single_mut();

        *index = (*index + 1) % BORDERS.len();
        if let Some(border) = BORDERS[*index].clone() {
            term.set_border(border);
        } else {
            term.remove_border();
        }
        cleared = true;
    }

    if input.just_pressed(KeyCode::Right) {
        let mut term = q_term.single_mut();

        let size = [term.width() + 1, term.height()];
        term.resize(size);
        cleared = true;
    }
    if input.just_pressed(KeyCode::Left) {
        let mut term = q_term.single_mut();

        let size = [term.width().saturating_sub(1).max(1), term.height()];
        term.resize(size);
        cleared = true;
    }
    if input.just_pressed(KeyCode::Up) {
        let mut term = q_term.single_mut();

        let size = [term.width(), term.height() + 1];
        term.resize(size);
        cleared = true;
    }
    if input.just_pressed(KeyCode::Down) {
        let mut term = q_term.single_mut();

        let size = [term.width(), term.height().saturating_sub(1).max(1)];
        term.resize(size);
        cleared = true;
    }

    if cleared {
        let mut term = q_term.single_mut();
        term.put_string([0, 0].pivot(Pivot::TopLeft), INFO_STRING);
    }
}
