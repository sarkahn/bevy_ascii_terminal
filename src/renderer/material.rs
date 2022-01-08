use bevy::app::{App, Plugin};
use bevy::asset::{AssetServer, Assets, Handle, HandleUntyped, HandleId};
use bevy::ecs::system::{lifetimeless::SRes, SystemParamItem};
use bevy::math::Vec4;
use bevy::reflect::TypeUuid;
use bevy::render::texture::ImageType;
use bevy::render::{
    color::Color,
    prelude::Shader,
    render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
    render_resource::{
        std140::{AsStd140, Std140},
        *,
    },
    renderer::RenderDevice,
    texture::Image,
};

use bevy::sprite::{Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle, SpecializedMaterial2d};

//use super::font::BUILT_IN_FONTS;

pub const TERMINAL_MATERIAL_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3142086872234592509);

pub const TERMINAL_MATERIAL_HANDLE: HandleUntyped = 
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 2121056571224552501);

pub mod built_in_fonts {
    use bevy::{prelude::{HandleUntyped, Image}, reflect::TypeUuid};

    pub const JT_CURSES_12X12_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Image::TYPE_UUID, 2122046373215392208);

    pub const PASTICHE_8X8_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Image::TYPE_UUID, 1112381377212391458);

    pub const PX_437_8X8_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Image::TYPE_UUID, 2113081372514792108);
    
    pub const TAFFER_10X10_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Image::TYPE_UUID, 3102186372218392902);

    pub const ZX_EVOLUTION_8X8_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Image::TYPE_UUID, 1111086172214312409);
    
}


macro_rules! include_font {
    ($font_name:expr) => {
        {
            let bytes = include_bytes!(concat!("builtin/", $font_name));
            Image::from_buffer(bytes, ImageType::Extension("png")).unwrap()
        }
    };
}


#[derive(Default)]
pub struct TerminalMaterialPlugin;

impl Plugin for TerminalMaterialPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            TERMINAL_MATERIAL_SHADER_HANDLE,
            Shader::from_wgsl(include_str!("terminal.wgsl")),
        );
        app.add_plugin(Material2dPlugin::<TerminalMaterial>::default());

        use built_in_fonts::*;

        let mut images = app.world.get_resource_mut::<Assets<Image>>().unwrap();

        images.set_untracked(
            PX_437_8X8_HANDLE,
            include_font!("px437_8x8.png")
        );
        
        images.set_untracked(
            PASTICHE_8X8_HANDLE,
            include_font!("pastiche_8x8.png")
        );
        
        images.set_untracked(
            JT_CURSES_12X12_HANDLE,
            include_font!("jt_curses_12x12.png")
        );
        
        images.set_untracked(
            TAFFER_10X10_HANDLE,
            include_font!("taffer_10x10.png")
        );
        
        images.set_untracked(
            ZX_EVOLUTION_8X8_HANDLE,
            include_font!("zx_evolution_8x8.png")
        );
        
        app.world
            .get_resource_mut::<Assets<TerminalMaterial>>()
            .unwrap()
            .set_untracked(
                Handle::<TerminalMaterial>::default(),
                built_in_fonts::JT_CURSES_12X12_HANDLE.typed().into()
            );
        
    }
}


#[derive(Debug, Clone, TypeUuid)]
#[uuid = "e228a534-e3ca-2e1e-ab9d-4d8bc1ad8c19"]
pub struct TerminalMaterial {
    pub clip_color: Color,
    pub texture: Option<Handle<Image>>,
}

impl Default for TerminalMaterial {
    fn default() -> Self {
        TerminalMaterial {
            clip_color: Color::BLACK,
            texture: None,
        }
    }
}

