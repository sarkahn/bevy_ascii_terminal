//! Utilities for converting srgba colors to linear colors in a const context.
//! Tile colors are stored as linear for correct rendering with the shader.

use std::hash::Hash;

use bevy::{color::LinearRgba, platform::collections::HashMap, reflect::Reflect};
use case_insensitive_hashmap::CaseInsensitiveHashMap;
use unicase::UniCase;

/// Convert an Srgba hex color to linear. Works at compile time.
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

pub fn parse_color_string(input: &str) -> Option<LinearRgba> {
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

    let up = s.to_ascii_uppercase();

    Some(match up.as_str() {
        "WHITE" => WHITE,
        "BLACK" => BLACK,
        "RED" => RED,
        "GREEN" => GREEN,
        "BLUE" => BLUE,
        "CSS_ALICE_BLUE" => css::ALICE_BLUE,
        "CSS_ANTIQUE_WHITE" => css::ANTIQUE_WHITE,
        "CSS_AQUA" => css::AQUA,
        "CSS_AQUAMARINE" => css::AQUAMARINE,
        "CSS_AZURE" => css::AZURE,
        "CSS_BEIGE" => css::BEIGE,
        "CSS_BISQUE" => css::BISQUE,
        "CSS_BLACK" => css::BLACK,
        "CSS_BLANCHED_ALMOND" => css::BLANCHED_ALMOND,
        "CSS_BLUE" => css::BLUE,
        "CSS_BLUE_VIOLET" => css::BLUE_VIOLET,
        "CSS_BROWN" => css::BROWN,
        "CSS_BURLY_WOOD" => css::BURLY_WOOD,
        "CSS_CADET_BLUE" => css::CADET_BLUE,
        "CSS_CHARTREUSE" => css::CHARTREUSE,
        "CSS_CHOCOLATE" => css::CHOCOLATE,
        "CSS_CORAL" => css::CORAL,
        "CSS_CORNFLOWER_BLUE" => css::CORNFLOWER_BLUE,
        "CSS_CORNSILK" => css::CORNSILK,
        "CSS_CRIMSON" => css::CRIMSON,
        "CSS_CYAN" => css::CYAN,
        "CSS_DARK_BLUE" => css::DARK_BLUE,
        "CSS_DARK_CYAN" => css::DARK_CYAN,
        "CSS_DARK_GOLDENROD" => css::DARK_GOLDENROD,
        "CSS_DARK_GRAY" => css::DARK_GRAY,
        "CSS_DARK_GREEN" => css::DARK_GREEN,
        "CSS_DARK_KHAKI" => css::DARK_KHAKI,
        "CSS_DARK_MAGENTA" => css::DARK_MAGENTA,
        "CSS_DARK_OLIVE_GREEN" => css::DARK_OLIVE_GREEN,
        "CSS_DARK_ORANGE" => css::DARK_ORANGE,
        "CSS_DARK_ORCHID" => css::DARK_ORCHID,
        "CSS_DARK_RED" => css::DARK_RED,
        "CSS_DARK_SALMON" => css::DARK_SALMON,
        "CSS_DARK_SEA_GREEN" => css::DARK_SEA_GREEN,
        "CSS_DARK_SLATE_BLUE" => css::DARK_SLATE_BLUE,
        "CSS_DARK_SLATE_GRAY" => css::DARK_SLATE_GRAY,
        "CSS_DARK_TURQUOISE" => css::DARK_TURQUOISE,
        "CSS_DARK_VIOLET" => css::DARK_VIOLET,
        "CSS_DEEP_PINK" => css::DEEP_PINK,
        "CSS_DEEP_SKY_BLUE" => css::DEEP_SKY_BLUE,
        "CSS_DIM_GRAY" => css::DIM_GRAY,
        "CSS_DODGER_BLUE" => css::DODGER_BLUE,
        "CSS_FIREBRICK" => css::FIREBRICK,
        "CSS_FLORAL_WHITE" => css::FLORAL_WHITE,
        "CSS_FOREST_GREEN" => css::FOREST_GREEN,
        "CSS_FUCHSIA" => css::FUCHSIA,
        "CSS_GAINSBORO" => css::GAINSBORO,
        "CSS_GHOST_WHITE" => css::GHOST_WHITE,
        "CSS_GOLD" => css::GOLD,
        "CSS_GOLDENROD" => css::GOLDENROD,
        "CSS_GRAY" => css::GRAY,
        "CSS_GREEN" => css::GREEN,
        "CSS_GREEN_YELLOW" => css::GREEN_YELLOW,
        "CSS_HONEYDEW" => css::HONEYDEW,
        "CSS_HOT_PINK" => css::HOT_PINK,
        "CSS_INDIAN_RED" => css::INDIAN_RED,
        "CSS_INDIGO" => css::INDIGO,
        "CSS_IVORY" => css::IVORY,
        "CSS_KHAKI" => css::KHAKI,
        "CSS_LAVENDER" => css::LAVENDER,
        "CSS_LAVENDER_BLUSH" => css::LAVENDER_BLUSH,
        "CSS_LAWN_GREEN" => css::LAWN_GREEN,
        "CSS_LEMON_CHIFFON" => css::LEMON_CHIFFON,
        "CSS_LIGHT_BLUE" => css::LIGHT_BLUE,
        "CSS_LIGHT_CORAL" => css::LIGHT_CORAL,
        "CSS_LIGHT_CYAN" => css::LIGHT_CYAN,
        "CSS_LIGHT_GOLDENROD_YELLOW" => css::LIGHT_GOLDENROD_YELLOW,
        "CSS_LIGHT_GRAY" => css::LIGHT_GRAY,
        "CSS_LIGHT_GREEN" => css::LIGHT_GREEN,
        "CSS_LIGHT_PINK" => css::LIGHT_PINK,
        "CSS_LIGHT_SALMON" => css::LIGHT_SALMON,
        "CSS_LIGHT_SEA_GREEN" => css::LIGHT_SEA_GREEN,
        "CSS_LIGHT_SKY_BLUE" => css::LIGHT_SKY_BLUE,
        "CSS_LIGHT_SLATE_GRAY" => css::LIGHT_SLATE_GRAY,
        "CSS_LIGHT_STEEL_BLUE" => css::LIGHT_STEEL_BLUE,
        "CSS_LIGHT_YELLOW" => css::LIGHT_YELLOW,
        "CSS_LIME" => css::LIME,
        "CSS_LIME_GREEN" => css::LIME_GREEN,
        "CSS_LINEN" => css::LINEN,
        "CSS_MAGENTA" => css::MAGENTA,
        "CSS_MAROON" => css::MAROON,
        "CSS_MEDIUM_AQUAMARINE" => css::MEDIUM_AQUAMARINE,
        "CSS_MEDIUM_BLUE" => css::MEDIUM_BLUE,
        "CSS_MEDIUM_ORCHID" => css::MEDIUM_ORCHID,
        "CSS_MEDIUM_PURPLE" => css::MEDIUM_PURPLE,
        "CSS_MEDIUM_SEA_GREEN" => css::MEDIUM_SEA_GREEN,
        "CSS_MEDIUM_SLATE_BLUE" => css::MEDIUM_SLATE_BLUE,
        "CSS_MEDIUM_SPRING_GREEN" => css::MEDIUM_SPRING_GREEN,
        "CSS_MEDIUM_TURQUOISE" => css::MEDIUM_TURQUOISE,
        "CSS_MEDIUM_VIOLET_RED" => css::MEDIUM_VIOLET_RED,
        "CSS_MIDNIGHT_BLUE" => css::MIDNIGHT_BLUE,
        "CSS_MINT_CREAM" => css::MINT_CREAM,
        "CSS_MISTY_ROSE" => css::MISTY_ROSE,
        "CSS_MOCCASIN" => css::MOCCASIN,
        "CSS_NAVAJO_WHITE" => css::NAVAJO_WHITE,
        "CSS_NAVY" => css::NAVY,
        "CSS_OLD_LACE" => css::OLD_LACE,
        "CSS_OLIVE" => css::OLIVE,
        "CSS_OLIVE_DRAB" => css::OLIVE_DRAB,
        "CSS_ORANGE" => css::ORANGE,
        "CSS_ORANGE_RED" => css::ORANGE_RED,
        "CSS_ORCHID" => css::ORCHID,
        "CSS_PALE_GOLDENROD" => css::PALE_GOLDENROD,
        "CSS_PALE_GREEN" => css::PALE_GREEN,
        "CSS_PALE_TURQUOISE" => css::PALE_TURQUOISE,
        "CSS_PALE_VIOLET_RED" => css::PALE_VIOLET_RED,
        "CSS_PAPAYA_WHIP" => css::PAPAYA_WHIP,
        "CSS_PEACH_PUFF" => css::PEACH_PUFF,
        "CSS_PERU" => css::PERU,
        "CSS_PINK" => css::PINK,
        "CSS_PLUM" => css::PLUM,
        "CSS_POWDER_BLUE" => css::POWDER_BLUE,
        "CSS_PURPLE" => css::PURPLE,
        "CSS_REBECCA_PURPLE" => css::REBECCA_PURPLE,
        "CSS_RED" => css::RED,
        "CSS_ROSY_BROWN" => css::ROSY_BROWN,
        "CSS_ROYAL_BLUE" => css::ROYAL_BLUE,
        "CSS_SADDLE_BROWN" => css::SADDLE_BROWN,
        "CSS_SALMON" => css::SALMON,
        "CSS_SANDY_BROWN" => css::SANDY_BROWN,
        "CSS_SEA_GREEN" => css::SEA_GREEN,
        "CSS_SEA_SHELL" => css::SEA_SHELL,
        "CSS_SIENNA" => css::SIENNA,
        "CSS_SILVER" => css::SILVER,
        "CSS_SKY_BLUE" => css::SKY_BLUE,
        "CSS_SLATE_BLUE" => css::SLATE_BLUE,
        "CSS_SLATE_GRAY" => css::SLATE_GRAY,
        "CSS_SNOW" => css::SNOW,
        "CSS_SPRING_GREEN" => css::SPRING_GREEN,
        "CSS_STEEL_BLUE" => css::STEEL_BLUE,
        "CSS_TAN" => css::TAN,
        "CSS_TEAL" => css::TEAL,
        "CSS_THISTLE" => css::THISTLE,
        "CSS_TOMATO" => css::TOMATO,
        "CSS_TURQUOISE" => css::TURQUOISE,
        "CSS_VIOLET" => css::VIOLET,
        "CSS_WHEAT" => css::WHEAT,
        "CSS_WHITE" => css::WHITE,
        "CSS_WHITE_SMOKE" => css::WHITE_SMOKE,
        "CSS_YELLOW" => css::YELLOW,
        "CSS_YELLOW_GREEN" => css::YELLOW_GREEN,
        "RAYLIB_LIGHT_GRAY" => raylib::LIGHT_GRAY,
        "RAYLIB_GRAY" => raylib::GRAY,
        "RAYLIB_DARK_GRAY" => raylib::DARK_GRAY,
        "RAYLIB_YELLOW" => raylib::YELLOW,
        "RAYLIB_GOLD" => raylib::GOLD,
        "RAYLIB_ORANGE" => raylib::ORANGE,
        "RAYLIB_PINK" => raylib::PINK,
        "RAYLIB_RED" => raylib::RED,
        "RAYLIB_MAROON" => raylib::MAROON,
        "RAYLIB_GREEN" => raylib::GREEN,
        "RAYLIB_LIME" => raylib::LIME,
        "RAYLIB_DARK_GREEN" => raylib::DARK_GREEN,
        "RAYLIB_SKY_BLUE" => raylib::SKY_BLUE,
        "RAYLIB_BLUE" => raylib::BLUE,
        "RAYLIB_DARK_BLUE" => raylib::DARK_BLUE,
        "RAYLIB_PURPLE" => raylib::PURPLE,
        "RAYLIB_VIOLET" => raylib::VIOLET,
        "RAYLIB_DARK_PURPLE" => raylib::DARK_PURPLE,
        "RAYLIB_BEIGE" => raylib::BEIGE,
        "RAYLIB_BROWN" => raylib::BROWN,
        "RAYLIB_DARK_BROWN" => raylib::DARK_BROWN,
        "RAYLIB_WHITE" => raylib::WHITE,
        "RAYLIB_BLACK" => raylib::BLACK,
        "RAYLIB_BLANK" => raylib::BLANK,
        "RAYLIB_MAGENTA" => raylib::MAGENTA,
        "RAYLIB_RAY_WHITE" => raylib::RAY_WHITE,
        _ => return None,
    })
}

