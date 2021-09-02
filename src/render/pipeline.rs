use bevy::reflect::TypeUuid;
use bevy::render::shader::{ShaderStage, ShaderStages};
use bevy::{prelude::*, render::pipeline::PipelineDescriptor};

const VERTEX_SHADER: &str = include_str!("terminal.vert");
const FRAGMENT_SHADER: &str = include_str!("terminal.frag");

pub const TERMINAL_RENDERER_PIPELINE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 12121362113012541389);

pub fn build_terminal_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    })
}