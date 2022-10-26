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
    let mut term = Terminal::new([80, 35]);

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

    commands.spawn((TerminalBundle::from(term), AutoCamera));
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::MAROON,
            custom_size: Some(Vec2::ONE),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()
    });
}
