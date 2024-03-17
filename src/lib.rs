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
//!     // Create the terminal
//!     let mut terminal = Terminal::new([20,3]).with_border(Border::single_line());
//!     // Draw a blue "Hello world!" to the terminal
//!     terminal.put_string([1, 1], "Hello world!".fg(Color::BLUE));
//!
//!     
//!     commands.spawn((
//!         // Spawn the terminal bundle from our terminal
//!         TerminalBundle::from(terminal),
//!         // Automatically set up the camera to render the terminal
//!         AutoCamera
//!     ));
//! }
//!
//! fn main () {
//!     App::new()
//!     .add_plugins((DefaultPlugins, TerminalPlugin))
//!     .add_systems(Startup, setup)
//!     .run();
//! }
//! ```
//!
//! ## Versions
//! | bevy  | bevy_ascii_terminal |
//! | ----- | ------------------- |
//! | 0.13  | 0.15.0              |
//! | 0.12  | 0.14.0              |
//! | 0.11  | 0.13.0              |
//! | 0.9   | 0.12.1              |
//! | 0.8.1 | 0.11.1-4            |
//! | 0.8   | 0.11                |
//! | 0.7   | 0.9-0.10            |
mod border;
mod entity;
mod formatting;
mod renderer;
mod terminal;
mod to_world;

use bevy::prelude::{App, IntoSystemConfigs, Last, Plugin};
#[cfg(feature = "camera")]
pub use renderer::{AutoCamera, TiledCamera, TiledCameraBundle};

pub use renderer::{code_page_437, TerminalFont, TerminalLayout, TerminalMaterial};

pub use to_world::ToWorld;

pub use sark_grids::{grid::Side, GridPoint, Pivot, Size2d};

/// The primary terminal rendering function labels
pub use crate::renderer::{
    TerminalChangeFont, TerminalInit, TerminalLayoutChange, TerminalLayoutUpdate,
    TerminalMaterialChange, TerminalRender, TerminalUpdateTiles,
};

pub use prelude::*;

pub mod prelude {
    #[cfg(feature = "camera")]
    pub use crate::renderer::{AutoCamera, TileScaling};
    pub use crate::{
        border::{AlignedStringFormatter, Border, Edge},
        entity::ClearAfterRender,
        entity::TerminalBundle,
        formatting::*,
        terminal::{Terminal, Tile},
        TerminalPlugin,
    };
    pub use sark_grids::{grid::Side, GridPoint, Pivot, Size2d};
}

/// Plugin for terminal rendering and related components and systems.
pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((renderer::TerminalRendererPlugin, to_world::ToWorldPlugin))
            .add_systems(Last, entity::clear_after_render.after(TerminalRender));
    }
}
