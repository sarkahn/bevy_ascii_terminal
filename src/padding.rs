use bevy::{color::LinearRgba, reflect::Reflect};

use crate::strings::decode_char;

#[derive(Default, Debug, Copy, Clone, Reflect)]
pub enum ColorWrite {
    /// Set a tile to the terminal's clear tile color.
    #[default]
    Clear,
    Set(LinearRgba),
}

#[derive(Debug, Reflect, Clone, Copy, Default, PartialEq, Eq)]
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
    // Reset the padding when writing - only used in [crate::Terminal::put_border]
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

    pub fn with_fg_color(mut self, col: LinearRgba) -> Self {
        self.fg_color = Some(ColorWrite::Set(col));
        self
    }

    pub fn with_bg_color(mut self, col: LinearRgba) -> Self {
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
