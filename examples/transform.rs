//! Demonstrates how to use TerminalTransform and TerminalCamera to
//! convert world coordinates into terminal tile coordinates.

use bevy::{color::palettes::css::BLACK, prelude::*};
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(PostStartup, |mut q: Query<&mut Terminal>| {
            for mut term in &mut q {
                clear_term(&mut term);
                term.put_string([0, 0], "Cursor out of bounds".bg(BLACK));
            }
        })
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Terminal::new([20, 8]),
        TerminalBorder::single_line(),
        TerminalMeshPivot::BottomLeft,
    ));
    commands.spawn((
        Terminal::new([20, 10]),
        TerminalBorder::single_line(),
        TerminalMeshPivot::BottomRight,
    ));
    commands.spawn((
        Terminal::new([20, 10]),
        TerminalBorder::single_line(),
        TerminalMeshPivot::TopCenter,
    ));
    commands.spawn(TerminalCamera::new());
}

fn update(mut q_term: Query<(&mut Terminal, &TerminalTransform)>, q_cam: Query<&TerminalCamera>) {
    let cam = q_cam.single().unwrap();
    let Some(cursor_pos) = cam.cursor_world_pos() else {
        return;
    };
    for (mut term, transform) in &mut q_term {
        clear_term(&mut term);
        if let Some(xy) = transform.world_to_tile(cursor_pos) {
            term.put_string([0, 0], format!("Cursor pos: {}", xy).bg(BLACK));
        } else {
            term.put_string([0, 0], "Cursor out of bounds".bg(BLACK));
        }
    }
}

fn clear_term(term: &mut Terminal) {
    term.clear();
    for (p, t) in term.iter_xy_mut() {
        let grid_color = if (p.x + p.y) % 2 == 0 {
            color::hex_color(0x151515)
        } else {
            color::hex_color(0x040404)
        };
        t.glyph = ' ';
        t.bg_color = grid_color;
    }
}
