use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_ascii_terminal::{prelude::*, TerminalFont};
use strum::{EnumCount, IntoEnumIterator};

fn main() {
    App::new()
        .init_resource::<FontIndex>()
        // This ensures our font loaded at runtime is set to
        // nearest sampling by default. Failing to do this
        // will result in visual artifacts!
        .insert_resource(ImageSettings::default_nearest())
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
    let mut term = Terminal::with_size(size);

    term.draw_border(BorderGlyphs::single_line());

    let font = TerminalFont::default();
    draw_title(&mut term, font.as_ref());

    let bg_color = Color::MIDNIGHT_BLUE;
    term.put_string(
        [1, 2].pivot(Pivot::TopLeft),
        "Press spacebar to change fonts".bg(bg_color),
    );
    term.put_string(
        [1, 4].pivot(Pivot::TopLeft),
        "!@#$%^&*()_+=-`~".bg(bg_color),
    );
    term.put_string(
        [1, 6].pivot(Pivot::TopLeft),
        "The quick brown fox jumps over the lazy dog.".bg(bg_color),
    );
    term.put_string(
        [1, 8].pivot(Pivot::TopLeft),
        "☺☻♥♦♣♠•'◘'○'◙'♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼".bg(bg_color),
    );
    term.put_string(
        [1, 10].pivot(Pivot::TopLeft),
        "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞".bg(bg_color),
    );
    commands
        .spawn_bundle(TerminalBundle::from(term))
        .insert(AutoCamera);
}

fn draw_title(term: &mut Terminal, title: &str) {
    let ui_box =
        UiBox::single_line().filled(TileFormat::new().fg(Color::WHITE).bg(Color::MIDNIGHT_BLUE));
    term.draw_box([0, 0], term.size(), &ui_box);

    let title = &title[0..title.len() - 4];
    term.draw_border(BorderGlyphs::single_line());
    term.put_string([1, 0].pivot(Pivot::TopLeft), "[ ");
    term.put_string(
        [3, 0].pivot(Pivot::TopLeft),
        title.to_uppercase().fg(Color::RED),
    );
    term.put_string([3 + title.len(), 0].pivot(Pivot::TopLeft), " ]");
}

fn change_font(
    keys: Res<Input<KeyCode>>,
    server: Res<AssetServer>,
    mut font_index: ResMut<FontIndex>,
    mut q: Query<(Entity, &mut Terminal)>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Space) {
        let count = TerminalFont::COUNT;
        for (entity, mut term) in q.iter_mut() {
            font_index.0 = (font_index.0 + 1) % count;

            // Load custom font - note above during app initialization we
            // set the default sampling to nearest to ensure proper
            // rendering with the terminal camera
            let (font, name) = if font_index.0 == TerminalFont::COUNT - 1 {
                let name = "VGA9x16.png";
                let image = server.load(name);
                (TerminalFont::Custom(image), name.to_string())
            // Load a built in font
            } else {
                let font = TerminalFont::iter().nth(font_index.0).unwrap();
                let name = font.as_ref().to_string();
                (font, name)
            };
            commands.entity(entity).insert(font.clone());

            draw_title(&mut term, name.as_str());
        }
    }
}
