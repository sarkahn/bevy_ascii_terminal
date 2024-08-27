use bevy::{color::palettes::basic, prelude::*};
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(TerminalCameraBundle::with_tile_resolution([30, 30], [8, 8]));
    let mut term = Terminal::new([21, 21]);
    for x in 0..term.width() {
        for y in 0..term.height() {
            if ((x + y) % 2) == 0 {
                term.put_bg_color([x, y], basic::GRAY.with_alpha(0.1));
            } else {
                term.put_bg_color([x, y], basic::RED.with_alpha(0.1));
            }
        }
    }
    term.put_char([0, 0].pivot(Pivot::Center), 'O');
    commands.spawn(TerminalBundle::from(term).with_depth(-1));
    commands.spawn(
        TerminalBundle::new([3, 3])
            .with_clear_tile(Glyph::SmilingFace.to_char())
            .with_grid_position([2, 2])
            .with_mesh_pivot(Pivot::BottomLeft),
    );
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: basic::BLUE.into(),
            custom_size: Some(Vec2::ONE),
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    // commands.spawn(
    //     TerminalBundle::new([3, 3])
    //         .with_clear_tile(Glyph::SmilingFaceInverse.to_char())
    //         .with_grid_position([-2, -2])
    //         .with_mesh_pivot(Pivot::TopRight),
    // );

    // commands.spawn(
    //     TerminalBundle::new([4, 4])
    //         .with_clear_tile(Glyph::ShadeLight.to_char())
    //         .with_grid_position([-2, -2])
    //         .with_mesh_pivot(Pivot::BottomRight),
    // );
}
