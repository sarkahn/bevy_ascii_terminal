//! A terminal component which determines how glyphs are mapped to their
//! corresponding uvs on the tile sheet.

use bevy::{prelude::*, utils::HashMap};

use crate::point::{Point2d, Size2d};

use super::code_page_437::CP_437_CHARS;

#[derive(Component)]
pub struct UvMapping {
    uv_map: HashMap<u16, [[f32; 2]; 4]>,
}

impl UvMapping {
    pub fn code_page_437() -> Self {
        let mut uv_map = HashMap::default();

        for (i, ch) in CP_437_CHARS.iter().cloned().enumerate() {
            uv_map.insert(ch as u16, UvMapping::uvs_from_grid_index(i, [16, 16]));
        }

        UvMapping { uv_map }
    }

    pub fn from_grid(tile_count: impl Size2d) -> Self {

        let mut uv_map = HashMap::default();

        for i in 0..tile_count.len() as u32 {
            let x = i % tile_count.width() as u32;
            let y = i / tile_count.height() as u32;

            uv_map.insert(i as u16, Self::uvs_from_grid_xy([x,y], tile_count));
        }

        UvMapping { uv_map }
    }

    fn uvs_from_grid_index(index: usize, tile_count: impl Size2d) -> [[f32; 2]; 4] {
        let x = index % tile_count.width();
        let y = index / tile_count.width();
        UvMapping::uvs_from_grid_xy([x as i32, y as i32], tile_count)
    }

    fn uvs_from_grid_xy(xy: impl Point2d, tile_count: impl Size2d) -> [[f32; 2]; 4] {
        let uv_size = Vec2::new(
            1.0 / tile_count.width() as f32,
            1.0 / tile_count.height() as f32,
        );
        let right = Vec2::new(uv_size.x, 0.0);
        let up = Vec2::new(0.0, uv_size.y);
        let origin = uv_size * xy.xy().as_vec2();
        [
            origin.into(),
            (origin + up).into(),
            (origin + right).into(),
            (origin + up + right).into(),
        ]
    }

    /// Create a uv mapping where the keys from the iterator are mapped to their corresponding
    /// uvs on a 2d tile sheet in sequential order.
    pub fn from_iterator(tile_count: [u32; 2], iter: impl Iterator<Item = u16>) -> Self {
        let mut uv_map = HashMap::default();

        for (i, key) in iter.enumerate() {
            let x = i as u32 % tile_count[0];
            let y = i as u32 / tile_count[0];
            let uvs = Self::get_grid_uvs([x, y], tile_count);
            uv_map.insert(key, uvs);
        }

        Self { uv_map }
    }

    pub fn get_grid_uvs(xy: [u32; 2], tile_count: [u32; 2]) -> [[f32; 2]; 4] {
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

    pub fn uvs_from_key(&self, key: u16) -> &[[f32; 2]; 4] {
        // debug_assert!(
        //     self.uv_map.contains_key(&key),
        //     "Error retrieving uvs, key {key}:{} not found",
        //     key as u8 as char
        // );
        &self.uv_map[&key]
    }
}

impl Default for UvMapping {
    fn default() -> Self {
        Self::code_page_437()
    }
}
