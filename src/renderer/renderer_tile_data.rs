use bevy::{math::UVec2, prelude::Component};

use crate::terminal::Tile;

use super::uv_mapping::UvMapping;

#[derive(Component, Default)]
pub struct TerminalRendererTileData {
    pub fg_colors: Vec<[f32; 4]>,
    pub bg_colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
}

impl TerminalRendererTileData {
    pub fn with_size(size: UVec2) -> Self {
        let mut v = Self::default();
        v.resize(size);
        v
    }

    pub fn resize(&mut self, size: UVec2) {
        let len = (size.x * size.y) as usize;

        self.fg_colors.resize(len * 4, Default::default());
        self.bg_colors.resize(len * 4, Default::default());
        self.uvs.resize(len * 4, Default::default());
    }

    pub fn update_from_tiles<'a>(&mut self, tiles: impl Iterator<Item=&'a Tile>, uv_mapping: &UvMapping) {
        for (i, tile) in tiles.enumerate() {
            let key = tile.key;

            let vi = i * 4;
            let uvs = &mut self.uvs;

            let tile_uvs = uv_mapping.uvs_from_key(key as u16);

            for (a, b) in uvs[vi..vi + 4].iter_mut().zip(tile_uvs) {
                *a = *b;
            }

            for j in vi..vi + 4 {
                self.fg_colors[j] = tile.fg_color.as_linear_rgba_f32();
                self.bg_colors[j] = tile.bg_color.as_linear_rgba_f32();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::UVec2;
    use bevy::prelude::Color;

    use crate::renderer::uv_mapping::UvMapping;
    use crate::{renderer::renderer_tile_data::TerminalRendererTileData, terminal::Tile};

    #[test]
    fn resize_test() {
        let mut tiles: Vec<Tile> = vec![Tile::default(); 50];

        for tile in tiles.iter_mut() {
            *tile = Tile {
                fg_color: Color::BLUE,
                ..Default::default()
            }
        }

        let mut colors: TerminalRendererTileData =
            TerminalRendererTileData::with_size(UVec2::new(25, 25));
        colors.update_from_tiles(tiles.iter(), &UvMapping::default());

        assert_eq!([0.0, 0.0, 1.0, 1.0], colors.fg_colors[0]);
    }
}
