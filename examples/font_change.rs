use std::ops::Sub;

use bevy::{
    color::palettes::css::{MAROON, MIDNIGHT_BLUE},
    prelude::*,
    reflect::{DynamicVariant, Enum},
};
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, (input, update))
        .run();
}

fn setup(mut commands: Commands) {
    let size = [47, 12];
    let clear_tile = Tile::default().with_bg(MIDNIGHT_BLUE);
    let string = String::from_iter(CP437.chars());
    let term = Terminal::new(size)
        .with_clear_tile(clear_tile)
        // Unlike put_char, put_string defaults to a top left pivot
        .with_string([0, 0], "Press spacebar to change fonts")
        .with_string([0, 1], "The quick brown fox jumps over the lazy dog.")
        .with_string([0, 2], string.fg(color::TAN));
    // .with_string([0, 7], "☺☻♥♦♣♠•'◘'○'◙'♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼");
    // .with_string([0, 9], "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞");
    commands.spawn((term, TerminalBorder::single_line()));
    commands.spawn(TerminalCamera::new());
}

fn input(input: Res<ButtonInput<KeyCode>>, mut q_term: Query<&mut TerminalFont>) {
    if input.just_pressed(KeyCode::Space) {
        let mut font = q_term.single_mut();
        let info = font
            .get_represented_type_info()
            .expect("Error getting terminal font enum info");
        let info = match info {
            bevy::reflect::TypeInfo::Enum(info) => info,
            _ => unreachable!(),
        };
        // Exclude custom variant
        let max = info.variant_len().sub(2);
        let i = font.variant_index();
        let i = (i + 1).rem_euclid(max);
        let mut dynamic = font.clone_dynamic();
        dynamic.set_variant_with_index(i, info.variant_names()[i], DynamicVariant::Unit);
        font.apply(&dynamic);
    }
}

fn update(mut q_term: Query<(&TerminalFont, &mut TerminalBorder), Changed<TerminalFont>>) {
    if let Ok((font, mut border)) = q_term.get_single_mut() {
        border.clear_strings();
        border.put_title(font.variant_name().fg(MAROON).delimiters("[]"));
    }
}

const CP437: &str = r#"
.☺☻♥♦♣♠•◘○◙♂♀♪♫☼ ►◄↕‼¶§▬↨↑↓→←∟↔▲▼
!\"\#$%&'()*+,-./ 0123456789:;<=>?
@ABCDEFGHIJKLMNO PQRSTUVWXYZ[\]^_
`abcdefghijklmno pqrstuvwxyz{|}~⌂
ÇüéâäàåçêëèïîìÄÅ ÉæÆôöòûùÿÖÜ¢£¥₧ƒ
áíóúñÑªº¿⌐¬½¼¡«» ░▒▓│┤╡╢╖╕╣║╗╝╜╛┐
└┴┬├─┼╞╟╚╔╩╦╠═╬╧ ╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀
αßΓπΣσµτΦΘΩδ∞φε∩ ≡±≥≤⌠⌡÷≈°∙·√ⁿ²■□
"#;