pub const WHITE: LinearRgba = LinearRgba::WHITE;
pub const BLACK: LinearRgba = LinearRgba::BLACK;
pub const RED: LinearRgba = LinearRgba::RED;
pub const GREEN: LinearRgba = LinearRgba::GREEN;
pub const BLUE: LinearRgba = LinearRgba::BLUE;

pub mod css {
    use super::{LinearRgba, srgba_hex};
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
    pub const DARK_TURQUOISE: LinearRgba = srgba_hex(0x00CED1);
    pub const DARK_VIOLET: LinearRgba = srgba_hex(0x9400D3);
    pub const DEEP_PINK: LinearRgba = srgba_hex(0xFF1493);
    pub const DEEP_SKY_BLUE: LinearRgba = srgba_hex(0x00BFFF);
    pub const DIM_GRAY: LinearRgba = srgba_hex(0x696969);
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
    pub const LIGHT_PINK: LinearRgba = srgba_hex(0xFFB6C1);
    pub const LIGHT_SALMON: LinearRgba = srgba_hex(0xFFA07A);
    pub const LIGHT_SEA_GREEN: LinearRgba = srgba_hex(0x20B2AA);
    pub const LIGHT_SKY_BLUE: LinearRgba = srgba_hex(0x87CEFA);
    pub const LIGHT_SLATE_GRAY: LinearRgba = srgba_hex(0x778899);
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

// https://github.com/raysan5/raylib/blob/master/src/raylib.h
pub mod raylib {
    use super::{LinearRgba, srgb_bytes, srgba_bytes};

