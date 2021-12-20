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
//! use bevy_tiled_camera::*;
//!
//! fn setup(mut commands: Commands) {
//!     let size = (20, 3);
//!
//!     let mut term_bundle = TerminalBundle::new().with_size(size);
//!     let terminal = &mut term_bundle.terminal;
//!
//!     terminal.draw_border_single();
//!     terminal.put_string((1, 1), "Hello world!");
//!
//!     commands.spawn_bundle(term_bundle);
//!
//!     commands.spawn_bundle(TiledCameraBundle::new()
//!         .with_tile_count(size));
//! }
//!
//! fn main () {
//!     App::build()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugin(TerminalPlugin)
//!     .add_plugin(TiledCameraPlugin)
//!     .add_startup_system(setup.system())
//!     .run();
//! }
//! ```
pub mod renderer;

mod color;
mod color_blend;
mod colors;
mod terminal;

pub use color::*;
pub use terminal::{BorderGlyphs, Terminal, Tile};

use bevy::prelude::*;
use renderer::{TerminalRendererBundle, TerminalRendererPlugin};

pub use renderer::TerminalAssetLoadState;

/// A bundle with all the required components for a terminal.
///
/// Can specify properties of the terminal on initilaization.
///
/// # Example
/// ```
/// use bevy_ascii_terminal::*;
/// use bevy::prelude::*;
///
/// fn setup(mut commands: Commands) {
///     commands.spawn_bundle(TerminalBundle::new()
///         .with_size((10,10))
///         .with_font("jt_curses_12x12.png"));
///     }
///
/// ```
#[derive(Bundle, Default)]
pub struct TerminalBundle {
    pub terminal: Terminal,
    pub transform: Transform,
    pub global_transform: GlobalTransform,

    #[bundle]
    pub renderer: TerminalRendererBundle,
}

impl TerminalBundle {
    pub fn new() -> Self {
        TerminalBundle::default()
    }

    /// Set the initial size of the terminal.
    pub fn with_size(mut self, size: (u32, u32)) -> Self {
        self.terminal.resize(size);
        self
    }

    /// Set the initial font for the terminal. This should be the file name of the font
    /// in the `assets/textures` directory, including the extension.
    ///
    /// # Example
    ///
    /// ```
    /// use bevy_ascii_terminal::*;
    ///
    /// let bundle = TerminalBundle::new()
    ///     .with_font("zx_evolution_8x8.png");
    /// ```
    pub fn with_font(mut self, font_name: &str) -> Self {
        self.renderer.font.change_font(font_name);
        self
    }
}

/// Plugin for terminal rendering and related components and systems.
pub struct TerminalPlugin;
impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(TerminalRendererPlugin);
    }
}
