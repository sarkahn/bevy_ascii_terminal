use bevy::prelude::*;
use bevy_ascii_terminal::color::*;
use bevy_ascii_terminal::*;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

const FONTS: &[TerminalFontBuiltIn] = &[
    FONT_PX437_8X8,
    FONT_ZX_EVOLUTION_8X8,
    FONT_JT_CURSES_12X12,
    FONT_PASTICHE_8X8,
    FONT_TAFFER_10X10,
];

#[derive(Default)]
struct FontIndex(pub usize);

fn spawn_terminal(mut commands: Commands) {
    let (w, h) = (47, 12);
    let mut term_bundle = TerminalBundle::with_size(w, h);

    term_bundle.terminal.draw_border_single();

    draw_title(&mut term_bundle.terminal, FONTS[0].name);

    term_bundle
        .terminal
        .put_string(1, 2, "Press spacebar to change fonts");
    term_bundle.terminal.put_string(1, 4, "!@#$%^&*()_+=-`~");
    term_bundle
        .terminal
        .put_string(1, 6, "The quick brown fox jumps over the lazy dog.");
    term_bundle
        .terminal
        .put_string(1, 8, "☺☻♥♦♣♠•'◘'○'◙'♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼");
    term_bundle
        .terminal
        .put_string(1, 10, "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞");
    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(PixelCameraBundle::from_resolution(
        w as i32 * 12,
        h as i32 * 12,
    ));
}

fn draw_title(term: &mut Terminal, title: &str) {
    let title = &title[0..title.len() - 4];

    term.draw_border_single_color(WHITE, BLACK);
    term.put_string(1, 0, "[ ");
    term.put_string_color(3, 0, title.to_string().to_uppercase().as_str(), BLUE, BLACK);
    term.put_string(4 + title.len() as i32 - 1, 0, " ]");
}

fn change_font(
    keys: Res<Input<KeyCode>>,
    mut font_index: ResMut<FontIndex>,
    mut q: Query<(&mut Terminal, &mut TerminalRendererFont)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (mut term, mut font) in q.iter_mut() {
            font_index.0 = (font_index.0 + 1) % FONTS.len();
            let new_font = &FONTS[font_index.0];
            font.font_name = String::from(new_font.name);
            draw_title(&mut term, new_font.name);
        }
    }
}

fn main() {
    App::build()
        .init_resource::<FontIndex>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(PixelCameraPlugin)
        .add_startup_system(spawn_terminal.system())
        .add_system(change_font.system())
        .run()
}
