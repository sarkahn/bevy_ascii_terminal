use bevy::{
    prelude::*,
    reflect::{DynamicVariant, Enum},
};
use bevy_ascii_terminal::{color::hex_color, *};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, (input, update))
        .run();
}

fn setup(mut commands: Commands) {
    let size = [36, 14];
    let term = Terminal::new(size)
        .with_bg_clear_color(hex_color(0x000063))
        .with_border(BoxStyle::SINGLE)
        .with_padding(Padding::ONE)
        .with_string([0, 0], "Press spacebar to change fonts")
        .with_string([0, 1], "The quick brown fox jumps over the lazy dog.")
        .with_string([0, 3], BIG_STRING);
    commands.spawn(term);
    commands.spawn(TerminalCamera::new());
}

fn input(input: Res<ButtonInput<KeyCode>>, mut q_term: Query<&mut TerminalFont>) {
    if input.just_pressed(KeyCode::Space) {
        let mut font = q_term.single_mut().unwrap();
        let info = font
            .get_represented_type_info()
            .expect("Error getting terminal font enum info");
        let info = match info {
            bevy::reflect::TypeInfo::Enum(info) => info,
            _ => unreachable!(),
        };
        // Exclude custom variant
        let max = info.variant_len() - 2;
        let i = font.variant_index();
        let i = (i + 1).rem_euclid(max);
        let mut dynamic = font.to_dynamic_enum();
        dynamic.set_variant_with_index(i, info.variant_names()[i], DynamicVariant::Unit);
        font.apply(&dynamic);
    }
}

fn update(mut q_term: Query<(&mut Terminal, &TerminalFont), Changed<TerminalFont>>) {
    if let Ok((mut term, font)) = q_term.single_mut() {
        let new_string = format!(" [<fg=maroon>{}</fg>]", font.variant_name());
        term.put_border(BoxStyle::SINGLE);
        term.put_title(new_string.as_str());
    }
}

/// A Big string with unicode and fancy colors
const BIG_STRING: &str = r#"
<bg=002b59><fg=#6ceded>.☺☻♥♦♣♠•◘○◙♂♀♪♫☼ ►◄↕‼¶§▬↨↑↓→←∟↔▲▼</fg>
<fg=#6cb9c9>!"\#$%&'()*+,-./ 0123456789:;/<=>?</fg></bg>
<bg=#005f8c><fg=#6d85a5>@ABCDEFGHIJKLMNO PQRSTUVWXYZ[\]^_</fg>
<fg=#6e5181>`abcdefghijklmno pqrstuvwxyz{|}~⌂</fg></bg>
<bg=#00b9be><fg=#6f1d5c>ÇüéâäàåçêëèïîìÄÅ ÉæÆôöòûùÿÖÜ¢£¥₧ƒ</fg>
<fg=#4f1446>áíóúñÑªº¿⌐¬½¼¡«» ░▒▓│┤╡╢╖╕╣║╗╝╜╛┐</fg></bg>
<bg=#9ff4e5><fg=#2e0a30>└┴┬├─┼╞╟╚╔╩╦╠═╬╧ ╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀</fg>
<fg=#0d001a>αßΓπΣσµτΦΘΩδ∞φε∩ ≡±≥≤⌠⌡÷≈°∙·√ⁿ²■□</fg></bg>
"#;
