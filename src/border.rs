use std::collections::{btree_map, BTreeMap};

use bevy::math::IVec2;

use crate::{direction::Dir4, FormattedString, GridPoint, GridRect, Pivot, Tile};

#[derive(Debug, Clone)]
pub struct Border {
    edge_glyphs: [char; 8],
    tiles: BTreeMap<(i32, i32), Tile>,
    changed: bool,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            edge_glyphs: [' '; 8],
            tiles: Default::default(),
            changed: true,
        }
    }
}

impl Border {
    pub fn from_string(string: impl AsRef<str>) -> Self {
        let mut glyphs = [' '; 8];
        for (ch, glyph) in string.as_ref().chars().zip(glyphs.iter_mut()) {
            *glyph = ch;
        }
        Self {
            edge_glyphs: glyphs,
            tiles: BTreeMap::new(),
            changed: true,
        }
    }

    pub fn single_line() -> Self {
        Self::from_string("┌─┐││└─┘")
    }

    pub fn double_line() -> Self {
        Self::from_string("╔═╗║║╚═╝")
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    /// Set the border's edge tiles from it's edge glyphs. This will override
    /// any previously set border tiles.
    fn set_edge_tiles(&mut self, size: IVec2, clear_tile: Tile) {
        let rect = GridRect::from_points([-1, -1], size);
        let mut tile = clear_tile;
        self.set_edge(Pivot::TopLeft, rect, *tile.glyph(self.edge_glyphs[0]));
        self.set_edge(Pivot::TopCenter, rect, *tile.glyph(self.edge_glyphs[1]));
        self.set_edge(Pivot::TopRight, rect, *tile.glyph(self.edge_glyphs[2]));
        self.set_edge(Pivot::LeftCenter, rect, *tile.glyph(self.edge_glyphs[3]));
        self.set_edge(Pivot::RightCenter, rect, *tile.glyph(self.edge_glyphs[4]));
        self.set_edge(Pivot::BottomLeft, rect, *tile.glyph(self.edge_glyphs[5]));
        self.set_edge(Pivot::BottomCenter, rect, *tile.glyph(self.edge_glyphs[6]));
        self.set_edge(Pivot::BottomRight, rect, *tile.glyph(self.edge_glyphs[7]));
    }

    fn set_edge(&mut self, edge: Pivot, border_rect: GridRect, tile: Tile) {
        match edge {
            Pivot::TopLeft => self.put_tile(border_rect.top_left(), tile),
            Pivot::Center | Pivot::TopCenter => {
                for x in 1..border_rect.right() {
                    self.put_tile([x, border_rect.top()], tile);
                }
            }
            Pivot::TopRight => self.put_tile(border_rect.top_right(), tile),
            Pivot::LeftCenter => {
                for y in 1..border_rect.top() {
                    self.put_tile([border_rect.left(), y], tile);
                }
            }
            Pivot::RightCenter => {
                for y in 1..border_rect.top() {
                    self.put_tile([border_rect.right(), y], tile);
                }
            }
            Pivot::BottomLeft => self.put_tile(border_rect.bottom_left(), tile),
            Pivot::BottomCenter => {
                for x in 1..border_rect.right() {
                    self.put_tile([x, border_rect.bottom()], tile);
                }
            }
            Pivot::BottomRight => self.put_tile(border_rect.bottom_right(), tile),
        }
    }

    fn put_tile(&mut self, xy: impl GridPoint, value: Tile) {
        // Note tile positions are stored y-first for proper left-to-right,
        // down-to-up ordering
        self.tiles.insert((xy.y(), xy.x()), value);
    }

    pub fn iter(&self) -> impl Iterator<Item = (IVec2, Tile)> + '_ {
        self.tiles
            .iter()
            .map(|((y, x), t)| (IVec2::new(*x, *y), *t))
    }
}

/// A mutable reference to the terminal border
pub struct TerminalBorderMut<'a> {
    border: &'a mut Border,
    term_size: IVec2,
    clear_tile: Tile,
}

