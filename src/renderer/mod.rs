use bevy::{
    app::Plugin, asset::Handle, ecs::bundle::Bundle, math::IVec2, sprite::MaterialMesh2dBundle,
};

mod camera;
mod font;
mod material;
mod mesh;
mod mesher;
mod uv_mapping;

pub use self::{
    camera::TerminalCameraBundle,
    font::{TerminalFont, TerminalFontSystems},
    material::TerminalMaterial,
    mesh::{TerminalFontScaling, TerminalMeshPivot, TerminalMeshSystems},
    uv_mapping::UvMapping,
};
use self::{
    camera::TerminalCameraPlugin, font::TerminalFontPlugin, material::TerminalMaterialPlugin,
    mesh::TerminalMeshPlugin, uv_mapping::TerminalUvMappingPlugin,
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
