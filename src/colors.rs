use crate::color::TerminalColor;

pub const RED: TerminalColor = TerminalColor 
    { r: u8::MAX, g: 0, b: 0, a: u8::MAX };
pub const GREEN: TerminalColor = TerminalColor 
    { r: 0, g: u8::MAX, b: 0, a: u8::MAX };
pub const BLUE: TerminalColor = TerminalColor 
    { r: 0, g: 0, b: u8::MAX, a: u8::MAX };
pub const BLACK: TerminalColor = TerminalColor 
    { r: 0, g: 0, b: 0, a: u8::MAX };
pub const WHITE: TerminalColor = TerminalColor 
    { r: u8::MAX, g: u8::MAX, b: u8::MAX, a: u8::MAX };
pub const GREY: TerminalColor = TerminalColor 
    { r: 128, g: 128, b: 128, a: u8::MAX };