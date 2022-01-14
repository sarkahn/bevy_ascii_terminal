use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;

fn main() {
    App::new()
        .init_resource::<FontIndex>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugin(TiledCameraPlugin)
        .add_startup_system(spawn_terminal)
        .add_system(change_font)
        .run()
}

#[derive(Default)]
struct FontIndex(pub usize);

fn spawn_terminal(mut commands: Commands, fonts: Res<BuiltInFontHandles>) {
    let size = [47, 13];
    let mut term_bundle = TerminalBundle::new().with_size(size);

    term_bundle.terminal.draw_border_single();

    let fonts: Vec<_> = fonts.iter().collect();

    draw_title(&mut term_bundle.terminal, fonts[0].0);

    let format = Pivot::TopLeft.into();
    term_bundle
        .terminal
        .put_string_formatted([1, 2], "Press spacebar to change fonts", format);
    term_bundle.terminal.put_string([1, 4], "!@#$%^&*()_+=-`~");
    term_bundle.terminal.put_string_formatted(
        [1, 6],
        "The quick brown fox jumps over the lazy dog.",
        format,
    );
    term_bundle.terminal.put_string_formatted(
        [1, 8],
        "☺☻♥♦♣♠•'◘'○'◙'♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼",
        format,
    );
    term_bundle.terminal.put_string_formatted(
        [1, 10],
        "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞",
        format,
    );
    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(
        TiledCameraBundle::new()
            .with_pixels_per_tile(8)
            .with_tile_count(size),
    );
}

fn draw_title(term: &mut Terminal, title: &str) {
    let title = &title[0..title.len() - 4];

    term.draw_border_single();
    term.put_string_formatted([1, 0], "[ ", Pivot::TopLeft.into());
    term.put_string_formatted(
        [3, 0],
        title.to_string().to_uppercase().as_str(),
        StringFormat::new(Pivot::TopLeft, Color::BLUE, Color::BLACK),
    );
    term.put_string_formatted([4 + title.len() as i32 - 1, 0], " ]", Pivot::TopLeft.into());
}

fn change_font(
    keys: Res<Input<KeyCode>>,
    built_in_fonts: Res<BuiltInFontHandles>,
    images: Res<Assets<Image>>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
    mut font_index: ResMut<FontIndex>,
    mut q: Query<(&mut Terminal, &Handle<TerminalMaterial>)>,
    mut q_cam_projection: Query<&mut TiledProjection>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let mut projection = q_cam_projection.single_mut();
        for (mut term, mat) in q.iter_mut() {
            let fonts: Vec<_> = built_in_fonts.iter().collect();

            font_index.0 = (font_index.0 + 1) % fonts.len();

            let new_font = fonts[font_index.0];

            let tex = images.get(new_font.1);
            let ppu = tex.unwrap().texture_descriptor.size.height / 16;

            projection.pixels_per_tile = ppu;

            let mut mat = materials.get_mut(mat).unwrap();
            mat.texture = Some(new_font.1.clone());

            draw_title(&mut term, new_font.0);
        }
    }
}
