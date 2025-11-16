//! Utilities  for writing formatted/decorated strings to the terminal
//! without any extra allocations.
use std::{ops::Sub, str::Chars};

use bevy::{color::LinearRgba, math::IVec2, reflect::Reflect};
use sark_grids::{GridPoint, GridRect, GridSize, Pivot, PivotedPoint};

/// A string with optional [StringDecoration] and [StringFormatting] applied.
///
/// `dont_word_wrap` Can be used to disable word wrapping, which is enabled by
/// default for terminal strings.
///
/// `clear_colors` can be used to set the fg and bg colors of the string
/// tiles to match the terminal's clear tile.
///
/// The `bg` and `fg` methods can be used to set the background and foreground
/// colors of the string tiles if `clear_colors` isn't set. Otherwise the existing
/// colors in the terminal will remain unchanged.
#[derive(Default, Debug, Clone)]
pub struct TerminalString<T> {
    pub string: T,
    pub decoration: StringDecoration,
    pub formatting: StringFormatting,
}

impl<T: AsRef<str>> TerminalString<T> {
    pub fn fg(mut self, color: impl Into<LinearRgba>) -> Self {
        self.decoration.fg_color = Some(color.into());
        self
    }

    pub fn bg(mut self, color: impl Into<LinearRgba>) -> Self {
        self.decoration.bg_color = Some(color.into());
        self
    }

    pub fn delimiters(mut self, delimiters: impl AsRef<str>) -> Self {
        let mut chars = delimiters.as_ref().chars();
        self.decoration.delimiters = (chars.next(), chars.next());
        self
    }

    pub fn clear_colors(mut self) -> Self {
        self.decoration.clear_colors = true;
        self
    }

    pub fn ignore_spaces(mut self) -> Self {
        self.formatting.ignore_spaces = true;
        self
    }

    pub fn dont_word_wrap(mut self) -> Self {
        self.formatting.word_wrap = false;
        self
    }
}

/// Optional decoration to be applied to a string being written to a terminal.
#[derive(Default, Debug, Clone, Copy, Reflect)]
pub struct StringDecoration {
    /// An optional foreground color for the string. If set to None then the
    /// terminal's clear tile color will be used.
    pub fg_color: Option<LinearRgba>,
    /// An optional background color for the string. If set to None then the
    /// terminal's clear tile color will be used.
    pub bg_color: Option<LinearRgba>,
    /// An optional pair of delimiters to be placed around the string.
    pub delimiters: (Option<char>, Option<char>),
    /// If true, then the terminal's clear tile colors will be used for the
    /// string. If false then the fg and bg colors will be used if they are set.
    /// Otherwise the existing colors in the terminal will remain unchanged.
    pub clear_colors: bool,
    /// If true the terminal will parse the string for tags before writing.
    pub parse_tags: bool,
}

/// A string with optional [StringDecoration].
#[derive(Default)]
pub struct DecoratedString<T: AsRef<str>> {
    pub string: T,
    pub decoration: StringDecoration,
}

/// A trait for creating a [DecoratedString].
pub trait StringDecorator<T: AsRef<str>> {
    /// Sets the foreground color for string tiles.
    fn fg(self, color: impl Into<LinearRgba>) -> DecoratedString<T>;
    /// Sets the background color for string tiles.
    fn bg(self, color: impl Into<LinearRgba>) -> DecoratedString<T>;
    /// Add a pair of delimiters to the string. The first character will be the
    /// opening delimiter and the second character will be the closing delimiter.
    fn delimiters(self, delimiters: impl AsRef<str>) -> DecoratedString<T>;
    /// Sets the string tile colors to match the terminal's clear tile. This will
    /// override the string's fg and bg colors.
    fn clear_colors(self) -> DecoratedString<T>;

    /// If set, the terminal will parse the string for embedded tags before
    /// writing.
    fn parse_tags(self) -> DecoratedString<T>;
}

impl<T: AsRef<str>> StringDecorator<T> for T {
    fn fg(self, color: impl Into<LinearRgba>) -> DecoratedString<T> {
        DecoratedString {
            string: self,
            decoration: StringDecoration {
                fg_color: Some(color.into()),
                ..Default::default()
            },
        }
    }

    fn bg(self, color: impl Into<LinearRgba>) -> DecoratedString<T> {
        DecoratedString {
            string: self,
            decoration: StringDecoration {
                bg_color: Some(color.into()),
                ..Default::default()
            },
        }
    }

    fn clear_colors(self) -> DecoratedString<T> {
        DecoratedString {
            string: self,
            decoration: StringDecoration {
                clear_colors: true,
                ..Default::default()
            },
        }
    }

    fn delimiters(self, delimiters: impl AsRef<str>) -> DecoratedString<T> {
        let mut chars = delimiters.as_ref().chars();
        DecoratedString {
            string: self,
            decoration: StringDecoration {
                delimiters: (chars.next(), chars.next()),
                ..Default::default()
            },
        }
    }

    fn parse_tags(self) -> DecoratedString<T> {
        DecoratedString {
            string: self,
            decoration: StringDecoration {
                parse_tags: true,
                ..Default::default()
            },
        }
    }
}

impl<T: AsRef<str>> StringDecorator<T> for DecoratedString<T> {
    fn fg(mut self, color: impl Into<LinearRgba>) -> DecoratedString<T> {
        self.decoration.fg_color = Some(color.into());
        self
    }

