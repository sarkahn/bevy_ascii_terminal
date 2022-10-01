use bevy::{
    prelude::*,
    reflect::{DynamicEnum, DynamicVariant, Enum},
    render::texture::ImageSettings,
};
use bevy_ascii_terminal::{prelude::*, TerminalFont};

fn main() {
    App::new()
        .init_resource::<FontIndex>()
        // This ensures our font loaded at runtime is set to
        // nearest sampling by default. Failing to do this
        // will result in visual artifacts for the loaded font!
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(spawn_terminal)
        .add_system(change_font)
        .run()
}

#[derive(Default, Resource)]
struct FontIndex(pub usize);

fn spawn_terminal(mut commands: Commands) {
    let size = [47, 13];
    let font = TerminalFont::default();
    let title = BorderTitle::new(font.variant_name().to_uppercase()).color(Color::RED);
    let mut term = Terminal::with_size(size)
        .with_clear_tile(' '.fg(Color::WHITE).bg(Color::MIDNIGHT_BLUE))
        .with_border(Border::SINGLE_LINE.with_title(title));

    //draw_title(&mut term, font.variant_name());

    let bg_color = Color::MIDNIGHT_BLUE;
    term.put_string(
        [0, 1].pivot(Pivot::TopLeft),
        "Press spacebar to change fonts".bg(bg_color),
    );
    term.put_string(
        [0, 3].pivot(Pivot::TopLeft),
        "!@#$%^&*()_+=-`~".bg(bg_color),
    );
    term.put_string(
        [0, 5].pivot(Pivot::TopLeft),
        "The quick brown fox jumps over the lazy dog.".bg(bg_color),
    );
    term.put_string(
        [0, 7].pivot(Pivot::TopLeft),
        "☺☻♥♦♣♠•'◘'○'◙'♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼".bg(bg_color),
    );
    term.put_string(
        [0, 9].pivot(Pivot::TopLeft),
        "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞".bg(bg_color),
    );
    commands
        .spawn((TerminalBundle::from(term),AutoCamera));

    //commands.spawn_bundle(TiledCameraBundle::new().with_tile_count([50,50]));
}

// fn draw_title(term: &mut Terminal, title: &str) {
//     term.clear_line(term.side_index(Side::Top));
//     term.put_string([1, 0].pivot(Pivot::TopLeft), "[ ");
//     term.put_string(
//         [3, 0].pivot(Pivot::TopLeft),
//         title.to_uppercase().fg(Color::RED),
//     );
//     term.put_string([3 + title.len(), 0].pivot(Pivot::TopLeft), " ]");
// }

fn change_font(
    keys: Res<Input<KeyCode>>,
    server: Res<AssetServer>,
    mut font_index: ResMut<FontIndex>,
    mut q: Query<(Entity, &mut Terminal)>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (entity, mut term) in q.iter_mut() {
            let info = match TerminalFont::default().get_type_info() {
                bevy::reflect::TypeInfo::Enum(info) => info,
                _ => unreachable!(),
            };

            let count = info.variant_len();

            font_index.0 = (font_index.0 + 1) % count;

            // Load custom font - note above during app initialization we
            // set the default sampling to nearest to ensure proper
            // rendering with the terminal camera
            let (font, _) = if font_index.0 == count - 1 {
                let name = "VGA9x16";
                let image = server.load("VGA9x16.png");
                term.border_mut().unwrap().set_title_string(name);
                (TerminalFont::Custom(image), name.to_string())
            // Load a built in font
            } else {
                let variant = match info.variant_at(font_index.0).unwrap() {
                    bevy::reflect::VariantInfo::Unit(var) => var,
                    _ => unreachable!(),
                };
                let mut a = TerminalFont::default();
                let b = DynamicEnum::new_with_index(
                    info.type_name(),
                    font_index.0,
                    variant.name(),
                    DynamicVariant::Unit,
                );
                a.apply(&b);
                term.border_mut()
                    .unwrap()
                    .set_title_string(variant.name().to_uppercase());
                (a.clone(), variant.name().to_owned())
            };
            commands.entity(entity).insert(font.clone());

            //draw_title(&mut term, name.as_str());
        }
    }
}
