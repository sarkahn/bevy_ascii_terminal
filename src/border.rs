use std::collections::BTreeMap;

use bevy::math::IVec2;

use crate::{Dir4, Tile};

pub struct BorderState {
    pub(super) edge_glyphs: [char; 8],
    pub(super) tiles: BTreeMap<(i32, i32), Tile>,
    pub(crate) changed: bool,
}

impl BorderState {
    pub(crate) fn from_string(string: impl AsRef<str>) -> Self {
        let mut glyphs = [' '; 8];
        for (i, ch) in string.as_ref().chars().enumerate() {
            glyphs[i] = ch;
        }
        Self {
            edge_glyphs: glyphs,
            tiles: BTreeMap::new(),
            changed: true,
        }
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
