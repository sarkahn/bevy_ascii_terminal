use bevy::{
    prelude::Mesh,
    render::{
        mesh::{Indices, MeshVertexAttribute, VertexAttributeValues},
        render_resource::VertexFormat,
    },
};

pub const ATTRIBUTE_UV: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Uv", 1, VertexFormat::Float32x2);
pub const ATTRIBUTE_COLOR_BG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Bg", 2, VertexFormat::Float32x4);
pub const ATTRIBUTE_COLOR_FG: MeshVertexAttribute =
    MeshVertexAttribute::new("Vertex_Color_Fg", 3, VertexFormat::Float32x4);

pub struct VertData<'a> {
    pub verts: Vec<[f32; 3]>,
    pub indices: &'a mut Vec<u32>,
}

pub struct TileData {
    pub uvs: Vec<[f32; 2]>,
    pub fg_cols: Vec<[f32; 4]>,
    pub bg_cols: Vec<[f32; 4]>,
}

pub trait MeshData {
    fn init_mesh_data(&mut self);
    fn get_vert_data(&mut self) -> VertData;
    fn insert_vert_data(&mut self, verts: Vec<[f32; 3]>);
    fn get_tile_data(&mut self) -> TileData;
    fn insert_tile_data(&mut self, tile_data: TileData);
}

impl MeshData for Mesh {
    fn get_vert_data(&mut self) -> VertData {
        let verts = self.remove_attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        let verts = match verts {
            VertexAttributeValues::Float32x3(verts) => verts,
            _ => panic!("Unexpected vertex position format"),
        };
        let indices = self
            .indices_mut()
            .expect("Error retrieving terminal mesh indices");
        let indices = match indices {
            Indices::U32(indices) => indices,
            _ => panic!("Terminal indices are u16, expected u32"),
        };
        VertData { verts, indices }
    }

    /// Indices were only borrowed, no need to reinsert
    fn insert_vert_data(&mut self, data: Vec<[f32; 3]>) {
        self.insert_attribute(Mesh::ATTRIBUTE_POSITION, data);
    }

    fn get_tile_data(&mut self) -> TileData {
        let uvs = self.remove_attribute(ATTRIBUTE_UV).unwrap();
        let uvs = match uvs {
            VertexAttributeValues::Float32x2(uvs) => uvs,
            _ => panic!("Unexpected uv format"),
        };
        let fg_cols = self
            .remove_attribute(ATTRIBUTE_COLOR_FG)
            .expect("Error retrieving terminal mesh fg colors");
        let fg_cols = match fg_cols {
            VertexAttributeValues::Float32x4(fg) => fg,
            _ => panic!("Unexpected fg colors format"),
        };

        let bg_cols = self
            .remove_attribute(ATTRIBUTE_COLOR_BG)
            .expect("Error retrieving terminal mesh bg colors");
        let bg_cols = match bg_cols {
            VertexAttributeValues::Float32x4(bg) => bg,
            _ => panic!("Unexpected bg colors format"),
        };

        TileData {
            uvs,
            fg_cols,
            bg_cols,
        }
    }

    fn insert_tile_data(&mut self, data: TileData) {
        self.insert_attribute(ATTRIBUTE_UV, data.uvs);
        self.insert_attribute(ATTRIBUTE_COLOR_FG, data.fg_cols);
        self.insert_attribute(ATTRIBUTE_COLOR_BG, data.bg_cols);
    }

    fn init_mesh_data(&mut self) {
        self.set_indices(Some(Indices::U32(Vec::new())));
        self.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new());
        self.insert_attribute(ATTRIBUTE_UV, Vec::<[f32; 2]>::new());
        self.insert_attribute(ATTRIBUTE_COLOR_FG, Vec::<[f32; 4]>::new());
        self.insert_attribute(ATTRIBUTE_COLOR_BG, Vec::<[f32; 4]>::new());
    }
}
