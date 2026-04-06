use bevy::{
    color::LinearRgba,
    math::{IVec2, ivec2},
    reflect::Reflect,
};

use crate::{Pivot, terminal::ColorWrite};

#[derive(Debug, Reflect, Clone, Copy, Default)]
pub struct Padding {
    pub left: usize,
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
}

impl Padding {
    pub const ONE: Padding = Padding {
        left: 1,
        top: 1,
        right: 1,
        bottom: 1,
    };

    pub const ZERO: Padding = Padding {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    pub fn pivot_offset(&self, pivot: Pivot) -> IVec2 {
        match pivot {
            Pivot::LeftBottom => ivec2(self.left as i32, self.bottom as i32),
            Pivot::LeftCenter => ivec2(self.left as i32, self.bottom as i32),
            Pivot::LeftTop => ivec2(self.left as i32, -(self.top as i32)),
            _ => todo!(), // Pivot::CenterBottom => ivec2(self.left as i32, self.bottom as i32),
                          // Pivot::Center => ivec2(self.left as i32, self.bottom as i32),
                          // Pivot::CenterTop => ivec2(self.left as i32, self.bottom as i32),
                          // Pivot::RightBottom => ivec2(self.left as i32, self.bottom as i32),
                          // Pivot::RightCenter => ivec2(self.left as i32, self.bottom as i32),
                          // Pivot::RightTop => ivec2(self.left as i32, self.bottom as i32),
        }
    }
}

#[derive(Debug, Copy, Clone, Reflect)]
pub struct BoxStyle {
    pub top_left: char,
    pub top_center: char,
    pub top_right: char,
    pub center_left: char,
    pub center: char,
    pub center_right: char,
    pub bottom_left: char,
    pub bottom_center: char,
    pub bottom_right: char,
    pub fg_color: Option<ColorWrite>,
    pub bg_color: Option<ColorWrite>,
    /// Reset the padding when writing - only used in [crate::Terminal::put_border]
    pub reset_padding: bool,
}

impl Default for BoxStyle {
    fn default() -> Self {
        Self {
            top_left: Default::default(),
            top_center: Default::default(),
            top_right: Default::default(),
            center_left: Default::default(),
            center: Default::default(),
            center_right: Default::default(),
            bottom_left: Default::default(),
            bottom_center: Default::default(),
            bottom_right: Default::default(),
            fg_color: Some(ColorWrite::Clear),
            bg_color: Some(ColorWrite::Clear),
            reset_padding: true,
        }
    }
}

// UTF-8 encodes codepoints using 1–4 bytes. The leading byte tells you how
// many bytes the sequence uses, and continuation bytes always start with 10xxxxxx.
//
// 1-byte:  0xxxxxxx                             (U+0000  – U+007F)
// 2-byte:  110xxxxx 10xxxxxx                    (U+0080  – U+07FF)
// 3-byte:  1110xxxx 10xxxxxx 10xxxxxx           (U+0800  – U+FFFF)
// 4-byte:  11110xxx 10xxxxxx 10xxxxxx 10xxxxxx  (U+10000 – U+10FFFF)

// Masks to identify the sequence length from the leading byte.
const SINGLE_BYTE_PREFIX: u8 = 0b0000_0000;
const SINGLE_BYTE_MASK: u8 = 0b1000_0000;

const TWO_BYTE_PREFIX: u8 = 0b1100_0000;
const TWO_BYTE_MASK: u8 = 0b1110_0000;

const THREE_BYTE_PREFIX: u8 = 0b1110_0000;
const THREE_BYTE_MASK: u8 = 0b1111_0000;

// Anything else is a 4-byte sequence (0b11110xxx).

// Mask to extract the payload bits from a continuation byte (10xxxxxx → xxxxxx).
const CONTINUATION_PAYLOAD: u8 = 0b0011_1111;

const fn decode_char(bytes: &[u8], pos: usize) -> (char, usize) {
    let leading = bytes[pos];

    let (codepoint, len) = if leading & SINGLE_BYTE_MASK == SINGLE_BYTE_PREFIX {
        // Leading 0 → all 7 bits are the codepoint directly.
        (leading as u32, 1)
    } else if leading & TWO_BYTE_MASK == TWO_BYTE_PREFIX {
        // Leading 110 → 5 payload bits here, 6 in the continuation byte.
        let high = (leading & 0b0001_1111) as u32;
        let low = (bytes[pos + 1] & CONTINUATION_PAYLOAD) as u32;
        (high << 6 | low, 2)
    } else if leading & THREE_BYTE_MASK == THREE_BYTE_PREFIX {
        // Leading 1110 → 4 payload bits here, then 6 + 6 in the two continuation bytes.
        let high = (leading & 0b0000_1111) as u32;
        let mid = (bytes[pos + 1] & CONTINUATION_PAYLOAD) as u32;
        let low = (bytes[pos + 2] & CONTINUATION_PAYLOAD) as u32;
        (high << 12 | mid << 6 | low, 3)
    } else {
        // Leading 11110 → 3 payload bits here, then 6 + 6 + 6 in the three continuation bytes.
        let high = (leading & 0b0000_0111) as u32;
        let mid_high = (bytes[pos + 1] & CONTINUATION_PAYLOAD) as u32;
        let mid_low = (bytes[pos + 2] & CONTINUATION_PAYLOAD) as u32;
        let low = (bytes[pos + 3] & CONTINUATION_PAYLOAD) as u32;
        (high << 18 | mid_high << 12 | mid_low << 6 | low, 4)
    };

    (char::from_u32(codepoint).unwrap(), len)
}

impl BoxStyle {
    pub const fn from_string(s: &str) -> BoxStyle {
        let b = s.as_bytes();
        let mut pos = 0;

        let (top_left, n) = decode_char(b, pos);
        pos += n;
        let (top_center, n) = decode_char(b, pos);
        pos += n;
        let (top_right, n) = decode_char(b, pos);
        pos += n;
        let (center_left, n) = decode_char(b, pos);
        pos += n;
        let (center, n) = decode_char(b, pos);
        pos += n;
        let (center_right, n) = decode_char(b, pos);
        pos += n;
        let (bottom_left, n) = decode_char(b, pos);
        pos += n;
        let (bottom_center, n) = decode_char(b, pos);
        pos += n;
        let (bottom_right, _) = decode_char(b, pos);

        BoxStyle {
            top_left,
            top_center,
            top_right,
            center_left,
            center,
            center_right,
            bottom_left,
            bottom_center,
            bottom_right,
            fg_color: Some(ColorWrite::Clear),
            bg_color: Some(ColorWrite::Clear),
            reset_padding: true,
        }
    }

