mod border;
mod glyph;
mod grid;
pub mod renderer;
mod string;
mod terminal;
mod tile;
mod transform;

use bevy::{
    app::Plugin,
    ecs::{bundle::Bundle, system::Resource},
    math::{IVec2, UVec2, Vec2},
};
use border::Border;
pub use grid::{direction, GridPoint, GridRect, Pivot, PivotedPoint};
pub use renderer::TerminalFont;
use renderer::TerminalRenderBundle;
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
        app.insert_resource(TerminalGrid {
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
            render_bundle: TerminalRenderBundle::new(Pivot::Center, size),
        }
    }

    pub fn with_builtin_font(mut self, font: TerminalFont) -> Self {
        self.render_bundle.font = font;
        self
    }

    pub fn with_string<'a>(
        mut self,
        xy: impl GridPoint,
        string: impl Into<FormattedString<'a>>,
    ) -> Self {
        let string = string.into();
        self.terminal.put_string(xy, string);
        self
    }

    pub fn with_border(mut self, border: Border) -> Self {
        self.terminal.set_border(Some(border));
        self
    }

    pub fn with_border_title<'a>(
        mut self,
        border: Border,
        title: impl Into<FormattedString<'a>>,
    ) -> Self {
        let title = title.into();
        self.terminal.put_border(border).put_title(title);
        self
    }

    pub fn with_mesh_pivot(mut self, pivot: Pivot) -> Self {
        self.render_bundle.renderer.mesh_pivot = pivot;
        self
    }

    pub fn with_grid_position(mut self, grid_pos: impl GridPoint) -> Self {
        self.terminal_transform.grid_pos = grid_pos.as_ivec2();
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
    pub(crate) fn tile_size_world(&self, font_image_size: UVec2) -> Vec2 {
        match self {
            TileScaling::World => {
                let aspect = font_image_size.x as f32 / font_image_size.y as f32;
                Vec2::new(1.0 / aspect, 1.0)
            }
            TileScaling::Pixels => (font_image_size / 16).as_vec2(),
        }
    }
}

#[derive(Default, Resource)]
pub struct TerminalGrid {
    tile_scaling: TileScaling,
    /// The size of a single grid tile in world space
    world_tile_size: Vec2,
}

impl TerminalGrid {
    pub fn new(tile_scaling: TileScaling, pixels_per_tile: impl GridPoint) -> Self {
        Self {
            tile_scaling,
            world_tile_size: tile_scaling.tile_size_world(pixels_per_tile.as_uvec2()),
        }
    }
    // pub fn world_to_tile(&self, world_pos: Vec2) -> IVec2 {
    //     (world_pos / self.tile_size).floor().as_ivec2()
    // }
}
