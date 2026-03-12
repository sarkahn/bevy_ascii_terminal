use crate::color;
use anyhow::{Result, anyhow};
use bevy::{color::LinearRgba, math::IVec2};

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    Text(&'a str),
    Space,
    Newline,
    FgStart(LinearRgba),
    BgStart(LinearRgba),
    FgEnd,
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
    type Item = anyhow::Result<Token<'a>>;

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

        if let Some(rem) = remaining.strip_prefix("/<") {
            self.remaining = rem;
            self.position.x += 1;
            return Some(Ok(Token::Text("<")));
        }

        // Not a tag
        if !remaining.starts_with('<') {
            if let Some(next_token) = remaining.find([' ', '<', '\n', '/']) {
                if next_token == 0 {
                    let ch = remaining.chars().next().unwrap();
                    let len = ch.len_utf8();
                    self.remaining = &remaining[len..];
                    self.position.x += 1;
                    return Some(Ok(Token::Text(&remaining[..len])));
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
            return Some(Err(anyhow::anyhow!(
                "No closing brace on string tag starting at at {}",
                self.position
            )));
        };

        let tag = &remaining[1..tag_end];
        let rem = &remaining[tag_end + 1..];
        let prefix = &tag[..tag.len().min(3)];

        self.remaining = rem;

        if prefix.eq_ignore_ascii_case("/fg") {
            return Some(Ok(Token::FgEnd));
        }

        if prefix.eq_ignore_ascii_case("/bg") {
            return Some(Ok(Token::BgEnd));
        }

        if prefix.eq_ignore_ascii_case("fg=") {
            if let Some(col) = color::parse_color_string(&tag[3..]) {
                return Some(Ok(Token::FgStart(col)));
            }
            return Some(Err(anyhow::anyhow!(
                "Invalid color in tag {} at {}",
                tag,
                self.position
            )));
        }

        if prefix.eq_ignore_ascii_case("bg=") {
            if let Some(col) = color::parse_color_string(&tag[3..]) {
                return Some(Ok(Token::BgStart(col)));
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

#[cfg(test)]
mod tests {
    use super::{Token, TokenIterator};
    use crate::color;
    use anyhow::Result;

    #[test]
    fn parsing_tests() -> Result<()> {
        let input = "Hello <fg=blue>Wor<bg=orange>ld!</fg>.\n<fg=red>How</fg></bg>'s it going?";

        let expected = [
            Token::Text("Hello"),
            Token::Space,
            Token::FgStart(color::BLUE),
            Token::Text("Wor"),
            Token::BgStart(color::ORANGE),
            Token::Text("ld!"),
            Token::FgEnd,
            Token::Text("."),
            Token::Newline,
            Token::FgStart(color::RED),
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
    fn big_unicode_string() {
        for _ in TokenIterator::new(UNICODE) {}
    }

    const UNICODE: &str = r#"
.☺☻♥♦♣♠•◘○◙♂♀♪♫☼ ►◄↕‼¶§▬↨↑↓→←∟↔▲▼
!\"\#$%&'()*+,-./ 0123456789:;<=>?
@ABCDEFGHIJKLMNO PQRSTUVWXYZ[\]^_
`abcdefghijklmno pqrstuvwxyz{|}~⌂
ÇüéâäàåçêëèïîìÄÅ ÉæÆôöòûùÿÖÜ¢£¥₧ƒ
áíóúñÑªº¿⌐¬½¼¡«» ░▒▓│┤╡╢╖╕╣║╗╝╜╛┐
└┴┬├─┼╞╟╚╔╩╦╠═╬╧ ╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀
αßΓπΣσµτΦΘΩδ∞φε∩ ≡±≥≤⌠⌡÷≈°∙·√ⁿ²■□
"#;
}
