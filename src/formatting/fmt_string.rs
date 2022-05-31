

// #[allow(clippy::len_without_is_empty)]
// /// A trait for writing a string of formatted tiles to the terminal.
// pub trait TilesWriter<'a> {
//     /// Returns the formatted tiles from a base type.
//     fn formatted(self) -> FormattedTiles<'a>;
//     /// Set the foreground color for the tiles.
//     fn fg(self, fg_color: Color) -> FormattedTiles<'a>;
//     /// Set the background color for the tiles.
//     fn bg(self, bg_color: Color) -> FormattedTiles<'a>;
//     /// Apply the formatting to the given set of tiles.
//     fn write(&self, tiles: impl Iterator<Item = &'a mut Tile>);
//     /// The length of the writer tiles.
//     fn len(&self) -> usize;
// }

// /// A set of formatted tiles for writing to the terminal.
// #[derive(Debug, Clone, Default)]
// pub struct FormattedTiles<'a> {
//     pub string: &'a str,
//     pub fg_color: Option<Color>,
//     pub bg_color: Option<Color>,
// }

// impl<'a> FormattedTiles<'a> {
//     pub fn new(chars: &'a str) -> Self {
//         FormattedTiles {
//             string: chars,
//             ..Default::default()
//         }
//     }
// }

// impl<'a> TilesWriter<'a> for &'a str {
//     fn formatted(self) -> FormattedTiles<'a> {
//         FormattedTiles {
//             string: self,
//             ..Default::default()
//         }
//     }

//     fn fg(self, fg_color: Color) -> FormattedTiles<'a> {
//         FormattedTiles {
//             string: self,
//             fg_color: Some(fg_color),
//             ..Default::default()
//         }
//     }

//     fn bg(self, bg_color: Color) -> FormattedTiles<'a> {
//         FormattedTiles {
//             string: self,
//             bg_color: Some(bg_color),
//             ..Default::default()
//         }
//     }

//     fn write(&self, tiles: impl Iterator<Item = &'a mut Tile>) {
//         for (tile, ch) in tiles.zip(self.chars()) {
//             tile.set_glyph(ch);
//         }
//     }

//     fn len(&self) -> usize {
//         (self as &str).len()
//     }
// }
