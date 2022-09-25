//! The material used for terminal rendering.
//!
//! By default the terminal expects a [code page 437](https://dwarffortresswiki.org/Tileset_repository)
//! texture with 16x16 characters. New font textures can be added to the assets directory and loaded via
//! the bevy `AssetLoader`.

use bevy::{
    math::Vec4,
    prelude::{default, Assets, Color, Handle, HandleUntyped, Image, Mesh, Plugin, Shader},
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_asset::RenderAssets,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType, RenderPipelineDescriptor, ShaderRef, ShaderType,
            SpecializedMeshPipelineError,
        },
    },
    sprite::{Material2d, Material2dKey, Material2dPlugin},
};

use crate::TerminalFont;

use super::{
    font::TerminalFontPlugin,
    //mesh::{ATTRIBUTE_COLOR_BG, ATTRIBUTE_COLOR_FG, ATTRIBUTE_UV},
    BuiltInFontHandles, mesh_data::{ATTRIBUTE_UV, ATTRIBUTE_COLOR_BG, ATTRIBUTE_COLOR_FG},
};

/// The default shader handle used by terminals.
pub const TERMINAL_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3142086811234592509);

pub struct TerminalMaterialPlugin;

impl Plugin for TerminalMaterialPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(TerminalFontPlugin);

        app.add_plugin(Material2dPlugin::<TerminalMaterial>::default());

        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().expect(
            "Error initializing TerminalPlugin. Ensure TerminalPlugin is added AFTER
            DefaultPlugins during app initialization. (issue #1255)",
        );

        shaders.set_untracked(
            TERMINAL_MATERIAL_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("terminal.wgsl")),
        );

        let fonts = app
            .world
            .get_resource::<BuiltInFontHandles>()
            .expect("Couldn't get font handles");
        let font = fonts.get(&TerminalFont::default());
        let material = TerminalMaterial::from(font.clone());

        app.world
            .resource_mut::<Assets<TerminalMaterial>>()
            .set_untracked(Handle::<TerminalMaterial>::default(), material);
    }
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "e228a534-e3ca-2e1e-ab9d-4d8bc1ad8c19"]
#[uniform(0, TerminalMaterialUniform)]
pub struct TerminalMaterial {
    /// This determines the "background color" for the texture,
    /// which will be clipped and replaced with a tile color.
    pub bg_clip_color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

impl Default for TerminalMaterial {
    fn default() -> Self {
        Self {
            bg_clip_color: Color::BLACK,
            texture: None,
        }
    }
}

impl From<Handle<Image>> for TerminalMaterial {
    fn from(image: Handle<Image>) -> Self {
        TerminalMaterial {
            texture: Some(image),
            ..default()
        }
    }
}

// NOTE: These must match the bit flags in shader.wgsl!
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct TerminalMaterialFlags: u32 {
        const TEXTURE           = (1 << 0);
        const NONE              = 0;
        const UNINITIALIZED     = 0xFFFF;
    }
}

/// The GPU representation of the uniform data of a [`TerminalMaterial`].
#[derive(Clone, Default, ShaderType)]
struct TerminalMaterialUniform {
    pub color: Vec4,
    pub flags: u32,
}

impl AsBindGroupShaderType<TerminalMaterialUniform> for TerminalMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<Image>) -> TerminalMaterialUniform {
        let mut flags = TerminalMaterialFlags::NONE;
        if self.texture.is_some() {
            flags |= TerminalMaterialFlags::TEXTURE;
        }

        TerminalMaterialUniform {
            color: self.bg_clip_color.as_linear_rgba_f32().into(),
            flags: flags.bits(),
        }
    }
}

impl Material2d for TerminalMaterial {
    fn fragment_shader() -> ShaderRef {
        TERMINAL_MATERIAL_SHADER_HANDLE.typed().into()
    }

    fn vertex_shader() -> ShaderRef {
        TERMINAL_MATERIAL_SHADER_HANDLE.typed().into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _: Material2dKey<Self>,
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
