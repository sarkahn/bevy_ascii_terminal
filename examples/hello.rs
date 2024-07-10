use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(TerminalCameraBundle::auto());
    commands.spawn(
        TerminalBundle::new([15, 3]).put_string([1, 1], "Hello world!"), //.with_border(Border::single_line()),
    );
}
