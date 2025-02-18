//! Demonstrates how SetTerminalGridPosition and SetTerminalLayerPosition can
//! be used to position terminals on the virtual grid.

use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Terminal::new([9, 9]).with_clear_tile(Tile::new(
            ascii::Glyph::SmilingFace.into(),
            color::WHITE,
            color::RED.with_alpha(0.2),
        )),
        SetTerminalGridPosition::from([3, 3]),
        SetTerminalLayerPosition(3),
    ));
    commands.spawn((
        Terminal::new([9, 9]).with_clear_tile(Tile::new(
            ascii::Glyph::FractionQuarter.into(),
            color::GREEN,
            color::BLUE.with_alpha(0.6),
        )),
        SetTerminalGridPosition::from([-3, 3]),
        SetTerminalLayerPosition(2),
    ));
    commands.spawn((
        Terminal::new([20, 10]).with_clear_tile(Tile::new(
            ascii::Glyph::AtSymbol.into(),
            color::ORANGE.with_alpha(0.5),
            color::GRAY.with_alpha(0.7),
        )),
        SetTerminalGridPosition::from([0, -3]),
        SetTerminalLayerPosition(1),
    ));
    commands.spawn(TerminalCamera::new());
}
