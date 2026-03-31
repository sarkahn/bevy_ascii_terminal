use crate::color;
use anyhow::{Result, anyhow};
use bevy::{color::LinearRgba, math::IVec2};

/// Tokens from a tagged string.
#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    Text(&'a str),
    Space,
    Newline,
    /// <fg=COLOR> Color + text of color from tag (either name or hex string)
    FgStart(LinearRgba, &'a str),
    /// <bg=COLOR> Color + text of color from tag (either name or hex string)
    BgStart(LinearRgba, &'a str),
    /// </fg>
    FgEnd,
    /// </bg>
    BgEnd,
}

/// An iterator over the tokens in a tagged string.
pub struct TokenIterator<'a> {
    remaining: &'a str,
    // For error reporting
    position: IVec2,
}

impl<'a> TokenIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            remaining: input,
            position: IVec2::ZERO,
        }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None; // Done
        }

        let remaining = self.remaining;

        if let Some(rem) = remaining.strip_prefix('\n') {
            self.remaining = rem;
            self.position.y += 1;
            self.position.x = 0;
            return Some(Ok(Token::Newline));
        }

        if let Some(rem) = remaining.strip_prefix(' ') {
            self.remaining = rem;
            self.position.x += 1;
            return Some(Ok(Token::Space));
        }

        // Escape opening bracket
        if let Some(rem) = remaining.strip_prefix("/<") {
            self.remaining = rem;
            self.position.x += 2;
            return Some(Ok(Token::Text("<")));
        }

        if let Some(rem) = remaining.strip_prefix("/") {
            self.remaining = rem;
            self.position.x += 1;
            return Some(Ok(Token::Text("/")));
        }

        // Not a tag
        if !remaining.starts_with('<') {
            if let Some(next_token) = remaining.find([' ', '<', '\n', '/']) {
                if next_token == 0 {
                    // We should have handled all the expected tags above
                    return Some(Err(anyhow!(
                        "Unhandled tag from string `{}` at {}",
                        self.remaining,
                        self.position,
                    )));
                }

                let text = &remaining[..next_token];
                let rem = &remaining[next_token..];

                self.remaining = rem;
                self.position.x += next_token as i32;
                return Some(Ok(Token::Text(text)));
            } else {
                // No more tags, consume remaining text
                self.remaining = "";
                return Some(Ok(Token::Text(remaining)));
            }
        }

        // Tags
        let Some(tag_end) = remaining.find('>') else {
            return Some(Err(anyhow!(
                "No closing brace on string `{}` starting at at {}",
                self.remaining,
                self.position
            )));
        };

        let tag = &remaining[1..tag_end];
        let prefix = &tag[..tag.len().min(3)];

        self.remaining = &remaining[tag_end + 1..];

        if prefix.eq_ignore_ascii_case("/fg") {
            self.position.x += 5;
            return Some(Ok(Token::FgEnd)); // tag + brackets
        }

        if prefix.eq_ignore_ascii_case("/bg") {
            self.position.x += 5;
            return Some(Ok(Token::BgEnd));
        }

        if prefix.eq_ignore_ascii_case("fg=") {
            let colortext = &tag[3..];
            if let Some(col) = color::parse_color_string(colortext) {
                self.position.x += 5 + colortext.len() as i32;
                return Some(Ok(Token::FgStart(col, colortext)));
            }
            return Some(Err(anyhow::anyhow!(
                "Invalid color in tag {} at {}",
                tag,
                self.position
            )));
        }

        if prefix.eq_ignore_ascii_case("bg=") {
            let colortext = &tag[3..];
            if let Some(col) = color::parse_color_string(colortext) {
                self.position.x += 5 + colortext.len() as i32;
                return Some(Ok(Token::BgStart(col, colortext)));
            }

            return Some(Err(anyhow::anyhow!(
                "Invalid color in tag {} at {}",
                tag,
                self.position
            )));
        }

        Some(Err(anyhow::anyhow!(
            "Unknown tag {} at {}",
            tag,
            self.position
        )))
    }
}

