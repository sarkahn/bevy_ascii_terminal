use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::prelude::*;

use bevy_terminal::terminal::{Terminal, Tile};
use bevy_terminal::{TerminalBundle, TerminalPlugin};
use rand::Rng;
use rand::prelude::ThreadRng;

#[derive(Default)]
struct Pause(bool);

fn setup(
    mut commands: Commands
) {
    commands.spawn_bundle(TerminalBundle::with_size(50,50)).insert(Pause);

    let mut cam = PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };
    cam.transform.translation += Vec3::new(25.0,25.5,0.0);

    commands.spawn_bundle(cam);
}

fn rand_color(rng: &mut ThreadRng) -> Color {
    let r = 1.0 / (rng.gen_range(0,255) as f32);
    let g = 1.0 / (rng.gen_range(0,255) as f32);
    let b = 1.0 / (rng.gen_range(0,255) as f32);
    Color::rgb(r,g,b)
}

fn spam_terminal(
    keys: Res<Input<KeyCode>>,
    mut pause: ResMut<Pause>,
    mut q: Query<&mut Terminal>
) {
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