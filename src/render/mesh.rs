//! Systems for building the terminal mesh.

use bevy::{
    app::{Plugin, PostUpdate},
    asset::{AssetEvent, Assets, RenderAssetUsages},
    color::ColorToComponents,
    ecs::{
        change_detection::DetectChangesMut,
        component::Component,
        entity::Entity,
        message::{MessageReader, MessageWriter},
        query::{Added, Changed, Or, With},
        schedule::{IntoScheduleConfigs, SystemSet},
        system::{Commands, Query, Res, ResMut},
    },
    image::Image,
    math::{IVec2, Vec2},
    mesh::{Indices, Mesh, MeshVertexAttribute, VertexAttributeValues},
    prelude::{Mesh2d, On, Replace},
    render::render_resource::{PrimitiveTopology, VertexFormat},
    sprite_render::MeshMaterial2d,
};

use crate::{Terminal, Tile, border::TerminalBorder, transform::TerminalTransform};

use super::{
    UpdateTerminalViewportEvent,
    material::TerminalMaterial,
    uv_mapping::{UvMapping, UvMappingHandle},
};

pub const ATTRIBUTE_UV: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Uv", 1123131, VertexFormat::Float32x2);
pub const ATTRIBUTE_COLOR_BG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Bg", 1123132, VertexFormat::Float32x4);
pub const ATTRIBUTE_COLOR_FG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Fg", 1123133, VertexFormat::Float32x4);

pub struct TerminalMeshPlugin;

impl Plugin for TerminalMeshPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_observer(on_border_removed);
        app.add_systems(
            PostUpdate,
            (
                init_mesh,
                on_image_load,
                on_material_changed,
                on_terminal_resized,
                rebuild_mesh_verts,
                rebuild_mesh_uvs,
            )
                .chain()
                .in_set(TerminalSystemsUpdateMesh),
        );
    }
}

/// Systems for rebuilding/updating the terminal mesh. Runs in [PostUpdate].
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalSystemsUpdateMesh;

/// A sparse set component to force the mesh vertices to be rebuilt when added to a terminal.
#[derive(Component, Default)]
#[component(storage = "SparseSet")]
pub struct RebuildMeshVerts;

/// Component for the terminal which determines where terminal mesh tiles
/// are built relative to the terminal's transform position.
///
/// Two terminals with the same position and a different [TerminalMeshPivot] will
/// not overlap.
///
/// Defaults to bottom left.
#[derive(Component, Default)]
pub enum TerminalMeshPivot {
    TopLeft,
    TopCenter,
    TopRight,
    LeftCenter,
    Center,
    RightCenter,
    #[default]
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl TerminalMeshPivot {
    /// Returns the pivot normalized in the 0..1 range where 0 is the bottom/left
    /// and 1 is the top/right.
    pub fn normalized(&self) -> Vec2 {
        match self {
            Self::TopLeft => [0., 1.],
            Self::TopCenter => [0.5, 1.],
            Self::TopRight => [1., 1.],
            Self::LeftCenter => [0., 0.5],
            Self::Center => [0.5, 0.5],
            Self::RightCenter => [1., 0.5],
            Self::BottomLeft => [0., 0.],
            Self::BottomCenter => [0.5, 0.],
            Self::BottomRight => [1., 0.],
        }
        .into()
    }
}

/// An optional component to scale terminal tiles after [crate::TerminalMeshWorldScaling] is
/// applied.
#[derive(Component)]
pub struct TerminalMeshTileScaling(pub Vec2);

impl Default for TerminalMeshTileScaling {
    fn default() -> Self {
        Self(Vec2::ONE)
    }
}

fn init_mesh(
    mut q_term: Query<&mut Mesh2d, (Added<Mesh2d>, With<MeshMaterial2d<TerminalMaterial>>)>,
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

// Force a mesh rebuild when a terminal's font finishes loading.
fn on_image_load(
    mut q_term: Query<(Entity, &MeshMaterial2d<TerminalMaterial>)>,
    materials: Res<Assets<TerminalMaterial>>,
    mut img_evt: MessageReader<AssetEvent<Image>>,
    mut commands: Commands,
) {
    for evt in img_evt.read() {
        let image_id = match evt {
            AssetEvent::LoadedWithDependencies { id } => id,
            _ => continue,
        };
        for (entity, mat_handle) in &mut q_term {
            let mat = materials
                .get(&*mat_handle.clone())
                .expect("Error getting terminal material");
            if mat
                .texture
                .as_ref()
                .is_some_and(|image| image.id() == *image_id)
            {
                commands.entity(entity).insert(RebuildMeshVerts);
            }
        }
    }
}

// Force a mesh rebuild when a terminal's material changes.
fn on_material_changed(
    mut q_term: Query<(Entity, &MeshMaterial2d<TerminalMaterial>)>,
    mut mat_evt: MessageReader<AssetEvent<TerminalMaterial>>,
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

            commands.entity(entity).insert(RebuildMeshVerts);
        }
    }
}

