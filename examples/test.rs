use bevy_ascii_terminal::*;
use bevy::prelude::*;

fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins, TerminalPlugin::default()
    )).add_systems(Startup, setup).run()
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(TerminalCameraBundle::auto());
    commands.spawn(TerminalBundle::new([10,10]).with_string([1,1], "Hello"));
}