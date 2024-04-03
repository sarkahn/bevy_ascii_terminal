use bevy::{
    app::Plugin,
    ecs::{query::With, system::Query},
    sprite::Mesh2dHandle,
};

use crate::border_entity::TerminalBorder;

pub struct TerminalBorderMeshPlugin;

impl Plugin for TerminalBorderMeshPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        todo!()
    }
}

// #[allow(clippy::type_complexity)]
// fn rebuild_verts(
//     mut q_term: Query<
//         (
//             Entity,
//             &mut TerminalBorder,
//             &Mesh2dHandle,
//             &TerminalTransform,
//             &Handle<TerminalMaterial>,
//         ),
//         Or<(
//             Changed<TerminalMeshPivot>,
//             Changed<TerminalFontScaling>,
//             With<RebuildVerts>,
//         )>,
//     >,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut commands: Commands,
//     materials: Res<Assets<TerminalMaterial>>,
//     images: Res<Assets<Image>>,
// ) {
//     for (entity, mut term, mesh_handle, transform, mat_handle) in &mut q_term {
//         commands.entity(entity).remove::<RebuildVerts>();

//         let mesh = meshes
//             .get_mut(mesh_handle.0.clone())
//             .expect("Error getting terminal mesh");

//         let mat = materials
//             .get(mat_handle)
//             .expect("Error getting terminal material");

//         // If the material texture is set to none, or if it's not loaded yet,
//         // clear the mesh. This function will be called again when a valid image
//         // is loaded
//         if mat.texture.is_none() || images.get(mat.texture.as_ref().unwrap()).is_none() {
//             resize_mesh_data(mesh, 0);
//             continue;
//         }

//         resize_mesh_data(mesh, term.tile_count());

//         let origin = transform.world_mesh_bounds().min;
//         let tile_size = transform.world_tile_size();

//         // We only need to build our vertex data, uvs/colors will be updated
//         // in "tile_mesh_update"
//         VertMesher::build_mesh_verts(origin, tile_size, mesh, |mesher| {
//             for (i, (p, _)) in term.iter_xy().enumerate() {
//                 mesher.set_tile(p.x, p.y, i);
//             }
//         });

//         // Force tile mesh update
//         term.set_changed();

//         // let origin = renderer.mesh_origin();
//         // let tile_size = renderer.tile_size_world();
//         // let origin = transform.world_mesh_bounds().min;
//         // let tile_size = transform.world_tile_size();
//         // VertMesher::build_mesh_verts(origin, tile_size, mesh, |mesher| {
//         //     for (p, _) in term.iter_xy() {
//         //         mesher.add_tile(p.x, p.y);
//         //     }
//         // });
//         // UVMesher::build_mesh_tile_data(mapping, mesh, |mesher| {
//         //     for t in term.tiles().iter() {
//         //         mesher.add_tile(t.glyph, t.fg_color, t.bg_color);
//         //     }
//         // });
//         // if let Some(border) = term.get_border() {
//         //     VertMesher::build_mesh_verts(origin, tile_size, mesh, |mesher| {
//         //         for (p, _) in border.iter() {
//         //             mesher.add_tile(p.x, p.y);
//         //         }
//         //     });

//         //     UVMesher::build_mesh_tile_data(mapping, mesh, |mesher| {
//         //         for (_, t) in border.iter() {
//         //             mesher.add_tile(t.glyph, t.fg_color, t.bg_color);
//         //         }
//         //     });
//         // }
//     }
// }