fn on_terminal_resized(
    q_term: Query<(Entity, &Terminal, &Mesh2d, Option<&TerminalBorder>), Changed<Terminal>>,
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
) {
    for (e, term, mesh, border) in &q_term {
        let tile_count = term.tile_count() + border.as_ref().map_or(0, |b| b.tiles().len());
        let mesh = meshes.get(mesh).expect("Couldn't find terminal mesh");
        if mesh_vertex_count(mesh) == tile_count * 4 {
            continue;
        }
        commands.entity(e).insert(RebuildMeshVerts);
    }
}

fn on_border_removed(trigger: On<Replace, TerminalBorder>, mut commands: Commands) {
    commands.entity(trigger.entity).insert(RebuildMeshVerts);
}

// Rebuilding mesh verts is a more expensive and complicated operation compared
// to updating uvs and colors. Generally it only needs to be done when terminal
// assets are changed or a terminal is resized.
#[allow(clippy::type_complexity)]
fn rebuild_mesh_verts(
    mut q_term: Query<
        (
            Entity,
            &mut Terminal,
            &Mesh2d,
            &MeshMaterial2d<TerminalMaterial>,
            &TerminalTransform,
            Option<&mut TerminalBorder>,
        ),
        Or<(
            Changed<TerminalMeshPivot>,
            Changed<TerminalMeshTileScaling>,
            Changed<TerminalBorder>,
            With<RebuildMeshVerts>,
        )>,
    >,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
    mut evt: MessageWriter<UpdateTerminalViewportEvent>,
) {
    for (entity, mut term, mesh_handle, mat_handle, transform, mut border) in &mut q_term {
        let Some(mesh) = meshes.get_mut(&mesh_handle.0.clone()) else {
            continue;
        };

        let Some(mat) = materials.get(&*mat_handle.clone()) else {
            continue;
        };

        // If the material texture is set to none, or if it's not loaded yet,
        // clear the mesh. This function will be called again when a valid image
        // is loaded
        if mat.texture.is_none() || images.get(mat.texture.as_ref().unwrap()).is_none() {
            resize_mesh_data(mesh, 0);
            continue;
        }

        let Some(transform_data) = &transform.cached_data else {
            // Transform has not yet been updated.
            continue;
        };

        if let Some(border) = border.as_mut() {
            border.rebuild(term.size(), term.clear_tile());
        }

        let tile_count = term.tile_count();
        let border_tile_count = border.as_ref().map_or(0, |b| b.tiles().len());

        resize_mesh_data(mesh, tile_count + border_tile_count);

        let tile_size = transform_data.world_tile_size;
        let mesh_bl = transform_data.local_inner_mesh_bounds.min;

        let Some(Indices::U32(mut indices)) = mesh.remove_indices() else {
            panic!("Incorrect terminal mesh indices format");
        };
        let Some(VertexAttributeValues::Float32x3(mut verts)) =
            mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("Incorrect mesh terminal vertex format");
        };

        let right = (Vec2::X * tile_size).extend(0.0);
        let up = (Vec2::Y * tile_size).extend(0.0);
        let mut set_tile_verts = |xy: IVec2, mesh_tile_index: usize| {
            let xy = (mesh_bl + xy.as_vec2() * tile_size).extend(0.0);
            let i = mesh_tile_index * 4;
            verts[i] = (xy + up).into();
            verts[i + 1] = xy.into();
            verts[i + 2] = (xy + right + up).into();
            verts[i + 3] = (xy + right).into();

            let vi = i as u32;
            let i = mesh_tile_index * 6;
            indices[i] = vi;
            indices[i + 1] = vi + 1;
            indices[i + 2] = vi + 2;
            indices[i + 3] = vi + 3;
            indices[i + 4] = vi + 2;
            indices[i + 5] = vi + 1;
        };

        for (i, (xy, _)) in term.iter_xy().enumerate() {
            set_tile_verts(xy, i);
        }

        if let Some(tiles) = border.as_ref().map(|b| b.tiles()) {
            let mesh_index = tile_count;
            for (i, (p, _)) in tiles.iter().enumerate() {
                set_tile_verts(*p, mesh_index + i);
            }
        }

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
        mesh.insert_indices(Indices::U32(indices));

        commands.entity(entity).remove::<RebuildMeshVerts>();
        // Force tile mesh update
        term.set_changed();
        evt.write(UpdateTerminalViewportEvent);
    }
}

