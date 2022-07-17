use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}

fn setup(mut commands: Commands) {
    let size = [40, 15];
    let mut term = Terminal::new(
        size,
        Tile {
            glyph: '█',
            fg_color: Color::rgba(0.0, 0.0, 1.0, 0.45),
            ..default()
        },
    );

    for x in 0..size[0] {
        for y in 0..size[1] {
            if (x + y) % 2 == 0 {
                term.put_color([x, y], Color::rgba(0.75, 0.0, 0.0, 0.45).fg());
            }
        }
    }
    term.put_char(
        [0, 0].pivot(Pivot::Center),
        'O'.fg(Color::WHITE).bg(Color::BLACK),
    );

    commands
        .spawn_bundle(TerminalBundle::from(term))
        .insert(AutoCamera)
        .insert(ToWorld::default());
}

fn update(mut q_term: Query<(&mut Terminal, &ToWorld)>, windows: Res<Windows>) {
    if q_term.is_empty() {
        return;
    }

    if let Some(window) = windows.get_primary() {
        for (mut term, tw) in q_term.iter_mut() {
            term.iter_row_mut(0).for_each(|t| *t = Tile::default());
            term.iter_row_mut(1).for_each(|t| *t = Tile::default());

            if let Some(cursor_pos) = window.cursor_position() {
                if let Some(world_pos) = tw.screen_to_world(cursor_pos) {
                    let [x, y] = world_pos.to_array();
                    let string = format!("Cursor world pos [{:.2},{:.2}]", x, y);
                    term.put_string([2, 0], string);

                    let tile_pos = tw.world_to_tile(world_pos);
                    let [x, y] = tile_pos.to_array();
                    let string = format!("Cursor tile pos [{},{}]", x, y);
                    term.put_string([2, 1], string);
                }
            }
        }
    }
}