impl From<Handle<Image>> for TerminalMaterial {
    fn from(texture: Handle<Image>) -> Self {
        TerminalMaterial {
            texture: Some(texture),
            clip_color: Color::BLACK,
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
#[derive(Clone, Default, AsStd140)]
pub struct TerminalMaterialUniformData {
    pub color: Vec4,
    pub flags: u32,
}

// The data from our material that gets copied to the gpu
#[derive(Debug, Clone)]
pub struct GpuTerminalMaterial {
    pub buffer: Buffer,
    pub bind_group: BindGroup,
    pub flags: TerminalMaterialFlags,
    pub texture: Option<Handle<Image>>,
}

// Boilerplate copied from `ColorMaterial`. Allows us to reference
// our texture and mesh/view structs from the shader.
impl RenderAsset for TerminalMaterial {
    type ExtractedAsset = TerminalMaterial;
    type PreparedAsset = GpuTerminalMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<TerminalMaterial>>,
        SRes<RenderAssets<Image>>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        material: Self::ExtractedAsset,
        (render_device, pipeline, gpu_images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let (texture_view, sampler) = if let Some(result) = pipeline
            .mesh2d_pipeline
            .get_image_texture(gpu_images, &material.texture)
        {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(material));
        };

        let mut flags = TerminalMaterialFlags::NONE;
        if material.texture.is_some() {
            flags |= TerminalMaterialFlags::TEXTURE;
        }

        let value = TerminalMaterialUniformData {
            color: material.clip_color.as_linear_rgba_f32().into(),
            flags: flags.bits(),
        };
        let value_std140 = value.as_std140();

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("terminal_material_uniform_buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: value_std140.as_bytes(),
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(texture_view),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(sampler),
                },
            ],
            label: Some("terminal_material_bind_group"),
            layout: &pipeline.material2d_layout,
        });

        Ok(GpuTerminalMaterial {
            buffer,
            bind_group,
            flags,
            texture: material.texture,
        })
    }
}

impl SpecializedMaterial2d for TerminalMaterial {
    fn fragment_shader(_asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(TERMINAL_MATERIAL_SHADER_HANDLE.typed())
    }

    fn vertex_shader(_asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(TERMINAL_MATERIAL_SHADER_HANDLE.typed())
    }

    #[inline]
    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }


    fn bind_group_layout(
        render_device: &RenderDevice,
    ) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            TerminalMaterialUniformData::std140_size_static() as u64,
                        ),
                    },
                    count: None,
                },
                // Texture
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                // Texture Sampler
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("color_material_layout"),
        })
    }

    type Key = ();

    fn key(_material: &<Self as RenderAsset>::PreparedAsset) -> Self::Key {
        ()
    }

    fn specialize(_key: Self::Key, descriptor: &mut RenderPipelineDescriptor) {
        let vertex_attributes = vec![
            // Until https://github.com/bevyengine/bevy/pull/3120 is merged,
            // attributes have a bizarre ordering. "Built-in" attributes appear to be packed first
            // in alphabetical order, then remaining attributes appear to packed in alphabetical order
            // afterwards?

            // Vertex_Position
            VertexAttribute {
                format: VertexFormat::Float32x3,
                // this offset is the size of the color attribute, which is stored first
                offset: 0,
                // position is available at location 0 in the shader
                shader_location: 0,
            },
            // Vertex_UV
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 12,
                shader_location: 1,
            },
            // bg_color
            VertexAttribute {
                format: VertexFormat::Float32x4,
                // this offset is the size of the color attribute, which is stored first
                offset: 12 + 8,
                // position is available at location 0 in the shader
                shader_location: 2,
            },
            // fg_color
            VertexAttribute {
                format: VertexFormat::Float32x4,
                offset: 12 + 8 + 16,
                shader_location: 3,
            },
        ];
        // Verts, UVs, FGColors, BGColors
        let stride = 12 + 8 + 16 + 16;

        let buffers = vec![VertexBufferLayout {
            array_stride: stride,
            step_mode: VertexStepMode::Vertex,
            attributes: vertex_attributes,
        }];

        descriptor.vertex.buffers = buffers;
    }
}

