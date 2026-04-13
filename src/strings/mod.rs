mod tagged_string;
mod terminal_string;

pub use tagged_string::{
    Token, TokenIterator, wrap_line_count, wrap_string, wrap_tagged_line_count, wrap_tagged_string,
};
pub use terminal_string::{TerminalString, TerminalStringBuilder};

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

/// Decode the unicode character and bytes it occupies from the next part of
/// a slice of string bytes.
pub const fn decode_char(bytes: &[u8], pos: usize) -> (char, usize) {
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
