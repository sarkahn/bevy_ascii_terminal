use bevy::{
    app::Plugin, asset::Handle, ecs::bundle::Bundle, math::IVec2, sprite::MaterialMesh2dBundle,
};

mod camera;
mod font;
mod material;
mod mesh;
mod mesher;
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
    mesh::{TerminalFontScaling, TerminalMeshPivot, TerminalRenderSystems},
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

#[derive(Bundle, Default)]
pub struct TerminalRenderBundle {
    pub mesh_pivot: TerminalMeshPivot,
    pub font: TerminalFont,
    pub scaling: TerminalFontScaling,
    pub mapping: Handle<UvMapping>,
    pub mesh_bundle: MaterialMesh2dBundle<TerminalMaterial>,
}

// impl TerminalRenderBundle {
//     pub fn new() -> Self {
//         Self {
//             mesh_pivot: TerminalMeshPivot::default(),
//             font: Default::default(),
//             scaling: Default::default(),
//             mapping: Default::default(),
//             mesh_bundle: Default::default(),
//         }
//     }
// }
