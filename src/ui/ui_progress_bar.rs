use bevy::{prelude::{Color, Component}, math::IVec2};
use sark_grids::GridPoint;

use crate::{ Terminal};

#[derive(Debug,Clone)]
pub struct UiProgressBar {
    max: i32,
    value: i32,
    glyph_fill: GlyphFill,
    color_fill: ColorFill,
}

#[derive(Debug, Clone)]
pub enum GlyphFill {
    /// The entire progress bar is always a single glyph, regardless of value.
    Static(char),
    /// Each segment will be filled or empty based on value. 
    EmptyOrFilled(char,char),
    /// The right-most segment will transition through the glyph string based on value. 
    /// All other segments will be filled or empty. 
    EmptyOrFilledWithTransition(String),
}
impl Default for GlyphFill {
    fn default() -> Self {
        GlyphFill::EmptyOrFilled('░','▓')
    }
}

#[derive(Debug,Clone)]
pub enum ColorFill {
    /// The entire bar is always a single color, reglardless of value.
    Static(Color),
    /// Each segment will be either the empty color or the filled color. 
    EmptyOrFilled(Color, Color),
    /// The right-most segment will transition from filled color to empty color.
    /// All other segments will be filled or empty.
    EmptyToFilledSegmentedTransition(Color,Color),
    /// The entire bar will transition from filled color to empty color based on value. 
    EmptyToFilledFullTransition(Color,Color),
    /// Each segment will be colored based on it's normalized position. Value is ignored.
    EmptyToFilledStaticTransition(Color, Color),
}
impl Default for ColorFill {
    fn default() -> Self {
        ColorFill::EmptyOrFilled(
            Color::WHITE,

            Color::BLUE,
        )
    }
}


impl UiProgressBar {
    pub fn new(value: i32, max: i32) -> Self {
        UiProgressBar {
            max,
            value,
            ..UiProgressBar::default()
        }
    }

    pub fn transition_bar(value: i32, max: i32) -> UiProgressBar {
        UiProgressBar::new(value, max).glyph_fill(
            GlyphFill::EmptyOrFilledWithTransition(" ░▒▓█".to_string())
        )
    }

    pub fn value_normalized(&self) -> f32 {
        self.value as f32 / self.max as f32
    }


    pub fn set_value_normalized(&mut self, value: f32) {
        self.value = (value * self.max as f32).round() as i32
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn set_value(&mut self, value: i32) {
        self.value = i32::min(value, self.max);
    }
    
    pub fn glyph_fill(mut self, glyph_fill: GlyphFill) -> UiProgressBar {
        self.glyph_fill = glyph_fill;
        self
    }

    pub fn color_fill(mut self, color_fill: ColorFill) -> UiProgressBar {
        self.color_fill = color_fill;
        self
    }

    pub fn draw(&self, xy: impl GridPoint, size: usize, term: &mut Terminal) {
        let val_normalized = match self.max {
            0 => 0.0,
            _ => self.value_normalized()
        };
        // Bar segment index with fraction representing progress between segments.
        let seg_value_float = match self.glyph_fill {
            GlyphFill::Static(_) => todo!(),
            GlyphFill::EmptyOrFilled(_, _) => (size - 1) as f32 * val_normalized,
            GlyphFill::EmptyOrFilledWithTransition(_) => size as f32 * val_normalized,
        };
        // Index of the value segment
        let seg_value_index = match self.glyph_fill {
            GlyphFill::Static(_) => todo!(),
            GlyphFill::EmptyOrFilled(_, _) => seg_value_float.floor() as usize,
            GlyphFill::EmptyOrFilledWithTransition(_) => (seg_value_float - 1.0).ceil().max(0.0) as usize,
        };

        let empty_glyph = match &self.glyph_fill {
            GlyphFill::Static(glyph) => *glyph,
            GlyphFill::EmptyOrFilled(empty,_) => *empty,
            GlyphFill::EmptyOrFilledWithTransition(string) => {
                string.chars().next().expect("Error parsing progress bar empty glyph from transition string.")
            },
        };

        let filled_glyph = match &self.glyph_fill {
            GlyphFill::Static(glyph) => *glyph,
            GlyphFill::EmptyOrFilled(_, filled) => *filled,
            GlyphFill::EmptyOrFilledWithTransition(string) => {
                string.chars().last().expect("Error parsing progress bar filled glyph from transition string.")
            },
        };

        let value_glyph = match &self.glyph_fill {
            GlyphFill::Static(glyph) => *glyph,
            GlyphFill::EmptyOrFilled(empty, filled) => {
                if self.value == 0 {
                    *empty
                } else {
                    *filled
                }
            },
            GlyphFill::EmptyOrFilledWithTransition(string) => {
                let seg_t = seg_value_float.fract();

                let seg_t = if seg_t == 0.0  {
                    if seg_value_float != 0.0 {
                        1.0
                    } else {
                        seg_t
                    }
                } else {
                    seg_t
                };
                
                //println!("Seg normalized: {}. Seg T {}. Seg_value_index {}", seg_normalized, seg_t, seg_value_index);

                let count = string.chars().count();
                let max_char_index = count - 1;
                let transition_index = (max_char_index as f32 * seg_t).ceil() as usize;
                string.chars().nth(transition_index).unwrap_or_else(||
                panic!("Error parsing value glyph from progress bar glyph string. 
                Couldn't get index {} from {}. Seg_T: {}. Stringlen {}",
                 transition_index, string, seg_t, max_char_index + 1))                
            },
        };
        
        let pos = xy.as_ivec2();

        for i in 0..seg_value_index {
            let pos = pos + IVec2::new(i as i32, 0);
            term.put_char(pos, filled_glyph);
        }

        term.put_char(pos + IVec2::new(seg_value_index as i32, 0), value_glyph);

        for i in seg_value_index + 1..size as usize {
            let pos = pos + IVec2::new(i as i32, 0);
            term.put_char(pos, empty_glyph);
        }
    }
}

impl Default for UiProgressBar {
    fn default() -> Self {
        Self { 
            max: 100, 
            value: 0,
            color_fill: Default::default(),
            glyph_fill: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Terminal;

    use super::*;

    fn print_bar(max: i32, bar: &mut UiProgressBar, term: &mut Terminal) {
        for i in (0..=max).rev() {
            bar.set_value(i);
            bar.draw([0, 0], 10, term);
            println!("{}", term.get_string([0,0], 10));
        }
    }

    #[test]
    fn transition() {
        let mut term = Terminal::with_size([10,1]);
        let glyph_fill = GlyphFill::EmptyOrFilledWithTransition(" ░▒▓█".to_string());
        let max = 30;

        let mut bar = UiProgressBar::new(0, max).glyph_fill(glyph_fill);

        print_bar(max, &mut bar, &mut term);
    }

    #[test]
    fn empty_or_filled() {
        let mut term = Terminal::with_size([10,1]);
        let glyph_fill = GlyphFill::EmptyOrFilled(' ', '█');
        let max = 30;

        let mut bar = UiProgressBar::new(0, max).glyph_fill(glyph_fill);

        print_bar(max, &mut bar, &mut term);
    }
}