    fn bg(mut self, color: impl Into<LinearRgba>) -> DecoratedString<T> {
        self.decoration.bg_color = Some(color.into());
        self
    }

    fn clear_colors(mut self) -> DecoratedString<T> {
        self.decoration.clear_colors = true;
        self
    }

    fn delimiters(self, delimiters: impl AsRef<str>) -> DecoratedString<T> {
        let mut chars = delimiters.as_ref().chars();
        DecoratedString {
            string: self.string,
            decoration: StringDecoration {
                delimiters: (chars.next(), chars.next()),
                ..self.decoration
            },
        }
    }

    fn parse_tags(mut self) -> DecoratedString<T> {
        self.decoration.parse_tags = true;
        self
    }
}

impl<T: AsRef<str>> DecoratedString<T> {
    pub fn ignore_spaces(self) -> TerminalString<T> {
        TerminalString {
            string: self.string,
            decoration: self.decoration,
            formatting: StringFormatting {
                ignore_spaces: true,
                ..Default::default()
            },
        }
    }
}

impl<T: AsRef<str>> From<T> for DecoratedString<T> {
    fn from(value: T) -> Self {
        DecoratedString {
            string: value,
            decoration: Default::default(),
        }
    }
}

/// Optional formatting to be applied to a string being written to a terminal.
#[derive(Debug, Clone, Reflect, Copy)]
pub struct StringFormatting {
    /// Defines whether or not 'empty' (" ") tiles will be modified when writing
    /// strings to the terminal. If set to false then colors will be
    /// applied even to empty tiles.
    ///
    /// Defaults to false.
    // TODO: move to decoration?
    pub ignore_spaces: bool,
    /// Word wrap prevents words from being split across lines.
    ///
    /// Defaults to true.
    pub word_wrap: bool,
}

impl StringFormatting {
    pub fn without_word_wrap() -> Self {
        Self {
            word_wrap: false,
            ..Self::default()
        }
    }
}

impl Default for StringFormatting {
    fn default() -> Self {
        Self {
            ignore_spaces: Default::default(),
            word_wrap: true,
        }
    }
}

#[derive(Default)]
pub struct FormattedString<T: AsRef<str>> {
    pub string: T,
    pub formatting: StringFormatting,
}

pub trait StringFormatter<T: AsRef<str>> {
    fn ignore_spaces(self) -> FormattedString<T>;
    fn dont_word_wrap(self) -> FormattedString<T>;
}

impl<T: AsRef<str>> StringFormatter<T> for T {
    fn ignore_spaces(self) -> FormattedString<T> {
        FormattedString {
            string: self,
            formatting: StringFormatting {
                ignore_spaces: true,
                ..Default::default()
            },
        }
    }

    fn dont_word_wrap(self) -> FormattedString<T> {
        FormattedString {
            string: self,
            formatting: StringFormatting {
                word_wrap: false,
                ..Default::default()
            },
        }
    }
}

impl<T: AsRef<str>> StringFormatter<T> for FormattedString<T> {
    fn ignore_spaces(mut self) -> FormattedString<T> {
        self.formatting.ignore_spaces = true;
        self
    }

    fn dont_word_wrap(mut self) -> FormattedString<T> {
        self.formatting.word_wrap = false;
        self
    }
}

impl<T: AsRef<str>> From<DecoratedString<T>> for TerminalString<T> {
    fn from(value: DecoratedString<T>) -> Self {
        TerminalString {
            string: value.string,
            decoration: value.decoration,
            formatting: Default::default(),
        }
    }
}

impl<T: AsRef<str>> From<FormattedString<T>> for TerminalString<T> {
    fn from(value: FormattedString<T>) -> Self {
        TerminalString {
            string: value.string,
            formatting: value.formatting,
            decoration: Default::default(),
        }
    }
}

impl<T: AsRef<str>> FormattedString<T> {
    pub fn fg(self, color: impl Into<LinearRgba>) -> TerminalString<T> {
        TerminalString {
            string: self.string,
            decoration: StringDecoration {
                fg_color: Some(color.into()),
                ..Default::default()
            },
            formatting: self.formatting,
        }
    }
    pub fn bg(self, color: impl Into<LinearRgba>) -> TerminalString<T> {
        TerminalString {
            string: self.string,
            decoration: StringDecoration {
                bg_color: Some(color.into()),
                ..Default::default()
            },
            formatting: self.formatting,
        }
    }

    pub fn delimiters(self, delimiters: impl AsRef<str>) -> TerminalString<T> {
        let mut chars = delimiters.as_ref().chars();
        TerminalString {
            string: self.string,
            decoration: StringDecoration {
                delimiters: (chars.next(), chars.next()),
                ..Default::default()
            },
            formatting: self.formatting,
        }
    }

    // pub fn clear_colors(self) -> TerminalString<T> {
    //     TerminalString {
    //         string: self.string,
    //         decoration: StringDecoration {
    //             clear_colors: true,
    //             ..Default::default()
    //         },
    //         formatting: self.formatting,
    //     }
    // }
}

impl<T: AsRef<str> + Default> From<T> for TerminalString<T> {
    fn from(value: T) -> Self {
        Self {
            string: value,
            ..Default::default()
        }
    }
}
