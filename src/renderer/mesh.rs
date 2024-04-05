use std::iter::repeat;

use bevy::{
    app::{Last, Plugin, PostUpdate},
    asset::{AssetEvent, Assets, Handle},
    ecs::{
        change_detection::DetectChangesMut,
        component::Component,
        entity::Entity,
        event::EventReader,
        query::{Added, Changed, Or, With},
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Commands, Query, Res, ResMut},
    },
    math::Vec2,
    render::{
        color::Color,
        mesh::{Indices, Mesh, MeshVertexAttribute, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::{PrimitiveTopology, VertexFormat},
        texture::Image,
    },
    sprite::Mesh2dHandle,
};

use crate::{
    border_entity::TerminalBorder, transform::TerminalTransformSystems, Pivot, Terminal,
    TerminalFont, TerminalTransform,
};

use super::{
    material::TerminalMaterial,
    mesher::{UvMesher, VertMesher},
    uv_mapping::UvMapping,
};

pub const ATTRIBUTE_UV: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Uv", 1123131, VertexFormat::Float32x2);
pub const ATTRIBUTE_COLOR_BG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Bg", 1123132, VertexFormat::Float32x4);
pub const ATTRIBUTE_COLOR_FG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Fg", 1123133, VertexFormat::Float32x4);

pub struct TerminalMeshPlugin;

/// Systems for building the terminal mesh.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalRenderSystems;

impl Plugin for TerminalMeshPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (
                init_mesh,
                on_mat_change,
                on_image_load,
                //on_pivot_font_size_change,
            )
                .chain()
                .in_set(TerminalRenderSystems)
                .after(TerminalTransformSystems),
        );

        app.add_systems(
            Last,
            (rebuild_verts, tile_mesh_update, reset_terminal_state)
                .chain()
                .in_set(TerminalRenderSystems),
        );
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RebuildVerts;

/// A component that will determine how/if terminal fonts are scaled on each
/// axis when rendered. Defaults to `[1.0, 1.0]` (no scaling).
#[derive(Component)]
pub struct TerminalFontScaling(pub Vec2);

impl Default for TerminalFontScaling {
    fn default() -> Self {
        Self(Vec2::ONE)
    }
}

#[derive(Component)]
pub struct TerminalMeshPivot(pub Pivot);

impl From<Pivot> for TerminalMeshPivot {
    fn from(value: Pivot) -> Self {
        Self(value)
    }
}

impl Default for TerminalMeshPivot {
    fn default() -> Self {
        Self(Pivot::Center)
    }
}

fn init_mesh(
    q_term: Query<&Mesh2dHandle, (Added<Mesh2dHandle>, With<Handle<TerminalMaterial>>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for mesh_handle in &q_term {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_indices(Indices::U32(Vec::new()));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new());
        mesh.insert_attribute(ATTRIBUTE_UV, Vec::<[f32; 2]>::new());
        mesh.insert_attribute(ATTRIBUTE_COLOR_FG, Vec::<[f32; 4]>::new());
        mesh.insert_attribute(ATTRIBUTE_COLOR_BG, Vec::<[f32; 4]>::new());
        meshes.insert(mesh_handle.0.clone(), mesh);
    }
}

fn on_image_load(
    mut q_term: Query<(Entity, &Handle<TerminalMaterial>)>,
    materials: Res<Assets<TerminalMaterial>>,
    mut img_evt: EventReader<AssetEvent<Image>>,
    mut commands: Commands,
) {
    for evt in img_evt.read() {
        let image_id = match evt {
            AssetEvent::LoadedWithDependencies { id } => id,
            _ => continue,
        };
        for (entity, mat_handle) in &mut q_term {
            let mat = materials
                .get(mat_handle.clone())
                .expect("Error getting terminal material");
            if mat
                .texture
                .as_ref()
                .is_some_and(|image| image.id() == *image_id)
            {
                commands.entity(entity).insert(RebuildVerts);
            }
        }
    }
}

