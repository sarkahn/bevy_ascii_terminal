use bevy::{
    ecs::prelude::*,
    prelude::{Assets, Handle, Image, Mesh, UVec2, Vec2},
    render::{
        mesh::{Indices, MeshVertexAttribute},
        render_resource::{PrimitiveTopology, VertexFormat},
    },
    sprite::Mesh2dHandle,
};
use sark_grids::{Size2d, point::Point2d};

use crate::{Terminal, TerminalFont, TerminalMaterial};

use super::{
    tile_data::TileData,
    TileScaling, TerminalLayout, mesh_data::VertexData,
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
    mut q_term: Query<(&Handle<TerminalMaterial>, &mut TerminalLayout), Changed<Handle<TerminalMaterial>>>,
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

    // let font_size = layout.pixels_per_tile.as_vec2();
}

#[allow(clippy::type_complexity)]
fn terminal_resize(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(
            &Terminal,
            &TerminalLayout,
            &Mesh2dHandle,
            &mut VertexData,
            &mut TileData,
        ),
        Or<(
            Changed<Handle<Mesh>>,
            Changed<TerminalLayout>,
            Changed<Handle<TerminalMaterial>>,
        )>,
    >,
) {
    for (
        terminal,
        layout,
        mesh,
        mut verts,
        mut tiles,
    ) in q.iter_mut()
    {
        let tile_size = layout.tile_size;
        let origin = terminal_mesh_origin(
            terminal.size(), layout.term_pivot, tile_size, layout.tile_pivot);

        //vert_data.terminal_resize(origin, terminal.size(), tile_size);
        let len = terminal.size().len();
        let width = terminal.width();

        verts.clear();
        verts.reserve(len);

        let mut helper = super::mesh_data::VertHelper {
            origin: origin.as_vec2(),
            tile_size,
            data: &mut verts,
        };

        for i in 0..len {
            let x = i % width;
            let y = i / width;

            helper.tile_at([x, y]);
        }

        tiles.terminal_resize(terminal.size());

        let mesh = meshes
            .get_mut(&mesh.0)
            .expect("Error retrieving mesh from terminal renderer");

        mesh.set_indices(Some(Indices::U32(verts.indices.clone())));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts.verts.clone());
    }
}

fn push_mesh_data_from_tiles(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(&TileData, &Mesh2dHandle), Changed<TileData>>,
) {
    for (tile_data, mesh) in q.iter_mut() {
        let mesh = meshes
            .get_mut(&mesh.0)
            .expect("Error accessing terminal mesh");
        //info!("writing colors and uvs to mesh. Count {}", tile_data.uvs.len());
        //info!("First fg Colors: {:?}", &tile_data.fg_colors[0..4]);
        //info!("First bg Colors: {:?}", &tile_data.bg_colors[0..4]);
        //info!("First uvs: {:?}", &tile_data.uvs[0..4]);

        mesh.insert_attribute(ATTRIBUTE_COLOR_BG, tile_data.bg_colors.clone());
        mesh.insert_attribute(ATTRIBUTE_COLOR_FG, tile_data.fg_colors.clone());
        mesh.insert_attribute(ATTRIBUTE_UV, tile_data.uvs.clone());
    }
}

fn terminal_mesh_origin(
    term_size: UVec2,
    term_pivot: Vec2,
    tile_size: Vec2,
    tile_pivot: Vec2,
) -> Vec2 {
    let term_size = term_size.as_vec2();
    let term_offset = -(term_size * tile_size * term_pivot);
    let tile_offset = -(tile_size * tile_pivot);
    term_offset + tile_offset
}

fn update_border_mesh() {
    
}