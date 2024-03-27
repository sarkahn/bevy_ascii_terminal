use std::{collections::BTreeMap, iter::once};

use bevy::{
    math::{IVec2, Mat2},
    render::color::Color,
};

use crate::{
    direction::{self, Dir4, DOWN, LEFT, RIGHT, UP},
    string::StringFormatter,
    FormattedString, GridPoint, GridRect, Pivot, Tile,
};

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

    pub fn edge_glyph(&self, edge: Pivot) -> char {
        match edge {
            Pivot::TopLeft => self.edge_glyphs[0],
            Pivot::TopCenter | Pivot::Center => self.edge_glyphs[1],
            Pivot::TopRight => self.edge_glyphs[2],
            Pivot::LeftCenter => self.edge_glyphs[3],
            Pivot::RightCenter => self.edge_glyphs[4],
            Pivot::BottomLeft => self.edge_glyphs[5],
            Pivot::BottomCenter => self.edge_glyphs[6],
            Pivot::BottomRight => self.edge_glyphs[7],
        }
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub(crate) fn build_edge_tiles(&mut self, size: IVec2, mut clear_tile: Tile) {
        let rect = GridRect::from_points([-1, -1], size);
        clear_tile.glyph = self.edge_glyphs[0];
        self.put_tile(rect.top_left(), clear_tile);

        clear_tile.glyph = self.edge_glyphs[1];
        for x in 1..rect.right() {
            self.put_tile([x, rect.top()], clear_tile);
        }

        clear_tile.glyph = self.edge_glyphs[2];
        self.put_tile(rect.top_right(), clear_tile);

        clear_tile.glyph = self.edge_glyphs[3];
        for y in 1..rect.top() {
            self.put_tile([rect.left(), y], clear_tile);
        }

        clear_tile.glyph = self.edge_glyphs[4];
        for y in 1..rect.top() {
            self.put_tile([rect.right(), y], clear_tile);
        }

        clear_tile.glyph = self.edge_glyphs[5];
        self.put_tile(rect.bottom_left(), clear_tile);

        clear_tile.glyph = self.edge_glyphs[6];
        for x in 1..rect.right() {
            self.put_tile([x, rect.bottom()], clear_tile);
        }

        clear_tile.glyph = self.edge_glyphs[7];
        self.put_tile(rect.bottom_right(), clear_tile);
    }

    fn put_tile(&mut self, xy: impl GridPoint, value: Tile) {
        // Note tile positions are stored y-first for proper left-to-right, 
        // down-to-up sorting
        self.tiles.insert((xy.y(), xy.x()), value);
    }

    pub fn iter(&self) -> impl Iterator<Item = (IVec2, Tile)> + '_ {
        self.tiles
            .iter()
            .map(|((y, x), t)| (IVec2::new(*x, *y), *t))
    }
}

fn edge_from_point(rect: GridRect, xy: impl GridPoint) -> Option<Pivot> {
    let [x, y] = xy.as_array();

    if y == rect.top() {
        if x == rect.left() {
            Some(Pivot::TopLeft)
        } else if x == rect.right() {
            Some(Pivot::TopRight)
        } else {
            Some(Pivot::TopCenter)
        }
    } else if y == rect.bottom() {
        if x == rect.left() {
            Some(Pivot::BottomLeft)
        } else if x == rect.right() {
            Some(Pivot::BottomRight)
        } else {
            Some(Pivot::BottomCenter)
        }
    } else if x == rect.left() {
        Some(Pivot::LeftCenter)
    } else if x == rect.right() {
        Some(Pivot::RightCenter)
    } else {
        None
    }
}

pub struct TerminalBorderMut<'a> {
    border: &'a mut Border,
    term_size: IVec2,
    clear_tile: Tile,
}

impl<'a> TerminalBorderMut<'a> {
    pub fn new(border: &'a mut Border, term_size: IVec2, clear_tile: Tile) -> Self {
        Self {
            border,
            term_size,
            clear_tile,
        }
    }
    pub fn put_string(
        &mut self,
        edge: Pivot,
        direction: Dir4,
        offset: i32,
        string: impl Into<FormattedString<'a>>,
    ) -> &mut Self {
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

    pub fn put_title(&mut self, string: impl Into<FormattedString<'a>>) -> &mut Self {
        self.put_string(Pivot::TopLeft, Dir4::Right, 1, string);
        self
    }

    /// Set all the border tile colors to match the terminal's clear tile
    pub fn clear_colors(&'a mut self) -> &'a mut Self {
        let clear = self.clear_tile;
        for tile in self.border.tiles.values_mut() {
            tile.fg_color = clear.fg_color;
            tile.bg_color = clear.bg_color;
        }
        self
    }

    /// Remove any text written to the border.
    pub fn clear_strings(&'a mut self) -> &'a mut Self {
        self.border.tiles.clear();
        self
    }

    
    pub(crate) fn reset_changed_state(&mut self) {
        self.border.changed = false;
    }
}

#[cfg(test)]
mod tests {
    use bevy::{math::IVec2, render::color::Color};

    use crate::{string::StringFormatter, GridPoint, GridRect, Tile};

    use super::{Border, TerminalBorderMut};

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
