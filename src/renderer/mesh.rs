use bevy::{
    ecs::prelude::*,
    prelude::{Assets, Mesh,},
    render::{render_resource::PrimitiveTopology, mesh::Indices},
    sprite::{ Mesh2dHandle}, 
};

use super::{
    mesh_data::{ATTRIBUTE_UV, ATTRIBUTE_COLOR_FG, ATTRIBUTE_COLOR_BG, TileData, VertData},
};

pub(crate) fn init_mesh(
    mut q_mesh: Query<&mut Mesh2dHandle, Added<VertData>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for mut handle in &mut q_mesh {
    // Initialize terminal mesh
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_indices(Some(Indices::U32(Vec::new())));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new());
        mesh.insert_attribute(ATTRIBUTE_UV, Vec::<[f32; 2]>::new());
        mesh.insert_attribute(ATTRIBUTE_COLOR_FG, Vec::<[f32; 4]>::new());
        mesh.insert_attribute(ATTRIBUTE_COLOR_BG, Vec::<[f32; 4]>::new());
        *handle = Mesh2dHandle(meshes.add(mesh));
    }
}

pub(crate) fn update_mesh_verts(
    mut q_mesh: Query<(&mut VertData, &Mesh2dHandle), Changed<VertData>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut vd, handle) in &mut q_mesh {
        if let Some(mut mesh) = meshes.get_mut(&handle.0) {
            vd.build_mesh_verts(&mut mesh);
            //println!("Updating mesh verts. Indices count {}", mesh.indices().unwrap().len());
        }
    }
}

pub(crate) fn update_mesh_tiles(
    mut q_mesh: Query<(&mut TileData, &Mesh2dHandle), Changed<TileData>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (mut td, handle) in &mut q_mesh {
        if let Some(mut mesh) = meshes.get_mut(&handle.0) {
            td.build_mesh_tiles(&mut mesh);
            //println!("Update mesh uvs. UvCount {}", mesh.attribute(ATTRIBUTE_UV).unwrap().len());
        }
    }
}
