use bevy::{
    app::Plugin,
    ecs::{bundle::Bundle, system::Resource},
    math::{UVec2, Vec2},
};

mod camera;
mod font;
mod material;
mod mesh;
mod uv_mapping;

use self::{
    camera::TerminalCameraPlugin, font::TerminalFontPlugin, material::TerminalMaterialPlugin,
    mesh::TerminalMeshPlugin, uv_mapping::UvMappingPlugin,
};
pub use self::{
    camera::TerminalCameraSystems,
    font::{TerminalFont, TerminalFontSystems},
    material::TerminalMaterial,
    mesh::{TerminalMeshRenderer, TerminalMeshSystems},
};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
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
pub struct TerminalRenderSettings {
    pub tile_scaling: TileScaling,
}

pub struct TerminalRendererPlugin;

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<TerminalRenderSettings>();
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

}