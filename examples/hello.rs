//! A minimal example with a terminal and camera.

use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_input)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        Terminal::new([14, 3])
            .with_border(BoxStyle::SINGLE_LINE)
            .with_bg_clear_color(color::DARK_BLUE)
            .with_string(
                [0, 0],
                "<bg=midnight_blue><fg=firebrick>Hello</fg> <fg=#5772e5>world!</fg></bg>",
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
