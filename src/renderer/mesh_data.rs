use bevy::{prelude::{Component, Vec2, Vec3, Mesh}, render::{mesh::{VertexAttributeValues, Indices, MeshVertexAttribute}, render_resource::VertexFormat}};
use sark_grids::{GridPoint, Size2d};

pub const ATTRIBUTE_UV: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Uv", 1, VertexFormat::Float32x2);
pub const ATTRIBUTE_COLOR_BG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Bg", 2, VertexFormat::Float32x4);
pub const ATTRIBUTE_COLOR_FG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Fg", 3, VertexFormat::Float32x4);

#[derive(Component, Default)]
pub struct VertexData {
    pub verts: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

impl VertexData {
    pub fn clear(&mut self) {
        self.verts.clear();
        self.indices.clear();
    }

    /// Reserve enough capacity for the given number of tiles.
    pub fn reserve(&mut self, tile_count: usize) {
        self.verts.reserve(tile_count * 4);
        self.indices.reserve(tile_count * 6);
    }
}

#[derive(Component, Default)]
pub struct TileData {
    pub fg_colors: Vec<[f32; 4]>,
    pub bg_colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
}

impl TileData {
    pub fn clear(&mut self) {
        self.fg_colors.clear();
        self.bg_colors.clear();
        self.uvs.clear();
    }

    /// Reserve enough capacity for the given number of tiles
    pub fn reserve(&mut self, tile_count: usize) {
        self.fg_colors.reserve(tile_count);
        self.bg_colors.reserve(tile_count);
        self.uvs.reserve(tile_count  * 4);
    }
}

/*

    pub fg_colors: Vec<[f32; 4]>,
    pub bg_colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
*/
pub trait MeshData {
    /// Verts, Indices
    fn get_vert_data(&mut self) -> (Vec<[f32;3]>, &mut Vec<u32>);
    fn insert_vert_data(&mut self, verts: Vec<[f32;3]>);
    /// UVs, FGCol, BGCol
    fn get_tile_data(&mut self) -> (Vec<[f32;2]>, Vec<[f32;4]>, Vec<[f32;4]>);
    fn insert_tile_data(&mut self, uvs: Vec<[f32;2]>, fg: Vec<[f32;4]>, bg: Vec<[f32;4]>);
    // fn indices_u32_mut(&mut self) -> &mut Vec<u32>;
    // fn remove_verts(&mut self) -> Vec<[f32;3]>;
    // fn remove_fg_colors(&mut self) -> Vec<[f32;4]>;
    // fn remove_bfg_colors(&mut self) -> Vec<[f32;4]>;
    // fn remove_uvs(&mut self) -> Vec<[f32;2]>;
}

impl MeshData for Mesh {
    fn get_vert_data(&mut self) -> (Vec<[f32;3]>, &mut Vec<u32>) {
        let verts = self.remove_attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        let verts = match verts {
            VertexAttributeValues::Float32x3(verts) => verts,
            _ => panic!("Unexpected vertex position format"),
        };
        let indices = self.indices_mut().expect("Error retrieving terminal mesh indices");
        let indices = match indices {
            Indices::U32(indices) => indices,
            _ => panic!("Terminal indices are u16, expected u32")
        };
        (verts,indices)
    }

    /// Indices were only borrowed, no need to reinsert
    fn insert_vert_data(&mut self, verts: Vec<[f32;3]>) {
        self.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    }

    fn get_tile_data(&mut self) -> (Vec<[f32;2]>, Vec<[f32;4]>, Vec<[f32;4]>) {
        let uvs = self.remove_attribute(ATTRIBUTE_UV).unwrap();
        let uvs = match uvs {
            VertexAttributeValues::Float32x2(uvs) => uvs,
            _ => panic!("Unexpected uv format"),
        };
        let fg = self.remove_attribute(ATTRIBUTE_COLOR_FG)
            .expect("Error retrieving terminal mesh fg colors");
        let fg = match fg {
            VertexAttributeValues::Float32x4(fg) => fg,
            _ => panic!("Unexpected fg colors format")
        };

        let bg = self.remove_attribute(ATTRIBUTE_COLOR_BG)
            .expect("Error retrieving terminal mesh bg colors");
        let bg = match bg {
            VertexAttributeValues::Float32x4(bg) => bg,
            _ => panic!("Unexpected bg colors format")
        };

        (uvs, fg, bg)
    }

    fn insert_tile_data(&mut self, uvs: Vec<[f32;2]>, fg: Vec<[f32;4]>, bg: Vec<[f32;4]>) {
        self.insert_attribute(ATTRIBUTE_UV, uvs);
        self.insert_attribute(ATTRIBUTE_COLOR_FG, fg);
        self.insert_attribute(ATTRIBUTE_COLOR_BG, bg);
    }

    // fn remove_verts(&mut self) -> Vec<[f32;3]> {
    //     let verts = self.remove_attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
    //     match verts {
    //         VertexAttributeValues::Float32x3(verts) => verts,
    //         _ => panic!("Unexpected vertex position format"),
    //     }
    // }
    
    // fn indices_u32_mut(&mut self) -> &mut Vec<u32> {
    //     let indices = self.indices_mut().expect("Error retrieving terminal mesh indices");
    //     match indices {
    //         Indices::U32(indices) => indices,
    //         _ => panic!("Terminal indices are u16, expected u32")
    //     }
    // }

    // fn remove_fg_colors(&mut self) -> &mut Vec<[f32;4]> {
    //     let cols = self.attribute_mut(ATTRIBUTE_COLOR_FG)
    //         .expect("Error retrieving terminal mesh fg colors");
    //     match cols {
    //         VertexAttributeValues::Float32x4(cols) => cols,
    //         _ => panic!("Unexpected fg colors format")
    //     }
    // }

    // fn remove_bfg_colors(&mut self) -> &mut Vec<[f32;4]> {
    //     let cols = self.attribute_mut(ATTRIBUTE_COLOR_BG)
    //         .expect("Error retrieving terminal mesh bg colors");
    //     match cols {
    //         VertexAttributeValues::Float32x4(cols) => cols,
    //         _ => panic!("Unexpected bg colors format")
    //     }
    // }

    // fn remove_uvs(&mut self) -> &mut Vec<[f32;2]> {
    //     let uvs = self.attribute_mut(ATTRIBUTE_UV).unwrap();
    //     match uvs {
    //         VertexAttributeValues::Float32x2(uvs) => uvs,
    //         _ => panic!("Unexpected uv format"),
    //     }
    // }
}