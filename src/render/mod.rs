mod camera;
mod font;
mod material;
mod mesh;
mod uv_mapping;

use bevy::prelude::Resource;
pub use camera::TerminalCamera;
pub use font::TerminalFont;
pub use material::TerminalMaterial;
pub use mesh::{RebuildMeshVerts, TerminalMeshPivot, TerminalMeshTileScaling};
pub use uv_mapping::{UvMapping, UvMappingHandle};

pub(crate) use camera::TerminalCameraPlugin;
pub(crate) use font::TerminalFontPlugin;
pub(crate) use material::TerminalMaterialPlugin;
pub(crate) use mesh::TerminalMeshPlugin;
pub(crate) use uv_mapping::TerminalUvMappingPlugin;

pub use camera::{
    TerminalSystemsCacheCameraData, TerminalSystemsUpdateCamera, UpdateTerminalViewportEvent,
};
pub use font::TerminalSystemsUpdateFont;
pub use mesh::TerminalSystemsUpdateMesh;

/// A global resource to configure how terminal mesh tiles are scaled in world
/// space.
///
/// Mesh scaling can be further customized with the [TerminalMeshTileScaling] component.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Resource)]
pub enum TerminalMeshWorldScaling {
    /// Each terminal tile will be 1 world unit in height. The width will be
    /// set automatically based on the font's aspect ratio.
    ///
    /// This is the expected default when using the [TerminalCamera].
    #[default]
    World,
    /// Every terminal tile will be scaled so each pixel is one world unit in
    /// size. This means the terminal's world size will change when the font
    /// changes.
    ///
    /// This is the expected default when using bevy's default [Camera2d].
    Pixels,
}
