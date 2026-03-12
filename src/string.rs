// //! Utilities  for writing formatted/decorated strings to the terminal
// //! without any extra allocations.
// use std::{ops::Sub, str::Chars};

pub fn wrap_line(input: &str, max_len: usize, wrap: bool) -> (&str, &str) {
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
fn line_count(input: &str, max_len: usize, word_wrap: bool) -> usize {
    let mut count = 1;
    let mut res = wrap_line(input, max_len, word_wrap);
    while !res.1.is_empty() {
        res = wrap_line(res.1, max_len, word_wrap);
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newline_line_wrap() {
        let (split, remaining) = wrap_line("A simple string\nWith a newline", 12, false);
        assert_eq!("A simple str", split);
        assert_eq!("ing\nWith a newline", remaining);
        let (split, remaining) = wrap_line(remaining, 12, false);
        assert_eq!("ing", split);
        assert_eq!("With a newline", remaining);
        let (split, remaining) = wrap_line(remaining, 12, false);
        assert_eq!("With a newli", split);
        assert_eq!("ne", remaining);
        let (split, remaining) = wrap_line(remaining, 12, false);
        assert_eq!("ne", split);
        assert_eq!("", remaining);
    }

    #[test]
    fn newline_word_wrap() {
        let (wrapped, remaining) = wrap_line("A simple string\nWith a newline", 12, true);
        assert_eq!("A simple", wrapped);
        assert_eq!("string\nWith a newline", remaining);
        let (wrapped, remaining) = wrap_line(remaining, 12, true);
        assert_eq!("string", wrapped);
        assert_eq!("With a newline", remaining);
        let (wrapped, remaining) = wrap_line(remaining, 12, true);
        assert_eq!("With a", wrapped);
        assert_eq!("newline", remaining);
        let (wrapped, remaining) = wrap_line(remaining, 12, true);
        assert_eq!("newline", wrapped);
        assert_eq!("", remaining);
    }

    #[test]
    fn wrap_line_count() {
        let string = "A somewhat longer line\nWith a newline or two\nOkay? WHEEEEEE.";
        assert_eq!(7, line_count(string, 12, true));
        assert_eq!(6, line_count(string, 12, false));
    }
}
