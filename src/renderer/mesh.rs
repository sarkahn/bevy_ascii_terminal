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
    math::{IVec2, Vec2},
    render::{
        color::Color,
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
use crate::{
    direction::Dir4, transform::UpdateTransformPositionSystem, Pivot, Terminal, TerminalTransform,
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
pub struct TerminalMeshSystems;

impl Plugin for TerminalMeshPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (init_mesh, on_mat_change, on_image_load)
                .chain()
                .in_set(TerminalMeshSystems)
                .after(UpdateTransformPositionSystem),
        );

        app.add_systems(
            Last,
            (rebuild_verts, tile_mesh_update, border_mesh_update)
                .chain()
                .in_set(TerminalMeshSystems),
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
                .get(mat_handle.clone())
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
            &Mesh2dHandle,
            &TerminalTransform,
            &Handle<TerminalMaterial>,
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
    for (entity, mut term, mesh_handle, transform, mat_handle) in &mut q_term {
        commands.entity(entity).remove::<RebuildTerminalMeshVerts>();

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

        let border_offset: IVec2 = if let Some(border) = term.get_border() {
            let edges = border.edge_opacity(term.clear_tile(), term.size());
            let x = edges[Dir4::Left.as_index()] as i32;
            let y = edges[Dir4::Down.as_index()] as i32;
            [x, y]
        } else {
            [0, 0]
        }
        .into();

        // We only need to update our vertex data, uvs/colors will be updated
        // in "tile_mesh_update"
        VertMesher::build_mesh_verts(origin, tile_size, mesh, |mesher| {
            for (i, (p, _)) in term.iter_xy().enumerate() {
                let p = p + border_offset;
                mesher.set_tile(p.x, p.y, i);
            }
        });

        // Force tile mesh update
        term.set_changed();
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
            .get_mut(mesh_handle.0.clone())
            .expect("Couldn't find terminal mesh");

        if mesh_vertex_count(mesh) == 0 {
            continue;
        }

        let mapping = mappings
            .get(mapping_handle.clone())
            .expect("Couldn't find terminal uv mapping");

        UvMesher::build_mesh_tile_data(mapping, mesh, |mesher| {
            for (i, t) in term.tiles().iter().enumerate() {
                mesher.set_tile(t.glyph, t.fg_color, t.bg_color, i);
            }
        });
    }
}

fn border_mesh_update(
    mut q_term: Query<
        (
            // Only mutable to reset border state - bypasses change detection
            &mut Terminal,
            &TerminalTransform,
            &Mesh2dHandle,
            &Handle<UvMapping>,
        ),
        Changed<Terminal>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mappings: Res<Assets<UvMapping>>,
) {
    for (mut term, transform, mesh_handle, mapping_handle) in &mut q_term {
        let Some(border) = term.get_border() else {
            continue;
        };
        if !border.changed() {
            continue;
        };

        let mesh = meshes
            .get_mut(mesh_handle.0.clone())
            .expect("Error getting terminal mesh");

        let vert_count = mesh_vertex_count(mesh);
        if vert_count == 0 {
            continue;
        }
        if vert_count == term.tile_count() * 4 {
            resize_mesh_data(mesh, term.tile_count() + border.tile_count());
        }

        let mapping = mappings
            .get(mapping_handle.clone())
            .expect("Couldn't find terminal uv mapping");

        let origin = transform.world_bounds().min;
        let tile_size = transform.world_tile_size();

        // Our mesh needs to be offset by one tile if the left or bottom sides
        // of the border are not blank.
        let edges = border.edge_opacity(term.clear_tile(), term.size());
        let x = edges[Dir4::Left.as_index()] as i32;
        let y = edges[Dir4::Down.as_index()] as i32;
        let border_offset = IVec2::new(x, y);

        VertMesher::build_mesh_verts(origin, tile_size, mesh, |mesher| {
            for (i, (p, _)) in border.iter().enumerate() {
                let i = i + term.tile_count();
                // Border offset
                let p = p + border_offset;
                mesher.set_tile(p.x, p.y, i);
            }
        });
        UvMesher::build_mesh_tile_data(mapping, mesh, |mesher| {
            for (i, (_, t)) in border.iter().enumerate() {
                let i = i + term.tile_count();
                let transparent = Color::rgba_u8(0, 0, 0, 0);
                let (fg, bg) = if t.glyph == ' ' {
                    (transparent, transparent)
                } else {
                    (t.fg_color, t.bg_color)
                };
                mesher.set_tile(t.glyph, fg, bg, i);
            }
        });

        term.bypass_change_detection()
            .border_mut()
            .reset_changed_state();
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
