use bevy::math::{UVec2, Vec2};

use crate::terminal::Tile;

use super::glyph_mapping::GlyphMapping;

#[derive(Default)]
pub struct TerminalRendererTileData {
    pub fg_colors: Vec<[u8; 4]>,
    pub bg_colors: Vec<[u8; 4]>,
    pub uvs: Vec<[f32; 2]>,
    pub mapping: GlyphMapping,
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

    pub fn update_from_tiles(&mut self, tiles: &[Tile]) {
        let uv_size = Vec2::new(1.0 / 16.0, 1.0 / 16.0);
        let right = Vec2::new(uv_size.x, 0.0);
        let up = Vec2::new(0.0, uv_size.y);

        for (i, tile) in tiles.iter().enumerate() {
            let glyph = tile.glyph;
            let (tile_x, tile_y) = self.mapping.get_index(glyph);

            let origin = Vec2::new(tile_x as f32 * uv_size.x, tile_y as f32 * uv_size.y);

            let vi = i * 4;
            let uvs = &mut self.uvs;
            uvs[vi] = origin.into();
            uvs[vi + 1] = (origin + up).into();
            uvs[vi + 2] = (origin + right).into();
            uvs[vi + 3] = (origin + up + right).into();

            for j in vi..vi + 4 {
                self.fg_colors[j] = tile.fg_color.into();
                self.bg_colors[j] = tile.bg_color.into();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::UVec2;

    use crate::{renderer::renderer_tile_data::TerminalRendererTileData, terminal::Tile};

    use crate::color::*;

    #[test]
    fn resize_test() {
        let mut tiles: Vec<Tile> = vec![Tile::default(); 50];

        for tile in tiles.iter_mut() {
            *tile = Tile {
                fg_color: BLUE,
                ..Default::default()
            }
        }

        let mut colors: TerminalRendererTileData =
            TerminalRendererTileData::with_size(UVec2::new(25, 25));
        colors.update_from_tiles(&tiles);

        assert_eq!([0, 0, u8::MAX, u8::MAX], colors.fg_colors[0]);
    }
}
