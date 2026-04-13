//! Spamming the entire terminal with random glyphs and colors.
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode};
use bevy_ascii_terminal::*;
use rand::Rng;
use rand::rngs::ThreadRng;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                ..Default::default()
            }),
            ..Default::default()
        }),
        TerminalPlugins,
        FrameTimeDiagnosticsPlugin::default(),
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, spam_terminal)
    .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        Terminal::new([82, 52])
            .with_border(BoxStyle::SINGLE_LINE)
            .with_title(" Press space to pause"),
    );
    commands.spawn(TerminalCamera::new());
}

fn rand_color(rng: &mut ThreadRng) -> LinearRgba {
    let r = rng.gen_range(0..=255) as u8;
    let g = rng.gen_range(0..=255) as u8;
    let b = rng.gen_range(0..=255) as u8;
    let a = rng.gen_range(100..=255) as u8;
    color::srgba_bytes(r, g, b, a)
}

fn spam_terminal(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut q: Query<&mut Terminal>,
    mut pause: Local<bool>,
    input: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
    diag: Res<DiagnosticsStore>,
) {
    if input.just_pressed(KeyCode::Space) {
        *pause = !(*pause);
    }
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    if input.just_pressed(KeyCode::KeyF) {
        if window.mode == WindowMode::BorderlessFullscreen(MonitorSelection::Current) {
            window.mode = WindowMode::Windowed;
        } else {
            window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        }
    }

    let mut term = q.single_mut().unwrap();
    if !*pause {
        let mut rng = rand::thread_rng();

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

    let Some(fps) = diag
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
    else {
        return;
    };

    let col = if fps < 60.0 { "red" } else { "green" };

    term.set_pivot(Pivot::RightTop);
    term.put_string(
        [0, 0],
        format!("FPS: <fg={}>{}</fg>", col, fps.round() as u32),
    );
    term.set_pivot(Pivot::LeftTop);
}
