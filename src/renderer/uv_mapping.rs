use bevy::{prelude::*, utils::HashMap};

use crate::code_page_437;

use super::code_page_437::CP_437_CHARS;


pub struct UvMapping {
    uv_map: HashMap<char,[[f32;2];4]>,
}

impl UvMapping {
    pub fn code_page_437() -> Self {
        UvMapping::from_grid([16,16], CP_437_CHARS.iter().cloned())
    }

    /// Create a uv mapping where the keys from the iterator are mapped to their corresponding 
    /// uvs on a 2d tile sheet in sequential order.
    pub fn from_grid(
        tile_count: [u32;2], 
        iter: impl Iterator<Item = char>
    ) -> Self {

        let mut uv_map = HashMap::default();

        for (i,ch) in iter.enumerate() {
            let x = i as u32 % tile_count[0];
            let y = i as u32 / tile_count[0];
            let uvs = Self::get_grid_uvs([x,y], tile_count);
            uv_map.insert(ch, uvs);
        }

        Self {
            uv_map,
        }
    }

    pub fn get_grid_uvs(xy: [u32;2], tile_count: [u32;2]) -> [[f32;2];4] {
        let xy = Vec2::new(xy[0] as f32, xy[1] as f32);
        let uv_size = Vec2::new(1.0 / tile_count[0] as f32, 1.0 / tile_count[1] as f32);
        let right = Vec2::new(uv_size.x, 0.0);
        let up = Vec2::new(0.0, uv_size.y);
        let origin = uv_size * xy;
        [
            origin.into(),
            (origin + up).into(),
            (origin + right).into(),
            (origin + up + right).into(),
        ]
    }

    pub fn uvs_from_glyph(&self, ch: char) -> &[[f32;2];4] {
        &self.uv_map[&ch]
    }

    pub fn uvs_from_index(&self, index: u8) -> &[[f32;2];4] {
        let char = code_page_437::index_to_glyph(index);
        self.uvs_from_glyph(char)
    }
}

impl Default for UvMapping {
    fn default() -> Self {
        Self::code_page_437()
    }
}
