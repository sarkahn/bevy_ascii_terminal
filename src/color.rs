//! Utilities for converting srgba colors to linear colors in a const context.
//! All terminal colors are stored as [LinearRgba] to avoid runtime conversions when
//! passing color data to the shader, which is twice (foreground and background)
//! for every single terminal tile.

use bevy::{color::LinearRgba, platform::collections::HashMap, reflect::Reflect};

/// Convert an Srgba hex color to [LinearRgba]. Works at compile time.
pub const fn srgba_hex(hex: u32) -> LinearRgba {
    let rgba = if hex < 0xFFFFFFFF {
        hex << 8 | 0xFF
    } else {
        hex
    };

    let r = (rgba >> 24 & 0xFF) as u8;
    let g = (rgba >> 16 & 0xFF) as u8;
    let b = (rgba >> 8 & 0xFF) as u8;
    let a = (rgba & 0xFF) as u8;

    srgba_bytes(r, g, b, a)
}

/// Convert srgb bytes to [LinearRgba]. Works at compile time.
pub const fn srgb_bytes(r: u8, g: u8, b: u8) -> LinearRgba {
    srgba_bytes(r, g, b, 255)
}

/// Convert srgba color bytes into a LinearRgba color. Works at compile time.
pub const fn srgba_bytes(r: u8, g: u8, b: u8, a: u8) -> LinearRgba {
    LinearRgba {
        red: SRGB_TO_LINEAR[r as usize],
        green: SRGB_TO_LINEAR[g as usize],
        blue: SRGB_TO_LINEAR[b as usize],
        alpha: SRGB_TO_LINEAR[a as usize],
    }
}

pub fn parse_color_string(input: &str) -> LinearRgba {
    try_parse_color_string(input)
        .expect("Ascii color strings must be ascii and fewer than 32 characters")
}

