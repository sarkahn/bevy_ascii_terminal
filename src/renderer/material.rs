//! The material used for terminal rendering.
//!
//! By default the terminal expects a [code page 437](https://dwarffortresswiki.org/Tileset_repository)
//! texture with 16x16 characters. New font textures can be added to the assets directory and loaded via
//! the bevy `AssetLoader`.

use bevy::{reflect::TypeUuid, prelude::{Plugin, Handle, Image, Color, default, HandleUntyped, Shader, AssetServer, Mesh, Assets}, render::{render_resource::{AsBindGroup, ShaderType, AsBindGroupShaderType, ShaderRef, RenderPipelineDescriptor, SpecializedMeshPipelineError}, render_asset::RenderAssets, mesh::{MeshVertexBufferLayout, self}}, math::Vec4, sprite::{ColorMaterialFlags, Material2d, Material2dKey, MaterialMesh2dBundle, Material2dPlugin}};

use crate::TerminalFont;

use super::{plugin::{ATTRIBUTE_UV, ATTRIBUTE_COLOR_BG, ATTRIBUTE_COLOR_FG}, font::TerminalFontPlugin, BuiltInFontHandles};

/// The default shader handle used by terminals.
pub const TERMINAL_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3142086872234592509);

    
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
            .set_untracked(
                Handle::<TerminalMaterial>::default(),
                material
            );
    }
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "e228a534-e3ca-2e1e-ab9d-4d8bc1ad8c19"]
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
        Self { bg_clip_color: Color::BLACK, texture: None }
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
struct TerminalMaterialUniformData {
    pub color: Vec4,
    pub flags: u32,
}

impl AsBindGroupShaderType<TerminalMaterialUniformData> for TerminalMaterial {
    fn as_bind_group_shader_type(&self, _: &RenderAssets<Image>) -> TerminalMaterialUniformData {
        let mut flags = TerminalMaterialFlags::NONE;
        if self.texture.is_some() {
            flags |= TerminalMaterialFlags::TEXTURE;
        }

        TerminalMaterialUniformData { 
            color: self.bg_clip_color.as_linear_rgba_f32().into(), 
            flags: flags.bits() 
        }
    }
}

