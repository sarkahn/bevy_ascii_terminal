//! Utilities for translating between glyphs and Code Page 437 indices.

/// Array containing, in order, all the chars from Code Page 437.
pub const CP_437_CHARS: [char;255] = [
    ' ', '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼', 
    '►', '◄', '↕', '‼', '¶', '§', '▬', '↨', '↑', '↓', '→', '←', '∟', '↔', '▲', '▼', 
    ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', 
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', 
    '@', 'A', 'B','C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', 
    '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~', '⌂', 
    'Ç', 'ü', 'é', 'â', 'ä', 'à', 'å', 'ç', 'ê', 'ë', 'è', 'ï', 'î', 'ì', 'Ä', 'Å', 
    'É', 'æ', 'Æ', 'ô', 'ö', 'ò', 'û', 'ù', 'ÿ', 'Ö', 'Ü', '¢', '£', '¥', '₧', 'ƒ', 
    'á', 'í', 'ó', 'ú', 'ñ', 'Ñ', 'ª', 'º', '¿', '⌐', '¬', '½', '¼', '¡', '«', '»', 
    '░', '▒', '▓', '│', '┤', '╡', '╢', '╖', '╕', '╣', '║', '╗', '╝', '╜', '╛', '┐',  
    '└', '┴', '┬', '├', '─', '┼', '╞', '╟', '╚', '╔', '╩', '╦', '╠', '═', '╬', '╧', 
    '╨', '╤', '╥', '╙', '╘', '╒', '╓', '╫', '╪', '┘', '┌', '█', '▄', '▌', '▐', '▀', 
    'α', 'ß', 'Γ', 'π', 'Σ', 'σ', 'µ', 'τ', 'Φ', 'Θ', 'Ω', 'δ', '∞', 'φ', 'ε', '∩', 
    '≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■',
    ];

/// Convert an index (0..=255) to a cp437 glyph.
pub fn index_to_glyph(i: u8) -> char {
    match i {
        0 => ' ',
        255 => ' ',
        _ => CP_437_CHARS[i as usize]
    }
}

