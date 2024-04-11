use bevy::{prelude::*, render::camera::ScalingMode, sprite::Anchor};
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
    //         scaling_mode: ScalingMode::FixedVertical(40.0),
    //         far: 1000.,
    //         near: -1000.,
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });
    commands.spawn(
        TerminalBundle::new([10, 10])
            .put_char([0, 0], 'O')
            .put_string([1, 1], "Hello")
            .put_char([2, 2], 'a')
            .with_mesh_pivot(Pivot::BottomLeft)
            .with_border(Border::single_line()),
    );
    commands.spawn(
        TerminalBundle::new([10, 10])
            .put_char([0, 0], 'O')
            .put_string([1, 1], "Hello")
            .put_char([2, 2], 'a')
            .with_mesh_pivot(Pivot::TopLeft)
            .with_border(Border::single_line()),
    );
    commands.spawn(
        TerminalBundle::new([10, 10])
            .put_char([0, 0], 'O')
            .put_string([1, 1], "Hello")
            .put_char([2, 2], 'a')
            .with_mesh_pivot(Pivot::BottomRight)
            .with_border(Border::single_line()),
    );
    commands.spawn(
        TerminalBundle::new([10, 10])
            .put_char([0, 0], 'O')
            .put_string([1, 1], "Hello")
            .put_char([2, 2], 'a')
            .with_mesh_pivot(Pivot::TopRight)
            .with_border(Border::single_line()),
    );
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::splat(2.0)),
            anchor: Anchor::BottomLeft,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..Default::default()
    });
}
