use std::ops::RangeBounds;

use bevy::math::{Vec2, IVec2, UVec2};
use itertools::Itertools;

use crate::{formatting::Pivot, grid_rect::GridRect};

pub struct StringFormatter<'a> {
    pivot: Pivot,
    string: &'a str,
}

impl<'a> StringFormatter<'a> {
    pub fn from_pivot(&mut self, alignment: Pivot) {
        self.pivot = alignment;
    }

    // pub fn write(&self, rect: GridRect) -> impl Iterator<Item=(IVec2,char)> + 'a {
    //     let pivot = self.pivot;
    //     let iter = rect.iter_from_pivot(pivot);
    //     let chars = self.string.chars();
    //     let len = self.string.len();
    //     if pivot == Pivot::TopRight || pivot == Pivot::BottomRight {
    //         return iter.take(len).zip(chars.rev())
    //     }
    //     iter.take(len).zip(chars)
    // }
}

#[cfg(test)]
mod test {
    use crate::{formatting::Pivot, grid_rect::GridRect};

    use super::StringFormatter;

    #[test]
    fn write() {
        let str = "Hello".to_string();

        let formatter = StringFormatter {
            pivot: Pivot::TopRight,
            string: &str,
        };

        let rect = GridRect::new([3,3], [8,2]);
        //let iter = formatter.write(rect);

        // for (p,c) in iter {
        //     println!("{}: {}", p, c);
        // }
    }
}