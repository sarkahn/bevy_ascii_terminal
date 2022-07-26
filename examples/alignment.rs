use bevy::prelude::*;
use bevy_ascii_terminal::{prelude::*, ui::UiProgressBar};
use sark_grids::Pivot;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    let size = [25, 25];
    let mut term = Terminal::with_size(size);

    term.iter_mut().for_each(|t| t.glyph = 'n');

    let ui_box = UiBox::single_line();

    term.draw_box([0, 0].pivot(Pivot::TopLeft), [4, 4], &ui_box);

    let bar = UiProgressBar::default();

    term.draw_progress_bar([0, 0].pivot(Pivot::TopRight), 10, &bar);

    commands
        .spawn_bundle(TerminalBundle::from(term))
        .insert(AutoCamera);
}