/// Parse a color string. Accepts: #rrggbb, rrggbb, 0xrrggbb, or a named color
/// from the list of css colors. Case insensitive.
pub fn try_parse_color_string(input: &str) -> Option<LinearRgba> {
    let s = input.trim();

    // --- Try parsing as hex ---
    // Allow: "#RRGGBB", "0xRRGGBB", "RRGGBB"
    let hex_str = s
        .strip_prefix('#')
        .or_else(|| s.strip_prefix("0x"))
        .unwrap_or(s);

    if hex_str.len() == 6
        && hex_str.chars().all(|c| c.is_ascii_hexdigit())
        && let Ok(v) = u32::from_str_radix(hex_str, 16)
    {
        return Some(srgba_hex(v));
    }

    let mut buffer: [u8; 32] = [0; 32];
    debug_assert!(input.is_ascii(), "Color tag names must be ascii");
    debug_assert!(
        input.len() < 32,
        "Color tag names must be 32 characters or less"
    );
    for (i, c) in input.as_bytes().iter().enumerate() {
        buffer[i] = c.to_ascii_uppercase();
    }
    let up = unsafe { std::str::from_utf8_unchecked(&buffer[0..input.len()]) };

    use css::*;
    Some(match up {
        "WHITE" => WHITE,
        "BLACK" => BLACK,
        "RED" => RED,
        "GREEN" => GREEN,
        "BLUE" => BLUE,
        "ALICE_BLUE" => ALICE_BLUE,
        "ANTIQUE_WHITE" => ANTIQUE_WHITE,
        "AQUA" => AQUA,
        "AQUAMARINE" => AQUAMARINE,
        "AZURE" => AZURE,
        "BEIGE" => BEIGE,
        "BISQUE" => BISQUE,
        "BLANCHED_ALMOND" => BLANCHED_ALMOND,
        "BLUE_VIOLET" => BLUE_VIOLET,
        "BROWN" => BROWN,
        "BURLY_WOOD" => BURLY_WOOD,
        "CADET_BLUE" => CADET_BLUE,
        "CHARTREUSE" => CHARTREUSE,
        "CHOCOLATE" => CHOCOLATE,
        "CORAL" => CORAL,
        "CORNFLOWER_BLUE" => CORNFLOWER_BLUE,
        "CORNSILK" => CORNSILK,
        "CRIMSON" => CRIMSON,
        "CYAN" => CYAN,
        "DARK_BLUE" => DARK_BLUE,
        "DARK_CYAN" => DARK_CYAN,
        "DARK_GOLDENROD" => DARK_GOLDENROD,
        "DARK_GRAY" => DARK_GRAY,
        "DARK_GREEN" => DARK_GREEN,
        "DARK_GREY" => DARK_GREY,
        "DARK_KHAKI" => DARK_KHAKI,
        "DARK_MAGENTA" => DARK_MAGENTA,
        "DARK_OLIVE_GREEN" => DARK_OLIVE_GREEN,
        "DARK_ORANGE" => DARK_ORANGE,
        "DARK_ORCHID" => DARK_ORCHID,
        "DARK_RED" => DARK_RED,
        "DARK_SALMON" => DARK_SALMON,
        "DARK_SEA_GREEN" => DARK_SEA_GREEN,
        "DARK_SLATE_BLUE" => DARK_SLATE_BLUE,
        "DARK_SLATE_GRAY" => DARK_SLATE_GRAY,
        "DARK_SLATE_GREY" => DARK_SLATE_GREY,
        "DARK_TURQUOISE" => DARK_TURQUOISE,
        "DARK_VIOLET" => DARK_VIOLET,
        "DEEP_PINK" => DEEP_PINK,
        "DEEP_SKY_BLUE" => DEEP_SKY_BLUE,
        "DIM_GRAY" => DIM_GRAY,
        "DIM_GREY" => DIM_GREY,
        "DODGER_BLUE" => DODGER_BLUE,
        "FIREBRICK" => FIREBRICK,
        "FLORAL_WHITE" => FLORAL_WHITE,
        "FOREST_GREEN" => FOREST_GREEN,
        "FUCHSIA" => FUCHSIA,
        "GAINSBORO" => GAINSBORO,
        "GHOST_WHITE" => GHOST_WHITE,
        "GOLD" => GOLD,
        "GOLDENROD" => GOLDENROD,
        "GRAY" => GRAY,
        "GREEN_YELLOW" => GREEN_YELLOW,
        "GREY" => GREY,
        "HONEYDEW" => HONEYDEW,
        "HOT_PINK" => HOT_PINK,
        "INDIAN_RED" => INDIAN_RED,
        "INDIGO" => INDIGO,
        "IVORY" => IVORY,
        "KHAKI" => KHAKI,
        "LAVENDER" => LAVENDER,
        "LAVENDER_BLUSH" => LAVENDER_BLUSH,
        "LAWN_GREEN" => LAWN_GREEN,
        "LEMON_CHIFFON" => LEMON_CHIFFON,
        "LIGHT_BLUE" => LIGHT_BLUE,
        "LIGHT_CORAL" => LIGHT_CORAL,
        "LIGHT_CYAN" => LIGHT_CYAN,
        "LIGHT_GOLDENROD_YELLOW" => LIGHT_GOLDENROD_YELLOW,
        "LIGHT_GRAY" => LIGHT_GRAY,
        "LIGHT_GREEN" => LIGHT_GREEN,
        "LIGHT_GREY" => LIGHT_GREY,
        "LIGHT_PINK" => LIGHT_PINK,
        "LIGHT_SALMON" => LIGHT_SALMON,
        "LIGHT_SEA_GREEN" => LIGHT_SEA_GREEN,
        "LIGHT_SKY_BLUE" => LIGHT_SKY_BLUE,
        "LIGHT_SLATE_GRAY" => LIGHT_SLATE_GRAY,
        "LIGHT_SLATE_GREY" => LIGHT_SLATE_GREY,
        "LIGHT_STEEL_BLUE" => LIGHT_STEEL_BLUE,
        "LIGHT_YELLOW" => LIGHT_YELLOW,
        "LIME" => LIME,
        "LIME_GREEN" => LIME_GREEN,
        "LINEN" => LINEN,
        "MAGENTA" => MAGENTA,
        "MAROON" => MAROON,
        "MEDIUM_AQUAMARINE" => MEDIUM_AQUAMARINE,
        "MEDIUM_BLUE" => MEDIUM_BLUE,
        "MEDIUM_ORCHID" => MEDIUM_ORCHID,
        "MEDIUM_PURPLE" => MEDIUM_PURPLE,
        "MEDIUM_SEA_GREEN" => MEDIUM_SEA_GREEN,
        "MEDIUM_SLATE_BLUE" => MEDIUM_SLATE_BLUE,
        "MEDIUM_SPRING_GREEN" => MEDIUM_SPRING_GREEN,
        "MEDIUM_TURQUOISE" => MEDIUM_TURQUOISE,
        "MEDIUM_VIOLET_RED" => MEDIUM_VIOLET_RED,
        "MIDNIGHT_BLUE" => MIDNIGHT_BLUE,
        "MINT_CREAM" => MINT_CREAM,
        "MISTY_ROSE" => MISTY_ROSE,
        "MOCCASIN" => MOCCASIN,
        "NAVAJO_WHITE" => NAVAJO_WHITE,
        "NAVY" => NAVY,
        "OLD_LACE" => OLD_LACE,
        "OLIVE" => OLIVE,
        "OLIVE_DRAB" => OLIVE_DRAB,
        "ORANGE" => ORANGE,
        "ORANGE_RED" => ORANGE_RED,
        "ORCHID" => ORCHID,
        "PALE_GOLDENROD" => PALE_GOLDENROD,
        "PALE_GREEN" => PALE_GREEN,
        "PALE_TURQUOISE" => PALE_TURQUOISE,
        "PALE_VIOLET_RED" => PALE_VIOLET_RED,
        "PAPAYA_WHIP" => PAPAYA_WHIP,
        "PEACH_PUFF" => PEACH_PUFF,
        "PERU" => PERU,
        "PINK" => PINK,
        "PLUM" => PLUM,
        "POWDER_BLUE" => POWDER_BLUE,
        "PURPLE" => PURPLE,
        "REBECCA_PURPLE" => REBECCA_PURPLE,
        "ROSY_BROWN" => ROSY_BROWN,
        "ROYAL_BLUE" => ROYAL_BLUE,
        "SADDLE_BROWN" => SADDLE_BROWN,
        "SALMON" => SALMON,
        "SANDY_BROWN" => SANDY_BROWN,
        "SEA_GREEN" => SEA_GREEN,
        "SEA_SHELL" => SEA_SHELL,
        "SIENNA" => SIENNA,
        "SILVER" => SILVER,
        "SKY_BLUE" => SKY_BLUE,
        "SLATE_BLUE" => SLATE_BLUE,
        "SLATE_GRAY" => SLATE_GRAY,
        "SLATE_GREY" => SLATE_GREY,
        "SNOW" => SNOW,
        "SPRING_GREEN" => SPRING_GREEN,
        "STEEL_BLUE" => STEEL_BLUE,
        "TAN" => TAN,
        "TEAL" => TEAL,
        "THISTLE" => THISTLE,
        "TOMATO" => TOMATO,
        "TURQUOISE" => TURQUOISE,
        "VIOLET" => VIOLET,
        "WHEAT" => WHEAT,
        "WHITE_SMOKE" => WHITE_SMOKE,
        "YELLOW" => YELLOW,
        "YELLOW_GREEN" => YELLOW_GREEN,

        _ => return None,
    })
}

