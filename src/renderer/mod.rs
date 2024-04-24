use bevy::{app::Plugin, asset::Handle, ecs::bundle::Bundle, sprite::MaterialMesh2dBundle};

mod camera;
mod font;
mod material;
mod mesh;
mod mesher;
mod uv_mapping;

use self::{
    camera::TerminalCameraPlugin, font::TerminalFontPlugin, material::TerminalMaterialPlugin,
    mesh::TerminalMeshPlugin, uv_mapping::TerminalUvMappingPlugin,
};
pub use self::{
    camera::{TerminalCamera, TerminalCameraBundle, UpdateTerminalViewportEvent},
    font::TerminalFont,
    material::TerminalMaterial,
    mesh::{RebuildTerminalMeshVerts, TerminalFontScaling, TerminalMeshPivot, TerminalMeshSystems},
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

#[derive(Bundle, Default)]
pub struct TerminalRenderBundle {
    pub mesh_pivot: TerminalMeshPivot,
    pub font: TerminalFont,
    pub scaling: TerminalFontScaling,
    pub mapping: Handle<UvMapping>,
    pub mesh_bundle: MaterialMesh2dBundle<TerminalMaterial>,
}
