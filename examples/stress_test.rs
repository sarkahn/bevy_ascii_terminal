use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::prelude::*;

use bevy::render::camera::ScalingMode;
use bevy_ascii_terminal::{terminal::Terminal, terminal::Tile, TerminalBundle, TerminalPlugin};
use rand::prelude::ThreadRng;
use rand::Rng;

#[derive(Default)]
struct Pause(bool);

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(TerminalBundle::with_size(50, 50))
        .insert(Pause);

        let mut cam = OrthographicCameraBundle::new_2d();
        cam.orthographic_projection.scaling_mode = ScalingMode::FixedVertical;
        cam.orthographic_projection.scale = 25.0;
        cam.transform.translation += Vec3::new(25.0, 25.0, 0.0);

    commands.spawn_bundle(cam);
}

fn rand_color(rng: &mut ThreadRng) -> Color {
    let r = 1.0 / (rng.gen_range(0, 255) as f32);
    let g = 1.0 / (rng.gen_range(0, 255) as f32);
    let b = 1.0 / (rng.gen_range(0, 255) as f32);
    Color::rgb(r, g, b)
}

fn spam_terminal(keys: Res<Input<KeyCode>>, mut pause: ResMut<Pause>, mut q: Query<&mut Terminal>) {
    if keys.just_pressed(KeyCode::Space) {
        pause.0 = !pause.0;
    }

    if pause.0 {
        return;
    }

    let mut rng = rand::thread_rng();
    for mut term in q.iter_mut() {
        for t in term.iter_mut() {
            let glyph = (rng.gen_range(0, 255) as u8) as char;
            let fg = rand_color(&mut rng);
            let bg = rand_color(&mut rng);

            *t = Tile {
                glyph,
                fg_color: fg,
                bg_color: bg,
            }
        }
        term.clear_box(0,0,25,3);
        term.draw_box_single(0, 0, 25, 3);
        term.put_string(1,1, "Press space to pause");
    }
}

fn main() {
    App::build()
        .init_resource::<Pause>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(spam_terminal.system())
        .run();
}