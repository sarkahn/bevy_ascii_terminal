use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin::default()))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(TerminalCameraBundle::auto());

    commands
        .spawn(TerminalBundle::from(make_terminal([10, 10])).with_mesh_pivot(Pivot::BottomLeft));
    commands.spawn(TerminalBundle::from(make_terminal([10, 10])).with_mesh_pivot(Pivot::TopLeft));
    commands.spawn(TerminalBundle::from(make_terminal([12, 12])).with_mesh_pivot(Pivot::TopRight));
    commands
        .spawn(TerminalBundle::from(make_terminal([13, 15])).with_mesh_pivot(Pivot::BottomRight));
}

fn make_terminal(size: impl GridPoint) -> Terminal {
    let mut term = Terminal::new(size);
    term.put_border(Border::single_line());
    for (i, (p, t)) in term.iter_xy_mut().enumerate() {
        let ch = char::from_u32((i as u32 % 26) + 97).unwrap();

        if (p.x + p.y) % 2 == 0 {
            t.bg_color = Color::rgb(0.0, 0.15, 0.55);
        } else {
            t.bg_color = Color::rgb(0.25, 0.0, 0.0);
        }
        t.glyph = ch;
    }
    term
}
