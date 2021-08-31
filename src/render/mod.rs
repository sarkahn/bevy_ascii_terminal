pub mod pipeline;
pub mod entity;

pub mod renderer_tile_data;
pub mod renderer_vertex_data;

pub use pipeline::{TerminalRendererPipeline};
use bevy::{prelude::*, render::{mesh::Indices, pipeline::PrimitiveTopology}};
use crate::terminal::{Terminal, TerminalSize};
use self::{renderer_tile_data::TerminalRendererTileData, renderer_vertex_data::TerminalRendererVertexData};

#[derive(Default)]
pub struct TerminalMesh(pub Handle<Mesh>);

pub struct TerminalResources {
    pub mesh: Handle<Mesh>,
    pub pipelines: RenderPipelines,
}

pub struct TerminalRendererPlugin;

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TerminalRendererPipeline>()
        .add_system(terminal_init.system().label("terminal_init"))
        .add_system(terminal_renderer_update_size.system().label("update_size_system").after("terminal_init"))
        .add_system(terminal_renderer_update_tile_data.system().label("update_data_system").after("update_size_system"))
        .add_system(terminal_renderer_update_mesh.system().after("update_data_system"))
        ;
    }
}

// pub fn terminal_init(
//     mut meshes: ResMut<Assets<Mesh>>,
//     pipeline: Res<TerminalRendererPipeline>,
//     mut q: Query<(&mut Handle<Mesh>, &mut RenderPipelines), 
//                  (Added<Handle<Mesh>>, With<TerminalRendererVertexData>)>) {
//     for (mut mesh, mut pipelines) in q.iter_mut() {
//         let new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
//         *mesh = meshes.add(new_mesh);
//         *pipelines = pipeline.get_pipelines();
//     }
// }

pub fn terminal_init(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    pipeline: Res<TerminalRendererPipeline>,
    mut q: Query<(&mut Handle<Mesh>, &mut RenderPipelines, &mut Handle<ColorMaterial>), 
                 (Added<Handle<Mesh>>, With<TerminalRendererVertexData>)>) {
    for (mut mesh, mut pipelines, mut mat) in q.iter_mut() {
        info!("intializing terminal resources");

        let tex = asset_server.load("zx_evolution_8x8.png");
        *mat = materials.add(ColorMaterial::texture(tex));

        let new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        *mesh = meshes.add(new_mesh);
        *pipelines = pipeline.get_pipelines();
    }
}


pub fn terminal_renderer_update_size(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(&TerminalSize, &mut Handle<Mesh>, &mut TerminalRendererVertexData, &mut TerminalRendererTileData), 
        Or<(Changed<TerminalSize>, Changed<Handle<Mesh>>)>>) { 
    for (size, mut mesh, mut vert_data, mut tile_data) in q.iter_mut() {
        let (w,h) = size.into();
        vert_data.resize(w, h);
        tile_data.resize(w, h);

        let mesh = meshes.get_mut(mesh.clone()).expect("Error retrieving mesh from terminal renderer");

        info!("Renderer update size: {}!", vert_data.indices.len());
        info!("First 4 verts: {:?}", &vert_data.verts[0..4]);
        info!("First 6 indices: {:?}", &vert_data.indices[0..6]);
        mesh.set_indices(Some(Indices::U32(vert_data.indices.clone())));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vert_data.verts.clone());
    }
}

pub fn terminal_renderer_update_tile_data(
    mut q: Query<(&Terminal, &mut TerminalRendererTileData), Changed<Terminal>>) {
    for (term, mut data) in q.iter_mut() {
        info!("Renderer update tile data!");
        info!("First tiles: {:?}", &term.tiles[0..4]);
        data.update_from_tiles(&term.tiles);
    } 
}

pub fn terminal_renderer_update_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(&TerminalRendererTileData, &Handle<Mesh>), Changed<TerminalRendererTileData>>) {
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