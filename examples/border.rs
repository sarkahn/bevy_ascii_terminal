use bevy::{app::AppExit, prelude::*};
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin::default()))
        .add_systems(Update, input)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(TerminalCameraBundle::auto());
    commands.spawn(
        TerminalBundle::new([10, 5])
            .with_border(Border::from_string("┌─┬││└─┴"))
            .with_mesh_pivot(Pivot::RightCenter)
            .put_string([0, 0], "0123456789"),
    );
    commands.spawn(
        TerminalBundle::new([10, 5])
            // Empty border characters will not be rendered. If an entire border
            // side is empty, the terminal bounds will be adjusted accordingly.
            // Here we leave the left side of the border blank since the first
            // terminal will draw the needed glyphs
            .with_border(Border::from_string(" ─┐ │ ─┘"))
            .with_mesh_pivot(Pivot::LeftCenter)
            .put_string([0, 0], "0123456789"),
    );
}

fn input(input: Res<ButtonInput<KeyCode>>, mut evt_quit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        evt_quit.send(AppExit::Success);
    }
}
