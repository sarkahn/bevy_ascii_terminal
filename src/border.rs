use std::collections::BTreeMap;

use bevy::{math::IVec2, render::color::Color};

use crate::{string::StringFormatter, Dir4, FormattedString, GridPoint, GridRect, Pivot, Tile};

pub struct BorderState {
    pub(super) edge_glyphs: [char; 8],
    pub(super) tiles: BTreeMap<(i32, i32), Tile>,
    pub(crate) changed: bool,
}

impl Default for BorderState {
    fn default() -> Self {
        Self {
            edge_glyphs: [' '; 8],
            tiles: Default::default(),
            changed: true,
        }
    }
}

impl BorderState {
    pub(crate) fn from_string(string: impl AsRef<str>) -> Self {
        let mut border = Self::default();
        for (i, ch) in string.as_ref().chars().enumerate() {
            border.edge_glyphs[i] = ch;
        }
        border
    }

    // pub fn put_string(
    //     &mut self,
    //     xy: impl Into<IVec2>,
    //     direction: Dir4,
    //     string: impl AsRef<str>,
    // ) -> &mut Self {
    //     let string = string.as_ref();

    //     self
    // }

    // pub fn put_title(&mut self, string: impl AsRef<str>) -> &mut Self {
    //     let y = self.terminal_size.y + 1;

    //     self.put_string([0, y], Dir4::Right, string)
    // }
}

pub struct TerminalBorder<'a> {
    pub(crate) border: &'a mut BorderState,
    pub(crate) term_size: IVec2,
    pub(crate) clear_tile: Tile,
}

impl<'a> TerminalBorder<'a> {
    pub fn put_string(
        &mut self,
        edge: Pivot,
        direction: Dir4,
        offset: i32,
        string: impl Into<FormattedString<'a>>,
    ) -> &mut Self {
        let mut xy = edge.size_offset(self.term_size);
        let fmt: FormattedString = string.into();
        let fg_color = fmt.fg_color.unwrap_or(self.clear_tile.fg_color);
        let bg_color = fmt.bg_color.unwrap_or(self.clear_tile.bg_color);

        let dir = direction.as_ivec2();
        let rect = GridRect::new([-1, -1], self.term_size + 1);
        for ch in fmt.string.chars() {
            if !rect.contains_point(xy) {
                break;
            }
            if ch != ' ' {
                let [x, y] = xy.as_array();
                // Note tile positions are stored y-first for proper left-to-right, down-to-up sorting
                self.border.tiles.insert(
                    (y, x),
                    Tile {
                        glyph: ch,
                        fg_color,
                        bg_color,
                    },
                );
            }

            xy += dir;
        }
        self
    }

    pub fn put_title(&mut self, string: impl Into<FormattedString<'a>>) -> &mut Self {
        self.put_string(Pivot::TopLeft, Dir4::Right, 1, string);
        self
    }

    pub fn clear_colors(&'a mut self) -> &'a mut Self {
        let clear = self.clear_tile;
        for tile in self.border.tiles.values_mut() {
            tile.fg_color = clear.fg_color;
            tile.bg_color = clear.bg_color;
        }
        self
    }
}

pub struct BorderTiles<'a> {
    border: TerminalBorder<'a>,
    xy: IVec2,
    edge: Pivot,
    dir: Dir4,
    offset: i32,
}

pub struct BorderString<'a> {
    string: &'a str,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
}

impl<'a> BorderString<'a> {
    pub fn fg(&'a mut self, color: Color) -> &'a mut Self {
        self.fg_color = Some(color);
        self
    }
    pub fn bg(&'a mut self, color: Color) -> &'a mut Self {
        self.bg_color = Some(color);
        self
    }
}

impl<'a> From<&'static str> for BorderString<'a> {
    fn from(value: &'static str) -> Self {
        BorderString {
            string: value,
            fg_color: None,
            bg_color: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::{math::IVec2, render::color::Color};

    use crate::{string::StringFormatter, Tile};

    use super::{BorderState, TerminalBorder};

    fn put_string() {
        let mut state = BorderState::default();
        let mut term_border = TerminalBorder {
            border: &mut state,
            term_size: IVec2::splat(20),
            clear_tile: Tile::DEFAULT,
        };
        term_border.put_title("Hello".fg(Color::BLUE).bg(Color::RED));
    }
}
