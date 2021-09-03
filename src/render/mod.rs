pub mod entity;
pub mod pipeline;

mod font_data;
mod glyph_mapping;
pub mod renderer_tile_data;
pub mod renderer_vertex_data;

use self::{
    font_data::{
        check_terminal_assets_loading, terminal_load_assets, LoadingTerminalTextures, TerminalFonts,
    },
    renderer_tile_data::TerminalRendererTileData,
    renderer_vertex_data::TerminalRendererVertexData,
};
use crate::terminal::{Terminal, TerminalSize};
use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        pipeline::{PipelineDescriptor, PrimitiveTopology},
    },
};

const DEFAULT_TEX_PATH: &str = "alloy_curses_12x12.png";

pub struct TerminalPivot(Vec2);
impl Default for TerminalPivot {
    fn default() -> Self {
        Self(Vec2::new(0.5, 0.5))
    }
}

#[derive(Default)]
pub struct TilePivot(Vec2);

pub struct TerminalRendererFont(pub String);
impl Default for TerminalRendererFont {
    fn default() -> Self {
        Self(String::from(DEFAULT_TEX_PATH))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum AppState {
    AssetsLoading,
    AssetsDoneLoading,
}

#[derive(Clone, Copy)]
pub enum TerminalTileScaling {
    /// Scale terminal tiles based on the size of their texture, such that 1 pixel == 1 world unit.
    /// This behavior matches the expected defaults for bevy's orthographic camera.
    Pixels,
    /// Each tile will take up 1 unit of world space
    World,
}

impl Default for TerminalTileScaling {
    fn default() -> Self {
        TerminalTileScaling::Pixels
    }
}

pub struct TerminalRendererPlugin;

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut AppBuilder) {
        {
            let world = &mut app.world_mut();

            let pipeline = pipeline::build_terminal_pipeline(
                &mut world.get_resource_mut::<Assets<Shader>>().unwrap(),
            );
            let mut pipelines = world
                .get_resource_mut::<Assets<PipelineDescriptor>>()
                .unwrap();
            pipelines.set_untracked(pipeline::TERMINAL_RENDERER_PIPELINE, pipeline);
        }

        app.init_resource::<LoadingTerminalTextures>()
            .init_resource::<TerminalFonts>()
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
    }
}

pub fn terminal_renderer_init(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<&mut Handle<Mesh>, (Added<Handle<Mesh>>, With<TerminalRendererVertexData>)>,
) {
    for mut mesh in q.iter_mut() {
        //info!("Initializing terminal renderer");
        let new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        *mesh = meshes.add(new_mesh);
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
        //info!("Updating terminal renderer material");
        let existing_mat = materials.get(mat.clone_weak());

        if existing_mat.is_some() {
            materials.remove(mat.clone_weak());
        }

        let mut path = "textures/".to_owned();
        path.push_str(font.0.as_str());
        let tex_handle = asset_server.load(path.as_str());
        let tex = textures.get(tex_handle.clone());
        debug_assert!(tex.is_some());
        *mat = materials.add(ColorMaterial::texture(tex_handle));
    }
}

pub fn terminal_renderer_update_size(
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
        )>,
    >,
) {
    for (size, font, scaling, term_pivot, tile_pivot, mesh, mut vert_data, mut tile_data) in
        q.iter_mut()
    {
        let mut tile_size = UVec2::ONE;
        if let TerminalTileScaling::Pixels = scaling {
            tile_size = fonts.get(font.0.as_str()).tile_size;
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
        // info!("writing colors and uvs to mesh");
        // info!("First fg Colors: {:?}", &tile_data.fg_colors[0..4]);
        // info!("First bg Colors: {:?}", &tile_data.bg_colors[0..4]);
        // info!("First uvs: {:?}", &tile_data.uvs[0..4]);
        mesh.set_attribute("FG_Color", tile_data.fg_colors.clone());
        mesh.set_attribute("BG_Color", tile_data.bg_colors.clone());
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, tile_data.uvs.clone());
    }
}
