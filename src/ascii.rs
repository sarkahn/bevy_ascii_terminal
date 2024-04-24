//! Extended ascii used as the default for terminal writing functions.
use enum_ordinalize::Ordinalize;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Unable to convert from char to terminal glyph")]
pub struct GlyphFromCharError;

/// An ascii glyph that can be drawn to a terminal.
///
/// Can be converted directly into a [char].
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Ordinalize)]
pub enum Glyph {
    /// '\0' (c string terminator)
    Null = 0,
    /// ☺
    SmilingFace = 1,
    /// ☻
    SmilingFaceInverse = 2,
    /// ♥
    Heart = 3,
    /// ♦
    Diamond = 4,
    /// ♣
    Club = 5,
    /// ♠
    Spade = 6,
    /// •
    Bullet = 7,
    /// ◘
    BulletInverse = 8,
    /// ○
    Circle = 9,
    /// ◙
    CircleInverse = 10,
    /// ♂
    Male = 11,
    /// ♀
    Female = 12,
    /// ♪
    EighthNote = 13,
    /// ♫
    BeamedEighthNotes = 14,
    /// ☼
    SunWithRays = 15,

    /// ►
    TriangleRight = 16,
    /// ◄
    TriangleLeft = 17,
    /// ↕
    ArrowUpDown = 18,
    /// ‼
    ExclamationDouble = 19,
    /// ¶
    Pilcrow = 20,
    /// §
    SectionSign = 21,
    /// ▬
    BlackRectangle = 22,
    /// ↨
    ArrowUpDownWithBase = 23,
    /// ↑
    ArrowUp = 24,
    /// ↓
    ArrowDown = 25,
    /// →
    ArrowRight = 26,
    /// ←
    ArrowLeft = 27,
    /// ∟
    AngleRight = 28,
    /// ↔
    ArrowLeftRight = 29,
    /// ▲
    TriangleUp = 30,
    /// ▼
    TriangleDown = 31,

    #[default]
    /// ' '
    Space = 32,
    /// !
    Exclamation = 33,
    /// "
    DoubleQuote = 34,
    /// #
    NumberSign = 35,
    /// $
    Dollar = 36,
    /// %
    Percent = 37,
    /// &
    Ampersand = 38,
    /// '
    Apostrophe = 39,
    /// (
    ParenthesisLeft = 40,
    /// )
    ParenthesisRight = 41,
    /// *
    Asterisk = 42,
    /// +
    Plus = 43,
    /// ,
    Comma = 44,
    /// -
    Minus = 45,
    /// .
    Period = 46,
    /// /
    Slash = 47,

    /// 0
    Zero = 48,
    /// 1
    One = 49,
    /// 2
    Two = 50,
    /// 3
    Three = 51,
    /// 4
    Four = 52,
    /// 5
    Five = 53,
    /// 6
    Six = 54,
    /// 7
    Seven = 55,
    /// 8
    Eight = 56,
    /// 9
    Nine = 57,
    /// :
    Colon = 58,
    /// ;
    Semicolon = 59,
    /// <
    LessThan = 60,
    /// =
    Equals = 61,
    /// >
    GreaterThan = 62,
    /// ?
    QuestionMark = 63,

    /// @
    AtSymbol = 64,
    /// A
    AUpper = 65,
    /// B
    BUpper = 66,
    /// C
    CUpper = 67,
    /// D
    DUpper = 68,
    /// E
    EUpper = 69,
    /// F
    FUpper = 70,
    /// G
    GUpper = 71,
    /// H
    HUpper = 72,
    /// I
    IUpper = 73,
    /// J
    JUpper = 74,
    /// K
    KUpper = 75,
    /// L
    LUpper = 76,
    /// M
    MUpper = 77,
    /// N
    NUpper = 78,
    /// O
    OUpper = 79,
    /// P
    PUpper = 80,
    /// Q
    QUpper = 81,
    /// R
    RUpper = 82,
    /// S
    SUpper = 83,
    /// T
    TUpper = 84,
    /// U
    UUpper = 85,
    /// V
    VUpper = 86,
    /// W
    WUpper = 87,
    /// X
    XUpper = 88,
    /// Y
    YUpper = 89,
    /// Z
    ZUpper = 90,
    /// [
    SquareBracketLeft = 91,
    /// \
    Backslash = 92,
    /// ]
    SquareBracketRight = 93,
    /// ^
    Caret = 94,
    /// _
    Underscore = 95,

    /// `
    GraveAccent = 96,
    /// a
    ALower = 97,
    /// b
    BLower = 98,
    /// c
    CLower = 99,
    /// d
    DLower = 100,
    /// e
    ELower = 101,
    /// f
    FLower = 102,
    /// g
    GLower = 103,
    /// h
    HLower = 104,
    /// i
    ILower = 105,
    /// j
    JLower = 106,
    /// k
    KLower = 107,
    /// l
    LLower = 108,
    /// m
    MLower = 109,
    /// n
    NLower = 110,
    /// o
    OLower = 111,
    /// p
    PLower = 112,
    /// q
    QLower = 113,
    /// r
    RLower = 114,
    /// s
    SLower = 115,
    /// t
    TLower = 116,
    /// u
    ULower = 117,
    /// v
    VLower = 118,
    /// w
    WLower = 119,
    /// x
    XLower = 120,
    /// y
    YLower = 121,
    /// z
    ZLower = 122,
    /// {
    CurlyBraceLeft = 123,
    /// |
    Pipe = 124,
    /// }
    CurlyBraceRight = 125,
    /// ~
    Tilde = 126,
    /// ⌂
    House = 127,