/// Convert a srgba hex string to a linear color. Works at compile time.
pub const fn from_hex_string(input: &str) -> LinearRgba {
    let bytes = input.as_bytes();
    let mut start = 0;

    if !bytes.is_empty() && bytes[0] == b'#' {
        start = 1;
    } else if bytes.len() >= 2 && bytes[0] == b'0' && bytes[1] == b'x' {
        start = 2;
    }

    let len = bytes.len() - start;
    if len <= 8 {
        let mut value: u32 = 0;
        let mut i = 0;

        while i < len {
            let c = bytes[start + i];
            let digit = match hex_val(c) {
                Some(v) => v,
                None => break,
            };
            value = (value << 4) | digit as u32;
            i += 1;
        }

        return srgba_hex(value);
    }

    panic!("Unable to parse color string");
}

const fn hex_val(c: u8) -> Option<u8> {
    if c >= b'0' && c <= b'9' {
        Some(c - b'0')
    } else if c >= b'a' && c <= b'f' {
        Some(c - b'a' + 10)
    } else if c >= b'A' && c <= b'F' {
        Some(c - b'A' + 10)
    } else {
        None
    }
}

pub mod css {
    use super::srgba_hex;
    use bevy::color::LinearRgba;

    pub const ALICE_BLUE: LinearRgba = srgba_hex(0xF0F8FF);
    pub const ANTIQUE_WHITE: LinearRgba = srgba_hex(0xFAEBD7);
    pub const AQUA: LinearRgba = srgba_hex(0x00FFFF);
    pub const AQUAMARINE: LinearRgba = srgba_hex(0x7FFFD4);
    pub const AZURE: LinearRgba = srgba_hex(0xF0FFFF);
    pub const BEIGE: LinearRgba = srgba_hex(0xF5F5DC);
    pub const BISQUE: LinearRgba = srgba_hex(0xFFE4C4);
    pub const BLACK: LinearRgba = srgba_hex(0x000000);
    pub const BLANCHED_ALMOND: LinearRgba = srgba_hex(0xFFEBCD);
    pub const BLUE: LinearRgba = srgba_hex(0x0000FF);
    pub const BLUE_VIOLET: LinearRgba = srgba_hex(0x8A2BE2);
    pub const BROWN: LinearRgba = srgba_hex(0xA52A2A);
    pub const BURLY_WOOD: LinearRgba = srgba_hex(0xDEB887);
    pub const CADET_BLUE: LinearRgba = srgba_hex(0x5F9EA0);
    pub const CHARTREUSE: LinearRgba = srgba_hex(0x7FFF00);
    pub const CHOCOLATE: LinearRgba = srgba_hex(0xD2691E);
    pub const CORAL: LinearRgba = srgba_hex(0xFF7F50);
    pub const CORNFLOWER_BLUE: LinearRgba = srgba_hex(0x6495ED);
    pub const CORNSILK: LinearRgba = srgba_hex(0xFFF8DC);
    pub const CRIMSON: LinearRgba = srgba_hex(0xDC143C);
    pub const CYAN: LinearRgba = srgba_hex(0x00FFFF);
    pub const DARK_BLUE: LinearRgba = srgba_hex(0x00008B);
    pub const DARK_CYAN: LinearRgba = srgba_hex(0x008B8B);
    pub const DARK_GOLDENROD: LinearRgba = srgba_hex(0xB8860B);
    pub const DARK_GRAY: LinearRgba = srgba_hex(0xA9A9A9);
    pub const DARK_GREEN: LinearRgba = srgba_hex(0x006400);
    pub const DARK_GREY: LinearRgba = srgba_hex(0xA9A9A9);
    pub const DARK_KHAKI: LinearRgba = srgba_hex(0xBDB76B);
    pub const DARK_MAGENTA: LinearRgba = srgba_hex(0x8B008B);
    pub const DARK_OLIVE_GREEN: LinearRgba = srgba_hex(0x556B2F);
    pub const DARK_ORANGE: LinearRgba = srgba_hex(0xFF8C00);
    pub const DARK_ORCHID: LinearRgba = srgba_hex(0x9932CC);
    pub const DARK_RED: LinearRgba = srgba_hex(0x8B0000);
    pub const DARK_SALMON: LinearRgba = srgba_hex(0xE9967A);
    pub const DARK_SEA_GREEN: LinearRgba = srgba_hex(0x8FBC8F);
    pub const DARK_SLATE_BLUE: LinearRgba = srgba_hex(0x483D8B);
    pub const DARK_SLATE_GRAY: LinearRgba = srgba_hex(0x2F4F4F);
    pub const DARK_SLATE_GREY: LinearRgba = srgba_hex(0x2F4F4F);
    pub const DARK_TURQUOISE: LinearRgba = srgba_hex(0x00CED1);
    pub const DARK_VIOLET: LinearRgba = srgba_hex(0x9400D3);
    pub const DEEP_PINK: LinearRgba = srgba_hex(0xFF1493);
    pub const DEEP_SKY_BLUE: LinearRgba = srgba_hex(0x00BFFF);
    pub const DIM_GRAY: LinearRgba = srgba_hex(0x696969);
    pub const DIM_GREY: LinearRgba = srgba_hex(0x696969);
    pub const DODGER_BLUE: LinearRgba = srgba_hex(0x1E90FF);
    pub const FIREBRICK: LinearRgba = srgba_hex(0xB22222);
    pub const FLORAL_WHITE: LinearRgba = srgba_hex(0xFFFAF0);
    pub const FOREST_GREEN: LinearRgba = srgba_hex(0x228B22);
    pub const FUCHSIA: LinearRgba = srgba_hex(0xFF00FF);
    pub const GAINSBORO: LinearRgba = srgba_hex(0xDCDCDC);
    pub const GHOST_WHITE: LinearRgba = srgba_hex(0xF8F8FF);
    pub const GOLD: LinearRgba = srgba_hex(0xFFD700);
    pub const GOLDENROD: LinearRgba = srgba_hex(0xDAA520);
    pub const GRAY: LinearRgba = srgba_hex(0x808080);
    pub const GREEN: LinearRgba = srgba_hex(0x008000);
    pub const GREEN_YELLOW: LinearRgba = srgba_hex(0xADFF2F);
    pub const GREY: LinearRgba = srgba_hex(0x808080);
    pub const HONEYDEW: LinearRgba = srgba_hex(0xF0FFF0);
    pub const HOT_PINK: LinearRgba = srgba_hex(0xFF69B4);
    pub const INDIAN_RED: LinearRgba = srgba_hex(0xCD5C5C);
    pub const INDIGO: LinearRgba = srgba_hex(0x4B0082);
    pub const IVORY: LinearRgba = srgba_hex(0xFFFFF0);
    pub const KHAKI: LinearRgba = srgba_hex(0xF0E68C);
    pub const LAVENDER: LinearRgba = srgba_hex(0xE6E6FA);
    pub const LAVENDER_BLUSH: LinearRgba = srgba_hex(0xFFF0F5);
    pub const LAWN_GREEN: LinearRgba = srgba_hex(0x7CFC00);
    pub const LEMON_CHIFFON: LinearRgba = srgba_hex(0xFFFACD);
    pub const LIGHT_BLUE: LinearRgba = srgba_hex(0xADD8E6);
    pub const LIGHT_CORAL: LinearRgba = srgba_hex(0xF08080);
    pub const LIGHT_CYAN: LinearRgba = srgba_hex(0xE0FFFF);
    pub const LIGHT_GOLDENROD_YELLOW: LinearRgba = srgba_hex(0xFAFAD2);
    pub const LIGHT_GRAY: LinearRgba = srgba_hex(0xD3D3D3);
    pub const LIGHT_GREEN: LinearRgba = srgba_hex(0x90EE90);
    pub const LIGHT_GREY: LinearRgba = srgba_hex(0xD3D3D3);
    pub const LIGHT_PINK: LinearRgba = srgba_hex(0xFFB6C1);
    pub const LIGHT_SALMON: LinearRgba = srgba_hex(0xFFA07A);
    pub const LIGHT_SEA_GREEN: LinearRgba = srgba_hex(0x20B2AA);
    pub const LIGHT_SKY_BLUE: LinearRgba = srgba_hex(0x87CEFA);
    pub const LIGHT_SLATE_GRAY: LinearRgba = srgba_hex(0x778899);
    pub const LIGHT_SLATE_GREY: LinearRgba = srgba_hex(0x778899);
    pub const LIGHT_STEEL_BLUE: LinearRgba = srgba_hex(0xB0C4DE);
    pub const LIGHT_YELLOW: LinearRgba = srgba_hex(0xFFFFE0);
    pub const LIME: LinearRgba = srgba_hex(0x00FF00);
    pub const LIME_GREEN: LinearRgba = srgba_hex(0x32CD32);
    pub const LINEN: LinearRgba = srgba_hex(0xFAF0E6);
    pub const MAGENTA: LinearRgba = srgba_hex(0xFF00FF);
    pub const MAROON: LinearRgba = srgba_hex(0x800000);
    pub const MEDIUM_AQUAMARINE: LinearRgba = srgba_hex(0x66CDAA);
    pub const MEDIUM_BLUE: LinearRgba = srgba_hex(0x0000CD);
    pub const MEDIUM_ORCHID: LinearRgba = srgba_hex(0xBA55D3);
    pub const MEDIUM_PURPLE: LinearRgba = srgba_hex(0x9370DB);
    pub const MEDIUM_SEA_GREEN: LinearRgba = srgba_hex(0x3CB371);
    pub const MEDIUM_SLATE_BLUE: LinearRgba = srgba_hex(0x7B68EE);
    pub const MEDIUM_SPRING_GREEN: LinearRgba = srgba_hex(0x00FA9A);
    pub const MEDIUM_TURQUOISE: LinearRgba = srgba_hex(0x48D1CC);
    pub const MEDIUM_VIOLET_RED: LinearRgba = srgba_hex(0xC71585);
    pub const MIDNIGHT_BLUE: LinearRgba = srgba_hex(0x191970);
    pub const MINT_CREAM: LinearRgba = srgba_hex(0xF5FFFA);
    pub const MISTY_ROSE: LinearRgba = srgba_hex(0xFFE4E1);
    pub const MOCCASIN: LinearRgba = srgba_hex(0xFFE4B5);
    pub const NAVAJO_WHITE: LinearRgba = srgba_hex(0xFFDEAD);
    pub const NAVY: LinearRgba = srgba_hex(0x000080);
    pub const OLD_LACE: LinearRgba = srgba_hex(0xFDF5E6);
    pub const OLIVE: LinearRgba = srgba_hex(0x808000);
    pub const OLIVE_DRAB: LinearRgba = srgba_hex(0x6B8E23);
    pub const ORANGE: LinearRgba = srgba_hex(0xFFA500);
    pub const ORANGE_RED: LinearRgba = srgba_hex(0xFF4500);
    pub const ORCHID: LinearRgba = srgba_hex(0xDA70D6);
    pub const PALE_GOLDENROD: LinearRgba = srgba_hex(0xEEE8AA);
    pub const PALE_GREEN: LinearRgba = srgba_hex(0x98FB98);
    pub const PALE_TURQUOISE: LinearRgba = srgba_hex(0xAFEEEE);
    pub const PALE_VIOLET_RED: LinearRgba = srgba_hex(0xDB7093);
    pub const PAPAYA_WHIP: LinearRgba = srgba_hex(0xFFEFD5);
    pub const PEACH_PUFF: LinearRgba = srgba_hex(0xFFDAB9);
    pub const PERU: LinearRgba = srgba_hex(0xCD853F);
    pub const PINK: LinearRgba = srgba_hex(0xFFC0CB);
    pub const PLUM: LinearRgba = srgba_hex(0xDDA0DD);
    pub const POWDER_BLUE: LinearRgba = srgba_hex(0xB0E0E6);
    pub const PURPLE: LinearRgba = srgba_hex(0x800080);
    pub const REBECCA_PURPLE: LinearRgba = srgba_hex(0x663399);
    pub const RED: LinearRgba = srgba_hex(0xFF0000);
    pub const ROSY_BROWN: LinearRgba = srgba_hex(0xBC8F8F);
    pub const ROYAL_BLUE: LinearRgba = srgba_hex(0x4169E1);
    pub const SADDLE_BROWN: LinearRgba = srgba_hex(0x8B4513);
    pub const SALMON: LinearRgba = srgba_hex(0xFA8072);
    pub const SANDY_BROWN: LinearRgba = srgba_hex(0xF4A460);
    pub const SEA_GREEN: LinearRgba = srgba_hex(0x2E8B57);
    pub const SEA_SHELL: LinearRgba = srgba_hex(0xFFF5EE);
    pub const SIENNA: LinearRgba = srgba_hex(0xA0522D);
    pub const SILVER: LinearRgba = srgba_hex(0xC0C0C0);
    pub const SKY_BLUE: LinearRgba = srgba_hex(0x87CEEB);
    pub const SLATE_BLUE: LinearRgba = srgba_hex(0x6A5ACD);
    pub const SLATE_GRAY: LinearRgba = srgba_hex(0x708090);
    pub const SLATE_GREY: LinearRgba = srgba_hex(0x708090);
    pub const SNOW: LinearRgba = srgba_hex(0xFFFAFA);
    pub const SPRING_GREEN: LinearRgba = srgba_hex(0x00FF7F);
    pub const STEEL_BLUE: LinearRgba = srgba_hex(0x4682B4);
    pub const TAN: LinearRgba = srgba_hex(0xD2B48C);
    pub const TEAL: LinearRgba = srgba_hex(0x008080);
    pub const THISTLE: LinearRgba = srgba_hex(0xD8BFD8);
    pub const TOMATO: LinearRgba = srgba_hex(0xFF6347);
    pub const TURQUOISE: LinearRgba = srgba_hex(0x40E0D0);
    pub const VIOLET: LinearRgba = srgba_hex(0xEE82EE);
    pub const WHEAT: LinearRgba = srgba_hex(0xF5DEB3);
    pub const WHITE: LinearRgba = srgba_hex(0xFFFFFF);
    pub const WHITE_SMOKE: LinearRgba = srgba_hex(0xF5F5F5);
    pub const YELLOW: LinearRgba = srgba_hex(0xFFFF00);
    pub const YELLOW_GREEN: LinearRgba = srgba_hex(0x9ACD32);
}

