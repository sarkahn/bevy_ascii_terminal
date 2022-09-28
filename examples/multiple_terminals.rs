use bevy::prelude::*;
use bevy_ascii_terminal::{prelude::*, TerminalFont, TiledCameraBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut term = Terminal::with_size([17, 1]).with_border(Border::SINGLE_LINE);

    term.put_string([0, 0], "Hello, world! ☺");

    commands.spawn_bundle(
        TerminalBundle::from(term.clone())
            .with_position([0, 3])
            .with_font(TerminalFont::Px4378x8),
    );

    commands.spawn_bundle(
        TerminalBundle::from(term.clone())
            .with_position([0, 0])
            .with_font(TerminalFont::Pastiche8x8),
    );
    commands.spawn_bundle(
        TerminalBundle::from(term.clone())
            .with_position([0, -3])
            .with_font(TerminalFont::ZxEvolution8x8),
    );

    commands.spawn_bundle(
        TiledCameraBundle::new()
            .with_tile_count([term.width_with_border(), term.height_with_border() * 3]),
    );
}
