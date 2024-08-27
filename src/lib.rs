mod ascii;
pub mod border;
mod grid;
pub mod renderer;
mod string;
mod terminal;
mod tile;
mod transform;

use std::ops::Mul;

pub use ascii::Glyph;
use bevy::{
    app::{Last, Plugin, PostUpdate},
    asset::Handle,
    ecs::{bundle::Bundle, system::Resource},
    math::Vec2,
    prelude::IntoSystemSetConfigs,
    sprite::MaterialMesh2dBundle,
};
pub use border::TerminalBorder;
pub use grid::{direction, GridPoint, GridRect, GridSize, Pivot, PivotedPoint};
use renderer::{
    camera::{TerminalSystemCameraCacheData, TerminalSystemCameraViewportUpdate},
    font::TerminalSystemFontUpdate,
    TerminalSystemMeshRebuild,
};
pub use renderer::{TerminalCameraBundle, TerminalFont};
pub use string::{DecoratedFormattedText, StringDecorator};
pub use terminal::Terminal;
pub use tile::{ColorWriter, FormattedTile, Tile, TileFormatter};
pub use transform::TerminalTransform;
use transform::{
    TerminalSystemTransformCacheData, //, TerminalSystemTransformPositionUpdate
};

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TerminalGridSettings {
            tile_scaling: self.tile_scaling,
        });
        app.add_plugins((
            transform::TerminalTransformPlugin,
            renderer::TerminalRendererPlugin,
        ));
        app.configure_sets(
            PostUpdate,
            (
                //TerminalSystemTransformPositionUpdate,
                TerminalSystemFontUpdate,
                TerminalSystemTransformCacheData,
                TerminalSystemMeshRebuild,
                TerminalSystemCameraCacheData,
            )
                .chain(),
        );
    }
}

// TODO: Change to TerminalPluginS Impl plugin group for universal grid settings
#[derive(Default)]
pub struct TerminalPlugin {
    tile_scaling: TileScaling,
}

impl TerminalPlugin {
    pub fn with_tile_scaling(mut self, tile_scaling: TileScaling) -> Self {
        self.tile_scaling = tile_scaling;
        self
    }
}

/// The set of components for a default terminal. Contains a variety of builder
/// functions to help with initial terminal setup.
#[derive(Bundle, Default)]
pub struct TerminalBundle {
    pub terminal: Terminal,
    pub terminal_transform: TerminalTransform,
    pub mesh_pivot: renderer::mesh::TerminalMeshPivot,
    pub font: renderer::TerminalFont,
    pub scaling: renderer::TerminalFontScaling,
    pub mapping: Handle<renderer::uv_mapping::UvMapping>,
    pub mesh_bundle: MaterialMesh2dBundle<renderer::material::TerminalMaterial>,
}

impl TerminalBundle {
    pub fn new(size: impl GridSize) -> Self {
        let size = size.as_uvec2();
        Self {
            terminal: Terminal::new(size),
            terminal_transform: transform::TerminalTransform::new(size),
            ..Default::default()
        }
    }

    /// Set the [TerminalFont] for the terminal.
    pub fn with_font(mut self, font: TerminalFont) -> Self {
        self.font = font;
        self
    }

    /// Write a [FormattedString] to the terminal.
    pub fn put_string<T: AsRef<str>>(
        mut self,
        xy: impl Into<PivotedPoint>,
        string: impl Into<DecoratedFormattedText<T>>,
    ) -> Self {
        self.terminal.put_string(xy, string);
        self
    }

    /// Write a char to the terminal.
    pub fn put_char(mut self, xy: impl GridPoint, ch: char) -> Self {
        self.terminal.put_char(xy, ch);
        self
    }

    pub fn with_clear_tile(mut self, clear_tile: impl Into<Tile>) -> Self {
        self.terminal.set_clear_tile(clear_tile.into());
        self.terminal.clear();
        self
    }

    /// Set a [Border] for the terminal.
    pub fn with_border(mut self, border: TerminalBorder) -> Self {
        self.terminal.put_border(border);
        self
    }

    // /// Set a border with a title for the terminal.
    // pub fn with_border_title<'a>(
    //     mut self,
    //     border: Border,
    //     title: impl StringFormatter<'a>,
    // ) -> Self {
    //     self.terminal.put_string([1, 0], title);
    //     self
    // }

    /// Set the mesh pivot for the terminal.
    ///
    /// Note this only affects how the terminal is rendered in world space. A
    /// separate pivot can be applied directly to positions when writing to the
    /// terminal, see [Terminal::put_char]
    pub fn with_mesh_pivot(mut self, pivot: Pivot) -> Self {
        self.mesh_pivot = pivot.into();
        self
    }

    /// Set the terminal font scaling. Each tile of the terminal will be scaled
    /// by this amount.
    pub fn with_font_scaling(mut self, scaling: Vec2) -> Self {
        self.scaling.0 = scaling;
        self
    }

    /// Set the initial grid position for the terminal. The final grid position
    /// in world space is based on the size of the terminal font as well as the
    /// [TerminalGridSettings] resource.
    pub fn with_grid_position(mut self, grid_pos: impl GridPoint) -> Self {
        self.terminal_transform.grid_position = grid_pos.as_ivec2();
        self
    }

    /// Set the initial z position of the terminal in world space.
    pub fn with_depth(mut self, depth: i32) -> Self {
        let p = self.mesh_bundle.transform.translation;
        self.mesh_bundle.transform.translation = p.with_z(depth as f32);
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

/// Global setting that defines how the tiles of terminal meshes are scaled in
/// world space. This defines how world positions are translated into terminal
/// grid positions and vice versa.
///
/// The [crate::renderer::camera::TerminalCamera] can automatically adjust the
/// viewport to properly render terminals regardless of [TileScaling], and the
/// [TerminalTransform] component can be used to translate world positions to
/// and from terminal grid positions.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub enum TileScaling {
    /// Terminal tiles are scaled such that a single tile takes up one unit of
    /// world space vertically, regardless of texture resolution. This requires
    /// a canonical 'pixels per tile/pixels per unit' to be determined by the
    /// [crate::renderer::TerminalCamera].
    #[default]
    World,
    /// Terminal tiles are scaled such that a single pixel of a tile takes up
    /// one unit in world space. This is how bevy's default orthographic camera
    /// is set up.
    Pixels,
}

impl TileScaling {
    /// Calculate the size of a single tile in world space from a font image size
    /// based on the tile scaling.
    pub(crate) fn calculate_world_tile_size(
        &self,
        ppu: impl GridPoint,
        font_scaling: Option<Vec2>,
    ) -> Vec2 {
        let scaling = font_scaling.unwrap_or(Vec2::ONE);
        match self {
            TileScaling::World => {
                let aspect = ppu.x() as f32 / ppu.y() as f32;
                Vec2::new(aspect, 1.0)
            }
            TileScaling::Pixels => ppu.as_vec2(),
        }
        .mul(scaling)
    }
}

/// Global settings for how terminals are sized and positionsed in world space.
#[derive(Default, Resource)]
pub struct TerminalGridSettings {
    tile_scaling: TileScaling,
}

impl TerminalGridSettings {
    pub fn tile_scaling(&self) -> TileScaling {
        self.tile_scaling
    }
}
