use crate::color::TileColor;

const MAX: u8 = u8::MAX;
const HALF: u8 = 128;

pub const RED: TileColor = TileColor {
    r: MAX,
    g: 0,
    b: 0,
    a: MAX,
};
pub const GREEN: TileColor = TileColor {
    r: 0,
    g: MAX,
    b: 0,
    a: MAX,
};
pub const BLUE: TileColor = TileColor {
    r: 0,
    g: 0,
    b: MAX,
    a: MAX,
};
pub const BLACK: TileColor = TileColor {
    r: 0,
    g: 0,
    b: 0,
    a: MAX,
};
pub const WHITE: TileColor = TileColor {
    r: MAX,
    g: MAX,
    b: MAX,
    a: MAX,
};
pub const GREY: TileColor = TileColor {
    r: HALF,
    g: HALF,
    b: HALF,
    a: MAX,
};
