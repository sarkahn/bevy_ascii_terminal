use bevy::{
    color::{ColorToComponents, LinearRgba},
    math::Vec2,
    render::mesh::{Indices, Mesh, VertexAttributeValues},
};

use crate::GridPoint;

use super::{
    mesh::{ATTRIBUTE_COLOR_BG, ATTRIBUTE_COLOR_FG, ATTRIBUTE_UV},
    UvMapping,
};

/// Utility for updating terminal mesh vertices.
pub struct VertMesher {
    origin: Vec2,
    tile_size: Vec2,
    indices: Vec<u32>,
    verts: Vec<[f32; 3]>,
}

impl VertMesher {
    /// Build terminal mesh verts by removing the relevant mesh attributes and
    /// modifying them with the [VertMesher]. The attributes will be reinserted
    /// into the mesh after this function completes.
    ///
    /// This is done to prevent the borrow checker from complaining when trying
    /// to modify multiple mesh attributes at the same time.
    pub fn build_mesh_verts(
        origin: Vec2,
        tile_size: Vec2,
        mesh: &mut Mesh,
        modify_mesh: impl FnOnce(&mut Self),
    ) {
        let Some(Indices::U32(indices)) = mesh.remove_indices() else {
            panic!("Incorrect terminal mesh indices format");
        };
        let Some(VertexAttributeValues::Float32x3(verts)) =
            mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("Incorrect mesh terminal vertex format");
        };

        let mut mesher = Self {
            origin,
            tile_size,
            indices,
            verts,
        };
        modify_mesh(&mut mesher);
        mesh.insert_indices(Indices::U32(mesher.indices));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesher.verts);
    }

    /// Set the mesh vertex data for a tile at a given grid position.
    ///
    /// # Arguments
    ///
    /// * `x` - The local x coordinate of the tile
    /// * `y` - The local y coordinate of the tile.
    /// * `mesh_index` - The index of the tile's data within the mesh.
    #[inline]
    pub fn set_tile_verts(&mut self, xy: impl GridPoint, mesh_index: usize) {
        let p = (self.origin + xy.as_vec2() * self.tile_size).extend(0.0);
        let right = (Vec2::X * self.tile_size).extend(0.0);
        let up = (Vec2::Y * self.tile_size).extend(0.0);

        let i = mesh_index * 4;
        self.verts[i] = (p + up).into();
        self.verts[i + 1] = p.into();
        self.verts[i + 2] = (p + right + up).into();
        self.verts[i + 3] = (p + right).into();

        let vi = i as u32;
        let i = mesh_index * 6;
        self.indices[i] = vi;
        self.indices[i + 1] = vi + 1;
        self.indices[i + 2] = vi + 2;
        self.indices[i + 3] = vi + 3;
        self.indices[i + 4] = vi + 2;
        self.indices[i + 5] = vi + 1;
    }
}

/// Utility for updating terminal mesh vertex data.
pub struct UvMesher<'a> {
    mapping: &'a UvMapping,
    uvs: Vec<[f32; 2]>,
    fg: Vec<[f32; 4]>,
    bg: Vec<[f32; 4]>,
}

impl<'a> UvMesher<'a> {
    /// Update the mesh tile data by removing the relevant mesh attributes and
    /// modifying them with the [UvMesher]. The attributes will be reinserted
    /// into the mesh after this function completes.
    ///
    /// This is done to prevent the borrow checker from complaining when trying
    /// to modify multiple mesh attributes at the same time.
    pub fn build_mesh_tile_data(
        mapping: &'a UvMapping,
        mesh: &mut Mesh,
        modify_mesh: impl FnOnce(&mut Self),
    ) {
        let Some(VertexAttributeValues::Float32x2(uvs)) = mesh.remove_attribute(ATTRIBUTE_UV)
        else {
            panic!("Incorrect terminal mesh uv format");
        };
        let Some(VertexAttributeValues::Float32x4(fg)) = mesh.remove_attribute(ATTRIBUTE_COLOR_FG)
        else {
            panic!("Incorrect terminal mesh fg color format");
        };
        let Some(VertexAttributeValues::Float32x4(bg)) = mesh.remove_attribute(ATTRIBUTE_COLOR_BG)
        else {
            panic!("Incorrect terminal mesh bg color format");
        };

        let mut mesher = Self {
            mapping,
            uvs,
            fg,
            bg,
        };

        modify_mesh(&mut mesher);

        mesh.insert_attribute(ATTRIBUTE_UV, mesher.uvs);
        mesh.insert_attribute(ATTRIBUTE_COLOR_FG, mesher.fg);
        mesh.insert_attribute(ATTRIBUTE_COLOR_BG, mesher.bg);
    }

    /// Sets tile data at the given mesh index.
    ///
    /// # Arguments
    ///
    /// * `glyph` - The glyph which will be translated by the terminal's [UvMapping]
    ///   component to determine which tile to render.
    /// * `fg` - The foreground color of the tile.
    /// * `bg` - The background color of the tile.
    /// * `mesh_index` - The index of the tile's data within the mesh.
    #[inline]
    pub fn set_tile_data(
        &mut self,
        glyph: impl Into<char>,
        fg: LinearRgba,
        bg: LinearRgba,
        mesh_index: usize,
    ) {
        let glyph = glyph.into();
        let uvs = self.mapping.uvs_from_char(glyph);
        let i = mesh_index * 4;

        self.uvs[i..i + 4]
            .iter_mut()
            .zip(uvs)
            .for_each(|(tuv, uv)| *tuv = *uv);

        self.fg[i..i + 4].fill(fg.to_f32_array());
        self.bg[i..i + 4].fill(bg.to_f32_array());
    }
}
