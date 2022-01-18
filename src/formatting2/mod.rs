use bevy::{prelude::Color, math::UVec2};

mod formatters;

use formatters::*;

pub struct FormattedContent<T> {
    pub content: T,
    pub formatters: Vec<Box<dyn TileFormatter>>, 
}

impl<T> From<T> for FormattedContent<T> {
    fn from(t: T) -> Self {
        FormattedContent {
            content: t,
            formatters: Vec::new(),
        }
    }
}

pub trait Formatter<T>: Sized {
    fn fg_color(self, color: Color) -> FormattedContent<T>;
    fn bg_color(self, color: Color) -> FormattedContent<T>;
    fn flip_horizontal(self) -> FormattedContent<T>;
    fn jumble(self) -> FormattedContent<T>;
    fn invert(self) -> FormattedContent<T>;
}


macro_rules! impl_t_func {
    ($func_name:ident, $fmt_type:ident $(, $arg_name:ident:$arg_type:ty)? ) => {
        fn $func_name(self, $($arg_name:$arg_type)?) -> FormattedContent<Self> {
            FormattedContent {
                content: self,
                formatters: vec![
                    Box::new($fmt_type($($arg_name)?)),
                ],
            } 
        }
    }
}

macro_rules! impl_formatter_func {
    ($func_name:ident, $fmt_type:ident, $t:ty $(, $arg_name:ident:$arg_type:ty)? ) => {
        fn $func_name(mut self, $($arg_name:$arg_type)?) -> FormattedContent<$t> {
            self.formatters.push(Box::new($fmt_type($($arg_name)?)));
            self
        }
    }
}

macro_rules! impl_formatter_for_t {
    ($t:ty) => {
        impl Formatter<$t> for $t {
            impl_t_func!(fg_color, FGColorFormatter, color: Color );
            impl_t_func!(bg_color, BGColorFormatter, color: Color );
            impl_t_func!(jumble, Jumble);
            impl_t_func!(invert, Invert);
            impl_t_func!(flip_horizontal, FlipHorizontal);
        }
    }
}

impl<T> Formatter<T> for FormattedContent<T> {
    impl_formatter_func!(fg_color, FGColorFormatter, T, color: Color);
    impl_formatter_func!(bg_color, BGColorFormatter, T, color: Color);
    impl_formatter_func!(jumble, Jumble, T);
    impl_formatter_func!(invert, Invert, T);
    impl_formatter_func!(flip_horizontal, FlipHorizontal, T);
}

impl_formatter_for_t!(char);
impl_formatter_for_t!(&'static str);
impl_formatter_for_t!(String);

#[derive(Default)]
pub struct Writer {
    size: UVec2,
    fg: Vec<Color>,
    bg: Vec<Color>,
    verts: Vec<[f32;3]>,
    uvs: Vec<[f32;2]>,
}
impl Writer {
    pub fn new(len: usize) -> Self {
        Self {
            size: UVec2::ONE,
            fg: vec![Color::GREEN;len],
            bg: vec![Color::BLUE;len],
            verts: vec![[0.0,0.0,0.0];len * 4],
            uvs: vec![[0.0,0.0];len * 4],
        }
    }

    pub fn print_colors(&self) {
        println!("FG {:?}, BG {:?}", self.fg[0], self.bg[0]);
    }
    
    pub fn write_formatted<'a>(&mut self, xy: [i32;2], 
        content: FormattedContent<impl Into<&'a str>>,  
        //content: FormattedContent<&'static str>
    ) {
        let i = (xy[1] * self.size.x as i32 + xy[0]) as usize;
        let str = content.content.into();
        let len = str.len();
        
        for formatter in &content.formatters {
            formatter.apply(
                &mut self.fg[i..i+len],
                &mut self.bg[i..i+len],
                &mut self.verts[i * 4..i+len * 4],
                &mut self.uvs[i * 4..i+len * 4],
            );
        }
    }

    pub fn write_char(&mut self, xy: [i32;2], content:FormattedContent<impl Into<char>>) {
        //let c = content.content.into();
        let i = (xy[1] * self.size.x as i32 + xy[0]) as usize;
        for formatter in &content.formatters {
            formatter.apply(
                &mut self.fg[i..i+1],
                &mut self.bg[i..i+1],
                &mut self.verts[i * 4..i * 4 + 4],
                &mut self.uvs[i * 4..i * 4 + 4],
            );
        }
    }
}


#[test]
fn test() {
    let mut writer = Writer::new(5);

    writer.write_formatted([0,0], "hello".invert().jumble());
    writer.write_char([0,0], 'a'.flip_horizontal().jumble());
    assert_eq!(Color::BLUE, writer.fg[0]);
    assert_eq!(Color::GREEN, writer.bg[0]);
    assert_eq!(0.5, writer.verts[0][0]);
    assert_eq!(0.5, writer.verts[3][0]);
    assert_eq!(-0.5, writer.verts[4][0]);
}
