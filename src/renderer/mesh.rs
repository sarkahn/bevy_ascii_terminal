use bevy::{
    ecs::prelude::*,
    math::vec3,
    prelude::{Assets, BuildChildren, Children, Handle, Image, Mesh, UVec2, Vec2, Vec3},
    render::{
        mesh::{Indices, MeshVertexAttribute},
        render_resource::{PrimitiveTopology, VertexFormat},
    },
    sprite::Mesh2dHandle,
};
use sark_grids::Size2d;

use crate::{Terminal, TerminalMaterial};

use super::{
    border::TerminalBorder, mesh_data::VertexData, tile_data::TileData, TerminalLayout, TileScaling,
};

pub const ATTRIBUTE_UV: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Uv", 1, VertexFormat::Float32x2);
pub const ATTRIBUTE_COLOR_BG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Bg", 2, VertexFormat::Float32x4);
pub const ATTRIBUTE_COLOR_FG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Fg", 3, VertexFormat::Float32x4);

#[allow(clippy::type_complexity)]
fn init_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<&mut Mesh2dHandle, (Added<Mesh2dHandle>, With<VertexData>)>,
) {
    for mut mesh in q.iter_mut() {
        //info!("Initializing terminal mesh");
        let new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        *mesh = Mesh2dHandle(meshes.add(new_mesh));
    }
}

fn material_change(
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
    mut q_term: Query<
        (&Handle<TerminalMaterial>, &mut TerminalLayout),
        Changed<Handle<TerminalMaterial>>,
    >,
) {
    for (handle, mut layout) in &mut q_term {
        if let Some(material) = materials.get(handle) {
            if let Some(image) = material.texture.clone() {
                if let Some(image) = images.get(&image) {
                    // TODO: Should be derived from image size, can't assume 16x16 tilesheet for
                    // graphical terminals
                    let font_size = image.size() / 16.0;
                    layout.pixels_per_tile = font_size.as_uvec2();
                }
            }
        }
    }
}

fn update_layout(mut q_term: Query<(&Terminal, &mut TerminalLayout), Changed<Terminal>>) {
    for (term, mut layout) in &mut q_term {
        if layout.term_size() != term.size() || layout.has_border() != term.border.is_some() {
            layout.update_state(term);
        }
    }
}

#[allow(clippy::type_complexity)]
fn resize_terminal_mesh_data(
    mut q: Query<(&Terminal, &TerminalLayout, &mut VertexData), Changed<TerminalLayout>>,
) {
    for (term, layout, mut vert_data) in &mut q {
        let len = term.size().len();
        vert_data.clear();
        vert_data.reserve(len);

        let tile_size = layout.tile_size;
        let origin = layout.origin();
        let right = Vec3::X * tile_size.x;
        let up = Vec3::Y * tile_size.y;

        for i in 0..len {
            let x = (i % term.width()) as f32;
            let y = (i / term.width()) as f32;

            let xy = vec3(x, y, 0.0) * tile_size.extend(0.0);
            let p = origin.extend(0.0) + xy;

            let vi = vert_data.verts.len() as u32;

            vert_data
                .verts
                .extend(&[p + up, p, p + right + up, p + right].map(|p| p.to_array()));
            vert_data
                .indices
                .extend(&[vi + 0, vi + 1, vi + 2, vi + 3, vi + 2, vi + 1]);
        }
    }
}

fn update_border(
    q_term: Query<(Entity, &Terminal, &TerminalLayout, &Children), Changed<TerminalLayout>>,
    mut q_border: Query<(&mut VertexData, &mut TileData), With<TerminalBorder>>,
    mut commands: Commands,
) {
    for (term_entity, term, layout, children) in &q_term {
        let border_entity = children.iter().find(|e| q_border.get(**e).is_ok());
        if let Some(border_entity) = border_entity {
            if !term.has_border() {
                // Remove border if it exists
                commands
                    .entity(term_entity)
                    .remove_children(&[*border_entity]);
                commands.entity(*border_entity).despawn();
                continue;
            }

            let (mut vert_data, mut tile_data) = q_border.get_mut(*border_entity).unwrap();

            vert_data.clear();
            tile_data.clear();
        }
    }
}

fn border_resize(
    q_term: Query<(&TerminalLayout, &Children), Changed<TerminalLayout>>,
    mut q_border: Query<(&TerminalBorder, &mut VertexData)>,
) {
    for (layout, children) in &q_term {
        for child in children {
            if let Ok((border, mut vert_data)) = q_border.get_mut(*child) {
                let size = layout.term_size();
                let width = size.width() + 2;
                let height = size.height() + 2;
                let len = (width * 2) + ((height - 2) * 2);

                vert_data.verts.clear();
                vert_data.verts.reserve(len);

                vert_data.indices.clear();
                vert_data.indices.reserve(len);

                let origin = layout.origin().extend(0.0);
                let right = Vec3::X * layout.tile_size.x;
                let up = Vec3::Y * layout.tile_size.y;

                let mut tile_at = |x: usize, y: usize| {
                    let xy = vec3(x as f32, y as f32, 0.0);
                    let p = origin + xy;

                    let vi = vert_data.verts.len() as u32;
                    vert_data
                        .verts
                        .extend(&[p + up, p, p + right + up, p + right].map(|p| p.to_array()));
                    vert_data
                        .indices
                        .extend(&[vi + 0, vi + 1, vi + 2, vi + 3, vi + 2, vi + 1]);
                };

                let top = height - 1;
                let bottom = 0;
                let left = 0;
                let right = width - 1;

                tile_at(left, bottom);
                tile_at(left, top);
                tile_at(right, top);
                tile_at(right, bottom);

                for x in 1..width - 1 {
                    tile_at(x, bottom);
                    tile_at(x, top);
                }

                for y in 1..height - 1 {
                    tile_at(left, y);
                    tile_at(right, y);
                }
            }
        }
    }
}

fn push_mesh_vert_changes(
    q_mesh: Query<(&VertexData, &Mesh2dHandle), Changed<VertexData>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (vert_data, handle) in &q_mesh {
        let mesh = meshes
            .get_mut(&handle.0)
            .expect("Error retrieving mesh from terminal renderer");

        mesh.set_indices(Some(Indices::U32(vert_data.indices.clone())));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vert_data.verts.clone());
    }
}

fn push_mesh_tile_changes(
    mut q_mesh: Query<(&TileData, &Mesh2dHandle), Changed<TileData>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (tile_data, mesh) in q_mesh.iter_mut() {
        let mesh = meshes
            .get_mut(&mesh.0)
            .expect("Error accessing terminal mesh");

        mesh.insert_attribute(ATTRIBUTE_COLOR_BG, tile_data.bg_colors.clone());
        mesh.insert_attribute(ATTRIBUTE_COLOR_FG, tile_data.fg_colors.clone());
        mesh.insert_attribute(ATTRIBUTE_UV, tile_data.uvs.clone());
    }
}
