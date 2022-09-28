//! [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
//! [![Crates.io](https://img.shields.io/crates/v/bevy_ascii_terminal)](https://crates.io/crates/bevy_ascii_terminal/)
//! [![docs](https://docs.rs/bevy_ascii_terminal/badge.svg)](https://docs.rs/bevy_ascii_terminal/)
//!
//! # `Bevy Ascii Terminal`
//!
//! A simple ascii terminal integrated into bevy's ecs framework.
//!
//! ---
//!
//! The goal of this crate is to provide a simple, straightforward, and hopefully
//! fast method for rendering colorful ascii in bevy. It was made with "traditional
//! roguelikes" in mind, but should serve as a simple UI tool if needed.
//!
//! # Example
//!
//! ```rust no_run
//! use bevy::prelude::*;
//! use bevy_ascii_terminal::*;
//!
//! fn setup(mut commands: Commands) {
//!     // Create the terminal and write to it
//!     let mut terminal = Terminal::with_size([20,3]);
//!     terminal.draw_border(BorderGlyphs::single_line());
//!     // Draw a blue "Hello world!" to the terminal
//!     terminal.put_string([1, 1], "Hello world!".fg(Color::BLUE));
//!
//!     // Spawn the terminal bundle from our terminal
//!     commands.spawn_bundle(TerminalBundle::from(terminal))
//!     // Automatically set up the camera for this terminal
//!     .insert(AutoCamera);
//! }
//!
//! fn main () {
//!     App::new()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugin(TerminalPlugin)
//!     .add_startup_system(setup)
//!     .run();
//! }
//! ```
//!
//! ## Versions
//! | bevy | bevy_ascii_terminal |
//! | --- | --- |
//! | 0.8.1 | 0.11.1 |
//! | 0.8 | 0.11 |
//! | 0.7 | 0.9-0.10 |
mod border;
mod entity;
mod formatting;
mod renderer;
mod terminal;
mod to_world;

pub mod ui;

use bevy::prelude::{App, Plugin};
#[cfg(feature = "camera")]
pub use renderer::{AutoCamera, TiledCamera, TiledCameraBundle};

pub use renderer::{code_page_437, TerminalFont, TerminalMaterial};

pub use to_world::ToWorld;

pub use sark_grids::{grid::Side, GridPoint, Pivot, Size2d};

pub use ui::UiBox;

/// The primary terminal rendering function labels
pub mod term_func_labels {
    /// System label for the terminal me.
    pub use crate::renderer::{
        TERMINAL_CHANGE_FONT, TERMINAL_INIT, TERMINAL_UPDATE_MESH, TERMINAL_UPDATE_SIZE,
        TERMINAL_UPDATE_TILE_DATA,
    };
}

pub use prelude::*;

pub mod prelude {
    #[cfg(feature = "camera")]
    pub use crate::renderer::AutoCamera;
    pub use crate::{
        border::{Border, BorderTitle},
        entity::TerminalBundle,
        formatting::*,
        terminal::{Terminal, Tile},
        ui::UiBox,
        TerminalPlugin,
    };
    pub use sark_grids::{grid::Side, GridPoint, Pivot, Size2d};
}

/// Plugin for terminal rendering and related components and systems.
pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(renderer::TerminalRendererPlugin)
            .add_plugin(to_world::ToWorldPlugin);
    }
}
