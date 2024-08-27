//! Components and systems related to terminal rendering.

use bevy::app::Plugin;

pub mod camera;
pub mod font;
pub mod material;
pub mod mesh;
pub mod mesher;
pub mod uv_mapping;

use self::{
    camera::TerminalCameraPlugin, font::TerminalFontPlugin, material::TerminalMaterialPlugin,
    mesh::TerminalMeshPlugin, uv_mapping::TerminalUvMappingPlugin,
};
pub use self::{
    camera::{TerminalCamera, TerminalCameraBundle, UpdateTerminalViewportEvent},
    font::TerminalFont,
    material::TerminalMaterial,
    mesh::{TerminalFontScaling, TerminalMeshPivot, TerminalSystemMeshRebuild},
    uv_mapping::UvMapping,
};

pub struct TerminalRendererPlugin;

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            TerminalUvMappingPlugin,
            TerminalMaterialPlugin,
            TerminalMeshPlugin,
            TerminalFontPlugin,
            TerminalCameraPlugin,
        ));
    }
}
