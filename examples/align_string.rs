use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut term = Terminal::with_size([80, 35]);

    term.put_string([0, 0], "Hello\nThis is a multiline\nString!");
    term.put_string(
        [0, 0].pivot(Pivot::TopLeft),
        "Hello\nThis is a multiline\nString!",
    );
    term.put_string(
        [0, 0].pivot(Pivot::TopRight),
        "Hello\nThis is a multiline\nString!",
    );
    term.put_string(
        [0, 0].pivot(Pivot::BottomRight),
        "Hello\nThis is a multiline\nString!",
    );
    term.put_string(
        [0, 0].pivot(Pivot::Center),
        "Hello\nThis is a multiline\nString!",
    );

    commands
        .spawn_bundle(TerminalBundle::from(term))
        .insert(AutoCamera);
}
