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
pub mod renderer;

pub mod formatting;
mod terminal;
pub mod ui;

pub use terminal::{ColorModifier, Terminal, Tile};

pub use renderer::code_page_437;
pub use renderer::material::BuiltInFontHandles;
pub use renderer::material::TerminalMaterial;

use bevy::prelude::*;

pub use sark_grids::grid::Side;
pub use sark_grids::Grid;
pub use sark_grids::GridPoint;
pub use sark_grids::Size2d;
pub use sark_grids::Pivot;

pub use formatting::*;

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

    pub fn with_font(mut self, font: ChangeTerminalFont) -> Self {
        self.renderer.change_font = font;
        self
    }
}

/// Plugin for terminal rendering and related components and systems.
pub struct TerminalPlugin;
impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(renderer::TerminalRendererPlugin);
    }
}

/// Helper component for changing the terminal's font
#[derive(Debug, Clone, Component)]
pub enum ChangeTerminalFont {
    /// Change to one of the terminal's built in fonts:
    /// - jt_curses_12x12.png
    /// - pastiche_8x8.png
    /// - px437_8x8.png
    /// - taffer_10x10.png
    /// - zx_evolution_8x8.png
    BuiltIn(String),
    /// Change to a custom font texture
    Asset(Handle<Image>),
}

impl Default for ChangeTerminalFont {
    fn default() -> Self {
        ChangeTerminalFont::BuiltIn("px437_8x8.png".to_string())
    }
}