const SRGB_TO_LINEAR: [f32; 256] = [
    0.000000, 0.000304, 0.000607, 0.000911, 0.001214, 0.001518, 0.001821, 0.002125, 0.002428,
    0.002732, 0.003035, 0.003347, 0.003677, 0.004025, 0.004391, 0.004777, 0.005182, 0.005605,
    0.006049, 0.006512, 0.006995, 0.007499, 0.008023, 0.008568, 0.009134, 0.009721, 0.010330,
    0.010960, 0.011612, 0.012286, 0.012983, 0.013702, 0.014444, 0.015209, 0.015996, 0.016807,
    0.017642, 0.018500, 0.019382, 0.020289, 0.021219, 0.022174, 0.023153, 0.024158, 0.025187,
    0.026241, 0.027321, 0.028426, 0.029557, 0.030713, 0.031896, 0.033105, 0.034340, 0.035601,
    0.036889, 0.038204, 0.039546, 0.040915, 0.042311, 0.043735, 0.045186, 0.046665, 0.048172,
    0.049707, 0.051269, 0.052861, 0.054480, 0.056128, 0.057805, 0.059511, 0.061246, 0.063010,
    0.064803, 0.066626, 0.068478, 0.070360, 0.072272, 0.074214, 0.076185, 0.078187, 0.080220,
    0.082283, 0.084376, 0.086500, 0.088656, 0.090842, 0.093059, 0.095307, 0.097587, 0.099899,
    0.102242, 0.104616, 0.107023, 0.109462, 0.111932, 0.114435, 0.116971, 0.119538, 0.122139,
    0.124772, 0.127438, 0.130136, 0.132868, 0.135633, 0.138432, 0.141263, 0.144128, 0.147027,
    0.149960, 0.152926, 0.155926, 0.158961, 0.162029, 0.165132, 0.168269, 0.171441, 0.174647,
    0.177888, 0.181164, 0.184475, 0.187821, 0.191202, 0.194618, 0.198069, 0.201556, 0.205079,
    0.208637, 0.212231, 0.215861, 0.219526, 0.223228, 0.226966, 0.230740, 0.234551, 0.238398,
    0.242281, 0.246201, 0.250158, 0.254152, 0.258183, 0.262251, 0.266356, 0.270498, 0.274677,
    0.278894, 0.283149, 0.287441, 0.291771, 0.296138, 0.300544, 0.304987, 0.309469, 0.313989,
    0.318547, 0.323143, 0.327778, 0.332452, 0.337164, 0.341914, 0.346704, 0.351533, 0.356400,
    0.361307, 0.366253, 0.371238, 0.376262, 0.381326, 0.386430, 0.391573, 0.396755, 0.401978,
    0.407240, 0.412543, 0.417885, 0.423268, 0.428691, 0.434154, 0.439657, 0.445201, 0.450786,
    0.456411, 0.462077, 0.467784, 0.473532, 0.479320, 0.485150, 0.491021, 0.496933, 0.502887,
    0.508881, 0.514918, 0.520996, 0.527115, 0.533276, 0.539480, 0.545725, 0.552011, 0.558340,
    0.564712, 0.571125, 0.577581, 0.584078, 0.590619, 0.597202, 0.603827, 0.610496, 0.617207,
    0.623960, 0.630757, 0.637597, 0.644480, 0.651406, 0.658375, 0.665387, 0.672443, 0.679543,
    0.686685, 0.693872, 0.701102, 0.708376, 0.715694, 0.723055, 0.730461, 0.737911, 0.745404,
    0.752942, 0.760525, 0.768151, 0.775822, 0.783538, 0.791298, 0.799103, 0.806952, 0.814847,
    0.822786, 0.830770, 0.838799, 0.846873, 0.854993, 0.863157, 0.871367, 0.879622, 0.887923,
    0.896269, 0.904661, 0.913099, 0.921582, 0.930111, 0.938686, 0.947307, 0.955974, 0.964686,
    0.973445, 0.982251, 0.991102, 1.000000,
];

