//! Plugin for rendering related resources and systems.
use bevy::{
    ecs::prelude::*,
    math::Vec2,
    prelude::{App, Assets, Handle, Image, Mesh, Plugin},
    render::{
        mesh::{Indices, MeshVertexAttribute},
        render_resource::{PrimitiveTopology, VertexFormat},
    },
    sprite::Mesh2dHandle,
};

use crate::renderer::font::{BuiltInFontHandles, TerminalFont};

use super::{
    camera::TerminalCameraPlugin, material::TerminalMaterialPlugin, uv_mapping::UvMapping, *,
};

pub(crate) struct TerminalRendererPlugin;

pub const ATTRIBUTE_UV: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Uv", 1, VertexFormat::Float32x2);
pub const ATTRIBUTE_COLOR_BG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Bg", 2, VertexFormat::Float32x4);
pub const ATTRIBUTE_COLOR_FG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Fg", 3, VertexFormat::Float32x4);

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TerminalMaterialPlugin);
        app.add_plugin(TerminalCameraPlugin);

        app.add_system(terminal_renderer_init.label(TERMINAL_INIT))
            .add_system(
                terminal_renderer_change_font
                    .after(TERMINAL_INIT)
                    .label(TERMINAL_CHANGE_FONT),
            )
            .add_system(
                terminal_renderer_update_size
                    .after(TERMINAL_CHANGE_FONT)
                    .label(TERMINAL_UPDATE_SIZE),
            )
            .add_system(
                terminal_renderer_update_tile_data
                    .after(TERMINAL_UPDATE_SIZE)
                    .label(TERMINAL_UPDATE_TILE_DATA),
            )
            .add_system(
                terminal_renderer_update_mesh
                    .after(TERMINAL_UPDATE_TILE_DATA)
                    .label(TERMINAL_UPDATE_MESH),
            );
    }
}

#[allow(clippy::type_complexity)]
fn terminal_renderer_init(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<&mut Mesh2dHandle, (Added<Mesh2dHandle>, With<TerminalRendererVertexData>)>,
) {
    for mut mesh in q.iter_mut() {
        //info!("Initializing ascii terminal mesh");
        let new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        *mesh = Mesh2dHandle(meshes.add(new_mesh));
    }
}

#[allow(clippy::type_complexity)]
fn terminal_renderer_update_size(
    mut meshes: ResMut<Assets<Mesh>>,
    images: Res<Assets<Image>>,
    materials: Res<Assets<TerminalMaterial>>,
    mut q: Query<
        (
            &Terminal,
            &Handle<TerminalMaterial>,
            &TileScaling,
            &TerminalPivot,
            &TilePivot,
            &mut Mesh2dHandle,
            &mut TerminalRendererVertexData,
            &mut TerminalRendererTileData,
            &mut PixelsPerTile,
        ),
        Or<(
            Changed<Handle<Mesh>>,
            Changed<TileScaling>,
            Changed<Handle<TerminalMaterial>>,
            Added<TerminalFont>,
        )>,
    >,
) {
    for (
        terminal,
        material,
        scaling,
        term_pivot,
        tile_pivot,
        mesh,
        mut vert_data,
        mut tile_data,
        mut ppt,
    ) in q.iter_mut()
    {
        let material = materials.get(material).unwrap();
        let image = images.get(material.texture.as_ref().unwrap()).unwrap();
        // TODO: Should be derived from image size, can't assume 16x16 tilesheet for
        // graphical terminals
        let font_size = image.size() / 16.0;
        ppt.0 = font_size.as_uvec2();

        let tile_size = match scaling {
            TileScaling::World => {
                let aspect = font_size.x / font_size.y;
                Vec2::new(aspect, 1.0)
            }
            TileScaling::Pixels => font_size,
        };

        let size = terminal.size();
        vert_data.resize(size, term_pivot.0, tile_pivot.0, tile_size);
        tile_data.resize(size);

        let mesh = meshes
            .get_mut(&mesh.0)
            .expect("Error retrieving mesh from terminal renderer");

        //info!("Changing mesh size size: {}, Length: {}", size, vert_data.indices.len());
        //info!("First 4 verts: {:?}", &vert_data.verts[0..4]);
        //info!("First 6 indices: {:?}", &vert_data.indices[0..6]);
        mesh.set_indices(Some(Indices::U32(vert_data.indices.clone())));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vert_data.verts.clone());
    }
}

fn terminal_renderer_update_tile_data(
    mut q: Query<(&Terminal, &mut TerminalRendererTileData, &UvMapping), Changed<Terminal>>,
) {
    for (term, mut data, uv_mapping) in q.iter_mut() {
        //info!("Renderer update tile data (colors)!");
        //info!("First tiles: {:?}", &term.tiles[0..4]);
        data.update_from_tiles(term.iter(), uv_mapping);
    }
}

fn terminal_renderer_update_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(&TerminalRendererTileData, &Mesh2dHandle), Changed<TerminalRendererTileData>>,
) {
    for (tile_data, mesh) in q.iter_mut() {
        let mesh = meshes
            .get_mut(&mesh.0)
            .expect("Error accessing terminal mesh");
        //info!("writing colors and uvs to mesh");
        //info!("First fg Colors: {:?}", &tile_data.fg_colors[0..4]);
        //info!("First bg Colors: {:?}", &tile_data.bg_colors[0..4]);
        //info!("First uvs: {:?}", &tile_data.uvs[0..4]);

        //mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, tile_data.fg_colors.clone());

        mesh.insert_attribute(ATTRIBUTE_COLOR_BG, tile_data.bg_colors.clone());
        mesh.insert_attribute(ATTRIBUTE_COLOR_FG, tile_data.fg_colors.clone());
        mesh.insert_attribute(ATTRIBUTE_UV, tile_data.uvs.clone());
    }
}

fn terminal_renderer_change_font(
    built_in_fonts: Res<BuiltInFontHandles>,
    mut q_change: Query<
        (Entity, &mut Handle<TerminalMaterial>, &TerminalFont),
        Changed<TerminalFont>,
    >,
    mut materials: ResMut<Assets<TerminalMaterial>>,
    mut commands: Commands,
) {
    for (e, mut mat, font) in q_change.iter_mut() {
        let handle = match font {
            TerminalFont::Custom(handle) => handle,
            _ => built_in_fonts.get(font),
        };

        println!("Changing font to {}", font.as_ref());
        *mat = materials.add(handle.clone().into());
        //mat.texture = Some(handle.clone());
        commands.entity(e).remove::<TerminalFont>();
    }
}
