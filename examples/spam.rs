//! Spamming the entire terminal with random glyphs and colors.

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode};
use bevy_ascii_terminal::*;
use rand::Rng;
use rand::rngs::ThreadRng;

fn main() {
    let mut app = App::new();
    if !cfg!(debug_assertions) {
        app.add_plugins((
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::new(100),
        ));
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
    commands.spawn(
        Terminal::new([82, 52])
            .with_border(BoxStyle::SINGLE)
            .with_title(" Press space to pause"),
    );
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
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<&mut Terminal>,
    mut pause: Local<bool>,
) {
    if keys.just_pressed(KeyCode::Space) {
        *pause = !(*pause);
    }
    if keys.just_pressed(KeyCode::KeyF)
        && let Ok(mut window) = windows.single_mut()
    {
        if window.mode == WindowMode::BorderlessFullscreen(MonitorSelection::Current) {
            window.mode = WindowMode::Windowed;
        } else {
            window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        }
    }

    if *pause {
        return;
    }

    let mut rng = rand::thread_rng();
    let mut term = q.single_mut().unwrap();
    for y in 1..term.height() - 1 {
        for x in 1..term.width() - 1 {
            let index = rng.gen_range(0..=255) as u8;
            let glyph = ascii::index_to_char(index);
            let fg = rand_color(&mut rng);
            let bg = rand_color(&mut rng);

            let i = y * term.width() + x;
            let t = &mut term.tiles_mut()[i];
            t.glyph = glyph;
            t.fg_color = fg;
            t.bg_color = bg;
        }
    }
}
