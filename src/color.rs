use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use bevy::{
    math::{IVec4, Vec4},
    prelude::Color,
};

pub use super::color_blend::ColorBlend;
pub use super::colors::*;

/// Color for terminal tiles.
#[derive(Default, Clone, Copy, Debug, Eq, PartialEq)]
pub struct TileColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Display for TileColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

impl TileColor {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, u8::MAX)
    }

    /// Values are assumed to be in the [0.0..255.0] range
    pub fn rgba_f32(r: f32, g: f32, b: f32) -> Self {
        [r, g, b, 255.0].into()
    }

    /// Values are assumed to be in the [0..1] range
    pub fn rgba_f32_normalized(r: f32, g: f32, b: f32) -> Self {
        [r * 255.0, g * 255.0, b * 255.0, 255.0].into()
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Component-wise min
    pub fn min(&self, b: &TileColor) -> Self {
        Self {
            r: self.r.min(b.r),
            g: self.g.min(b.g),
            b: self.b.min(b.b),
            a: self.a.min(b.a),
        }
    }

    /// Component-wise max
    pub fn max(&self, b: &TileColor) -> Self {
        Self {
            r: self.r.max(b.r),
            g: self.g.max(b.g),
            b: self.b.max(b.b),
            a: self.a.max(b.a),
        }
    }

    pub fn lerp(self, rhs: TileColor, t: f32) -> TileColor {
        let lhs: Vec4 = self.into();
        let rhs: Vec4 = rhs.into();
        (lhs + (rhs - lhs) * t).into()
    }
}

/// Note the resulting values will be in the range of [0.0..255.0]
impl From<TileColor> for Vec4 {
    fn from(c: TileColor) -> Self {
        Vec4::new(c.r as f32, c.g as f32, c.b as f32, c.a as f32)
    }
}

impl From<Vec4> for TileColor {
    fn from(v: Vec4) -> Self {
        let v = v.round();
        TileColor {
            r: v.x as u8,
            g: v.y as u8,
            b: v.z as u8,
            a: v.w as u8,
        }
    }
}

impl From<TileColor> for IVec4 {
    fn from(c: TileColor) -> Self {
        Self::new(c.r as i32, c.g as i32, c.b as i32, c.a as i32)
    }
}

impl From<IVec4> for TileColor {
    fn from(v: IVec4) -> Self {
        Self {
            r: v.x as u8,
            g: v.y as u8,
            b: v.z as u8,
            a: v.w as u8,
        }
    }
}

impl From<[f32; 4]> for TileColor {
    fn from(c: [f32; 4]) -> Self {
        TileColor {
            r: c[0] as u8,
            g: c[1] as u8,
            b: c[2] as u8,
            a: c[3] as u8,
        }
    }
}

impl From<TileColor> for [f32; 4] {
    fn from(c: TileColor) -> Self {
        [c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0, c.a as f32 / 255.0]
    }
}

impl Mul<TileColor> for TileColor {
    type Output = TileColor;

    fn mul(mut self, rhs: TileColor) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<TileColor> for TileColor {
    fn mul_assign(&mut self, rhs: TileColor) {
        self.r = self.r.saturating_mul(rhs.r);
        self.g = self.g.saturating_mul(rhs.g);
        self.b = self.b.saturating_mul(rhs.b);
    }
}

impl Add<u8> for TileColor {
    type Output = TileColor;

    fn add(mut self, rhs: u8) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<u8> for TileColor {
    fn add_assign(&mut self, rhs: u8) {
        self.r = self.r.saturating_add(rhs);
        self.g = self.g.saturating_add(rhs);
        self.b = self.b.saturating_add(rhs);
    }
}

impl Sub<u8> for TileColor {
    type Output = TileColor;

    fn sub(mut self, rhs: u8) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<u8> for TileColor {
    fn sub_assign(&mut self, rhs: u8) {
        self.r = self.r.saturating_sub(rhs);
        self.g = self.g.saturating_sub(rhs);
        self.b = self.b.saturating_sub(rhs);
    }
}

impl Div<u8> for TileColor {
    type Output = TileColor;

