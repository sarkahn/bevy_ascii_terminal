use bevy::{prelude::*, reflect::TypeUuid, render::{pipeline::{PipelineDescriptor}, render_graph::{base, AssetRenderResourcesNode, RenderGraph}, shader::{ShaderStage, ShaderStages}, texture::{ImageType}}};

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


pub fn terminal_renderer_init(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<&mut Handle<Mesh>, (Added<Handle<Mesh>>, With<TerminalRendererVertexData>)>,
) {
    for mut mesh in q.iter_mut() {
        //info!("Initializing ascii terminal mesh");
        let new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        *mesh = meshes.add(new_mesh);
    }
}

fn terminal_renderer_update_material(
    fonts: Res<TerminalFonts>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
    textures: Res<Assets<Texture>>,
    mut q: Query<
        (&TerminalRendererFont, &mut Handle<TerminalMaterial>),
        Changed<TerminalRendererFont>,
    >,
) {
    for (font, mut mat) in q.iter_mut() {
        //info!("Updating terminal renderer material");
        let existing_mat = materials.get(mat.clone_weak());

        if existing_mat.is_some() {
            materials.remove(mat.clone_weak());
        }

        let handle = &fonts.get(font.font_name.as_str()).0;
        let tex = textures.get(handle.clone());
        debug_assert!(tex.is_some());

        *mat = materials.add(TerminalMaterial::from_texture(
            handle.clone(),
            font.clip_color,
        ));
    }
}

fn terminal_renderer_update_size(
    mut meshes: ResMut<Assets<Mesh>>,
    fonts: Res<TerminalFonts>,
    mut q: Query<
        (
            &TerminalSize,
            &TerminalRendererFont,
            &TerminalTileScaling,
            &TerminalPivot,
            &TilePivot,
            &mut Handle<Mesh>,
            &mut TerminalRendererVertexData,
            &mut TerminalRendererTileData,
        ),
        Or<(
            Changed<TerminalSize>,
            Changed<Handle<Mesh>>,
            Changed<TerminalTileScaling>,
            Changed<TerminalRendererFont>,
        )>,
    >,
) {
    for (size, font, scaling, term_pivot, tile_pivot, mesh, mut vert_data, mut tile_data) in
        q.iter_mut()
    {
        let mut tile_size = UVec2::ONE;
        if let TerminalTileScaling::Pixels = scaling {
            tile_size = fonts.get(font.font_name.as_str()).1.tile_size;
        }

        vert_data.resize(size.value, term_pivot.0, tile_pivot.0, tile_size);
        tile_data.resize(size.value);

        let mesh = meshes
            .get_mut(mesh.clone())
            .expect("Error retrieving mesh from terminal renderer");

        //info!("Renderer update size: {}!", vert_data.indices.len());
        //info!("First 4 verts: {:?}", &vert_data.verts[0..4]);
        //info!("First 6 indices: {:?}", &vert_data.indices[0..6]);
        mesh.set_indices(Some(Indices::U32(vert_data.indices.clone())));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vert_data.verts.clone());
    }
}

pub fn terminal_renderer_update_tile_data(
    mut q: Query<(&Terminal, &mut TerminalRendererTileData), Changed<Terminal>>,
) {
    for (term, mut data) in q.iter_mut() {
        //info!("Renderer update tile data!");
        //info!("First tiles: {:?}", &term.tiles[0..4]);
        data.update_from_tiles(&term.tiles);
    }
}

pub fn terminal_renderer_update_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(&TerminalRendererTileData, &Handle<Mesh>), Changed<TerminalRendererTileData>>,
) {
    for (tile_data, mesh) in q.iter_mut() {
        let mesh = meshes.get_mut(mesh).expect("Error accessing terminal mesh");
        //info!("writing colors and uvs to mesh");
        //info!("First fg Colors: {:?}", &tile_data.fg_colors[0..4]);
        //info!("First bg Colors: {:?}", &tile_data.bg_colors[0..4]);
        //info!("First uvs: {:?}", &tile_data.uvs[0..4]);
        mesh.set_attribute("FG_Color", tile_data.fg_colors.clone());
        mesh.set_attribute("BG_Color", tile_data.bg_colors.clone());
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, tile_data.uvs.clone());
    }
}
