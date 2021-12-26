//! Default plugin for rendering the terminal to a bevy mesh.

pub mod terminal_renderer_system_names {
    pub const TERMINAL_UPDATE_MATERIAL: &str = "terminal_update_material";
    pub const TERMINAL_UPDATE_SIZE: &str = "terminal_update_size";
    pub const TERMINAL_UPDATE_TILE_DATA: &str = "terminal_update_tile_data";
    pub const TERMINAL_UPDATE_MESH: &str = "terminal_update_mesh";
}

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::PipelineDescriptor,
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        shader::{ShaderStage, ShaderStages},
    },
};

use super::{font::*, *};

pub(crate) const TERMINAL_RENDERER_PIPELINE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 12121362113012541389);

const VERTEX_SHADER: &str = include_str!("terminal.vert");
const FRAGMENT_SHADER: &str = include_str!("terminal.frag");

const TERMINAL_MATERIAL_NAME: &str = "terminal_mat";

pub struct TerminalRendererPlugin;

/// AppState for loading terminal assets.
///
/// Systems can be added to the [AssetsDoneLoading](AssetsDoneLoading) state to safely read terminal assets.
///
/// ## Example
///
/// ```ignore
/// App::build().add_system_set(
///     SystemSet::on_enter(TerminalAssetLoadState::AssetsDoneLoading)
///         .with_system(read_font_system.system());
/// )
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TerminalAssetLoadState {
    AssetsLoading,
    AssetsDoneLoading,
}

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut AppBuilder) {
        use terminal_renderer_system_names::*;

        app.add_state(TerminalAssetLoadState::AssetsLoading);

        app.add_plugin(TerminalFontPlugin);

        app.add_asset::<TerminalMaterial>()
            .add_system_set(
                SystemSet::on_enter(TerminalAssetLoadState::AssetsDoneLoading)
                    .with_system(terminal_renderer_init.system()),
            )
            .add_system_set(
                SystemSet::on_update(TerminalAssetLoadState::AssetsDoneLoading)
                    .with_system(
                        terminal_renderer_update_material
                            .system()
                            .label(TERMINAL_UPDATE_MATERIAL),
                    )
                    .with_system(
                        terminal_renderer_update_size
                            .system()
                            .after(TERMINAL_UPDATE_MATERIAL)
                            .label(TERMINAL_UPDATE_SIZE),
                    )
                    .with_system(
                        terminal_renderer_update_tile_data
                            .system()
                            .after(TERMINAL_UPDATE_SIZE)
                            .label(TERMINAL_UPDATE_TILE_DATA),
                    )
                    .with_system(
                        terminal_renderer_update_mesh
                            .system()
                            .after(TERMINAL_UPDATE_TILE_DATA)
                            .label(TERMINAL_UPDATE_MESH),
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
    }
}

#[allow(clippy::type_complexity)]
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
    mut q: Query<(&TerminalFont, &mut Handle<TerminalMaterial>), Changed<TerminalFont>>,
) {
    for (font, mut mat) in q.iter_mut() {
        //info!("Updating terminal renderer material");
        let existing_mat = materials.get(mat.clone_weak());

        if existing_mat.is_some() {
            materials.remove(mat.clone_weak());
        }

        let handle = fonts.get(font.name()).texture_handle();

        *mat = materials.add(TerminalMaterial::from_texture(
            handle.clone(),
            font.clip_color(),
        ));
    }
}

#[allow(clippy::type_complexity)]
fn terminal_renderer_update_size(
    mut meshes: ResMut<Assets<Mesh>>,
    fonts: Res<TerminalFonts>,
    mut q: Query<
        (
            &Terminal,
            &TerminalFont,
            &TileScaling,
            &TerminalPivot,
            &TilePivot,
            &mut Handle<Mesh>,
            &mut TerminalRendererVertexData,
            &mut TerminalRendererTileData,
        ),
        Or<(
            Changed<Terminal>,
            Changed<Handle<Mesh>>,
            Changed<TileScaling>,
            Changed<TerminalFont>,
        )>,
    >,
) {
    for (terminal, font, scaling, term_pivot, tile_pivot, mesh, mut vert_data, mut tile_data) in
        q.iter_mut()
    {
        let mut tile_size = UVec2::ONE;
        if let TileScaling::Pixels = scaling {
            tile_size *= fonts.get(font.name()).pixels_per_unit();
        }

        let size = terminal.size();
        vert_data.resize(size, term_pivot.0, tile_pivot.0, tile_size);
        tile_data.resize(size);

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

#[cfg(test)]
pub mod tests {
    use bevy::prelude::*;

    #[test]
    fn mesh_test() {
        let _world = World::default();

        let _update_stage = SystemStage::parallel();
    }
}
