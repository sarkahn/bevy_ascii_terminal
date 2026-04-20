//! Utilities for customizing terminal strings.
use bevy::color::LinearRgba;

/// A string wrapper for customizing how strings are written to the terminal.
#[derive(Debug, Clone)]
pub struct TerminalString<T> {
    pub string: T,
    /// Prevent splitting words between lines. Defaults to true.
    pub word_wrap: bool,
    /// Parse tags in the string before writing. See [crate::Terminal::put_string] Defaults to true.
    pub parse_tags: bool,
    /// Set string colors to the terminal's clear color. Defaults to true, but
    /// will be overridden by [Self::fg_color] or [Self::bg_color] if they are set.
    pub clear_colors: bool,
    /// Write colors to the spaces in the string. Defaults to true.
    pub colored_spaces: bool,
    /// Set the foreground color for the string.
    /// Will override [Self::clear_colors] if set.
    /// This setting is ignored for tagged strings.
    pub fg_color: Option<LinearRgba>,
    /// Set the background color for the string.
    /// Will override [Self::clear_colors] if set.
    /// This setting is ignored for tagged strings.
    pub bg_color: Option<LinearRgba>,
}

impl<T: AsRef<str> + Default> Default for TerminalString<T> {
    fn default() -> Self {
        Self {
            string: Default::default(),
            word_wrap: true,
            clear_colors: true,
            parse_tags: true,
            colored_spaces: true,
            fg_color: Default::default(),
            bg_color: Default::default(),
        }
    }
}

/// A trait for creating a [TerminalString]
pub trait TerminalStringBuilder<T: AsRef<str>> {
    /// Sets the foreground color for string tiles. Will override clear colors if set.
    fn fg(self, color: impl Into<LinearRgba>) -> TerminalString<T>;

    /// Sets the background color for string tiles. Will override clear colors if set.
    fn bg(self, color: impl Into<LinearRgba>) -> TerminalString<T>;

    /// Sets the string tile colors to match the terminal's clear tile. Will be
    /// overriden by [Self::fg] or [Self::bg] if they are set
    fn dont_clear_colors(self) -> TerminalString<T>;

    /// Disable parsing for embedded string tags before writing. Tag parsing
    /// doesn't allocate but it isn't free.
    fn dont_parse_tags(self) -> TerminalString<T>;

    /// Disable word wrap, allowing words to be split between lines.
    fn dont_word_wrap(self) -> TerminalString<T>;

    /// Prevent colors from being written to space characters
    fn dont_color_spaces(self) -> TerminalString<T>;
}

impl<T: AsRef<str> + Default> TerminalStringBuilder<T> for T {
    fn fg(self, color: impl Into<LinearRgba>) -> TerminalString<T> {
        TerminalString {
            string: self,
            fg_color: Some(color.into()),
            ..Default::default()
        }
    }

    fn bg(self, color: impl Into<LinearRgba>) -> TerminalString<T> {
        TerminalString {
            string: self,
            fg_color: Some(color.into()),
            ..Default::default()
        }
    }

    fn dont_clear_colors(self) -> TerminalString<T> {
        TerminalString {
            string: self,
            clear_colors: false,
            ..Default::default()
        }
    }

    fn dont_parse_tags(self) -> TerminalString<T> {
        TerminalString {
            string: self,
            parse_tags: false,
            ..Default::default()
        }
    }

    fn dont_word_wrap(self) -> TerminalString<T> {
        TerminalString {
            string: self,
            word_wrap: false,
            ..Default::default()
        }
    }

    fn dont_color_spaces(self) -> TerminalString<T> {
        TerminalString {
            string: self,
            colored_spaces: false,
            ..Default::default()
        }
    }
}

impl<T: AsRef<str>> TerminalStringBuilder<T> for TerminalString<T> {
    fn fg(mut self, color: impl Into<LinearRgba>) -> Self {
        self.fg_color = Some(color.into());
        self
    }

    fn bg(mut self, color: impl Into<LinearRgba>) -> Self {
        self.bg_color = Some(color.into());
        self
    }

    fn dont_clear_colors(mut self) -> Self {
        self.clear_colors = false;
        self
    }

    fn dont_parse_tags(mut self) -> Self {
        self.parse_tags = false;
        self
    }

    fn dont_word_wrap(mut self) -> Self {
        self.word_wrap = false;
        self
    }

    fn dont_color_spaces(mut self) -> TerminalString<T> {
        self.colored_spaces = false;
        self
    }
}

impl<T: AsRef<str> + Default> From<T> for TerminalString<T> {
    fn from(value: T) -> Self {
        TerminalString {
            string: value,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    //   use super::*;

    // struct SomeType<T: AsRef<str>> {
    //     string: T,
    // }

    // impl<T: AsRef<str>> SomeType<T> {
    //     pub fn dont_word_wrap() {}
    // }

    // impl<T: AsRef<str>> From<T> for SomeType<T> {
    //     fn from(value: T) -> Self {
    //         SomeType { string: value }
    //     }
    // }

    // fn takes_impl<T: AsRef<str>>(string: impl Into<SomeType<T>>) {}

    // #[test]
    // fn trait_test() {
    //     let string = "hello".to_string();
    //     let otherstring = "hello".dont_word_wrap().ignore_spaces();

    //     let s = string.ignore_spaces().dont_word_wrap();
    // }
}
