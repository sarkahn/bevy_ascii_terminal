use bevy::prelude::*;

use bevy_ascii_terminal::{render::TerminalRendererFont, TerminalBundle, TerminalPlugin};
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin};

const FONTS: [&str; 2] = ["alloy_curses_12x12.png", "zx_evolution_8x8.png"];
#[derive(Default)]
struct FontIndex(pub usize);

fn spawn_terminal(mut commands: Commands) {
    let (w, h) = (20, 3);
    let mut term_bundle = TerminalBundle::with_size(w, h);

    term_bundle.terminal.draw_border_single();
    term_bundle.terminal.put_string(1, 1, "Press spacebar");
    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(PixelCameraBundle::from_resolution(
        w as i32 * 12,
        h as i32 * 12,
    ));
}

fn change_font(
    keys: Res<Input<KeyCode>>,
    mut font_index: ResMut<FontIndex>,
    mut q: Query<&mut TerminalRendererFont>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for mut font in q.iter_mut() {
            font_index.0 = 1 - font_index.0;
            font.font_name = String::from(FONTS[font_index.0]);
        }
    }
}

fn main() {
    App::build()
        .init_resource::<FontIndex>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_plugin(PixelCameraPlugin)
        .add_plugin(PixelBorderPlugin {
            color: Color::BLACK,
        })
        .add_startup_system(spawn_terminal.system())
        .add_system(change_font.system())
        .run()
}