impl<'a> TerminalBorderMut<'a> {
    pub(crate) fn new(border: &'a mut Border, term_size: IVec2, clear_tile: Tile) -> Self {
        Self {
            border,
            term_size,
            clear_tile,
        }
    }

    /// Insert a [FormattedString] into the terminal border. For simply setting
    /// a "title" string you should use [Self::put_title] instead.
    pub fn put_string(
        &mut self,
        edge: Pivot,
        direction: Dir4,
        offset: i32,
        string: impl Into<FormattedString<'a>>,
    ) -> &mut Self {
        self.border.changed = true;
        let fmt: FormattedString = string.into();
        let fg_color = fmt.fg_color.unwrap_or(self.clear_tile.fg_color);
        let bg_color = fmt.bg_color.unwrap_or(self.clear_tile.bg_color);

        let border_rect = GridRect::from_points([-1, -1], self.term_size);
        let term_rect = GridRect::new([0, 0], self.term_size);

        let mut xy = border_rect.pivot_point(edge);
        let dir = direction.as_ivec2();
        xy += dir * offset;
        println!(
            "Putting string {} at {}, dir {}, len {}",
            fmt.string,
            xy,
            dir,
            fmt.string.len()
        );
        for ch in fmt.string.chars() {
            if !border_rect.contains_point(xy) || term_rect.contains_point(xy) {
                break;
            }
            if fmt.ignore_spaces && ch == ' ' {
                xy += dir;
                continue;
            }
            let [x, y] = xy.as_array();
            self.border.put_tile(
                [x, y],
                Tile {
                    glyph: ch,
                    fg_color,
                    bg_color,
                },
            );
            xy += dir;
        }
        self
    }

    /// Set a "title" string for the terminal border. Note this will clear
    /// any previously set strings from the top edge
    pub fn put_title(&mut self, string: impl Into<FormattedString<'a>>) -> &mut Self {
        let clear_tile = *self.clear_tile.glyph(self.border.edge_glyphs[1]);
        let rect = GridRect::from_points([-1, -1], self.term_size);
        self.border.set_edge(Pivot::TopCenter, rect, clear_tile);
        self.put_string(Pivot::TopLeft, Dir4::Right, 1, string);
        self
    }

    /// Set all the border tile colors to match the terminal's clear tile
    pub fn clear_colors(&'a mut self) -> &'a mut Self {
        self.border.changed = true;
        let clear = self.clear_tile;
        for tile in self.border.tiles.values_mut() {
            tile.fg_color = clear.fg_color;
            tile.bg_color = clear.bg_color;
        }
        self
    }

    /// Reset all border tiles to the border's edge glyphs. This will override
    /// any previously set border tiles.
    ///
    /// To instead completey remove the border you should use
    /// `terminal.set_border(None)`.
    pub fn clear(&'a mut self) -> &'a mut Self {
        self.border.changed = true;
        self.border.set_edge_tiles(self.term_size, self.clear_tile);
        self
    }

    pub(crate) fn reset_changed_state(&mut self) {
        self.border.changed = false;
    }
}

#[cfg(test)]
mod tests {
    use bevy::{math::IVec2, render::color::Color};

    use crate::{string::StringFormatter, GridRect, Tile};

    use super::{Border, TerminalBorderMut};

    #[test]
    fn put_string() {
        let mut state = Border::default();
        let mut term_border = TerminalBorderMut {
            border: &mut state,
            term_size: IVec2::splat(20),
            clear_tile: Tile::DEFAULT,
        };
        term_border.put_title("Hello".fg(Color::BLUE).bg(Color::RED));
    }

    #[test]
    fn contains() {
        let size = IVec2::splat(10);
        let border_rect = GridRect::from_points([-1, -1], size);
        let term_rect = GridRect::new([0, 0], size);

        let border_contains =
            |p: [i32; 2]| -> bool { border_rect.contains_point(p) && !term_rect.contains_point(p) };

        assert!(border_contains([-1, -1]));
        assert!(!border_contains([0, 0]));
        assert!(!border_contains([9, 9]));
        assert!(border_contains([10, 10]));
    }
}
