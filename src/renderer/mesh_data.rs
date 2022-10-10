use bevy::{prelude::{Mesh, Component, UVec2, Vec2, Vec3, Color}, render::{mesh::{VertexAttributeValues, Indices, MeshVertexAttribute}, render_resource::VertexFormat}};
use sark_grids::{point::Point2d, Size2d, GridPoint};

use super::uv_mapping::UvMapping;

pub const ATTRIBUTE_UV: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Uv", 1, VertexFormat::Float32x2);
pub const ATTRIBUTE_COLOR_BG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Bg", 2, VertexFormat::Float32x4);
pub const ATTRIBUTE_COLOR_FG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Fg", 3, VertexFormat::Float32x4);

#[derive(Debug, Default, Component)]
pub struct VertData {
    pub verts: Vec<[f32;3]>,
    pub indices: Vec<u32>,
}

impl VertData {
    pub fn clear(&mut self) {
        self.verts.clear();
        self.indices.clear();
    }

    pub fn reserve(&mut self, tile_count: usize) {
        self.verts.reserve(tile_count * 4);
        self.indices.reserve(tile_count * 6);
    }

    pub fn build_mesh_verts(&mut self, mesh: &mut Mesh) {
        let verts = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
        let verts = match verts {
            VertexAttributeValues::Float32x3(verts) => verts,
            _ => panic!("Unexpected vertex position format"),
        };
        verts.clear();
        verts.append(&mut self.verts);

        let indices = mesh
            .indices_mut()
            .expect("Error retrieving terminal mesh indices");
        let indices = match indices {
            Indices::U32(indices) => indices,
            _ => panic!("Terminal indices are u16, expected u32"),
        };
        indices.clear();
        indices.append(&mut self.indices);
    }
}

#[derive(Debug, Default, Component)]
pub struct TileData {
    pub uvs: Vec<[f32;2]>,
    pub fg: Vec<[f32;4]>,
    pub bg: Vec<[f32;4]>,
}

impl TileData {
    pub fn clear(&mut self) {
        self.uvs.clear();
        self.fg.clear();
        self.bg.clear();
    }

    pub fn reserve(&mut self, tile_count: usize) {
        self.uvs.reserve(tile_count * 4);
        self.fg.reserve(tile_count * 4);
        self.bg.reserve(tile_count * 4);
    }

    pub fn build_mesh_tiles(&mut self, mesh: &mut Mesh) {
        let uvs = mesh.attribute_mut(ATTRIBUTE_UV)
            .expect("Error retrieving mesh uv data");
        let uvs = match uvs {
            VertexAttributeValues::Float32x2(uvs) => uvs,
            _ => panic!("Unexpected uv format"),
        };
        uvs.clear();
        uvs.append(&mut self.uvs);

        let fg_cols = mesh.attribute_mut(ATTRIBUTE_COLOR_FG)
            .expect("Error retrieving terminal mesh fg colors");
        let fg_cols = match fg_cols {
            VertexAttributeValues::Float32x4(fg) => fg,
            _ => panic!("Unexpected fg colors format"),
        };
        fg_cols.clear();
        fg_cols.append(&mut self.fg);

        let bg_cols = mesh.attribute_mut(ATTRIBUTE_COLOR_BG)
            .expect("Error retrieving terminal mesh bg colors");
        let bg_cols = match bg_cols {
            VertexAttributeValues::Float32x4(bg) => bg,
            _ => panic!("Unexpected bg colors format"),
        };
        bg_cols.clear();
        bg_cols.append(&mut self.bg);
    }
}


/// Helper for building the terminal mesh's vertex data.
pub struct VertMesher<'a> {
    pub tile_size: Vec2,
    pub origin: Vec2,
    vert_data: &'a mut VertData,
}

impl<'a> VertMesher<'a> {
    pub fn new(
        origin: impl Point2d, 
        tile_size: impl Point2d,
        vert_data: &'a mut VertData,
    ) -> Self {
        Self {
            tile_size: tile_size.as_vec2(),
            origin: origin.as_vec2(),
            vert_data,
        }
    }

    /// Generate vertex data for a tile at the given position.
    pub fn tile_verts_at(&mut self, xy: impl GridPoint) {
        let right = Vec3::X * self.tile_size.x;
        let up = Vec3::Y * self.tile_size.y;

        let p = (self.origin + xy.as_vec2() * self.tile_size).extend(0.0);

        let vd = &mut self.vert_data;

        let vi = vd.verts.len() as u32;
        vd.verts
            .extend(&[p + up, p, p + right + up, p + right].map(|p| p.to_array()));
        vd.indices
            .extend(&[vi, vi + 1, vi + 2, vi + 3, vi + 2, vi + 1]);

    }
}

/// Helper for building the terminal mesh's uv data.
pub struct UvMesher<'a> {
    mapping: &'a UvMapping,
    tile_data: &'a mut TileData,
}

impl<'a> UvMesher<'a> {

    pub fn new(
        mapping: &'a UvMapping,
        tile_data: &'a mut TileData,
    ) -> Self {
        Self {
            mapping,
            tile_data
        }
    }

    /// Generate tile uvs for the next tile. Note these are not positional,
    /// they must be added in the same order as the vert data.
    pub fn tile_uvs(&mut self, glyph: char, fg: Color, bg: Color) {
        let td = &mut self.tile_data;
        let glyph_uv = self.mapping.uvs_from_glyph(glyph);
        td.uvs.extend(glyph_uv);
        td.fg
            .extend(std::iter::repeat(fg.as_linear_rgba_f32()).take(4));
        td.bg
            .extend(std::iter::repeat(bg.as_linear_rgba_f32()).take(4));
    }
}

#[cfg(test)]
mod test {
    use bevy::{prelude::{Mesh, Color}, render::render_resource::PrimitiveTopology};

    use crate::renderer::{uv_mapping::UvMapping};

    use super::*;

    #[test]
    fn mesher() {
        let mut vd = VertData::default();
        let mut mesher = VertMesher::new([0,0], [1.0,1.0], &mut vd);

        mesher.tile_verts_at([1,1]);

        assert_eq!(4, vd.verts.len());
        assert_eq!(6, vd.indices.len());

        let mapping = UvMapping::default();
        let mut td = TileData::default();
        let mut mesher = UvMesher::new(&mapping, &mut td);

        mesher.tile_uvs('a', Color::BLUE, Color::YELLOW);

        assert_eq!(4, td.uvs.len());
        assert_eq!(4, td.fg.len());
        assert_eq!(4, td.bg.len());
    }
}