impl Material2d for TerminalMaterial {
    fn fragment_shader() -> ShaderRef {
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


// use bevy::app::{App, Plugin};
// use bevy::asset::{AssetServer, Assets, Handle, HandleUntyped};
// use bevy::ecs::system::{lifetimeless::SRes, SystemParamItem};
// use bevy::math::Vec4;
// use bevy::prelude::Mesh;
// use bevy::reflect::TypeUuid;
// use bevy::render::mesh::MeshVertexBufferLayout;
// use bevy::render::{
//     color::Color,
//     prelude::Shader,
//     render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
//     render_resource::*,
//     renderer::RenderDevice,
//     texture::Image,
// };

// use encase;

// use bevy::sprite::{Material2dPipeline, Material2dPlugin};

// use crate::TerminalFont;

// use super::font::TerminalFontPlugin;
// use super::plugin::{ATTRIBUTE_COLOR_BG, ATTRIBUTE_COLOR_FG, ATTRIBUTE_UV};
// use super::BuiltInFontHandles;

// /// The default shader handle used by the terminal.
// pub const TERMINAL_MATERIAL_SHADER_HANDLE: HandleUntyped =
//     HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3142086872234592509);

// /// The default material handle used by the terminal.
// pub const TERMINAL_DEFAULT_MATERIAL_HANDLE: HandleUntyped =
//     HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2121056571224552501);

// /// Plugin for the terminal renderer. Initializes resources and systems related to rendering.
// #[derive(Default)]
// pub struct TerminalMaterialPlugin;

// impl Plugin for TerminalMaterialPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugin(TerminalFontPlugin);
//         app.add_plugin(Material2dPlugin::<TerminalMaterial>::default());

//         let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().expect(
//             "Error initializing TerminalPlugin. Ensure TerminalPlugin is added AFTER
//             DefaultPlugins during app initialization. (issue #1255)",
//         );

//         shaders.set_untracked(
//             TERMINAL_MATERIAL_SHADER_HANDLE,
//             Shader::from_wgsl(include_str!("terminal.wgsl")),
//         );

//         let fonts = app
//             .world
//             .get_resource::<BuiltInFontHandles>()
//             .expect("Couldn't get font handles");
//         let font = fonts.get(&TerminalFont::default());
//         let material = TerminalMaterial::from(font.clone());

//         let mut materials = app
//             .world
//             .get_resource_mut::<Assets<TerminalMaterial>>()
//             .unwrap();
//         materials.set_untracked(Handle::<TerminalMaterial>::default(), material);
//     }
// }

// /// The material for rendering a terminal.
// #[derive(Debug, Clone, TypeUuid)]
// #[uuid = "e228a534-e3ca-2e1e-ab9d-4d8bc1ad8c19"]
// pub struct TerminalMaterial {
//     /// The clip color for the active font texture.
//     ///
//     /// Clip color determines which part of the texture is regarded as
//     /// "background color".
//     pub clip_color: Color,

//     /// The font texture rendered by the terminal.
//     pub texture: Option<Handle<Image>>,
// }

// impl Default for TerminalMaterial {
//     fn default() -> Self {
//         TerminalMaterial {
//             clip_color: Color::BLACK,
//             texture: None,
//         }
//     }
// }

// impl From<Handle<Image>> for TerminalMaterial {
//     fn from(texture: Handle<Image>) -> Self {
//         TerminalMaterial {
//             texture: Some(texture),
//             clip_color: Color::BLACK,
//         }
//     }
// }

// // NOTE: These must match the bit flags in shader.wgsl!
// bitflags::bitflags! {
//     #[repr(transparent)]
//     pub struct TerminalMaterialFlags: u32 {
//         const TEXTURE           = (1 << 0);
//         const NONE              = 0;
//         const UNINITIALIZED     = 0xFFFF;
//     }
// }

// /// The GPU representation of the uniform data of a [`TerminalMaterial`].
// #[derive(Clone, Default, ShaderType)]
// struct TerminalMaterialUniformData {
//     pub color: Vec4,
//     pub flags: u32,
// }

// // The data from our material that gets copied to the gpu
// #[derive(Debug, Clone)]
// pub struct GpuTerminalMaterial {
//     pub buffer: Buffer,
//     pub bind_group: BindGroup,
//     pub flags: TerminalMaterialFlags,
//     pub texture: Option<Handle<Image>>,
// }

// // Boilerplate copied from `ColorMaterial`. Allows us to reference
// // our texture and mesh/view structs from the shader.
// impl RenderAsset for TerminalMaterial {
//     type ExtractedAsset = TerminalMaterial;
//     type PreparedAsset = GpuTerminalMaterial;
//     type Param = (
//         SRes<RenderDevice>,
//         SRes<Material2dPipeline<TerminalMaterial>>,
//         SRes<RenderAssets<Image>>,
//     );

//     fn extract_asset(&self) -> Self::ExtractedAsset {
//         self.clone()
//     }

//     fn prepare_asset(
//         material: Self::ExtractedAsset,
//         (render_device, pipeline, gpu_images): &mut SystemParamItem<Self::Param>,
//     ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
//         let (texture_view, sampler) = if let Some(result) = pipeline
//             .mesh2d_pipeline
//             .get_image_texture(gpu_images, &material.texture)
//         {
//             result
//         } else {
//             return Err(PrepareAssetError::RetryNextUpdate(material));
//         };

//         let mut flags = TerminalMaterialFlags::NONE;
//         if material.texture.is_some() {
//             flags |= TerminalMaterialFlags::TEXTURE;
//         }

//         let value = TerminalMaterialUniformData {
//             color: material.clip_color.as_linear_rgba_f32().into(),
//             flags: flags.bits(),
//         };
//         let mut buffer = encase::UniformBuffer::new(Vec::new());
//         buffer.write(&value).unwrap();

//         let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
//             contents: buffer.as_ref(),
//             label: None,
//             usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
//         });
//         let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
//             entries: &[
//                 BindGroupEntry {
//                     binding: 0,
//                     resource: buffer.as_entire_binding(),
//                 },
//                 BindGroupEntry {
//                     binding: 1,
//                     resource: BindingResource::TextureView(texture_view),
//                 },
//                 BindGroupEntry {
//                     binding: 2,
//                     resource: BindingResource::Sampler(sampler),
//                 },
//             ],
//             label: Some("terminal_material_bind_group"),
//             layout: &pipeline.material2d_layout,
//         });

//         Ok(GpuTerminalMaterial {
//             buffer,
//             bind_group,
//             flags,
//             texture: material.texture,
//         })
//     }
// }

// impl Material2d for TerminalMaterial {
//     fn fragment_shader(_asset_server: &AssetServer) -> Option<Handle<Shader>> {
//         Some(TERMINAL_MATERIAL_SHADER_HANDLE.typed())
//     }

//     fn vertex_shader(_asset_server: &AssetServer) -> Option<Handle<Shader>> {
//         Some(TERMINAL_MATERIAL_SHADER_HANDLE.typed())
//     }

//     #[inline]
//     fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
//         &render_asset.bind_group
//     }

//     fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
//         render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
//             entries: &[
//                 BindGroupLayoutEntry {
//                     binding: 0,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Buffer {
//                         ty: BufferBindingType::Uniform,
//                         has_dynamic_offset: false,
//                         min_binding_size: Some(TerminalMaterialUniformData::min_size()),
//                     },
//                     count: None,
//                 },
//                 // Texture
//                 BindGroupLayoutEntry {
//                     binding: 1,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Texture {
//                         multisampled: false,
//                         sample_type: TextureSampleType::Float { filterable: true },
//                         view_dimension: TextureViewDimension::D2,
//                     },
//                     count: None,
//                 },
//                 // Texture Sampler
//                 BindGroupLayoutEntry {
//                     binding: 2,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Sampler(SamplerBindingType::Filtering),
//                     count: None,
//                 },
//             ],
//             label: Some("color_material_layout"),
//         })
//     }

//     type Key = ();

//     fn key(
//         _render_devicec: &RenderDevice,
//         _material: &<Self as RenderAsset>::PreparedAsset,
//     ) -> Self::Key {
//     }

//     fn specialize(
//         _key: Self::Key,
//         descriptor: &mut RenderPipelineDescriptor,
//         _layout: &MeshVertexBufferLayout,
//     ) -> Result<(), SpecializedMeshPipelineError> {
//         let formats = vec![
//             Mesh::ATTRIBUTE_POSITION.format,
//             ATTRIBUTE_UV.format,
//             ATTRIBUTE_COLOR_BG.format,
//             ATTRIBUTE_COLOR_FG.format,
//         ];

//         let vertex_layout =
//             VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);
//         descriptor.vertex.buffers = vec![vertex_layout];

//         Ok(())
//     }
// }
