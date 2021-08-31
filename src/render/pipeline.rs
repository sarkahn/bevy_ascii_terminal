use bevy::render::shader::{ShaderStage, ShaderStages};
use bevy::{prelude::*, render::pipeline::PipelineDescriptor};

const VERTEX_SHADER: &str = include_str!("terminal.vert");
const FRAGMENT_SHADER: &str = include_str!("terminal.frag");

pub struct TerminalRendererPipeline(pub Handle<PipelineDescriptor>);

impl FromWorld for TerminalRendererPipeline {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();
        let mut pipelines = world.get_resource_mut::<Assets<PipelineDescriptor>>().unwrap();

        let pipeline = PipelineDescriptor::default_config(
            ShaderStages {
                vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
                fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))
            )}
        );

        Self(pipelines.add(pipeline))
    }
}

impl TerminalRendererPipeline {
    pub fn get_pipelines(&self) -> RenderPipelines {
        let pipelines: Vec<Handle<PipelineDescriptor>> = vec![self.0.clone()];
        RenderPipelines::from_handles(&pipelines)
    }
}