/// Returns the wrapped string, character count (minus tags), and remaining string
pub fn wrap_tagged_string(
    input: &str,
    max_len: usize,
    word_wrap: bool,
) -> Result<(&str, usize, &str)> {
    if input.trim().is_empty() {
        return Ok(("", 0, ""));
    }

    let mut byte_count = 0;
    let mut char_count = 0;
    let mut trailing_spaces = 0;

    let iter = TokenIterator::new(input);
    'start: for token in iter {
        let token = token?;

        match token {
            Token::Text(text) => {
                let mut chars = char_count;
                let mut bytes = byte_count;

                for ch in text.chars() {
                    if chars + 1 > max_len {
                        // if we haven't counted any text yet we need to wrap at max_len
                        if char_count == 0 || !word_wrap {
                            char_count = chars;
                            byte_count = bytes;
                        }
                        break 'start;
                    }
                    bytes += ch.len_utf8();
                    chars += 1;
                }

                char_count = chars;
                byte_count = bytes;
                trailing_spaces = 0; // Reset spaces after processing text
            }
            Token::Space => {
                if char_count + 1 >= max_len {
                    break;
                };
                char_count += 1;
                byte_count += 1;
                trailing_spaces += 1;
            }
            Token::Newline => {
                byte_count += '\n'.len_utf8();
                break;
            }
            Token::FgStart(_, name) => byte_count += 5 + name.len(), // <fg=> + name
            Token::BgStart(_, name) => byte_count += 5 + name.len(), // <bg=> + name
            Token::FgEnd | Token::BgEnd => byte_count += 5,          // </fg>
        }
    }

    char_count -= trailing_spaces;

    let wrapped = input[..byte_count].trim_end();
    let remaining = input[byte_count..].trim_start();

    Ok((wrapped, char_count, remaining))
}

pub fn wrap_tagged_line_count(input: &str, max_len: usize, word_wrap: bool) -> Result<usize> {
    if input.is_empty() {
        return Ok(0);
    }
    let (_, _, mut remaining) = wrap_tagged_string(input, max_len, word_wrap)?;
    let mut count = 1;
    while !remaining.is_empty() {
        (_, _, remaining) = wrap_tagged_string(remaining, max_len, word_wrap)?;
        count += 1;
    }
    Ok(count)
}

pub fn wrap_string(input: &str, max_len: usize, wrap: bool) -> (&str, &str) {
    let line = &input[..max_len.min(input.len())];

    if let Some(newline_pos) = line.find('\n') {
        return (&input[..newline_pos], &input[newline_pos + 1..]);
    }

    if input.len() <= max_len {
        return (input, "");
    }

    if wrap && let Some(last_space) = line.rfind(' ') {
        return (&input[..last_space], &input[last_space + 1..]);
    }

    // Hard break at max_len (on a char boundary)
    let mut split_at = max_len;
    while !input.is_char_boundary(split_at) {
        split_at -= 1;
    }
    (&input[..split_at], &input[split_at..])
}

