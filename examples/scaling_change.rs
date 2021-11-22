use bevy::{
    prelude::*,
    render::camera::{OrthographicProjection, ScalingMode},
};

use bevy_ascii_terminal::{renderer::TerminalTileScaling, TerminalBundle, TerminalPlugin};
use bevy_tiled_camera::*;

fn main() {
    App::build()
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

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let size = (20, 3);
    let mut term_bundle = TerminalBundle::new().with_size(size);

    term_bundle.terminal.draw_border_single();
    term_bundle.terminal.put_string((1, 1), "Press spacebar");
    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::ONE,
            flip: false,
        })),
        transform: Transform::from_xyz(-1.5, -0.5, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(
        TiledCameraBundle::new()
            .with_pixels_per_tile(8)
            .with_tile_count(size),
    );
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
                TerminalTileScaling::Pixels => TerminalTileScaling::World,
                TerminalTileScaling::World => TerminalTileScaling::Pixels,
            };
            term_scaling = *scaling;
        }

        for mut cam in cam_q.iter_mut() {
            match term_scaling {
                TerminalTileScaling::Pixels => {
                    cam.scaling_mode = ScalingMode::WindowSize;
                    cam.scale = 1.0;
                }
                TerminalTileScaling::World => {
                    cam.scaling_mode = ScalingMode::FixedVertical;
                    cam.scale = 12.0;
                }
            }
        }
    }
}
