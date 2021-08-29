use std::{collections::{HashMap}, fs::File};
use ron::de::from_reader;

struct GlyphMapping {
    mapping: HashMap<char,usize>,
    reverse_mapping: HashMap<usize,char>,
}

impl GlyphMapping {
    fn new(mapping: HashMap<char,usize>) -> Self {
        let mut reverse = HashMap::new();
        for (c,i) in &mapping {
            reverse.insert(*i,*c);
        }
        GlyphMapping {
            mapping: mapping,
            reverse_mapping: reverse,
        }
    }

    fn From_File(path: &str) -> Self {
        let f = File::open(path).expect("Error opening file");
        let mapping = from_reader(f).expect("Error parsing mapping file");
        GlyphMapping::new(mapping)
    }

    fn Code_Page_437() -> Self {
        GlyphMapping::From_File("assets/code_page_437.mapping")
    }

    fn get_glyph(&self, index: usize) -> char {
        *self.reverse_mapping.get(&index).unwrap_or(&' ')
    }

    fn get_index(&self, ch: char) -> usize {
        *self.mapping.get(&ch).unwrap_or(&0)
    }
}

impl Default for GlyphMapping {
    fn default() -> Self {
        GlyphMapping::Code_Page_437()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cp437() {
        let mapping = GlyphMapping::default();
        assert_eq!(mapping.get_glyph(1), '☺');
        assert_eq!(mapping.get_glyph(2), '☻');
        assert_eq!(mapping.get_glyph(253), '²');
        assert_eq!(mapping.get_glyph(254), '■');

        assert_eq!(mapping.get_index('☺'), 1);
        assert_eq!(mapping.get_index('☻'), 2);
        assert_eq!(mapping.get_index('²'), 253);
        assert_eq!(mapping.get_index('■'), 254);
    }

}