/// Precalculate the number of vertical lines a wrapped string will occupy.
pub fn wrap_line_count(input: &str, max_len: usize, word_wrap: bool) -> usize {
    let mut count = 1;
    let mut res = wrap_string(input, max_len, word_wrap);
    while !res.1.is_empty() {
        res = wrap_string(res.1, max_len, word_wrap);
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::{Token, TokenIterator};
    use crate::{
        color,
        strings::{
            parse::{wrap_string, wrap_tagged_string},
            wrap_line_count,
        },
    };
    use anyhow::Result;

    #[test]
    fn parsing_tests() -> Result<()> {
        let input = "Hello <fg=blue>Wor<bg=orange>ld!</fg>.\n<fg=red>How</fg></bg>'s it going?";

        let expected = [
            Token::Text("Hello"),
            Token::Space,
            Token::FgStart(color::BLUE, "blue"),
            Token::Text("Wor"),
            Token::BgStart(color::ORANGE, "orange"),
            Token::Text("ld!"),
            Token::FgEnd,
            Token::Text("."),
            Token::Newline,
            Token::FgStart(color::RED, "red"),
            Token::Text("How"),
            Token::FgEnd,
            Token::BgEnd,
            Token::Text("'s"),
            Token::Space,
            Token::Text("it"),
            Token::Space,
            Token::Text("going?"),
        ];
        let tokens: Vec<Token> = TokenIterator::new(input)
            .map(|res| res.expect("Error parsing token"))
            .collect();
        assert_eq!(tokens, expected);

        Ok(())
    }

    #[test]
    fn empty_string() -> Result<()> {
        let input = "             \n       \t\t \n\n              \n \t";
        let (wrapped, count, remaining) = wrap_tagged_string(input, 15, true)?;

        assert_eq!("", wrapped);
        assert_eq!(0, count);
        assert_eq!("", remaining);
        Ok(())
    }

    #[test]
    fn wrapped_tagged_string_test() -> Result<()> {
        let input = r"A <bg=yellow><fg=blue>nice long string.</fg>
With some</bg> <bg=gray>tags that we want</bg> to wrap!
Also this line in particular is particularly long!";

        let (wrapped, count, remaining) = wrap_tagged_string(input, 15, true)?;

        assert_eq!(11, count);
        assert_eq!(wrapped, "A <bg=yellow><fg=blue>nice long");
        assert_eq!(
            remaining,
            r"string.</fg>
With some</bg> <bg=gray>tags that we want</bg> to wrap!
Also this line in particular is particularly long!"
        );

        let (wrapped, count, remaining) = wrap_tagged_string(remaining, 15, true)?;
        assert_eq!(7, count);
        assert_eq!(wrapped, "string.</fg>");
        assert_eq!(
            remaining,
            r"With some</bg> <bg=gray>tags that we want</bg> to wrap!
Also this line in particular is particularly long!"
        );

        let (wrapped, count, remaining) = wrap_tagged_string(remaining, 15, true)?;
        assert_eq!(14, count);
        assert_eq!(wrapped, "With some</bg> <bg=gray>tags");
        assert_eq!(
            remaining,
            r"that we want</bg> to wrap!
Also this line in particular is particularly long!"
        );

        let (wrapped, count, remaining) = wrap_tagged_string(remaining, 15, true)?;
        assert_eq!(15, count);
        assert_eq!(wrapped, "that we want</bg> to");
        assert_eq!(
            remaining,
            r"wrap!
Also this line in particular is particularly long!"
        );

        let (wrapped, count, remaining) = wrap_tagged_string(remaining, 15, true)?;
        assert_eq!(5, count);
        assert_eq!(wrapped, "wrap!");
        assert_eq!(
            remaining,
            "Also this line in particular is particularly long!"
        );

        let (wrapped, count, remaining) = wrap_tagged_string(remaining, 15, true)?;
        assert_eq!(14, count);
        assert_eq!(wrapped, "Also this line");
        assert_eq!(remaining, "in particular is particularly long!");

        let (wrapped, count, remaining) = wrap_tagged_string(remaining, 15, true)?;
        assert_eq!(13, count);
        assert_eq!(wrapped, "in particular");
        assert_eq!(remaining, "is particularly long!");

        let (wrapped, count, remaining) = wrap_tagged_string(remaining, 15, true)?;
        assert_eq!(15, count);
        assert_eq!(wrapped, "is particularly");
        assert_eq!(remaining, "long!");

        let (wrapped, count, remaining) = wrap_tagged_string(remaining, 15, true)?;
        assert_eq!(5, count);
        assert_eq!(wrapped, "long!");
        assert_eq!(remaining, "");

        Ok(())
    }

    #[test]
    fn plain_text_shorter_than_max_len() -> Result<()> {
        let (wrapped, count, rem) = wrap_tagged_string("hello", 10, true)?;
        assert_eq!("hello", wrapped);
        assert_eq!(5, count);
        assert_eq!("", rem);
        Ok(())
    }

    #[test]
    fn wraps_at_word_boundary_before_max_len() -> Result<()> {
        let (wrapped, count, rem) = wrap_tagged_string("hello world", 8, true)?;
        assert_eq!("hello", wrapped);
        assert_eq!(5, count);
        assert_eq!("world", rem);
        Ok(())
    }

    #[test]
    fn hard_wraps_without_spaces() -> Result<()> {
        let (wrapped, count, rem) = wrap_tagged_string("hellothere", 5, true)?;
        assert_eq!("hello", wrapped);
        assert_eq!(5, count);
        assert_eq!("there", rem);
        Ok(())
    }

    #[test]
    fn trailing_spaces_not_counted() -> Result<()> {
        let (wrapped, count, rem) = wrap_tagged_string("hi    ", 10, true)?;
        assert_eq!("hi", wrapped);
        assert_eq!(2, count);
        assert_eq!("", rem);
        Ok(())
    }

    #[test]
    fn leading_spaces_trimmed_in_remaining() -> Result<()> {
        let (wrapped, count, rem) = wrap_tagged_string("hello     world", 8, true)?;
        assert_eq!("hello", wrapped);
        assert_eq!(5, count);
        assert_eq!("world", rem);
        Ok(())
    }

    #[test]
    fn newline_breaks_the_line() -> Result<()> {
        let (wrapped, count, rem) = wrap_tagged_string("foo\nbar", 20, true)?;
        assert_eq!("foo", wrapped);
        assert_eq!(3, count);
        assert_eq!("bar", rem);
        Ok(())
    }

    #[test]
    fn fg_tag_bytes_not_counted_in_char_count() -> Result<()> {
        let (wrapped, count, rem) = wrap_tagged_string("<fg=red>Red</fg>", 10, true)?;
        assert_eq!("<fg=red>Red</fg>", wrapped);
        assert_eq!(3, count);
        assert_eq!("", rem);
        Ok(())
    }

    #[test]
    fn wrap_respects_char_count_not_byte_count_with_fg_tags() -> Result<()> {
        let (wrapped, count, rem) = wrap_tagged_string("<fg=red>Hi</fg> there", 5, true)?;
        assert_eq!("<fg=red>Hi</fg>", wrapped);
        assert_eq!(2, count);
        assert_eq!("there", rem);
        Ok(())
    }

    #[test]
    fn multibyte_unicode_characters_count_as_one_char_each() -> Result<()> {
        // "héllo" — é is 2 bytes but 1 char
        let (wrapped, count, rem) = wrap_tagged_string("héllo wörld", 5, true)?;
        assert_eq!("héllo", wrapped);
        assert_eq!(5, count);
        assert_eq!("wörld", rem);
        Ok(())
    }

    #[test]
    fn long_tagged_string() -> Result<()> {
        let input = "Normal <fg=red><bg=blue>Red on Blue</bg> Back to Red</fg> <fg=dark_magenta>Back to</fg> normal.";
        let (wrapped, count, rem) = wrap_tagged_string(input, 38, true)?;
        assert_eq!(
            "Normal <fg=red><bg=blue>Red on Blue</bg> Back to Red</fg> <fg=dark_magenta>Back to</fg>",
            wrapped
        );
        assert_eq!(38, count);
        assert_eq!("normal.", rem);
        Ok(())
    }

    #[test]
    fn long_string() -> Result<()> {
        let input = "This is what a really long string with no tags looks like.\nExplicit newline there. Otherwise it's just one loooooong string.";

        let (wrapped, _count, rem) = wrap_tagged_string(input, 38, true)?;
        assert_eq!("This is what a really long string with", wrapped);

        let (wrapped, _, rem) = wrap_tagged_string(rem, 38, true)?;
        assert_eq!("no tags looks like.", wrapped);

        let (wrapped, count, _) = wrap_tagged_string(rem, 38, true)?;
        assert_eq!(38, count);
        assert_eq!("Explicit newline there. Otherwise it's", wrapped);

        Ok(())
    }

    #[test]
    fn no_word_wrap() -> Result<()> {
        let input = "Here's a string but we're not word wrapping.";

        let (wrapped, _count, rem) = wrap_tagged_string(input, 12, false)?;
        assert_eq!("Here's a str", wrapped);

        let (wrapped, _count, rem) = wrap_tagged_string(rem, 12, false)?;
        assert_eq!("ing but we'r", wrapped);

        let (wrapped, _count, rem) = wrap_tagged_string(rem, 12, false)?;
        assert_eq!("e not word w", wrapped);

        let (wrapped, _count, _rem) = wrap_tagged_string(rem, 8, false)?;
        assert_eq!("rapping.", wrapped);

        Ok(())
    }

    #[test]
    fn wrap_untagged_string_test() {
        let input = "This is what a really long string with no tags looks like.\nExplicit newline there. Otherwise it's just one loooooong string.";

        let (wrapped, rem) = wrap_string(input, 38, true);
        assert_eq!("This is what a really long string with", wrapped);

        let (wrapped, rem) = wrap_string(rem, 38, true);
        assert_eq!("no tags looks like.", wrapped);

        let (wrapped, _) = wrap_string(rem, 38, true);

        assert_eq!("Explicit newline there. Otherwise it's", wrapped);
    }

    #[test]
    fn no_word_wrap_untagged() {
        let input = "Here's a string but we're not word wrapping.";

        let (wrapped, rem) = wrap_string(input, 12, false);
        assert_eq!("Here's a str", wrapped);

        let (wrapped, rem) = wrap_string(rem, 12, false);
        assert_eq!("ing but we'r", wrapped);

        let (wrapped, rem) = wrap_string(rem, 12, false);
        assert_eq!("e not word w", wrapped);

        let (wrapped, _rem) = wrap_string(rem, 8, false);
        assert_eq!("rapping.", wrapped);
    }

    #[test]
    fn small_untagged_string_wraps_to_two_lines() {
        let string = "Hello, how are";
        let count = wrap_line_count(string, 10, true);
        assert_eq!(count, 2);
    }
}