    /// Ç
    LatinCUpperWithCedilla = 128,
    /// ü
    LatinULowerWithDiaeresis = 129,
    /// é
    LatinELowerWithAcute = 130,
    /// â
    LatinALowerWithCircumflex = 131,
    /// ä
    LatinALowerWithDiaeresis = 132,
    /// à
    LatinALowerWithGrave = 133,
    /// å
    LatinALowerWithRingAbove = 134,
    /// ç
    LatinCLowerWithCedilla = 135,
    /// ê
    LatinELowerWithCircumflex = 136,
    /// ë
    LatinELowerWithDiaeresis = 137,
    /// è
    LatinELowerWithGrave = 138,
    /// ï
    LatinILowerWithDiaeresis = 139,
    /// î
    LatinILowerWithCircumflex = 140,
    /// ì
    LatinILowerWithGrave = 141,
    /// Ä
    LatinAUpperWithDiaeresis = 142,
    /// Å
    LatinAUpperWithRingAbove = 143,

    /// É
    LatinEUpperWithAcute = 144,
    /// æ
    LatinAELower = 145,
    /// Æ
    LatinAEUpper = 146,
    /// ô
    LatinOLowerWithCircumflex = 147,
    /// ö
    LatinOLowerWithDiaeresis = 148,
    /// ò
    LatinOLowerWithGrave = 149,
    /// û
    LatinULowerWithCircumflex = 150,
    /// ù
    LatinULowerWithGrave = 151,
    /// ÿ
    LatinYLowerWithDiaeresis = 152,
    /// Ö
    LatinOUpperWithDiaeresis = 153,
    /// Ü
    LatinUUpperWithDiaeresis = 154,
    /// ¢
    Cent = 155,
    /// £
    Pound = 156,
    /// ¥
    Yen = 157,
    /// ₧
    Peseta = 158,
    /// ƒ
    LatinFLowerWithHook = 159,

    /// á
    LatinALowerWithAcute = 160,
    /// í
    LatinILowerWithAcute = 161,
    /// ó
    LatinOLowerWithAcute = 162,
    /// ú
    LatinULowerWithAcute = 163,
    /// ñ
    LatinNLowerWithTilde = 164,
    /// Ñ
    LatinNUpperWithTilde = 165,
    /// ª
    OrdinalFeminine = 166,
    /// º
    OrdinalMasculine = 167,
    /// ¿
    QuestionMarkFlipped = 168,
    /// ⌐
    NotSignFlipped = 169,
    /// ¬
    NotSign = 170,
    /// ½
    FractionHalf = 171,
    /// ¼
    FractionQuarter = 172,
    /// ¡
    ExclamationFlipped = 173,
    /// «
    AngleBracketLeftDouble = 174,
    /// »
    AngleBracketRightDouble = 175,

    /// ░
    ShadeLight = 176,
    /// ▒
    ShadeMedium = 177,
    /// ▓
    ShadeDark = 178,
    /// │
    BoxVerticalSingle = 179,
    /// ┤
    BoxVerticalSingleAndLeftSingle = 180,
    /// ╡
    BoxVerticalSingleAndLeftDouble = 181,
    /// ╢
    BoxVerticalDoubleAndLeftSingle = 182,
    /// ╖
    BoxDownDoubleAndLeftSingle = 183,
    /// ╕
    BoxDownSingleAndLeftDouble = 184,
    /// ╣
    BoxVerticalDoubleAndLeftDouble = 185,
    /// ║
    BoxVerticalDouble = 186,
    /// ╗
    BoxDownDoubleAndLeftDouble = 187,
    /// ╝
    BoxUpDoubleAndLeftDouble = 188,
    /// ╜
    BoxUpDoubleAndLeftSingle = 189,
    /// ╛
    BoxUpSingleAndLeftDouble = 190,
    /// ┐
    BoxDownSingleAndLeftSingle = 191,

    /// └
    BoxUpSingleAndRightSingle = 192,
    /// ┴
    BoxUpSingleAndHorizontalSingle = 193,
    /// ┬
    BoxDownSingleAndHorizontalSingle = 194,
    /// ├
    BoxVerticalSingleAndRightSingle = 195,
    /// ─
    BoxHorizontalSingle = 196,
    /// ┼
    BoxVerticalSingleAndHorizontalSingle = 197,
    /// ╞
    BoxVerticalSingleAndRightDouble = 198,
    /// ╟
    BoxVerticalDoubleAndRightSingle = 199,
    /// ╚
    BoxUpDoubleAndRightDouble = 200,
    /// ╔
    BoxDownDoubleAndRightDouble = 201,
    /// ╩
    BoxUpDoubleAndHorizontalDouble = 202,
    /// ╦
    BoxHorizontalDoubleAndDownDouble = 203,
    /// ╠
    BoxVerticalDoubleAndRightDouble = 204,
    /// ═
    BoxHorizontalDouble = 205,
    /// ╬
    BoxVerticalDoubleAndHorizontalDouble = 206,
    /// ╧
    BoxUpSingleAndHorizontalDouble = 207,

