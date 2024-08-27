use bevy::{
    app::{Last, Plugin},
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
    prelude::Without,
    render::{
        mesh::{Indices, Mesh, MeshVertexAttribute, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        render_resource::{PrimitiveTopology, VertexFormat},
        texture::Image,
    },
    sprite::Mesh2dHandle,
};

use super::{
    material::TerminalMaterial,
    mesher::{UvMesher, VertMesher},
    uv_mapping::UvMapping,
};
use crate::{GridPoint, Pivot, Terminal, TerminalBorder, TerminalTransform};

pub const ATTRIBUTE_UV: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Uv", 1123131, VertexFormat::Float32x2);
pub const ATTRIBUTE_COLOR_BG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Bg", 1123132, VertexFormat::Float32x4);
pub const ATTRIBUTE_COLOR_FG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Fg", 1123133, VertexFormat::Float32x4);

pub struct TerminalMeshPlugin;

/// Systems for rebuilding/updating the terminal mesh. Runs in [Last].
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalSystemMeshRebuild;

impl Plugin for TerminalMeshPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Last,
            (
                init_mesh,
                on_mat_change,
                on_image_load,
                rebuild_verts,
                tile_mesh_update,
            )
                .chain()
                .in_set(TerminalSystemMeshRebuild),
        );
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RebuildTerminalMeshVerts;

/// A component that will determine how/if terminal fonts are scaled on each
/// axis when rendered. Defaults to `[1.0, 1.0]` (no scaling).
#[derive(Component)]
pub struct TerminalFontScaling(pub Vec2);

impl Default for TerminalFontScaling {
    fn default() -> Self {
        Self(Vec2::ONE)
    }
}

/// Pivot applied to the terminal mesh. This only affects how the terminal is
/// positioned in world space.
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
    mut q_term: Query<&mut Mesh2dHandle, (Added<Mesh2dHandle>, With<Handle<TerminalMaterial>>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for mut mesh_handle in &mut q_term {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_indices(Indices::U32(Vec::new()));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new());
        mesh.insert_attribute(ATTRIBUTE_UV, Vec::<[f32; 2]>::new());
        mesh.insert_attribute(ATTRIBUTE_COLOR_FG, Vec::<[f32; 4]>::new());
        mesh.insert_attribute(ATTRIBUTE_COLOR_BG, Vec::<[f32; 4]>::new());
        mesh_handle.0 = meshes.add(mesh);
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
                .get(&mat_handle.clone())
                .expect("Error getting terminal material");
            if mat
                .texture
                .as_ref()
                .is_some_and(|image| image.id() == *image_id)
            {
                commands.entity(entity).insert(RebuildTerminalMeshVerts);
            }
        }
    }
}

fn on_mat_change(
    mut q_term: Query<(Entity, &Handle<TerminalMaterial>)>,
    mut mat_evt: EventReader<AssetEvent<TerminalMaterial>>,
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

            commands.entity(entity).insert(RebuildTerminalMeshVerts);
        }
    }
}

// Updating verts is a more expensive and complicated operation, and only needs
// to be done rarely, hence the seperation from uv/color updates
#[allow(clippy::type_complexity)]
fn rebuild_verts(
    mut q_term: Query<
        (
            Entity,
            &mut Terminal,
            Option<&TerminalBorder>,
            &Mesh2dHandle,
            &TerminalTransform,
            &Handle<TerminalMaterial>,
            &TerminalMeshPivot,
        ),
        Or<(
            Changed<TerminalMeshPivot>,
            Changed<TerminalFontScaling>,
            With<RebuildTerminalMeshVerts>,
        )>,
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
) {
    for (entity, mut term, border, mesh_handle, transform, mat_handle, pivot) in &mut q_term {
        let mesh = meshes
            .get_mut(&mesh_handle.0.clone())
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

        let Some(transform_data) = transform.transform_data() else {
            // Transform has not yet been updated.
            continue;
        };
        commands.entity(entity).remove::<RebuildTerminalMeshVerts>();
        let origin = transform_data.local_mesh_bounds.min;
        let tile_size = transform_data.world_tile_size;

        // Adjust the mesh position based on the the terminal border and
        // the mesh pivot.
        let border_offset = if let Some(border) = border {
            let left = border.has_left_side() as i32;
            let right = border.has_right_side() as i32;
            let top = border.has_top_side() as i32;
            let bottom = border.has_bottom_side() as i32;
            match pivot.0 {
                Pivot::TopLeft => [left, -top],
                Pivot::TopCenter => [0, -top],
                Pivot::TopRight => [-right, -top],
                Pivot::LeftCenter => [left, 0],
                Pivot::RightCenter => [-right, 0],
                Pivot::BottomLeft => [left, bottom],
                Pivot::BottomCenter => [0, bottom],
                Pivot::BottomRight => [-right, bottom],
                Pivot::Center => [0, 0],
            }
        } else {
            [0, 0]
        }
        .as_ivec2();

        // We're only updating vertex data, uvs/colors will be updated in
        // "tile_mesh_update"
        VertMesher::build_mesh_verts(origin, tile_size, mesh, |mesher| {
            for (i, (p, _)) in term.iter_xy().enumerate() {
                let p = p + border_offset;
                mesher.set_tile_verts(p, i);
            }
        });

        // Force tile mesh update
        term.set_changed();
    }
}

