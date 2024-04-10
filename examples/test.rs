use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin::default()))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(TerminalCameraBundle::auto());
    // commands.spawn(Camera2dBundle {
    //     projection: OrthographicProjection {
    //         scaling_mode: ScalingMode::FixedVertical(15.0),
    //         far: 1000.,
    //         near: -1000.,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
    commands.spawn(
        TerminalBundle::new([10, 10])
            .put_string([1, 1], "Hello")
            .put_char([2, 2], 'a'),
    );
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::splat(5.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(10.0, 0.0, 1.0),
        ..Default::default()
    });
}
