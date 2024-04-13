use bevy::prelude::*;
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
        TerminalBundle::new([60, 25])
            .put_string(
                [0, 0].pivot(Pivot::TopLeft),
                "Top Left Pivot\nAnd here's a newline",
            )
            .put_string(
                [0, 0].pivot(Pivot::TopRight),
                "Top Right Pivot\nAnd here's a newline",
            )
            .put_string(
                [0, 0].pivot(Pivot::Center),
                "Center Pivot\nAnd here's a newline",
            )
            .put_string(
                [0, 0].pivot(Pivot::BottomLeft),
                "Bottom Left Pivot\nAnd here's a newline",
            )
            .put_string(
                [0, 0].pivot(Pivot::BottomRight),
                "Bottom Right Pivot\nAnd here's a newline",
            )
            .with_mesh_pivot(Pivot::TopRight)
            .with_border_title(Border::single_line(), "[Strings]".fg(Color::BLUE)),
    );
}
