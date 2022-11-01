use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    let pivots = [
        Pivot::BottomLeft,
        Pivot::TopLeft,
        Pivot::TopRight,
        Pivot::BottomRight,
    ];
    let mut term = Terminal::new([8, 2]);

    for (i, pivot) in pivots.iter().enumerate() {
        term.clear();
        term.put_string([0, 0].pivot(*pivot), "Hello");
        commands.spawn((
            TerminalBundle::from(term.clone())
                .with_depth(i as i32)
                .with_pivot(*pivot),
            AutoCamera,
        ));
    }

    //commands.spawn(TiledCameraBundle::new().with_tile_count([15,4]));
}
