//! Spamming the entire terminal with random glyphs and colors.
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
                present_mode: PresentMode::Immediate,
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
            .with_border(BoxStyle::SINGLE_LINE)
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

#[derive(Default)]
struct Fps {
    start: f32,
    count: u32,
    fps: u32,
}

fn spam_terminal(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut q: Query<&mut Terminal>,
    time: Res<Time>,
    mut fps: Local<Fps>,
    mut pause: Local<bool>,
    input: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    fps.count += 1;
    let now = time.elapsed_secs();
    if now - fps.start > 1.0 {
        fps.fps = fps.count;
        fps.count = 0;
        fps.start = now;
    }

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

    term.set_pivot(Pivot::RightTop);
    term.put_string([0, 0], format!("FPS: {}", fps.fps));
    term.set_pivot(Pivot::LeftTop);
}
