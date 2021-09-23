use super::color::*;

/// Methods to blend two colors together
pub enum ColorBlend {
    /// Set to new color, ignore old
    Set,
    Multiply,
    Lighten,
    Darken,
    Screen,
    ColorDodge,
    Add,
    Burn,
    Overlay,
    AddAlpha,
}

impl Default for ColorBlend {
    fn default() -> Self {
        ColorBlend::Set
    }
}

fn overlay(a: u8, b: u8) -> u8 {
    let a = a as i32;
    let b = b as i32;
    let a = if b <= 128 {
        2 * b * a / 255
    } else {
        255 - 2 * (255 - b) * (255 - a) / 255
    };
    a as u8
}

fn add_alpha(a: u8, b: u8, alpha: u8) -> u8 {
    let a = a as i32;
    let b = b as i32;
    let alpha = alpha as i32;
    (b + alpha * a / 255) as u8
}

impl ColorBlend {
    pub fn blend(&self, old: TerminalColor, new: TerminalColor) -> TerminalColor {
        match self {
            ColorBlend::Set => new,
            ColorBlend::Multiply => old * new,
            ColorBlend::Lighten => old.max(&new),
            ColorBlend::Darken => old.min(&new),
            ColorBlend::Screen => WHITE - (WHITE - new) * (WHITE - old),
            ColorBlend::ColorDodge => {
                if new == WHITE {
                    new
                } else {
                    WHITE * new / (WHITE - new)
                }
            }
            ColorBlend::Add => old + new,
            ColorBlend::Burn => old + new - WHITE,
            ColorBlend::Overlay => TerminalColor::rgb(
                overlay(old.r, new.r),
                overlay(old.g, new.g),
                overlay(old.b, new.b),
            ),
            ColorBlend::AddAlpha => {
                let alpha = old.a;
                TerminalColor::rgb(
                    add_alpha(old.r, new.r, alpha),
                    add_alpha(old.g, new.g, alpha),
                    add_alpha(old.b, new.b, alpha),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {}
