//! Spamming the entire terminal with random glyphs and colors.

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_ascii_terminal::*;
use rand::rngs::ThreadRng;
use rand::Rng;

fn main() {
    let mut app = App::new();
    if !cfg!(debug_assertions) {
        app.add_plugins((LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin));
    };
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }),
        TerminalPlugins,
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, spam_terminal)
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Terminal::new([80, 50]),
        TerminalBorder::single_line().with_title("Press space to pause"),
    ));
    commands.spawn(TerminalCamera::new());
}

fn rand_color(rng: &mut ThreadRng) -> LinearRgba {
    let r: f32 = rng.gen_range(0.0..=1.0);
    let g: f32 = rng.gen_range(0.0..=1.0);
    let b: f32 = rng.gen_range(0.0..=1.0);
    let a: f32 = rng.gen_range(0.2..=1.0);
    Color::linear_rgba(r, g, b, a).into()
}

fn spam_terminal(
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut Terminal>,
    mut pause: Local<bool>,
) {
    if keys.just_pressed(KeyCode::Space) {
        *pause = !(*pause);
    }

    if *pause {
        return;
    }

    let _ = info_span!("span_name", name = "span_name").entered();

    let mut rng = rand::thread_rng();
    let mut term = q.single_mut();
    for t in term.iter_mut() {
        let index = rng.gen_range(0..=255) as u8;
        let glyph = ascii::index_to_char(index);
        let fg = rand_color(&mut rng);
        let bg = rand_color(&mut rng);

        *t = Tile {
            glyph,
            fg_color: fg,
            bg_color: bg,
        }
    }
}