    pub const LIGHT_GRAY: LinearRgba = srgb_bytes(200, 200, 200);
    pub const GRAY: LinearRgba = srgb_bytes(130, 130, 130);
    pub const DARK_GRAY: LinearRgba = srgb_bytes(80, 80, 80);
    pub const YELLOW: LinearRgba = srgb_bytes(253, 249, 0);
    pub const GOLD: LinearRgba = srgb_bytes(255, 203, 0);
    pub const ORANGE: LinearRgba = srgb_bytes(255, 161, 0);
    pub const PINK: LinearRgba = srgb_bytes(255, 109, 194);
    pub const RED: LinearRgba = srgb_bytes(230, 41, 55);
    pub const MAROON: LinearRgba = srgb_bytes(190, 33, 55);
    pub const GREEN: LinearRgba = srgb_bytes(0, 228, 48);
    pub const LIME: LinearRgba = srgb_bytes(0, 158, 47);
    pub const DARK_GREEN: LinearRgba = srgb_bytes(0, 117, 44);
    pub const SKY_BLUE: LinearRgba = srgb_bytes(102, 191, 255);
    pub const BLUE: LinearRgba = srgb_bytes(0, 121, 241);
    pub const DARK_BLUE: LinearRgba = srgb_bytes(0, 82, 172);
    pub const PURPLE: LinearRgba = srgb_bytes(200, 122, 255);
    pub const VIOLET: LinearRgba = srgb_bytes(135, 60, 190);
    pub const DARK_PURPLE: LinearRgba = srgb_bytes(112, 31, 126);
    pub const BEIGE: LinearRgba = srgb_bytes(211, 176, 131);
    pub const BROWN: LinearRgba = srgb_bytes(127, 106, 79);
    pub const DARK_BROWN: LinearRgba = srgb_bytes(76, 63, 47);
    pub const WHITE: LinearRgba = srgb_bytes(255, 255, 255);
    pub const BLACK: LinearRgba = srgb_bytes(0, 0, 0);
    pub const BLANK: LinearRgba = srgba_bytes(0, 0, 0, 0); // alpha 0 — you may want a special case here
    pub const MAGENTA: LinearRgba = srgb_bytes(255, 0, 255);
    pub const RAY_WHITE: LinearRgba = srgb_bytes(245, 245, 245);
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

/// Create a distinct color palette type with named compile time colors that can
/// be parsed from a string
///
/// # Usage
/// ```
/// define_palette! {
///     BasicPalette {
///         RED   => srgba_hex(0xFF0000),
///         GREEN => srgb_bytes(0, 0, 255),
///         BLUE  => srgba_bytes(0, 0, 255, 255),
///    }
///}
/// ```
macro_rules! define_palette {
    (
        $name:ident {
            $(
                $color_name:ident => $value:expr
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Reflect)]
        pub struct $name;

        impl $name {
            $(
                pub const $color_name: LinearRgba = $value;
            )*

            pub fn parse_color_string(input: &str) -> Option<LinearRgba> {
                let s = input.trim();

                // --- Try parsing as hex ---
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

                let up = s.to_ascii_uppercase();

                Some(match up.as_str() {
                    $(
                        stringify!($color_name) => Self::$color_name,
                    )*
                    _ => return None,
                })
            }
        }
    };
}

define_palette! {
    BasicPalette {
        RED   => srgba_hex(0xFF0000),
        GREEN => srgba_hex(0x008000),
        BLUE  => srgba_hex(0x0000FF),

    }
}

define_palette! {
    RaylibPalette {
        LIGHT_GRAY => srgb_bytes(200, 200, 200),
        GRAY       => srgb_bytes(130, 130, 130),
        DARK_GRAY  => srgb_bytes(80, 80, 80),
        YELLOW     => srgb_bytes(253, 249, 0),
        GOLD       => srgb_bytes(255, 203, 0),
        ORANGE     => srgb_bytes(255, 161, 0),
        PINK       => srgb_bytes(255, 109, 194),
        RED        => srgb_bytes(230, 41, 55),
        GREEN      => srgb_bytes(0, 228, 48),
        BLUE       => srgb_bytes(0, 121, 241),
        WHITE      => srgb_bytes(255, 255, 255),
        BLACK      => srgb_bytes(0, 0, 0),
        BLANK      => srgba_bytes(0, 0, 0, 0),
    }
}

#[derive(Default, Debug, Reflect, Clone)]
pub struct ColorPalette {
    colors: Vec<LinearRgba>,
    map: HashMap<String, usize>,
}

pub struct ColorEntry<T: AsRef<str>>(T, LinearRgba);

impl ColorPalette {
    pub fn new<T: AsRef<str> + Eq + Hash>(
        named_colors: impl IntoIterator<Item = ColorEntry<T>>,
    ) -> Self {
        let mut colors = Vec::new();
        let mut map = HashMap::new();
        for (i, entry) in named_colors.into_iter().enumerate() {
            colors.push(entry.1);
            map.insert(entry.0.as_ref().to_ascii_uppercase().to_string(), i);
        }
        Self { colors, map }
    }

