use std::env;

use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Terminal::from_rexpaint_file("assets/hello_rexpaint.xp").unwrap(),
        TerminalBorder::single_line(),
    ));
    commands.spawn(TerminalCamera::new());
}
