pub mod entity;
pub mod pipeline;

mod glyph_mapping;
pub mod renderer_tile_data;
pub mod renderer_vertex_data;

use self::{
    renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData,
};
use crate::terminal::{Terminal, TerminalSize};
use bevy::{asset::LoadState, prelude::*, render::{mesh::Indices, pipeline::PrimitiveTopology}};
pub use pipeline::TerminalRendererPipeline;

const DEFAULT_TEX_PATH: &str = "textures/alloy_curses_12x12.png";

pub struct TerminalRendererFont(pub String);
impl Default for TerminalRendererFont {
    fn default() -> Self {
        Self(String::from(DEFAULT_TEX_PATH))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    AssetsLoading,
    AssetsDoneLoading,
}

pub struct TerminalRendererPlugin;

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TerminalRendererPipeline>() 
        .init_resource::<LoadingTerminalTextures>()
        .add_state(AppState::AssetsLoading) 

        .add_system_set(SystemSet::on_enter(AppState::AssetsLoading)
            .with_system(terminal_load_assets.system())
        )
        .add_system_set(SystemSet::on_update(AppState::AssetsLoading)
            .with_system(check_terminal_assets_loading.system())
        )

        .add_system_set(SystemSet::on_enter(AppState::AssetsDoneLoading)
             .with_system(
                 terminal_renderer_init.system()
             )
        )
        .add_system_set(SystemSet::on_update(AppState::AssetsDoneLoading)
            .with_system(
                terminal_renderer_update_material.system()
                .label("terminal_update_material")
            )
            .with_system(
                terminal_renderer_update_size.system()
                .after("terminal_update_material")
                .label("terminal_update_size")
            )
            .with_system(
                terminal_renderer_update_tile_data.system()
                .after("terminal_update_size")
                .label("terminal_update_tile_data")
            )
            .with_system(
                terminal_renderer_update_mesh.system()
                .after("terminal_update_tile_data")
                .label("terminal_update_mesh")
            )
        );
    }
}

#[derive(Default)]
struct LoadingTerminalTextures(Vec<HandleUntyped>);

fn terminal_load_assets(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<LoadingTerminalTextures>,
) {
    info!("Loading terminal textures");
    loading.0 = asset_server.load_folder("textures").expect("Error loading terminal textures folder");
}

fn check_terminal_assets_loading(
    asset_server: Res<AssetServer>,
    loading: Res<LoadingTerminalTextures>,
    mut state: ResMut<State<AppState>>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(loading.0.iter().map(|h|h.id)) {
        info!("Done loading terminal textures");
        state.set(AppState::AssetsDoneLoading).unwrap();
    }
}

pub fn terminal_renderer_init(
    mut meshes: ResMut<Assets<Mesh>>,
    pipeline: Res<TerminalRendererPipeline>,
    mut q: Query<
        (&mut Handle<Mesh>, &mut RenderPipelines),
        (Added<Handle<Mesh>>, With<TerminalRendererVertexData>),
    >,
) {
    for (mut mesh, mut pipelines) in q.iter_mut() {
        info!("Initializing terminal renderer");
        let new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        *mesh = meshes.add(new_mesh);
        *pipelines = pipeline.get_pipelines();
    }
}

pub fn terminal_renderer_update_material(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    textures: Res<Assets<Texture>>,
    mut q: Query<
        (&TerminalRendererFont, &mut Handle<ColorMaterial>),
        Changed<TerminalRendererFont>,
    >,
) {
    for (font, mut mat) in q.iter_mut() {
        info!("Updating terminal renderer material");
        let existing_mat = materials.get(mat.clone_weak());

        if existing_mat.is_some() {
            materials.remove(mat.clone_weak());
        }

        let tex_handle = asset_server.load(font.0.as_str());
        let tex = textures.get(tex_handle.clone());
        debug_assert!(tex.is_some());
        *mat = materials.add(ColorMaterial::texture(tex_handle));
    }
}

pub fn terminal_renderer_update_size(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<
        (
            &TerminalSize,
            &mut Handle<Mesh>,
            &mut TerminalRendererVertexData,
            &mut TerminalRendererTileData,
        ),
        Or<(Changed<TerminalSize>, Changed<Handle<Mesh>>)>,
    >,
) {
    for (size, mesh, mut vert_data, mut tile_data) in q.iter_mut() {
        let (w, h) = size.into();
        vert_data.resize(w, h);
        tile_data.resize(w, h);

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
        info!("writing colors and uvs to mesh");
        info!("First fg Colors: {:?}", &tile_data.fg_colors[0..4]);
        info!("First bg Colors: {:?}", &tile_data.bg_colors[0..4]);
        mesh.set_attribute("FG_Color", tile_data.fg_colors.clone());
        mesh.set_attribute("BG_Color", tile_data.bg_colors.clone());
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, tile_data.uvs.clone());
    }
}
