use bevy::prelude::*;
use bevy_ascii_terminal::{formatting::StringWriter, ui::{BorderGlyphs, UiBox}, *};
use bevy_tiled_camera::*;
use sark_grids::Pivot;

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

fn spawn_terminal(mut commands: Commands, fonts: Res<BuiltInFontHandles>) {
    let size = [47, 13];
    let mut term_bundle = TerminalBundle::new().with_size(size);

    term_bundle
        .terminal
        .draw_border(BorderGlyphs::single_line());

    let fonts: Vec<_> = fonts.iter().collect();
    let term = &mut term_bundle.terminal;

    draw_title(&mut term_bundle.terminal, fonts[0].0);

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
            let fonts: Vec<_> = built_in_fonts.iter().collect();

            font_index.0 = (font_index.0 + 1) % fonts.len();

            let new_font = fonts[font_index.0];

            let tex = images.get(new_font.1);
            let ppu = tex.unwrap().texture_descriptor.size.height / 16;

            cam.pixels_per_tile = ppu;

            let mut mat = materials.get_mut(mat).unwrap();
            mat.texture = Some(new_font.1.clone());

            draw_title(&mut term, new_font.0);
        }
    }
}
