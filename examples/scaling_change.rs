use bevy::{
    prelude::*,
    render::camera::{OrthographicProjection, ScalingMode},
};

use bevy_ascii_terminal::{renderer::TileScaling, TerminalBundle, TerminalPlugin};
use bevy_tiled_camera::*;

fn main() {
    App::new()
        .init_resource::<FontIndex>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_plugin(TiledCameraPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup.system())
        .add_system(change_font.system())
        .run()
}

#[derive(Default)]
struct FontIndex(pub usize);

fn setup(mut commands: Commands) {
    let size = [20, 3];
    let mut term_bundle = TerminalBundle::new().with_size(size);

    term_bundle.terminal.draw_border_single();
    term_bundle.terminal.put_string([1, 1], "Press spacebar");
    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(
        TiledCameraBundle::new()
            .with_pixels_per_tile(8)
            .with_tile_count(size),
    );
}

fn change_font(
    keys: Res<Input<KeyCode>>,
    mut q: Query<&mut TileScaling>,
    mut cam_q: Query<&mut OrthographicProjection>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let mut term_scaling = TileScaling::default();

        for mut scaling in q.iter_mut() {
            *scaling = match *scaling {
                TileScaling::Pixels => TileScaling::World,
                TileScaling::World => TileScaling::Pixels,
            };
            term_scaling = *scaling;
        }

        for mut cam in cam_q.iter_mut() {
            match term_scaling {
                TileScaling::Pixels => {
                    cam.scaling_mode = ScalingMode::WindowSize;
                    cam.scale = 1.0;
                }
                TileScaling::World => {
                    cam.scaling_mode = ScalingMode::FixedVertical;
                    cam.scale = 50.0;
                }
            }
        }
    }
}
