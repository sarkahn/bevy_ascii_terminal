//! A minimal example with a terminal and camera.

use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        Terminal::new([25, 3])
            .with_border(BoxStyle::SINGLE_LINE)
            .with_string(
                [0, 0],
                "# <fg=red>Bevy</fg> <fg=green>Ascii</fg> <fg=blue>Terminal</fg> #",
            ),
    );
    commands.spawn(TerminalCamera::new());
}

fn handle_input(
    input: Res<ButtonInput<KeyCode>>,
    mut win: Single<&mut Window>,
    mut exit: MessageWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    if input.just_pressed(KeyCode::KeyF) {
        if win.mode == WindowMode::BorderlessFullscreen(MonitorSelection::Current) {
            win.mode = WindowMode::Windowed;
        } else {
            win.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        }
    }
}