// Update tile uv and color data. This is called any time the terminal is
// modified in any way.
#[allow(clippy::type_complexity)]
fn rebuild_mesh_uvs(
    q_term: Query<
        (
            &Terminal,
            &Mesh2d,
            &UvMappingHandle,
            Option<&TerminalBorder>,
        ),
        Changed<Terminal>,
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mappings: Res<Assets<UvMapping>>,
) {
    for (term, mesh_handle, mapping_handle, border) in &q_term {
        let mesh = meshes
            .get_mut(&mesh_handle.0.clone())
            .expect("Couldn't find terminal mesh");

        // Mesh vertices not yet updated, this function will be called again
        // once the vertex update is completed.
        if mesh_vertex_count(mesh) == 0 {
            continue;
        }

        let mapping = mappings
            .get(&*mapping_handle.clone())
            .expect("Couldn't find terminal uv mapping");

        // Remove all our relevant attributes from the mesh. This is done
        // to prevent the borrow checker from complaining when trying to
        // modify multiple mesh attributes at the same time.
        let Some(VertexAttributeValues::Float32x2(mut uvs)) = mesh.remove_attribute(ATTRIBUTE_UV)
        else {
            panic!("Incorrect terminal mesh uv format");
        };
        let Some(VertexAttributeValues::Float32x4(mut fg)) =
            mesh.remove_attribute(ATTRIBUTE_COLOR_FG)
        else {
            panic!("Incorrect terminal mesh fg color format");
        };
        let Some(VertexAttributeValues::Float32x4(mut bg)) =
            mesh.remove_attribute(ATTRIBUTE_COLOR_BG)
        else {
            panic!("Incorrect terminal mesh bg color format");
        };

        let mut set_tile_uvs = |t: &Tile, tile_index: usize| {
            let i = tile_index * 4;
            let map_uvs = mapping.uvs_from_char(t.glyph);
            for (map_index, i) in (i..i + 4).enumerate() {
                uvs[i] = map_uvs[map_index];
                fg[i] = t.fg_color.to_f32_array();
                bg[i] = t.bg_color.to_f32_array();
            }
        };

        for (i, t) in term.iter().enumerate() {
            set_tile_uvs(t, i);
        }

        if let Some(tiles) = border.map(|b| b.tiles()) {
            let mesh_index = term.tile_count();
            for (i, (_, t)) in tiles.iter().enumerate() {
                set_tile_uvs(t, mesh_index + i);
            }
        }

        mesh.insert_attribute(ATTRIBUTE_UV, uvs);
        mesh.insert_attribute(ATTRIBUTE_COLOR_FG, fg);
        mesh.insert_attribute(ATTRIBUTE_COLOR_BG, bg);

        //println!("Rebuilding uvs: {}\n", time.elapsed_secs());
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
