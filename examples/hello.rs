//! A minimal example with a terminal and camera.

use bevy::{color::palettes::css, prelude::*};
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(
        Terminal::new([14, 3])
            .with_border(BoxStyle::SINGLE_LINE)
            .with_padding(Padding::ONE)
            .with_bg_clear_color(css::DARK_BLUE.into())
            .with_string(
                [0, 0],
                "<bg=midnight_blue><fg=firebrick>Hello</fg> <fg=#5772e5>world!</fg></bg>",
            ),
    );
    commands.spawn(TerminalCamera::new());
}
