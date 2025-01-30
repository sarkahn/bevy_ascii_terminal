use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(PostStartup, init)
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

fn init(mut q_term: Query<&mut Terminal>) {
    for mut t in &mut q_term {
        clear_term(&mut t);
    }
}

fn update(mut q_term: Query<(&mut Terminal, &TerminalTransform)>, q_cam: Query<&TerminalCamera>) {
    let cam = q_cam.single();
    let Some(cursor_pos) = cam.cursor_world_pos() else {
        return;
    };
    for (mut term, transform) in &mut q_term {
        clear_term(&mut term);
        if let Some(xy) = transform.world_to_tile(cursor_pos) {
            term.put_string([0, 0], format!("Cursor pos: {}", xy).fg(WHITE).bg(BLACK));
        } else {
            term.put_string([0, 0], "Cursor out of bounds".fg(WHITE).bg(BLACK));
        }
    }
}

fn clear_term(term: &mut Terminal) {
    for (p, t) in term.iter_xy_mut() {
        let grid_color = if (p.x + p.y) % 2 == 0 {
            Color::srgba_u8(0x15, 0x15, 0x15, 0xFF)
        } else {
            Color::srgba_u8(0x04, 0x04, 0x04, 0xFF)
        }
        .to_linear();
        t.glyph = ' ';
        t.bg_color = grid_color;
    }
}