    pub fn named_color(&self, col: &str) -> Option<LinearRgba> {
        self.map.get(col).map(|i| self.colors[*i])
    }

    pub fn color_from_index(&self, i: usize) -> LinearRgba {
        self.colors[i]
    }
}

#[cfg(test)]
mod tests {
    use bevy::color::{ColorToPacked, Srgba};

    use super::*;

    fn palettey() {
        let colors: Vec<_> = [
            ColorEntry("RED", srgba_hex(0xFF0000)),
            ColorEntry("GREEN", srgba_hex(0x008000)),
            ColorEntry("BLUE", srgba_hex(0x0000FF)),
        ]
        .into_iter()
        .collect();
        let palette = ColorPalette::new(colors);
        let r = palette.named_color("green").unwrap();
    }

    #[test]
    fn testparse() {
        let col = parse_color_string("ALICE_BLUE").unwrap();
        assert_eq!(css::ALICE_BLUE, col);

        let col = parse_color_string("0xF0F8FF").unwrap();
        assert_eq!(css::ALICE_BLUE, col);

        let col = parse_color_string("F0F8FF").unwrap();
        assert_eq!(css::ALICE_BLUE, col);
    }

    #[test]
    fn palettes() {
        let a = RaylibPalette::parse_color_string("GREEN").unwrap();
        let srgb = Srgba::from(a);
        assert_eq!(228, srgb.to_u8_array()[1]);
        let b = RaylibPalette::BLUE;
    }
}