fn on_mat_change(
    mut q_term: Query<(Entity, &Handle<TerminalMaterial>)>,
    mut mat_evt: EventReader<AssetEvent<TerminalMaterial>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // images: Res<Assets<Image>>,
    // materials: Res<Assets<TerminalMaterial>>,
    mut commands: Commands,
) {
    for evt in mat_evt.read() {
        let material_id = match evt {
            AssetEvent::Modified { id } => id,
            _ => continue,
        };
        for (entity, mat_handle) in &mut q_term {
            if mat_handle.id() != *material_id {
                continue;
            }

            commands.entity(entity).insert(RebuildVerts);
            // if materials
            //     .get(mat_handle)
            //     .and_then(|mat| mat.texture.as_ref())
            //     .and_then(|image| images.get(image.clone()))
            //     .is_some()
            // {
            //     commands.entity(entity).insert(RebuildVerts);
            //     continue;
            // }

            // let mesh = meshes
            //     .get_mut(mesh_handle.0.clone())
            //     .expect("Error getting terminal mesh");
            // resize_mesh_data(mesh, 0);
        }
    }
}

// fn on_pivot_font_size_change(
//     q_term: Query<(Entity, Ref<TerminalMeshPivot>, Ref<TerminalFontScaling>)>,
//     mut commands: Commands,
// ) {
//     for (entity, pivot, scaling) in &q_term {
//         if pivot.is_changed() || scaling.is_changed() {
//             commands.entity(entity).insert(RebuildVerts);
//         }
//     }
// }

#[allow(clippy::type_complexity)]
fn rebuild_verts(
    mut q_term: Query<
        (
            Entity,
            &mut Terminal,
            &Mesh2dHandle,
            &TerminalTransform,
            &Handle<TerminalMaterial>,
        ),
        Or<(
            Changed<TerminalMeshPivot>,
            Changed<TerminalFontScaling>,
            With<RebuildVerts>,
        )>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
) {
    for (entity, mut term, mesh_handle, transform, mat_handle) in &mut q_term {
        commands.entity(entity).remove::<RebuildVerts>();

        let mesh = meshes
            .get_mut(mesh_handle.0.clone())
            .expect("Error getting terminal mesh");

        let mat = materials
            .get(mat_handle)
            .expect("Error getting terminal material");

        // If the material texture is set to none, or if it's not loaded yet,
        // clear the mesh. This function will be called again when a valid image
        // is loaded
        if mat.texture.is_none() || images.get(mat.texture.as_ref().unwrap()).is_none() {
            resize_mesh_data(mesh, 0);
            continue;
        }

        resize_mesh_data(mesh, term.tile_count());

        let origin = transform.world_bounds().min;
        let tile_size = transform.world_tile_size();

        // We only need to build our vertex data, uvs/colors will be updated
        // in "tile_mesh_update"
        VertMesher::build_mesh_verts(origin, tile_size, mesh, |mesher| {
            for (i, (p, _)) in term.iter_xy().enumerate() {
                mesher.set_tile(p.x, p.y, i);
            }
        });

        // Force tile mesh update
        term.set_changed();

        // let origin = renderer.mesh_origin();
        // let tile_size = renderer.tile_size_world();
        // let origin = transform.world_mesh_bounds().min;
        // let tile_size = transform.world_tile_size();
        // VertMesher::build_mesh_verts(origin, tile_size, mesh, |mesher| {
        //     for (p, _) in term.iter_xy() {
        //         mesher.add_tile(p.x, p.y);
        //     }
        // });
        // UVMesher::build_mesh_tile_data(mapping, mesh, |mesher| {
        //     for t in term.tiles().iter() {
        //         mesher.add_tile(t.glyph, t.fg_color, t.bg_color);
        //     }
        // });
        // if let Some(border) = term.get_border() {
        //     VertMesher::build_mesh_verts(origin, tile_size, mesh, |mesher| {
        //         for (p, _) in border.iter() {
        //             mesher.add_tile(p.x, p.y);
        //         }
        //     });

        //     UVMesher::build_mesh_tile_data(mapping, mesh, |mesher| {
        //         for (_, t) in border.iter() {
        //             mesher.add_tile(t.glyph, t.fg_color, t.bg_color);
        //         }
        //     });
        // }
    }
}

