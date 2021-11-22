use bevy::prelude::*;
use bevy_ascii_terminal::renderer::font::BUILT_IN_FONT_NAMES;
use bevy_ascii_terminal::renderer::{TerminalFont, TerminalFonts};
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;

fn main() {
    App::build()
        .init_resource::<FontIndex>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(TiledCameraPlugin)
        .add_startup_system(spawn_terminal.system())
        .add_system(change_font.system())
        .run()
}

#[derive(Default)]
struct FontIndex(pub usize);

fn spawn_terminal(mut commands: Commands) {
    let size = (47, 12);
    let mut term_bundle = TerminalBundle::new().with_size(size);

    term_bundle.terminal.draw_border_single();

    draw_title(&mut term_bundle.terminal, BUILT_IN_FONT_NAMES[0]);

    term_bundle
        .terminal
        .put_string((1, 2), "Press spacebar to change fonts");
    term_bundle.terminal.put_string((1, 4), "!@#$%^&*()_+=-`~");
    term_bundle
        .terminal
        .put_string((1, 6), "The quick brown fox jumps over the lazy dog.");
    term_bundle
        .terminal
        .put_string((1, 8), "☺☻♥♦♣♠•'◘'○'◙'♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼");
    term_bundle
        .terminal
        .put_string((1, 10), "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞");
    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(
        TiledCameraBundle::new()
            .with_pixels_per_tile(8)
            .with_tile_count(size),
    );
}

fn draw_title(term: &mut Terminal, title: &str) {
    let title = &title[0..title.len() - 4];

    term.draw_border_single_color(WHITE, BLACK);
    term.put_string((1, 0), "[ ");
    term.put_string_color(
        (3, 0),
        title.to_string().to_uppercase().as_str(),
        BLUE,
        BLACK,
    );
    term.put_string((4 + title.len() as i32 - 1, 0), " ]");
}

fn change_font(
    keys: Res<Input<KeyCode>>,
    fonts: Res<TerminalFonts>,
    mut font_index: ResMut<FontIndex>,
    mut q: Query<(&mut Terminal, &mut TerminalFont)>,
    mut q_cam_projection: Query<&mut TiledProjection>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let mut projection = q_cam_projection.single_mut().unwrap();
        for (mut term, mut font) in q.iter_mut() {
            font_index.0 = (font_index.0 + 1) % BUILT_IN_FONT_NAMES.len();
            let new_font = BUILT_IN_FONT_NAMES[font_index.0];
            let font_data = fonts.get(new_font);
            projection.pixels_per_tile = font_data.1.tile_size.y;
            font.file_name = String::from(new_font);
            draw_title(&mut term, new_font);
        }
    }
}
