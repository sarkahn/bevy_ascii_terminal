//! A terminal component which determines how glyphs are mapped to their
//! corresponding uvs on the tile sheet.
use bevy::{
    math::{Rect, Vec2},
    prelude::{Asset, AssetApp, Assets, Handle, Plugin},
    reflect::TypePath,
    utils::HashMap,
};

pub struct TerminalUvMappingPlugin;

impl Plugin for TerminalUvMappingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_asset::<UvMapping>();
        let mut mappings = app.world_mut().resource_mut::<Assets<UvMapping>>();
        mappings.insert(&Handle::<UvMapping>::default(), UvMapping::default());
    }
}

/// An asset that defines how a rust [char] is converted into uv data for rendering
/// terminal tiles.
#[derive(Asset, Debug, Clone, TypePath)]
pub struct UvMapping {
    uv_map: HashMap<char, [[f32; 2]; 4]>,
}

impl UvMapping {
    pub fn code_page_437() -> Self {
        UvMapping::from_grid([16, 16], crate::ascii::CP_437_ARRAY.iter().cloned())
    }

    /// Create a uv mapping where the keys from the iterator are mapped to their corresponding
    /// uvs on a 2d tile sheet in sequential order, from top left increasing right and down.
    pub fn from_grid(tile_count: [u32; 2], iter: impl Iterator<Item = char>) -> Self {
        let mut uv_map = HashMap::default();

        for (i, ch) in iter.enumerate() {
            let x = i as u32 % tile_count[0];
            let y = i as u32 / tile_count[0];
            let uvs = Self::calc_grid_uvs([x, y], tile_count);
            uv_map.insert(ch, uvs);
        }

        Self { uv_map }
    }

    /// Calculate the uvs for a given tile based solely on grid size and position.
    pub fn calc_grid_uvs(xy: [u32; 2], tile_count: [u32; 2]) -> [[f32; 2]; 4] {
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

    /// Retrieve the uv data for a terminal mesh tile from it's corresponding
    /// [char]. Will panic if no uvs have been set for the char.
    pub fn uvs_from_char(&self, ch: char) -> &[[f32; 2]; 4] {
        self.uv_map.get(&ch).unwrap_or_else(|| {
            panic!(
                "Error retrieving uv mapping, '{}' was not present in map",
                ch
            )
        })
    }

    /// Retrieve the uv data for a terminal mesh tile from it's corresponding
    /// [char].
    pub fn get_uvs_from_char(&self, ch: char) -> Option<&[[f32; 2]; 4]> {
        self.uv_map.get(&ch)
    }

    /// Insert a set of uvs for a given char.
    pub fn add_uvs(&mut self, key: char, rect: Rect) {
        let [xmin, ymin] = rect.min.to_array();
        let [xmax, ymax] = rect.max.to_array();
        let uvs = [[xmin, ymin], [xmin, ymax], [xmax, ymin], [xmax, ymax]];
        self.uv_map.insert(key, uvs);
    }
}

impl Default for UvMapping {
    fn default() -> Self {
        Self::code_page_437()
    }
}
