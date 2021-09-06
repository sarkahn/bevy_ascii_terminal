use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::PipelineDescriptor,
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        shader::{ShaderStage, ShaderStages},
        texture::ImageType,
    },
};

use super::{font::*, *};

pub(crate) const TERMINAL_RENDERER_PIPELINE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 12121362113012541389);

const VERTEX_SHADER: &str = include_str!("terminal.vert");
const FRAGMENT_SHADER: &str = include_str!("terminal.frag");

const TERMINAL_MATERIAL_NAME: &str = "terminal_mat";

pub struct TerminalRendererPlugin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum AppState {
    AssetsLoading,
    AssetsDoneLoading,
}

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<LoadingTerminalTextures>()
            .init_resource::<TerminalFonts>()
            .add_asset::<TerminalMaterial>()
            .add_state(AppState::AssetsLoading)
            .add_system_set(
                SystemSet::on_enter(AppState::AssetsLoading)
                    .with_system(terminal_load_assets.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::AssetsLoading)
                    .with_system(check_terminal_assets_loading.system()),
            )
            .add_system_set(
                SystemSet::on_enter(AppState::AssetsDoneLoading)
                    .with_system(terminal_renderer_init.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::AssetsDoneLoading)
                    .with_system(
                        terminal_renderer_update_material
                            .system()
                            .label("terminal_update_material"),
                    )
                    .with_system(
                        terminal_renderer_update_size
                            .system()
                            .after("terminal_update_material")
                            .label("terminal_update_size"),
                    )
                    .with_system(
                        terminal_renderer_update_tile_data
                            .system()
                            .after("terminal_update_size")
                            .label("terminal_update_tile_data"),
                    )
                    .with_system(
                        terminal_renderer_update_mesh
                            .system()
                            .after("terminal_update_tile_data")
                            .label("terminal_update_mesh"),
                    ),
            );

        // Set up material/pipline for default terminal construction
        let cell = app.world_mut().cell();

        let mut graph = cell.get_resource_mut::<RenderGraph>().unwrap();
        let mut pipelines = cell
            .get_resource_mut::<Assets<PipelineDescriptor>>()
            .unwrap();
        let mut shaders = cell.get_resource_mut::<Assets<Shader>>().unwrap();
        let mut materials = cell.get_resource_mut::<Assets<TerminalMaterial>>().unwrap();
        let mut textures = cell.get_resource_mut::<Assets<Texture>>().unwrap();

        graph.add_system_node(
            TERMINAL_MATERIAL_NAME,
            AssetRenderResourcesNode::<TerminalMaterial>::new(true),
        );
        graph
            .add_node_edge(TERMINAL_MATERIAL_NAME, base::node::MAIN_PASS)
            .unwrap();

        materials.set_untracked(
            Handle::<TerminalMaterial>::default(),
            TerminalMaterial::default(),
        );

        let pipeline = PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
            fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
        });

        pipelines.set_untracked(TERMINAL_RENDERER_PIPELINE, pipeline);

        // Set up a font handle for default terminal construction
        let bytes = font::DEFAULT_FONT.bytes;
        let tex = Texture::from_buffer(bytes, ImageType::Extension("png")).unwrap();

        textures.set_untracked(DEFAULT_FONT_HANDLE, tex);
    }
}
