use bevy::{
    prelude::*,
    render::camera::{OrthographicProjection, ScalingMode},
};

use bevy_ascii_terminal::{render::TerminalTileScaling, TerminalBundle, TerminalPlugin};

#[derive(Default)]
struct FontIndex(pub usize);

fn spawn_terminal(mut commands: Commands) {
    let mut term_bundle = TerminalBundle::with_size(20, 3);

    term_bundle.terminal.draw_border_single();
    term_bundle.terminal.put_string(1, 1, "Press spacebar");
    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn change_font(
    keys: Res<Input<KeyCode>>,
    mut q: Query<&mut TerminalTileScaling>,
    mut cam_q: Query<&mut OrthographicProjection>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let mut term_scaling = TerminalTileScaling::default();

        for mut scaling in q.iter_mut() {
            *scaling = match *scaling {
                TerminalTileScaling::Pixels => TerminalTileScaling::Window,
                TerminalTileScaling::Window => TerminalTileScaling::Pixels,
            };
            term_scaling = scaling.clone();
        }

        for mut cam in cam_q.iter_mut() {
            match term_scaling {
                TerminalTileScaling::Pixels => {
                    cam.scaling_mode = ScalingMode::WindowSize;
                    cam.scale = 1.0;
                }
                TerminalTileScaling::Window => {
                    cam.scaling_mode = ScalingMode::FixedVertical;
                    cam.scale = 12.0;
                }
            }
        }
    }
}

fn main() {
    // TODO: Remove after bevy 0.5.0
    println!(
        "Warning! There's a bug in bevy 0.5.0 that prevents the camera from detecing changes. 
    You can force it to update by toggling maximing the window."
    );
    App::build()
        .init_resource::<FontIndex>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(spawn_terminal.system())
        .add_system(change_font.system())
        .run()
}