/// A color palette for definined named colors, see [crate::Terminal::with_color_palette]
#[derive(Debug, Default, Clone, Reflect)]
pub struct ColorPalette {
    colors: HashMap<String, LinearRgba>,
}

impl ColorPalette {
    pub fn new() -> ColorPalette {
        ColorPalette::default()
    }

    pub fn add_color(&mut self, name: impl Into<String>, color: LinearRgba) {
        self.colors.insert(name.into(), color);
    }

    pub fn from_colors<'a>(colors: impl Iterator<Item = (&'a str, LinearRgba)>) -> ColorPalette {
        let mut map = HashMap::new();
        for (name, col) in colors {
            map.insert(name.to_owned(), col);
        }
        ColorPalette { colors: map }
    }

    pub fn named_color<T: AsRef<str>>(&self, name: T) -> Option<LinearRgba> {
        let input = name.as_ref();
        let mut buffer: [u8; 32] = [0; 32];
        debug_assert!(input.is_ascii(), "Color tag names must be ascii");
        debug_assert!(
            input.len() < 32,
            "Color tag names must be 32 characters or less"
        );
        for (i, c) in input.as_bytes().iter().enumerate() {
            buffer[i] = c.to_ascii_uppercase();
        }
        let up = unsafe { std::str::from_utf8_unchecked(&buffer[0..input.len()]) };

        self.colors.get(up).cloned()
    }
}
