use bevy::{math::UVec2, prelude::{Component, Color}};
use sark_grids::Size2d;

use crate::terminal::Tile;

use super::uv_mapping::{UvMapping, self};

#[derive(Component, Default)]
pub struct TerminalRendererTileData {
    pub fg_colors: Vec<[f32; 4]>,
    pub bg_colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
}

impl TerminalRendererTileData {
    pub fn terminal_tiles(size: impl Size2d) -> Self {
        let mut v = Self::default();
        v.terminal_resize(size);
        v
    }

    pub fn border_tiles(size: impl Size2d) -> Self {
        let mut v = Self::default();
        v.border_resize(size);
        v
    } 

    pub fn terminal_resize(&mut self, size: impl Size2d) {
        let len = size.len();

        self.fg_colors.resize(len * 4, Default::default());
        self.bg_colors.resize(len * 4, Default::default());
        self.uvs.resize(len * 4, Default::default());
    }

    pub fn border_resize(&mut self, size: impl Size2d) {
        let len = (size.width() * 2) + (size.height() * 2) + 4; 
        let curr = self.uvs.capacity();
        self.fg_colors.reserve(len.saturating_sub(curr));
        self.bg_colors.reserve(len.saturating_sub(curr));
        self.uvs.reserve(len.saturating_sub(curr));
    }

    pub fn update_from_tiles<'a>(
        &mut self,
        tiles: impl Iterator<Item = &'a Tile>,
        uv_mapping: &UvMapping,
    ) {
        for (i, tile) in tiles.enumerate() {
            let glyph = tile.glyph;

            let vi = i * 4;
            let uvs = &mut self.uvs;

            let glyph_uvs = uv_mapping.uvs_from_glyph(glyph);

            for (a, b) in uvs[vi..vi + 4].iter_mut().zip(glyph_uvs) {
                *a = *b;
            }

            for j in vi..vi + 4 {
                self.fg_colors[j] = tile.fg_color.as_linear_rgba_f32();
                self.bg_colors[j] = tile.bg_color.as_linear_rgba_f32();
            }
        }
    }

    pub fn border_update(&mut self, 
        size: impl Size2d, 
        fg: Color,
        bg: Color,
        glyphs: &[char;6], 
        mapping: &UvMapping
    ) {
        self.border_resize(size);

        let bl = mapping.uvs_from_glyph(glyphs[0]);
        let tl = mapping.uvs_from_glyph(glyphs[1]);
        let tr = mapping.uvs_from_glyph(glyphs[2]);
        let br = mapping.uvs_from_glyph(glyphs[3]);
        let hor = mapping.uvs_from_glyph(glyphs[4]);
        let ver = mapping.uvs_from_glyph(glyphs[5]);

        let mut helper = TileHelper {
            fg,
            bg,
            data: self,
        };

        helper.add_tile(bl);
        helper.add_tile(tl);
        helper.add_tile(tr);
        helper.add_tile(br);

        let begin = 4;
        let w = size.width() * 2;
        let h = size.height() * 2;
        
        for _ in begin..begin + w {
            helper.add_tile(hor);
        }

        for _ in begin + w..begin + w + h {
            helper.add_tile(ver);
        }
    }

    fn set_tile(&mut self, index: usize, uvs: &[[f32;2]], fg: Color, bg: Color) {
        let vi = index * 4;            

        for (a, b) in self.uvs[vi..vi + 4].iter_mut().zip(uvs) {
            *a = *b;
        }

        for j in vi..vi + 4 {
            self.fg_colors[j] = fg.as_linear_rgba_f32();
            self.bg_colors[j] = bg.as_linear_rgba_f32();
        }
    }

}

struct TileHelper<'a> {
    fg: Color,
    bg: Color,
    data: &'a mut TerminalRendererTileData,
}

impl<'a> TileHelper<'a> {
    fn set_tile(&mut self, index: usize, uvs: &[[f32;2]]) {
        let vi = index * 4;            

        for (a, b) in self.data.uvs[vi..vi + 4].iter_mut().zip(uvs) {
            *a = *b;
        }

        for j in vi..vi + 4 {
            self.data.fg_colors[j] = self.fg.as_linear_rgba_f32();
            self.data.bg_colors[j] = self.bg.as_linear_rgba_f32();
        }
    }

    fn add_tile(&mut self, uvs: &[[f32;2]]) {
        self.data.uvs.extend(uvs);

        self.data.fg_colors.extend(std::iter::repeat(self.fg.as_linear_rgba_f32()).take(4));
        self.data.bg_colors.extend(std::iter::repeat(self.bg.as_linear_rgba_f32()).take(4));
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::UVec2;
    use bevy::prelude::Color;

    use crate::code_page_437;
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
            TerminalRendererTileData::terminal_tiles(UVec2::new(25, 25));
        colors.update_from_tiles(tiles.iter(), &UvMapping::default());

        assert_eq!([0.0, 0.0, 1.0, 1.0], colors.fg_colors[0]);
    }

    #[test]
    fn border() {
        let mut data = TerminalRendererTileData::border_tiles([5,5]);
        data.border_update([5,5], Color::WHITE, Color::BLACK, 
        &['a', 'b', 'c', 'd', 'e', 'f'], &UvMapping::code_page_437());

        assert_eq!(data.uvs.len(), 24 * 4);
    }
}
