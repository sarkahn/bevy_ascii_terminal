//! Provides for reading of REXPaint .xp files
//!
//! Copyright (C) 2018 Mara <cyphergothic@protonmail.com>
//! This work is free. You can redistribute it and/or modify it under the
//! terms of the Do What The Fuck You Want To Public License, Version 2,
//! https://crates.io/crates/rexpaint
#![deny(missing_debug_implementations)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

// NOTE: Modified to remove unused write capabilities, flip y coordinates and
// swap empty tiles from pink background to black background.

use std::io;
use std::io::prelude::*;

use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::GzDecoder;

/// Structure representing the components of one color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XpColor {
    /// Red component 0..255
    pub r: u8,
    /// Green component 0..255
    pub g: u8,
    /// Blue component 0..255
    pub b: u8,
}

impl XpColor {
    /// deepest black
    pub const BLACK: XpColor = XpColor { r: 0, g: 0, b: 0 };
    /// color 0xff00ff (hot pink) is regarded as transparent
    pub const TRANSPARENT: XpColor = XpColor {
        r: 255,
        g: 0,
        b: 255,
    };

    /// Return whether this color is considered transparent (if this is the background color of a
    /// cell, the layer below it will see through)
    pub fn is_transparent(self) -> bool {
        self == XpColor::TRANSPARENT
    }

    /// Read a RGB color from a `ReadBytesExt`
    fn read<T: ReadBytesExt>(rdr: &mut T) -> io::Result<XpColor> {
        let r = rdr.read_u8()?;
        let g = rdr.read_u8()?;
        let b = rdr.read_u8()?;
        Ok(XpColor { r, g, b })
    }
}

/// Structure representing a character and its foreground/background color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XpCell {
    /// Character index
    /// This depends on the font but will usually be a code page 437 character
    /// (one way to convert to a rust unicode character one way is to use
    /// `CP437_WINGDINGS.decode(...)` in the `codepage_437` crate!)
    pub ch: u32,
    /// Foreground color
    pub fg: XpColor,
    /// Background color
    pub bg: XpColor,
}

/// Structure representing a layer
/// Cells are in the same order as in the file, in column-major order (index of position x,y is y*height + x).
#[derive(Debug, Clone, PartialEq)]
pub struct XpLayer {
    /// Width of layer (in cells)
    pub width: usize,
    /// Height of layer (in cells)
    pub height: usize,
    /// Content of layer
    pub cells: Vec<XpCell>,
}

impl XpLayer {
    /// Get the cell at coordinates (x,y), or None if it is out of range.
    pub fn get(&self, x: usize, y: usize) -> Option<&XpCell> {
        if x < self.width && y < self.height {
            // flip y
            let y = self.height - 1 - y;
            Some(&self.cells[x * self.height + y])
        } else {
            None
        }
    }
}

/// Structure representing a REXPaint image file which is a stack of layers
#[derive(Debug, Clone, PartialEq)]
pub struct XpFile {
    /// Version number from header
    pub version: i32,
    /// Layers of the image
    pub layers: Vec<XpLayer>,
}

impl XpFile {
    /// Read a xp image from a stream
    pub fn read<R: Read>(f: &mut R) -> io::Result<XpFile> {
        let mut rdr = GzDecoder::new(f);
        let version = rdr.read_i32::<LittleEndian>()?;
        let num_layers = rdr.read_u32::<LittleEndian>()?;

        let mut layers = Vec::<XpLayer>::with_capacity(num_layers as usize);
        for _layer in 0..num_layers {
            let width = rdr.read_u32::<LittleEndian>()? as usize;
            let height = rdr.read_u32::<LittleEndian>()? as usize;

            let mut cells = Vec::<XpCell>::with_capacity(width * height);
            for _y in 0..width {
                // column-major order
                for _x in 0..height {
                    let ch = rdr.read_u32::<LittleEndian>()?;
                    let fg = XpColor::read(&mut rdr)?;
                    let bg = XpColor::read(&mut rdr)?;
                    // Rexpaint uses pink backgrounds as empty tiles. We default to black.
                    let bg = if bg.is_transparent() {
                        XpColor::BLACK
                    } else {
                        bg
                    };
                    cells.push(XpCell { ch, fg, bg });
                }
            }
            layers.push(XpLayer {
                width,
                height,
                cells,
            });
        }
        Ok(XpFile { version, layers })
    }
}
