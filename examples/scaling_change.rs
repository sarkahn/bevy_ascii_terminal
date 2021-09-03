use bevy::{
    prelude::*,
    render::camera::{OrthographicProjection, ScalingMode},
};

use bevy_ascii_terminal::{render::TerminalTileScaling, TerminalBundle, TerminalPlugin};
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin};

#[derive(Default)]
struct FontIndex(pub usize);

fn spawn_terminal(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let (w,h) = (20,3);
    let mut term_bundle = TerminalBundle::with_size(w, h);

    term_bundle.terminal.draw_border_single();
    term_bundle.terminal.put_string(1, 1, "Press spacebar");
    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad {
            size: Vec2::ONE,
            flip: false,
        })),
        transform: Transform::from_xyz(-1.5, -0.5, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PixelCameraBundle::from_resolution(w as i32 * 12, h as i32 * 12));
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

fn main() {
    App::build()
        .init_resource::<FontIndex>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_plugin(PixelCameraPlugin)
        .add_plugin(PixelBorderPlugin { color: Color::BLACK} )
        .add_startup_system(spawn_terminal.system())
        .add_system(change_font.system())
        .run()
}