    /// ╨
    BoxUpDoubleAndHorizontalSingle = 208,
    /// ╤
    BoxDownSingleAndHorizontalDouble = 209,
    /// ╥
    BoxDownDoubleAndHorizontalSingle = 210,
    /// ╙
    BoxUpDoubleAndRightSingle = 211,
    /// ╘
    BoxUpSingleAndRightDouble = 212,
    /// ╒
    BoxDownSingleAndRightDouble = 213,
    /// ╓
    BoxDownDoubleAndRightSingle = 214,
    /// ╫
    BoxVerticalDoubleAndHorizontalSingle = 215,
    /// ╪
    BoxVerticalSingleAndHorizontalDouble = 216,
    /// ┘
    BoxUpSingleAndLeftSingle = 217,
    /// ┌
    BoxDownSingleAndRightSingle = 218,
    /// █
    BlockFull = 219,
    /// ▄
    BlockLowerHalf = 220,
    /// ▌
    BlockLeftHalf = 221,
    /// ▐
    BlockRightHalf = 222,
    /// ▀
    BlockUpperHalf = 223,

    /// α
    GreekAlphaLower = 224,
    /// ß
    LatinSharpSLower = 225,
    /// Γ
    GreekGammaUpper = 226,
    /// π
    GreekPiLower = 227,
    /// Σ
    GreekSigmaUpper = 228,
    /// σ
    GreekSigmaLower = 229,
    /// µ
    MicroSign = 230,
    /// τ
    GreekTauLower = 231,
    /// Φ
    GreekPhiUpper = 232,
    /// Θ
    GreekThetaUpper = 233,
    /// Ω
    GreekOmegaUpper = 234,
    /// δ
    GreekDeltaLower = 235,
    /// ∞
    Infinity = 236,
    /// φ
    GreekPhiLower = 237,
    /// ε
    GreekEpsilonLower = 238,
    /// ∩
    Intersection = 239,

    /// ≡
    IdenticalTo = 240,
    /// ±
    PlusMinus = 241,
    /// ≥
    GreaterThanOrEqualTo = 242,
    /// ≤
    LessThanOrEqualTo = 243,
    /// ⌠
    TopHalfIntegral = 244,
    /// ⌡
    BottomHalfIntegral = 245,
    /// ÷
    Division = 246,
    /// ≈
    AlmostEqualTo = 247,
    /// °
    DegreeSign = 248,
    /// ∙
    BulletOperator = 249,
    /// ·
    MiddleDot = 250,
    /// √
    SquareRoot = 251,
    /// ⁿ
    SuperscriptLatinSmallN = 252,
    /// ²
    SuperscriptTwo = 253,
    /// ■
    SquareSmall = 254,
    /// □ (Note this is not actually a code page 437 glyph. It was added
    /// manually to all built in fonts for decorative purposes)
    SquareSmallEmpty = 255,
}

/// Array of the default ascii glyphs supported by the terminal.
#[rustfmt::skip]
pub(crate) const CP_437_ARRAY: [char; 256] = [
'\0', '☺', '☻', '♥', '♦', '♣', '♠', '•', '◘', '○', '◙', '♂', '♀', '♪', '♫', '☼', 
'►', '◄', '↕', '‼', '¶', '§', '▬', '↨', '↑', '↓', '→', '←', '∟', '↔', '▲', '▼', 
' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', 
'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', 
'@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 
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
'≡', '±', '≥', '≤', '⌠', '⌡', '÷', '≈', '°', '∙', '·', '√', 'ⁿ', '²', '■', '□'
];

impl From<Glyph> for char {
    fn from(value: Glyph) -> Self {
        value.to_char()
    }
}

impl TryFrom<char> for Glyph {
    type Error = GlyphFromCharError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Glyph::from_char(value).ok_or(GlyphFromCharError)
    }
}

impl Glyph {
    /// Convert an ascii glyph to it's corresponding char.
    pub const fn to_char(self) -> char {
        CP_437_ARRAY[self as usize]
    }

    /// Convert from a char to a terminal ascii glyph. Returns [None] if the char
    /// is not a valid terminal glyph.
    pub fn from_char(ch: char) -> Option<Self> {
        char_to_index(ch).and_then(Self::from_ordinal)
    }
}

/// Convert an ascii glyph to it's corresponding index (0..256)
const fn char_to_index(c: char) -> Option<u8> {
    let value = match c {
        '\0' => 0,
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
        '□' => 255,

        _ => return None,
    };
    Some(value)
}
