//! A terminal built from a rexpaint file.

use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    #[allow(deprecated)]
    commands.spawn(
        Terminal::from_rexpaint_file("assets/hello_rexpaint.xp")
            .unwrap()
            .with_border(BoxStyle::SINGLE_LINE)
            .with_title(" [<fg=4d65b4>Rexpaint</fg>]"),
    );
    commands.spawn(TerminalCamera::new());
}
