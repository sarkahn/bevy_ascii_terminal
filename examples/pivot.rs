use bevy::{prelude::*, sprite::Anchor};
use bevy_ascii_terminal::{prelude::*, TiledCameraBundle};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .add_startup_system(setup)
    .run();
}

fn setup(
    mut commands: Commands
) {
    let terms = [
        Terminal::new([10,10]).with_pivot(Pivot::BottomLeft),
        Terminal::new([10,10]).with_pivot(Pivot::TopLeft),
        Terminal::new([10,10]).with_pivot(Pivot::BottomRight),
        Terminal::new([10,10]).with_pivot(Pivot::TopRight),
    ];

    for (i, mut term) in terms.into_iter().enumerate() {
        term.put_string([0,0], "Hello");
        commands.spawn((
            TerminalBundle::from(term).with_depth(i as i32),
            AutoCamera
        ));
    }

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::ONE),
            color: Color::BLUE,
            anchor: Anchor::Center,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()
    });
}