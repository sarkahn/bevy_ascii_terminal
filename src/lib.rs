//! A simple ascii terminal integrated into bevy's ecs framework.
//!
//! The goal of this crate is to provide a simple, straightforward, and hopefully
//! fast method for rendering colorful ascii in bevy. It was made with "traditional
//! roguelikes" in mind, but should serve as a simple UI tool if needed.
//!
//! # Rendering
//! In order to render the terminal you must add the [TerminalPlugin] via your bevy `App`.
//! You then need a camera to display it. Though not a direct dependency, this crate
//! uses [TiledCamera](https://crates.io/crates/bevy_tiled_camera) to render it's examples.
//!
//! It's recommended to use this or [some other similar camera](https://crates.io/crates/bevy_pixel_camera)
//! for rendering, as bevy's default orthographic camera is not a good fit for how the
//! terminal is displayed.
//!
//! # Example
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_ascii_terminal::*;
//! use bevy_ascii_terminal::ui::BorderGlyphs;
//! use bevy_tiled_camera::*;
//!
//! fn setup(mut commands: Commands) {
//!     let size = [20, 3];
//!
//!     let mut term_bundle = TerminalBundle::new().with_size(size);
//!     let terminal = &mut term_bundle.terminal;
//!
//!     terminal.draw_border(BorderGlyphs::single_line());
//!     terminal.put_string([1, 1], "Hello world!");
//!
//!     commands.spawn_bundle(term_bundle);
//!
//!     commands.spawn_bundle(TiledCameraBundle::new()
//!         .with_tile_count(size));
//! }
//!
//! fn main () {
//!     App::new()
//!     .add_plugin(TiledCameraPlugin)
//!     .add_plugins(DefaultPlugins)
//!     .add_plugin(TerminalPlugin)
//!     .add_startup_system(setup)
//!     .run();
//! }
//! ```
mod terminal;

pub mod formatting;
pub mod renderer;
pub mod to_world;
pub mod ui;

pub use terminal::{Terminal, Tile};

pub use renderer::code_page_437;
pub use renderer::material::TerminalMaterial;
pub use renderer::TerminalFont;

pub use to_world::ToWorld;

#[cfg(feature = "camera")]
pub use renderer::camera::AutoCamera;
#[cfg(feature = "camera")]
pub use renderer::camera::TiledCamera;
#[cfg(feature = "camera")]
pub use renderer::camera::TiledCameraBundle;

pub use sark_grids::grid::Side;
pub use sark_grids::Grid;
pub use sark_grids::GridPoint;
pub use sark_grids::Pivot;
pub use sark_grids::Size2d;

use to_world::ToWorldPlugin;
pub use ui::BorderGlyphs;
pub use ui::UiBox;

pub use formatting::*;

use bevy::prelude::*;

/// A bundle with all the required components for a terminal.
///
/// Can specify some properties of the terminal on initilaization.
#[derive(Bundle, Default)]
pub struct TerminalBundle {
    pub terminal: Terminal,
    pub transform: Transform,
    pub global_transform: GlobalTransform,

    #[bundle]
    pub renderer: renderer::TerminalRendererBundle,
}

impl TerminalBundle {
    pub fn new() -> Self {
        TerminalBundle::default()
    }

    /// Set the initial size of the terminal.
    pub fn with_size(mut self, size: impl Size2d) -> Self {
        self.terminal.resize(size.as_array());
        self
    }

    pub fn with_font(mut self, font: TerminalFont) -> Self {
        self.renderer.font = font;
        self
    }

    pub fn with_position(mut self, pos: impl GridPoint) -> Self {
        let p = self.transform.translation;
        self.transform.translation = pos.as_vec2().extend(p.z);
        self
    }

    /// Sets the intial z position for the terminal.
    pub fn with_depth(mut self, depth: i32) -> Self {
        self.transform.translation.z = depth as f32;
        self
    }
}

impl From<Terminal> for TerminalBundle {
    fn from(terminal: Terminal) -> Self {
        TerminalBundle {
            terminal,
            ..default()
        }
    }
}

/// Plugin for terminal rendering and related components and systems.
pub struct TerminalPlugin;
impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(renderer::TerminalRendererPlugin)
            .add_plugin(ToWorldPlugin);
    }
}
