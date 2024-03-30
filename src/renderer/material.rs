use bevy::{
    prelude::{Asset, Assets, Color, Handle, Image, Mesh, Plugin, Shader},
    reflect::TypePath,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
};

use super::mesh::{ATTRIBUTE_COLOR_BG, ATTRIBUTE_COLOR_FG, ATTRIBUTE_UV};

pub const TERMINAL_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(13814445327411044821);

const TERMINAL_SHADER_STRING: &str = include_str!("terminal.wgsl");

pub struct TerminalMaterialPlugin;

impl Plugin for TerminalMaterialPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(Material2dPlugin::<TerminalMaterial>::default());
        let mut shaders = app.world.resource_mut::<Assets<Shader>>();
        shaders.insert(
            TERMINAL_SHADER_HANDLE,
            Shader::from_wgsl(
                TERMINAL_SHADER_STRING,
                "bevy_ascii_terminal::default_shader",
            ),
        );
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, PartialEq, Clone)]
pub struct TerminalMaterial {
    #[uniform(0)]
    pub clip_color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

impl Material2d for TerminalMaterial {
    fn vertex_shader() -> ShaderRef {
        TERMINAL_SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        TERMINAL_SHADER_HANDLE.into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_UV.at_shader_location(1),
            ATTRIBUTE_COLOR_BG.at_shader_location(2),
            ATTRIBUTE_COLOR_FG.at_shader_location(3),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];

        Ok(())
    }
}

impl Default for TerminalMaterial {
    fn default() -> Self {
        Self {
            clip_color: Color::BLACK,
            texture: None,
        }
    }
}