    pub const SINGLE_LINE: BoxStyle = BoxStyle::from_string("┌─┐│ │└─┘");
    pub const DOUBLE_LINE: BoxStyle = BoxStyle::from_string("╔═╗║ ║╚═╝");
    pub const ASCII: BoxStyle = BoxStyle::from_string("+-+| |+-+");

    /// Set the foreground colors on border tiles to the terminal's clear tile color
    pub fn clear_fg(mut self) -> Self {
        self.fg_color = Some(ColorWrite::Clear);
        self
    }

    /// Set the background colors on border tiles to the terminal's clear tile color
    pub fn clear_bg(mut self) -> Self {
        self.bg_color = Some(ColorWrite::Clear);
        self
    }

    pub fn set_fg(mut self, col: LinearRgba) -> Self {
        self.fg_color = Some(ColorWrite::Set(col));
        self
    }

    pub fn set_bg(mut self, col: LinearRgba) -> Self {
        self.bg_color = Some(ColorWrite::Set(col));
        self
    }

    pub fn dont_clear_fg(mut self) -> Self {
        self.fg_color = None;
        self
    }

    pub fn dont_clear_bg(mut self) -> Self {
        self.bg_color = None;
        self
    }

    /// When used with put_border this will prevent the terminal padding
    /// from being overwritten
    pub fn dont_reset_padding(mut self) -> Self {
        self.reset_padding = false;
        self
    }
}
