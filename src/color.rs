use std::{fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use bevy::{math::{IVec4, Vec4}, prelude::{Color}};

pub use super::color_blend::ColorBlend;
pub use super::colors::*;

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Display for TerminalColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

impl TerminalColor {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r,g,b,u8::MAX)
    }

    pub fn rgba_f32(r: f32, g: f32, b: f32) -> Self {
        [r,g,b,1.0].into()
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }

    /// Component-wise min
    pub fn min(&self, b: &TerminalColor) -> Self {
        Self {
            r: self.r.min(b.r),
            g: self.g.min(b.g),
            b: self.b.min(b.b),
            a: self.a.min(b.a),
        }
    }

    /// Component-wise min
    pub fn max(&self, b: &TerminalColor) -> Self {
        Self {
            r: self.r.max(b.r),
            g: self.g.max(b.g),
            b: self.b.max(b.b),
            a: self.a.max(b.a),
        }
    }

    pub fn lerp(self, rhs: TerminalColor, t: f32) -> TerminalColor {
        let lhs: Vec4 = self.into();
        let rhs: Vec4 = rhs.into();
        (lhs + (rhs - lhs) * t).into()
    } 
}

/// Note the resulting values are NOT normalized.
impl From<TerminalColor> for Vec4 {
    fn from(c: TerminalColor) -> Self {
        Vec4::new(
            c.r as f32,
            c.g as f32,
            c.b as f32,
            c.a as f32,
        )
    }
}

impl From<Vec4> for TerminalColor {
    fn from(v: Vec4) -> Self {
        let v = v.round();
        TerminalColor {
            r: v.x as u8,
            g: v.y as u8,
            b: v.z as u8,
            a: v.w as u8,
        }
    }
}

impl From<TerminalColor> for IVec4 {
    fn from(c: TerminalColor) -> Self {
        Self::new(
            c.r as i32,
            c.g as i32,
            c.b as i32,
            c.a as i32,
        )
    }
}

impl From<IVec4> for TerminalColor {
    fn from(v: IVec4) -> Self {
        Self {
            r: v.x  as u8,
            g: v.y  as u8,
            b: v.z  as u8,
            a: v.w  as u8,
        }
    }
}

impl From<[f32;4]> for TerminalColor {
    fn from(c: [f32;4]) -> Self {
        TerminalColor {
            r: (c[0] * 255.0) as u8,
            g: (c[1] * 255.0) as u8,
            b: (c[2] * 255.0) as u8,
            a: (c[3] * 255.0) as u8,
        }
    }
}

impl From<TerminalColor> for [f32;4] {
    fn from(c: TerminalColor) -> Self {
        [
            c.r as f32,
            c.g as f32,
            c.b as f32,
            c.a as f32,
        ]
    }
}

impl Mul<TerminalColor> for TerminalColor {
    type Output = TerminalColor;

    fn mul(mut self, rhs: TerminalColor) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<TerminalColor> for TerminalColor {
    fn mul_assign(&mut self, rhs: TerminalColor) {
        self.r = self.r.saturating_mul(rhs.r);
        self.g = self.g.saturating_mul(rhs.g);
        self.b = self.b.saturating_mul(rhs.b);
    }
}

impl Add<u8> for TerminalColor {
    type Output = TerminalColor;

    fn add(mut self, rhs: u8) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<u8> for TerminalColor {
    fn add_assign(&mut self, rhs: u8) {
        self.r = self.r.saturating_add(rhs);
        self.g = self.g.saturating_add(rhs);
        self.b = self.b.saturating_add(rhs);
    }
}

impl Sub<u8> for TerminalColor {
    type Output = TerminalColor;

    fn sub(mut self, rhs: u8) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<u8> for TerminalColor {
    fn sub_assign(&mut self, rhs: u8) {
        self.r = self.r.saturating_sub(rhs);
        self.g = self.g.saturating_sub(rhs);
        self.b = self.b.saturating_sub(rhs);
    }
}

impl Div<u8> for TerminalColor {
    type Output = TerminalColor;

    fn div(mut self, rhs: u8) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<u8> for TerminalColor {
    fn div_assign(&mut self, rhs: u8) {
        self.r = self.r / rhs;
        self.g = self.g / rhs;
        self.b = self.b / rhs;
    }
}

impl Sub<TerminalColor> for u8 {
    type Output = TerminalColor;

    fn sub(self, rhs: TerminalColor) -> Self::Output {
        TerminalColor::rgb(self, self, self) - rhs
    }
}

impl Sub<TerminalColor> for TerminalColor {
    type Output = TerminalColor;

    fn sub(mut self, rhs: TerminalColor) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<TerminalColor> for TerminalColor {
    fn sub_assign(&mut self, rhs: TerminalColor) {
        self.r = self.r.saturating_sub(rhs.r);
        self.g = self.g.saturating_sub(rhs.g);
        self.b = self.b.saturating_sub(rhs.b);
    }
}

impl Div<TerminalColor> for TerminalColor {
    type Output = TerminalColor;

    fn div(mut self, rhs: TerminalColor) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<TerminalColor> for TerminalColor {
    fn div_assign(&mut self, rhs: TerminalColor) {
        self.r = self.r.checked_div(rhs.r).unwrap_or_default();
        self.g = self.g.checked_div(rhs.g).unwrap_or_default();
        self.b = self.b.checked_div(rhs.b).unwrap_or_default();
    }
}

impl Mul<f32> for TerminalColor {
    type Output = TerminalColor;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut col:Vec4 = self.into();
        col *= rhs;
        TerminalColor::from(col)
    }
}

impl Add<TerminalColor> for TerminalColor {
    type Output = TerminalColor;

    fn add(mut self, rhs: TerminalColor) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign<TerminalColor> for TerminalColor {
    fn add_assign(&mut self, rhs: TerminalColor) {
        self.r = self.r.saturating_add(rhs.r);
        self.g = self.g.saturating_add(rhs.g);
        self.b = self.b.saturating_add(rhs.b);
    }
}


pub fn rgb(r: u8, g: u8, b: u8) -> TerminalColor {
    TerminalColor::rgba(r,g,b,u8::MAX)
}


impl From<Color> for TerminalColor {
    fn from(c: Color) -> Self {
        //let rgba = c.as_rgba_f32();
        let rgba = c.as_linear_rgba_f32();
        TerminalColor { 
            r: (rgba[0] * 255.0) as u8, 
            g: (rgba[1] * 255.0) as u8, 
            b: (rgba[2] * 255.0) as u8, 
            a: (rgba[3] * 255.0) as u8, 
        }
    }
}

impl From<TerminalColor> for [u8;4] {
    fn from(c: TerminalColor) -> Self {
        [c.r,c.g,c.b,c.a]
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

        let c: TerminalColor = v.into();
        
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
        let a = TerminalColor::rgb(15,15,15);
        assert_eq!(TerminalColor::rgb(30,30,30), a + a);
        let a = TerminalColor::rgb(200,200,200);
        assert_eq!(WHITE, a + a);
    }

    #[test]
    fn sub() {
        let a = TerminalColor::rgb(15,15,15);
        assert_eq!(BLACK, a - a );
        assert_eq!(BLACK, a - a - a);
    }
}
