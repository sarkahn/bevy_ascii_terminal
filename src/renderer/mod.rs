use bevy::{app::Plugin, asset::Handle, ecs::bundle::Bundle, sprite::MaterialMesh2dBundle};

mod camera;
mod font;
mod material;
mod mesh;
mod uv_mapping;

use crate::{GridPoint, Pivot};

use self::{
    camera::TerminalCameraPlugin, font::TerminalFontPlugin, material::TerminalMaterialPlugin,
    mesh::TerminalMeshPlugin, uv_mapping::UvMappingPlugin,
};
pub use self::{
    camera::TerminalCameraSystems,
    font::{TerminalFont, TerminalFontSystems},
    material::TerminalMaterial,
    mesh::{TerminalRenderSystems, TerminalRenderer},
    uv_mapping::UvMapping,
};

pub struct TerminalRendererPlugin;

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            UvMappingPlugin,
            TerminalMaterialPlugin,
            TerminalMeshPlugin,
            TerminalFontPlugin,
            TerminalCameraPlugin,
        ));
    }
}

#[derive(Bundle)]
pub struct TerminalRenderBundle {
    pub renderer: TerminalRenderer,
    pub font: TerminalFont,
    pub mapping: Handle<UvMapping>,
    pub mesh_bundle: MaterialMesh2dBundle<TerminalMaterial>,
}

impl TerminalRenderBundle {
    pub fn new(mesh_pivot: Pivot, size: impl GridPoint) -> Self {
        Self {
            renderer: TerminalRenderer::new(mesh_pivot, size),
            font: Default::default(),
            mapping: Default::default(),
            mesh_bundle: Default::default(),
        }
    }
}