    fn div(mut self, rhs: u8) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<u8> for TileColor {
    fn div_assign(&mut self, rhs: u8) {
        self.r = self.r / rhs;
        self.g = self.g / rhs;
        self.b = self.b / rhs;
    }
}

impl Sub<TileColor> for u8 {
    type Output = TileColor;

    fn sub(self, rhs: TileColor) -> Self::Output {
        TileColor::rgb(self, self, self) - rhs
    }
}

impl Sub<TileColor> for TileColor {
    type Output = TileColor;

    fn sub(mut self, rhs: TileColor) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<TileColor> for TileColor {
    fn sub_assign(&mut self, rhs: TileColor) {
        self.r = self.r.saturating_sub(rhs.r);
        self.g = self.g.saturating_sub(rhs.g);
        self.b = self.b.saturating_sub(rhs.b);
    }
}

impl Div<TileColor> for TileColor {
    type Output = TileColor;

    fn div(mut self, rhs: TileColor) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<TileColor> for TileColor {
    fn div_assign(&mut self, rhs: TileColor) {
        self.r = self.r.checked_div(rhs.r).unwrap_or_default();
        self.g = self.g.checked_div(rhs.g).unwrap_or_default();
        self.b = self.b.checked_div(rhs.b).unwrap_or_default();
    }
}

impl Mul<f32> for TileColor {
    type Output = TileColor;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut col: Vec4 = self.into();
        col *= rhs;
        TileColor::from(col)
    }
}

impl Add<TileColor> for TileColor {
    type Output = TileColor;

    fn add(mut self, rhs: TileColor) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<TileColor> for TileColor {
    fn add_assign(&mut self, rhs: TileColor) {
        self.r = self.r.saturating_add(rhs.r);
        self.g = self.g.saturating_add(rhs.g);
        self.b = self.b.saturating_add(rhs.b);
    }
}

pub fn rgb(r: u8, g: u8, b: u8) -> TileColor {
    TileColor::rgba(r, g, b, u8::MAX)
}

impl From<Color> for TileColor {
    fn from(c: Color) -> Self {
        //let rgba = c.as_rgba_f32();
        let rgba = c.as_linear_rgba_f32();
        TileColor {
            r: (rgba[0] * 255.0) as u8,
            g: (rgba[1] * 255.0) as u8,
            b: (rgba[2] * 255.0) as u8,
            a: (rgba[3] * 255.0) as u8,
        }
    }
}

impl From<TileColor> for [u8; 4] {
    fn from(c: TileColor) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec4;

    use crate::color::*;

    #[test]
    fn minmax() {
        let r = RED;
        let g = GREEN;
        let b = BLUE;

        let min = r.min(&g).min(&b);
        let max = r.max(&g).max(&b);

        assert_eq!(min, BLACK);
        assert_eq!(max, WHITE);
    }

    #[test]
    fn to_vec() {
        let v: Vec4 = RED.into();

        assert_eq!(Vec4::new(255.0, 0.0, 0.0, 255.0), v);

        let c: TileColor = v.into();

        assert_eq!(RED, c);
    }

    #[test]
    fn mul() {
        let r = RED * 0.5;
        let g = GREEN * 0.5;
        let b = BLUE * 0.5;

        assert_eq!(r.r, 128);
        assert_eq!(g.g, 128);
        assert_eq!(b.b, 128);
    }

    #[test]
    fn lerp() {
        assert_eq!(WHITE, WHITE.lerp(BLACK, 0.0));
        assert_eq!(BLACK, WHITE.lerp(BLACK, 1.0));
        assert_eq!(GREY, WHITE.lerp(BLACK, 0.5));
    }

    #[test]
    fn add() {
        let a = TileColor::rgb(15, 15, 15);
        assert_eq!(TileColor::rgb(30, 30, 30), a + a);
        let a = TileColor::rgb(200, 200, 200);
        assert_eq!(WHITE, a + a);
    }

    #[test]
    fn sub() {
        let a = TileColor::rgb(15, 15, 15);
        assert_eq!(BLACK, a - a);
        assert_eq!(BLACK, a - a - a);
    }
}
