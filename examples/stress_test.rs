use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_ascii_terminal::{code_page_437, prelude::*};
use rand::prelude::ThreadRng;
use rand::Rng;

fn main() {
    App::new()
        .init_resource::<Pause>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            TerminalPlugin,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, spam_terminal)
        .run();
}

#[derive(Resource, Default)]
struct Pause(bool);

fn setup(mut commands: Commands) {
    commands.spawn((
        TerminalBundle::new()
            .with_size([80, 50])
            .with_border(Border::single_line()),
        AutoCamera,
    ));
}

fn rand_color(rng: &mut ThreadRng) -> Color {
    let r: f32 = rng.gen_range(0.0..=1.0);
    let g: f32 = rng.gen_range(0.0..=1.0);
    let b: f32 = rng.gen_range(0.0..=1.0);
    Color::rgb(r, g, b)
}

fn spam_terminal(
    keys: Res<ButtonInput<KeyCode>>,
    mut pause: ResMut<Pause>,
    mut q: Query<&mut Terminal>,
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
            let index = rng.gen_range(0..=255) as u8;
            let glyph = code_page_437::index_to_glyph(index);
            let fg = rand_color(&mut rng);
            let bg = rand_color(&mut rng);

            *t = Tile {
                glyph,
                fg_color: fg,
                bg_color: bg,
            }
        }
        let top = term.side_index(Side::Top) as i32;
        term.clear_box([0, top], [25, 1]);
        term.put_string([0, top], "Press space to pause");
    }
}