/// Convert a cp437 glyph to an index (0..=255)
pub fn glyph_to_index(c: char) -> u8 {
    match c {
        '☺' => 1,
        '☻' => 2,
        '♥' => 3,
        '♦' => 4,
        '♣' => 5,
        '♠' => 6,
        '•' => 7,
        '◘' => 8,
        '○' => 9,
        '◙' => 10,
        '♂' => 11,
        '♀' => 12,
        '♪' => 13,
        '♫' => 14,
        '☼' => 15,

        '►' => 16,
        '◄' => 17,
        '↕' => 18,
        '‼' => 19,
        '¶' => 20,
        '§' => 21,
        '▬' => 22,
        '↨' => 23,
        '↑' => 24,
        '↓' => 25,
        '→' => 26,
        '←' => 27,
        '∟' => 28,
        '↔' => 29,
        '▲' => 30,
        '▼' => 31,

        ' ' => 32,
        '!' => 33,
        '"' => 34,
        '#' => 35,
        '$' => 36,
        '%' => 37,
        '&' => 38,
        '\'' => 39,
        '(' => 40,
        ')' => 41,
        '*' => 42,
        '+' => 43,
        ',' => 44,
        '-' => 45,
        '.' => 46,
        '/' => 47,

        '0' => 48,
        '1' => 49,
        '2' => 50,
        '3' => 51,
        '4' => 52,
        '5' => 53,
        '6' => 54,
        '7' => 55,
        '8' => 56,
        '9' => 57,
        ':' => 58,
        ';' => 59,
        '<' => 60,
        '=' => 61,
        '>' => 62,
        '?' => 63,

        '@' => 64,
        'A' => 65,
        'B' => 66,
        'C' => 67,
        'D' => 68,
        'E' => 69,
        'F' => 70,
        'G' => 71,
        'H' => 72,
        'I' => 73,
        'J' => 74,
        'K' => 75,
        'L' => 76,
        'M' => 77,
        'N' => 78,
        'O' => 79,

        'P' => 80,
        'Q' => 81,
        'R' => 82,
        'S' => 83,
        'T' => 84,
        'U' => 85,
        'V' => 86,
        'W' => 87,
        'X' => 88,
        'Y' => 89,
        'Z' => 90,
        '[' => 91,
        '\\' => 92,
        ']' => 93,
        '^' => 94,
        '_' => 95,

        '`' => 96,
        'a' => 97,
        'b' => 98,
        'c' => 99,
        'd' => 100,
        'e' => 101,
        'f' => 102,
        'g' => 103,
        'h' => 104,
        'i' => 105,
        'j' => 106,
        'k' => 107,
        'l' => 108,
        'm' => 109,
        'n' => 110,
        'o' => 111,

        'p' => 112,
        'q' => 113,
        'r' => 114,
        's' => 115,
        't' => 116,
        'u' => 117,
        'v' => 118,
        'w' => 119,
        'x' => 120,
        'y' => 121,
        'z' => 122,
        '{' => 123,
        '|' => 124,
        '}' => 125,
        '~' => 126,
        '⌂' => 127,

        'Ç' => 128,
        'ü' => 129,
        'é' => 130,
        'â' => 131,
        'ä' => 132,
        'à' => 133,
        'å' => 134,
        'ç' => 135,
        'ê' => 136,
        'ë' => 137,
        'è' => 138,
        'ï' => 139,
        'î' => 140,
        'ì' => 141,
        'Ä' => 142,
        'Å' => 143,

        'É' => 144,
        'æ' => 145,
        'Æ' => 146,
        'ô' => 147,
        'ö' => 148,
        'ò' => 149,
        'û' => 150,
        'ù' => 151,
        'ÿ' => 152,
        'Ö' => 153,
        'Ü' => 154,
        '¢' => 155,
        '£' => 156,
        '¥' => 157,
        '₧' => 158,
        'ƒ' => 159,

        'á' => 160,
        'í' => 161,
        'ó' => 162,
        'ú' => 163,
        'ñ' => 164,
        'Ñ' => 165,
        'ª' => 166,
        'º' => 167,
        '¿' => 168,
        '⌐' => 169,
        '¬' => 170,
        '½' => 171,
        '¼' => 172,
        '¡' => 173,
        '«' => 174,
        '»' => 175,

        '░' => 176,
        '▒' => 177,
        '▓' => 178,
        '│' => 179,
        '┤' => 180,
        '╡' => 181,
        '╢' => 182,
        '╖' => 183,
        '╕' => 184,
        '╣' => 185,
        '║' => 186,
        '╗' => 187,
        '╝' => 188,
        '╜' => 189,
        '╛' => 190,
        '┐' => 191,

        '└' => 192,
        '┴' => 193,
        '┬' => 194,
        '├' => 195,
        '─' => 196,
        '┼' => 197,
        '╞' => 198,
        '╟' => 199,
        '╚' => 200,
        '╔' => 201,
        '╩' => 202,
        '╦' => 203,
        '╠' => 204,
        '═' => 205,
        '╬' => 206,
        '╧' => 207,

        '╨' => 208,
        '╤' => 209,
        '╥' => 210,
        '╙' => 211,
        '╘' => 212,
        '╒' => 213,
        '╓' => 214,
        '╫' => 215,
        '╪' => 216,
        '┘' => 217,
        '┌' => 218,
        '█' => 219,
        '▄' => 220,
        '▌' => 221,
        '▐' => 222,
        '▀' => 223,

        'α' => 224,
        'ß' => 225,
        'Γ' => 226,
        'π' => 227,
        'Σ' => 228,
        'σ' => 229,
        'µ' => 230,
        'τ' => 231,
        'Φ' => 232,
        'Θ' => 233,
        'Ω' => 234,
        'δ' => 235,
        '∞' => 236,
        'φ' => 237,
        'ε' => 238,
        '∩' => 239,

        '≡' => 240,
        '±' => 241,
        '≥' => 242,
        '≤' => 243,
        '⌠' => 244,
        '⌡' => 245,
        '÷' => 246,
        '≈' => 247,
        '°' => 248,
        '∙' => 249,
        '·' => 250,
        '√' => 251,
        'ⁿ' => 252,
        '²' => 253,
        '■' => 254,

        _ => 0,
    }
}
