use bevy::prelude::*;
use bevy_ascii_terminal::{prelude::*, TerminalFont};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut term = Terminal::new([17, 1]).with_border(Border::single_line());

    term.put_string([0, 0], "Hello, world! ☺");

    commands.spawn((
        TerminalBundle::from(term.clone())
            .with_position([0, 3])
            .with_font(TerminalFont::Px4378x8),
        AutoCamera,
    ));

    commands.spawn((
        TerminalBundle::from(term.clone())
            .with_position([0, 0])
            .with_font(TerminalFont::Pastiche8x8),
        AutoCamera,
    ));
    commands.spawn((
        TerminalBundle::from(term.clone())
            .with_position([0, -3])
            .with_font(TerminalFont::ZxEvolution8x8),
        AutoCamera,
    ));
}
