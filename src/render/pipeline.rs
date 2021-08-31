use bevy::render::pipeline::RenderPipeline;
use bevy::render::render_graph::base::{node};
use bevy::render::render_graph::{AssetRenderResourcesNode, RenderGraph};
use bevy::render::shader::{ShaderStage, ShaderStages};
use bevy::render::{color::Color, renderer::RenderResources, shader::ShaderDefs, texture::Texture};
use bevy::{prelude::*, render::pipeline::PipelineDescriptor};
use bevy::reflect::{TypeUuid};

const VERTEX_SHADER: &str = r#"
#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec2 Vertex_Uv;
layout(location = 2) in vec3 FG_Color;
layout(location = 3) in vec3 BG_Color;

layout(location = 0) out vec2 v_Uv;
layout(location = 1) out vec3 Frag_FG_Color;
layout(location = 2) out vec3 Frag_BG_Color;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 2, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    vec2 uv = Vertex_Uv;

    v_Uv = uv;

    vec3 position = Vertex_Position;
    gl_Position = ViewProj * Model * vec4(position, 1.0);
    Frag_FG_Color = FG_Color;
    Frag_BG_Color = BG_Color;
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450
layout(location = 0) in vec2 v_Uv;
layout(location = 0) out vec4 o_Target;
layout(set = 1, binding = 0) uniform ColorMaterial_color {
    vec4 Color;
};

layout(location = 1) in vec3 Frag_FG_Color;
layout(location = 2) in vec3 Frag_BG_Color;

# ifdef COLORMATERIAL_TEXTURE 
layout(set = 1, binding = 1) uniform texture2D ColorMaterial_texture;
layout(set = 1, binding = 2) uniform sampler ColorMaterial_texture_sampler;
# endif

void main() {
    vec4 color = Color;
# ifdef COLORMATERIAL_TEXTURE
    vec4 texColor = texture(
        sampler2D(ColorMaterial_texture, ColorMaterial_texture_sampler),
        v_Uv);
    if(texColor.rgb == vec3(0.0)) {
        color.rgb = Frag_BG_Color;
    } else {
        color.rgb *= texColor.rgb * Frag_FG_Color;
    }
# endif
    //color = vec4(1.0, 1.0, 1.0, 1.0);
    //color.a = 1.0;
    o_Target = color;
}
"#;

pub const TERMINAL_PIPELINE_HANDLE: HandleUntyped = 
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 1185249840666725446);

pub fn get_pipelines(
    shaders: &mut ResMut<Assets<Shader>>,
    pipelines: &mut ResMut<Assets<PipelineDescriptor>>) -> RenderPipelines {
    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));
    RenderPipelines::from_pipelines(vec![RenderPipeline::new(
        pipeline_handle.clone(),
    )])
} 

struct LoadingTexture(Option<Handle<Texture>>);

#[derive(Default)]
pub struct TerminalRendererPipeline(pub Handle<PipelineDescriptor>);

impl TerminalRendererPipeline {
    pub fn get_pipelines(&self) -> RenderPipelines {
        let pipelines: Vec<Handle<PipelineDescriptor>> = vec![self.0.clone()];
        RenderPipelines::from_handles(&pipelines)
    }
}

pub fn setup_pipeline(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    let pipeline = PipelineDescriptor::default_config(
        ShaderStages {
            vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
            fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))
        )}
    );
    commands.insert_resource(TerminalRendererPipeline(pipelines.add(pipeline)));
}

pub fn setup_pipeline2(
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut pipeline: ResMut<TerminalRendererPipeline>) {
        
    pipeline.0 = pipelines.add(PipelineDescriptor::default_config(
        ShaderStages {
            vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
            fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))
        )}
    ));
}