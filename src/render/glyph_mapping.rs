use ron::{from_str};
use std::{collections::HashMap};

const DEFAULT_MAPPING: &str = include_str!("../../data/code_page_437.mapping");

pub struct GlyphMapping {
    mapping: HashMap<char, (usize, usize)>,
    reverse_mapping: HashMap<(usize, usize), char>,
    //size: (usize, usize),
}

impl GlyphMapping {
    pub fn new(mapping: HashMap<char, usize>, width: usize) -> Self {
        let mut indexed = HashMap::new();
        for (k, i) in mapping {
            indexed.insert(k, (i % width, i / width));
        }
        let mut reverse = HashMap::new();
        for (c, i) in &indexed {
            reverse.insert(*i, *c);
        }
        GlyphMapping {
            mapping: indexed,
            reverse_mapping: reverse,
            //size: (width, height),
        }
    }

    fn from_string(string: &str, width: usize) -> Self {
        let mapping = from_str(string).expect("Error parsing mapping file");
        GlyphMapping::new(mapping, width)
    }

    pub fn code_page_437() -> Self {
        GlyphMapping::from_string(DEFAULT_MAPPING, 16)
    }

    pub fn get_glyph(&self, index_x: usize, index_y: usize) -> char {
        *self
            .reverse_mapping
            .get(&(index_x, index_y))
            .unwrap_or(&' ')
    }

    pub fn get_index(&self, ch: char) -> (usize, usize) {
        *self.mapping.get(&ch).unwrap_or(&(0, 0))
    }
}

impl Default for GlyphMapping {
    fn default() -> Self {
        GlyphMapping::code_page_437()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cp437() {
        let mapping = GlyphMapping::default();
        assert_eq!(mapping.get_glyph(1, 0), '☺');
        assert_eq!(mapping.get_glyph(2, 0), '☻');
        assert_eq!(mapping.get_glyph(13, 15), '²');
        assert_eq!(mapping.get_glyph(14, 15), '■');
        //assert_eq!(mapping.get_glyph(12,2), ',');

        assert_eq!(mapping.get_index('☺'), (1, 0));
        assert_eq!(mapping.get_index('☻'), (2, 0));
        assert_eq!(mapping.get_index('²'), (13, 15));
        assert_eq!(mapping.get_index('■'), (14, 15));
        assert_eq!(mapping.get_index('─'), (4, 12));
        assert_eq!(mapping.get_index(','), (12, 2));
    }
}
