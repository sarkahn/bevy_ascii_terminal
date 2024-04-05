mod border;
mod border_entity;
mod camera;
mod glyph;
mod grid;
pub mod renderer;
mod string;
mod terminal;
mod tile;
mod transform;

use std::ops::Mul;

use bevy::{
    app::Plugin,
    ecs::{bundle::Bundle, system::Resource},
    math::{IVec2, UVec2, Vec2},
};
use border::Border;
pub use grid::{direction, GridPoint, GridRect, Pivot, PivotedPoint};
pub use renderer::TerminalFont;
use renderer::{TerminalFontScaling, TerminalRenderBundle};
pub use string::{FormattedString, StringFormatter};
pub use terminal::Terminal;
pub use tile::Tile;
pub use transform::TerminalTransform;

#[derive(Default)]
pub struct TerminalPlugin {
    tile_scaling: TileScaling,
    //pixels_per_tile: Option<UVec2>,
}

impl TerminalPlugin {
    pub fn with_tile_scaling(mut self, tile_scaling: TileScaling) -> Self {
        self.tile_scaling = tile_scaling;
        self
    }
}

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TerminalGridSettings {
            tile_scaling: self.tile_scaling,
            ..Default::default() //tile_size: self.tile_scaling.tile_size_world(self.pixels_per_tile),
        });
        app.add_plugins(renderer::TerminalRendererPlugin);
    }
}

#[derive(Bundle)]
pub struct TerminalBundle {
    terminal: Terminal,
    terminal_transform: TerminalTransform,
    render_bundle: TerminalRenderBundle,
}

impl TerminalBundle {
    pub fn new(size: impl GridPoint) -> Self {
        Self {
            terminal: Terminal::new(size),
            terminal_transform: TerminalTransform::new(size),
            render_bundle: Default::default(),
        }
    }

    /// Set the [TerminalFont] for the terminal.
    pub fn with_font(mut self, font: TerminalFont) -> Self {
        self.render_bundle.font = font;
        self
    }

    /// Write a [FormattedString] to the terminal.
    pub fn with_string<'a>(
        mut self,
        xy: impl GridPoint,
        string: impl Into<FormattedString<'a>>,
    ) -> Self {
        let string = string.into();
        self.terminal.put_string(xy, string);
        self
    }

    /// Set a border for the terminal.
    pub fn with_border(mut self, border: Border) -> Self {
        self.terminal.set_border(Some(border));
        self
    }

    /// Set a border with a title for the terminal.
    pub fn with_border_title<'a>(
        mut self,
        border: Border,
        title: impl Into<FormattedString<'a>>,
    ) -> Self {
        let title = title.into();
        self.terminal.put_border(border).put_title(title);
        self
    }

    /// Set the mesh pivot for the terminal.
    ///
    /// Note this only affects how the terminal is rendered in world space.
    pub fn with_mesh_pivot(mut self, pivot: Pivot) -> Self {
        self.render_bundle.mesh_pivot = pivot.into();
        self
    }

    /// Set the terminal font scaling. Each tile of the terminal will be scaled
    /// by this amount.
    pub fn with_font_scaling(mut self, scaling: Vec2) -> Self {
        self.render_bundle.scaling.0 = scaling;
        self
    }

    /// Set the initial grid position for the terminal. The final world position
    /// of the terminal will be based on [TerminalGridSettings].
    pub fn with_grid_position(mut self, grid_pos: impl GridPoint) -> Self {
        self.terminal_transform.grid_position = grid_pos.as_ivec2();
        self
    }
}

impl From<Terminal> for TerminalBundle {
    fn from(terminal: Terminal) -> Self {
        let size = terminal.size();
        Self {
            terminal,
            ..Self::new(size)
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum TileScaling {
    Pixels,
    #[default]
    World,
}

impl TileScaling {
    /// Calculate the size of a single tile in world space from a font image size
    /// based on the tile scaling.
    pub(crate) fn calculate_world_tile_size(
        &self,
        font_image_size: UVec2,
        font_scaling: Option<Vec2>,
    ) -> Vec2 {
        let scaling = font_scaling.unwrap_or(Vec2::ONE);
        match self {
            TileScaling::World => {
                let aspect = font_image_size.x as f32 / font_image_size.y as f32;
                Vec2::new(1.0 / aspect, 1.0)
            }
            TileScaling::Pixels => (font_image_size / 16).as_vec2(),
        }
        .mul(scaling)
    }
}

/// Global settings for how terminals are positioned in world space.
///
/// A terminal's grid position can be set via it's [TerminalTransform] component.
#[derive(Default, Resource)]
pub struct TerminalGridSettings {
    tile_scaling: TileScaling,
    world_grid_pixels_per_tile: Vec2,
}

impl TerminalGridSettings {
    pub fn new(tile_scaling: TileScaling, pixels_per_tile: impl GridPoint) -> Self {
        Self {
            tile_scaling,
            world_grid_pixels_per_tile: pixels_per_tile.as_vec2(),
        }
    }

    /// The size of a world grid tile, based on the global [TerminalGridSettings].
    ///
    /// This value determines how terminals are positioned in world space using
    /// their [TerminalTransform] component.
    pub fn world_grid_tile_size(&self) -> Vec2 {
        let ppu = self.world_grid_pixels_per_tile;
        match self.tile_scaling {
            TileScaling::Pixels => ppu,
            TileScaling::World => {
                let aspect = ppu.x / ppu.y;
                Vec2::new(1.0 / aspect, 1.0)
            }
        }
    }
}
