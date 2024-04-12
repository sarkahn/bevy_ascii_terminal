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
    commands
        .spawn(TerminalBundle::from(make_terminal([10, 10])).with_mesh_pivot(Pivot::BottomLeft));
    commands.spawn(TerminalBundle::from(make_terminal([10, 10])).with_mesh_pivot(Pivot::TopLeft));
    commands.spawn(TerminalBundle::from(make_terminal([12, 12])).with_mesh_pivot(Pivot::TopRight));
    commands
        .spawn(TerminalBundle::from(make_terminal([13, 15])).with_mesh_pivot(Pivot::BottomRight));
}

fn make_terminal(size: impl GridPoint) -> Terminal {
    let mut term = Terminal::new(size);
    term.put_border(Border::single_line());
    for (p, t) in term.iter_xy_mut() {
        if (p.x + p.y) % 2 == 0 {
            t.bg_color = Color::rgb(0.0, 0.15, 0.55);
        } else {
            t.bg_color = Color::rgb(0.25, 0.0, 0.0);
        }
    }
    term
}
