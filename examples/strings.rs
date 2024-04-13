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
    commands.spawn(
        TerminalBundle::new([15, 15])
            .put_string([1, 1].pivot(Pivot::TopLeft), "Testing\n123")
            .put_string([1, 1].pivot(Pivot::TopRight), "Testing\n123")
            .put_string([1, 1].pivot(Pivot::BottomLeft), "Testing\n123")
            .put_string([1, 1].pivot(Pivot::BottomRight), "Testing\n123")
            .with_mesh_pivot(Pivot::TopRight)
            .with_border_title(Border::single_line(), "Strings"),
    );
}
