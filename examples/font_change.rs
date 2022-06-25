use bevy::prelude::*;
use bevy_ascii_terminal::{formatting::StringWriter, ui::{BorderGlyphs, UiBox}, *, renderer::BuiltInFontHandles};
use bevy_tiled_camera::*;
use sark_grids::Pivot;
use strum::IntoEnumIterator;

fn main() {
    App::new()
        // Must add TiledCameraPlugin first: https://github.com/bevyengine/bevy/issues/1255
        .add_plugin(TiledCameraPlugin)
        .init_resource::<FontIndex>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(spawn_terminal)
        .add_system(change_font)
        .run()
}

#[derive(Default)]
struct FontIndex(pub usize);

fn spawn_terminal(mut commands: Commands) {
    let size = [47, 13];
    let mut term_bundle = TerminalBundle::new().with_size(size);

    term_bundle
        .terminal
        .draw_border(BorderGlyphs::single_line());

    let term = &mut term_bundle.terminal;

    //let current = TerminalFont[0];

    let font = TerminalFont::default();
    draw_title(&mut term_bundle.terminal, font.as_ref());

    let bg_color = Color::MIDNIGHT_BLUE;
    term_bundle.terminal.put_string(
        [1, 2].pivot(Pivot::TopLeft),
        "Press spacebar to change fonts".bg(bg_color),
    );
    term_bundle.terminal.put_string(
        [1, 4].pivot(Pivot::TopLeft), 
        "!@#$%^&*()_+=-`~".bg(bg_color)
    );
    term_bundle.terminal.put_string(
        [1, 6].pivot(Pivot::TopLeft),
        "The quick brown fox jumps over the lazy dog.".bg(bg_color),
    );
    term_bundle.terminal.put_string(
        [1, 8].pivot(Pivot::TopLeft),
        "☺☻♥♦♣♠•'◘'○'◙'♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼".bg(bg_color),
    );
    term_bundle.terminal.put_string(
        [1, 10].pivot(Pivot::TopLeft),
        "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞".bg(bg_color),
    );
    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(
        TiledCameraBundle::new()
            .with_pixels_per_tile(8)
            .with_tile_count(size),
    );
}

fn draw_title(term: &mut Terminal, title: &str) {
    let ui_box = UiBox::single_line().filled(
        TileFormat::new().fg(Color::WHITE).bg(Color::MIDNIGHT_BLUE)
    );
    term.draw_box([0,0], term.size(), &ui_box);

    let title = &title[0..title.len() - 4];
    term.draw_border(BorderGlyphs::single_line());
    term.put_string([1, 0].pivot(Pivot::TopLeft), "[ ");
    term.put_string(
        [3, 0].pivot(Pivot::TopLeft),
        title
        .to_uppercase()
        .fg(Color::RED)
    );
    term.put_string(
        [3 + title.len(), 0].pivot(Pivot::TopLeft),
        " ]"
    );
}

fn change_font(
    keys: Res<Input<KeyCode>>,
    built_in_fonts: Res<BuiltInFontHandles>,
    images: Res<Assets<Image>>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
    mut font_index: ResMut<FontIndex>,
    mut q: Query<(&mut Terminal, &Handle<TerminalMaterial>)>,
    mut q_cam_projection: Query<&mut TiledCamera>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let mut cam = q_cam_projection.single_mut();
        for (mut term, mat) in q.iter_mut() {

            font_index.0 = (font_index.0 + 1) % built_in_fonts.len();
            //let font = TerminalFont::from_index(font_index.0);
            let font = TerminalFont::iter().nth(font_index.0).unwrap();
            let name = font.as_ref();
            //let name = String::from_iter(name);
            let font_handle = built_in_fonts.get(&font);
            let font = images.get(font_handle).unwrap();

            let ppu = font.size().y as u32 / 16;

            cam.pixels_per_tile = ppu;

            let mut mat = materials.get_mut(mat).unwrap();
            mat.texture = Some(font_handle.clone());

            draw_title(&mut term, name);
        }
    }
}