#[allow(clippy::type_complexity)]
fn tile_mesh_update(
    q_term: Query<
        (
            &Terminal,
            &Mesh2dHandle,
            &Handle<UvMapping>,
            &TerminalTransform,
        ),
        Changed<Terminal>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mappings: Res<Assets<UvMapping>>,
) {
    for (term, mesh_handle, mapping, transform) in &q_term {
        let mesh = meshes
            .get_mut(mesh_handle.0.clone())
            .expect("Couldn't find terminal mesh");

        if mesh_vertex_count(mesh) == 0 {
            continue;
        }

        let mapping = mappings
            .get(mapping.clone())
            .expect("Couldn't find terminal uv mapping");

        UvMesher::build_mesh_tile_data(mapping, mesh, |mesher| {
            for (i, t) in term.tiles().iter().enumerate() {
                mesher.set_tile(t.glyph, t.fg_color, t.bg_color, i);
            }
        });

        // if let Some(border) = term.get_border() {
        //     if border.changed() {
        //         resize_mesh_data(mesh, term.tile_count());
        //         let origin = transform.world_mesh_bounds().min;
        //         let tile_size = transform.world_tile_size();
        //         VertMesher::build_mesh_verts(origin, tile_size, mesh, |mesher| {
        //             for (p, _) in border.iter() {
        //                 mesher.add_tile(p.x, p.y);
        //             }
        //         });

        //         UvMesher::build_mesh_tile_data(mapping, mesh, |mesher| {
        //             for (_, t) in border.iter() {
        //                 mesher.add_tile(t.glyph, t.fg_color, t.bg_color);
        //             }
        //         });
        //     }
        // } else if mesh_tile_count(mesh) != term.tile_count() {
        //     // No border - clear border verts
        //     resize_mesh_data(mesh, term.tile_count());
        // }
    }
}

fn reset_terminal_state(mut q_term: Query<&mut Terminal>) {
    for mut term in &mut q_term {
        if let Some(mut border) = term.bypass_change_detection().get_border_mut() {
            border.reset_changed_state();
        }
    }
}

fn mesh_vertex_count(mesh: &Mesh) -> usize {
    let Some(VertexAttributeValues::Float32x3(verts)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    else {
        panic!("Incorrect mesh terminal vertex format");
    };
    verts.len()
}

/// The tile count of the mesh, based on mesh vertices.
fn mesh_tile_count(mesh: &Mesh) -> usize {
    mesh_vertex_count(mesh) / 4
}

/// Resize all mesh attributes to accomodate the given tile count.
fn resize_mesh_data(mesh: &mut Mesh, tile_count: usize) {
    let Some(Indices::U32(indices)) = mesh.indices_mut() else {
        panic!("Incorrect terminal mesh indices format");
    };
    indices.resize(tile_count * 6, 0);
    let Some(VertexAttributeValues::Float32x3(verts)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    else {
        panic!("Incorrect mesh terminal vertex format");
    };
    verts.resize(tile_count * 4, [0.0; 3]);
    let Some(VertexAttributeValues::Float32x2(uvs)) = mesh.attribute_mut(ATTRIBUTE_UV) else {
        panic!("Incorrect terminal mesh uv format");
    };
    uvs.resize(tile_count * 4, [0.0; 2]);
    let Some(VertexAttributeValues::Float32x4(fg)) = mesh.attribute_mut(ATTRIBUTE_COLOR_FG) else {
        panic!("Incorrect terminal mesh fg color format");
    };
    fg.resize(tile_count * 4, [0.0; 4]);
    let Some(VertexAttributeValues::Float32x4(bg)) = mesh.attribute_mut(ATTRIBUTE_COLOR_BG) else {
        panic!("Incorrect terminal mesh bg color format");
    };
    bg.resize(tile_count * 4, [0.0; 4]);
}
