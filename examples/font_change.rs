use std::ops::Sub;

use bevy::{
    color::palettes::css,
    prelude::*,
    reflect::{DynamicVariant, Enum},
};
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, (input, update))
        .run();
}

fn setup(mut commands: Commands) {
    let size = [47, 13];
    let clear_tile = *Tile::default()
        .fg(Color::WHITE)
        .bg(css::MIDNIGHT_BLUE.into());
    let term = TerminalBundle::new(size)
        .with_clear_tile(clear_tile)
        // Unlike put_char, put_string defaults to a top left pivot
        .put_string([0, 1], "Press spacebar to change fonts")
        .put_string([0, 3], "!@#$%^&*()_+=-`~")
        .put_string([0, 5], "The quick brown fox jumps over the lazy dog.")
        .put_string([0, 7], "☺☻♥♦♣♠•'◘'○'◙'♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼")
        .put_string([0, 9], "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞")
        .with_border(Border::single_line());
    commands.spawn(term);
    commands.spawn(TerminalCameraBundle::auto());
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
        let max = info.variant_len().sub(1);
        let i = font.variant_index();
        // Exclude custom variant
        let i = (i + 1).rem_euclid(max);
        let mut dynamic = font.clone_dynamic();
        dynamic.set_variant_with_index(i, info.variant_names()[i], DynamicVariant::Unit);
        font.apply(&dynamic);
    }
}

fn update(mut q_term: Query<(&mut Terminal, &TerminalFont), Changed<TerminalFont>>) {
    if let Ok((mut term, font)) = q_term.get_single_mut() {
        term.border_mut()
            .put_title(format!("[{}]", font.variant_name()).fg(css::MAROON.into()));
    }
}
