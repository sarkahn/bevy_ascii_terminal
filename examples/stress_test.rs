use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::prelude::*;

use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
use rand::prelude::ThreadRng;
use rand::Rng;

fn main() {
    App::new()
        .init_resource::<Pause>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_plugin(TiledCameraPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_system(spam_terminal.system())
        .run();
}

#[derive(Component, Default)]
struct Pause(bool);

fn setup(mut commands: Commands) {
    let size = [80, 50];

    let term = TerminalBundle::new().with_size(size);
    commands.spawn_bundle(term).insert(Pause::default());

    commands.spawn_bundle(
        TiledCameraBundle::new()
            .with_pixels_per_tile(8)
            .with_tile_count(size),
    );
}

fn rand_color(rng: &mut ThreadRng) -> TileColor {
    let r = rng.gen_range(0, 255) as u8;
    let g = rng.gen_range(0, 255) as u8;
    let b = rng.gen_range(0, 255) as u8;
    TileColor::rgb(r, g, b)
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
            let index = rng.gen_range(0, 255) as u8;
            let glyph = code_page_437::index_to_glyph(index);
            let fg = rand_color(&mut rng);
            let bg = rand_color(&mut rng);

            *t = Tile {
                glyph,
                fg_color: fg,
                bg_color: bg,
            }
        }
        let top = term.top_index() as i32;
        term.clear_box([0, top - 1], [25, 1]);
        term.draw_border_single_color(WHITE, BLACK);
        term.put_string([1, top - 1], "Press space to pause");
    }
}
