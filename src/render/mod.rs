pub mod entity;

pub mod font;
mod glyph_mapping;
pub mod plugin;
pub(crate) mod renderer_tile_data;
pub(crate) mod renderer_vertex_data;

use self::{
    font::TerminalFonts, renderer_tile_data::TerminalRendererTileData,
    renderer_vertex_data::TerminalRendererVertexData,
};
use crate::terminal::{Terminal, TerminalSize};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::Indices, pipeline::PrimitiveTopology, renderer::RenderResources, shader::ShaderDefs,
    },
};

pub struct TerminalPivot(pub Vec2);
impl Default for TerminalPivot {
    fn default() -> Self {
        Self(Vec2::new(0.5, 0.5))
    }
}

#[derive(Default)]
pub struct TilePivot(Vec2);

pub struct TerminalRendererFont {
    pub font_name: String,
    pub clip_color: Color,
}
impl Default for TerminalRendererFont {
    fn default() -> Self {
        Self {
            font_name: String::from(font::DEFAULT_FONT.name),
            clip_color: Color::BLACK,
        }
    }
}

#[derive(Clone, Copy)]
pub enum TerminalTileScaling {
    /// Scale terminal tiles based on the size of their texture, such that 1 pixel == 1 world unit.
    /// This behavior matches the expected defaults for bevy's orthographic camera.
    Pixels,
    /// Each tile will take up 1 unit of world space
    World,
}

impl Default for TerminalTileScaling {
    fn default() -> Self {
        TerminalTileScaling::Pixels
    }
}

#[derive(Debug, RenderResources, ShaderDefs, Default, TypeUuid)]
#[uuid = "1e01121c-0b4a-315e-1bca-36733b11127e"]
pub struct TerminalMaterial {
    pub color: Color,
    pub clip_color: Color,
    #[shader_def] // This doesn't work for some reason...
    pub texture: Option<Handle<Texture>>,
}

impl TerminalMaterial {
    pub fn from_texture(tex: Handle<Texture>, clip_color: Color) -> Self {
        TerminalMaterial {
            color: Color::WHITE,
            clip_color,
            texture: Some(tex),
        }
    }
}

pub fn terminal_renderer_init(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<&mut Handle<Mesh>, (Added<Handle<Mesh>>, With<TerminalRendererVertexData>)>,
) {
    for mut mesh in q.iter_mut() {
        //info!("Initializing ascii terminal mesh");
        let new_mesh = Mesh::new(PrimitiveTopology::TriangleList);
        *mesh = meshes.add(new_mesh);
    }
}

fn terminal_renderer_update_material(
    fonts: Res<TerminalFonts>,
    mut materials: ResMut<Assets<TerminalMaterial>>,
    textures: Res<Assets<Texture>>,
    mut q: Query<
        (&TerminalRendererFont, &mut Handle<TerminalMaterial>),
        Changed<TerminalRendererFont>,
    >,
) {
    for (font, mut mat) in q.iter_mut() {
        //info!("Updating terminal renderer material");
        let existing_mat = materials.get(mat.clone_weak());

        if existing_mat.is_some() {
            materials.remove(mat.clone_weak());
        }

        let handle = &fonts.get(font.font_name.as_str()).0;
        let tex = textures.get(handle.clone());
        debug_assert!(tex.is_some());

        *mat = materials.add(TerminalMaterial::from_texture(
            handle.clone(),
            font.clip_color,
        ));
    }
}

fn terminal_renderer_update_size(
    mut meshes: ResMut<Assets<Mesh>>,
    fonts: Res<TerminalFonts>,
    mut q: Query<
        (
            &TerminalSize,
            &TerminalRendererFont,
            &TerminalTileScaling,
            &TerminalPivot,
            &TilePivot,
            &mut Handle<Mesh>,
            &mut TerminalRendererVertexData,
            &mut TerminalRendererTileData,
        ),
        Or<(
            Changed<TerminalSize>,
            Changed<Handle<Mesh>>,
            Changed<TerminalTileScaling>,
            Changed<TerminalRendererFont>,
        )>,
    >,
) {
    for (size, font, scaling, term_pivot, tile_pivot, mesh, mut vert_data, mut tile_data) in
        q.iter_mut()
    {
        let mut tile_size = UVec2::ONE;
        if let TerminalTileScaling::Pixels = scaling {
            tile_size = fonts.get(font.font_name.as_str()).1.tile_size;
        }

        vert_data.resize(size.value, term_pivot.0, tile_pivot.0, tile_size);
        tile_data.resize(size.value);

        let mesh = meshes
            .get_mut(mesh.clone())
            .expect("Error retrieving mesh from terminal renderer");

        //info!("Renderer update size: {}!", vert_data.indices.len());
        //info!("First 4 verts: {:?}", &vert_data.verts[0..4]);
        //info!("First 6 indices: {:?}", &vert_data.indices[0..6]);
        mesh.set_indices(Some(Indices::U32(vert_data.indices.clone())));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vert_data.verts.clone());
    }
}

pub fn terminal_renderer_update_tile_data(
    mut q: Query<(&Terminal, &mut TerminalRendererTileData), Changed<Terminal>>,
) {
    for (term, mut data) in q.iter_mut() {
        //info!("Renderer update tile data!");
        //info!("First tiles: {:?}", &term.tiles[0..4]);
        data.update_from_tiles(&term.tiles);
    }
}

pub fn terminal_renderer_update_mesh(
    mut meshes: ResMut<Assets<Mesh>>,
    mut q: Query<(&TerminalRendererTileData, &Handle<Mesh>), Changed<TerminalRendererTileData>>,
) {
    for (tile_data, mesh) in q.iter_mut() {
        let mesh = meshes.get_mut(mesh).expect("Error accessing terminal mesh");
        //info!("writing colors and uvs to mesh");
        //info!("First fg Colors: {:?}", &tile_data.fg_colors[0..4]);
        //info!("First bg Colors: {:?}", &tile_data.bg_colors[0..4]);
        //info!("First uvs: {:?}", &tile_data.uvs[0..4]);
        mesh.set_attribute("FG_Color", tile_data.fg_colors.clone());
        mesh.set_attribute("BG_Color", tile_data.bg_colors.clone());
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, tile_data.uvs.clone());
    }
}
