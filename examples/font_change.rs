use bevy::prelude::*;
use bevy_ascii_terminal::{renderer::BuiltInFontHandles, *};
use strum::IntoEnumIterator;

fn main() {
    App::new()
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
    built_in_fonts: Res<BuiltInFontHandles>,
    mut font_index: ResMut<FontIndex>,
    mut q: Query<(Entity, &mut Terminal)>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (entity, mut term) in q.iter_mut() {
            font_index.0 = (font_index.0 + 1) % built_in_fonts.len();

            let font = TerminalFont::iter().nth(font_index.0).unwrap();
            commands.entity(entity).insert(font.clone());

            let name = font.as_ref();

            draw_title(&mut term, name);
        }
    }
}