#[allow(clippy::type_complexity)]
fn rebuild_verts_border(
    q_term: Query<
        (
            Entity,
            &TerminalBorder,
            Option<&Terminal>,
            &Mesh2dHandle,
            &TerminalTransform,
            &Handle<TerminalMaterial>,
            &TerminalMeshPivot,
        ),
        Or<(
            Changed<TerminalMeshPivot>,
            Changed<TerminalFontScaling>,
            With<RebuildTerminalMeshVerts>,
        )>,
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
) {
    for (entity, border, term, mesh_handle, transform, mat_handle, pivot) in &q_term {
        // let mesh = meshes
        //     .get_mut(&mesh_handle.0.clone())
        //     .expect("Error getting terminal mesh");

        // let mat = materials
        //     .get(mat_handle)
        //     .expect("Error getting terminal material");

        // // If the material texture is set to none, or if it's not loaded yet,
        // // clear the mesh. This function will be called again when a valid image
        // // is loaded
        // if mat.texture.is_none() || images.get(mat.texture.as_ref().unwrap()).is_none() {
        //     resize_mesh_data(mesh, 0);
        //     continue;
        // }

        // let mut mesh_index = term.map_or(0, |t| t.tile_count());
        // let tile_count = mesh_index + border.tile_count();
        // resize_mesh_data(mesh, tile_count);

        // let Some(transform_data) = transform.transform_data() else {
        //     // Transform has not yet been updated
        //     continue;
        // };
        // commands.entity(entity).remove::<RebuildTerminalMeshVerts>();

        // // The bottom-left position of the terminal in world space.
        // let origin = transform.world_mesh_bounds().min;
        // let tile_size = transform.world_tile_size();

        // let area = border.bounds();
        // VertMesher::build_mesh_verts(origin, tile_size, mesh, |mesher| {
        //     if border.bottom_left_glyph().is_some() {
        //         mesher.set_tile_verts(area.bottom_left(), mesh_index);
        //         mesh_index += 1;
        //     }
        //     if border.top_left_glyph().is_some() {
        //         mesher.set_tile_verts(area.top_left(), mesh_index);
        //         mesh_index += 1;
        //     }
        //     if border.top_right_glyph().is_some() {
        //         mesher.set_tile_verts(area.top_right(), mesh_index);
        //         mesh_index += 1;
        //     }
        //     if border.bottom_right_glyph().is_some() {
        //         mesher.set_tile_verts(area.bottom_right(), mesh_index);
        //         mesh_index += 1;
        //     }

        //     // for _ in 0..border.pivot_tile_count(Pivot::LeftCenter) {
        //     //     for p in area.iter_column(0).skip(1).take(area.height() - 2) {
        //     //         mesher.set_tile_verts(p, mesh_index);
        //     //         mesh_index += 1;
        //     //     }
        //     // }
        // });

        // //if let Some(border) = term.border() {}

        // // Force tile mesh update
        // //term.set_changed();
    }
}

// Update tile uv and color data - we expect this to be called nearly every frame
#[allow(clippy::type_complexity)]
fn tile_mesh_update(
    q_term: Query<(&Terminal, &Mesh2dHandle, &Handle<UvMapping>), Changed<Terminal>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mappings: Res<Assets<UvMapping>>,
) {
    for (term, mesh_handle, mapping_handle) in &q_term {
        let mesh = meshes
            .get_mut(&mesh_handle.0.clone())
            .expect("Couldn't find terminal mesh");

        // Mesh vertices not yet updated - this function will be called again
        // once the vertex update is completed.
        if mesh_vertex_count(mesh) == 0 {
            continue;
        }

        let mapping = mappings
            .get(&mapping_handle.clone())
            .expect("Couldn't find terminal uv mapping");

        UvMesher::build_mesh_tile_data(mapping, mesh, |mesher| {
            for (i, t) in term.tiles().iter().enumerate() {
                mesher.set_tile_data(t.glyph, t.fg_color, t.bg_color, i);
            }
        });
    }
}

fn mesh_vertex_count(mesh: &Mesh) -> usize {
    let Some(VertexAttributeValues::Float32x3(verts)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
    else {
        panic!("Incorrect mesh terminal vertex format");
    };
    verts.len()
}

/// Resize all mesh attributes to accommodate the given terminal tile count.
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
