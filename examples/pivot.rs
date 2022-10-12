use bevy::{prelude::*, sprite::Anchor};
use bevy_ascii_terminal::{Terminal, TerminalBundle, AutoCamera, TiledCameraBundle, TerminalPlugin, Pivot};

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
    let mut term = Terminal::new([10,10]);
    term.put_string([0,0], "Hello");

    commands.spawn(
        TerminalBundle::from(term).with_pivot(Pivot::BottomLeft),
        //AutoCamera
    );

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

    commands.spawn(TiledCameraBundle::new().with_tile_count([10,10]));
}