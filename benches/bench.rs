#![feature(test)]

use std::borrow::Borrow;

extern crate test;


trait Formatter {
    fn invert(self) -> Formatted;
    fn fg_color(self, color: i32) -> Formatted;
}

trait CharFormatter {
    fn apply(&self, chars: &mut [char]);
}

struct Inverter();

impl CharFormatter for Inverter {
    fn apply(&self, chars: &mut [char]) {
        for c in chars.iter_mut() {
            *c = 'i';
        }
    }
}

struct ColorChange(i32);

impl CharFormatter for ColorChange {
    fn apply(&self, chars: &mut [char]) {
        for c in chars.iter_mut() {
            *c = self.0 as u8 as char;
        }
    }
}

impl Formatter for char {
    fn invert(self) -> Formatted {
        Formatted {
            content: self,
            formatters: vec![Box::new(Inverter())],
        }
    }

    fn fg_color(self, color: i32) -> Formatted {
        Formatted {
            content: self,
            formatters: vec![Box::new(Inverter())],
        }
    }
}

impl From<Formatted> for char {
    fn from(fmt: Formatted) -> Self {
        fmt.content
    }
}

impl From<char> for Formatted {
    fn from(c: char) -> Self {
        Formatted {
            content: c,
            formatters: Vec::new()
        }
    }
}

struct Formatted {
    content: char,
    formatters: Vec<Box<dyn CharFormatter>>,
}

struct Container {
    chars: Vec<char>,
}

impl Formatter for Formatted {
    fn invert(mut self) -> Formatted {
        self.formatters.push(Box::new(Inverter()));
        self
    }

    fn fg_color(mut self, color: i32) -> Formatted {
        self.formatters.push(Box::new(ColorChange(color)));
        self
    }
}

impl Container {
    pub fn new(len: usize) -> Self {
        Self {
            chars: vec![' '; len],
        }
    }
    pub fn raw(&mut self, i: usize, input: char) -> char {
        for j in 0..10 {
            let input = (input as u8).wrapping_add(self.chars[i] as u8).wrapping_add(j);
            self.chars[i] = input as char;
        }
        self.chars[i]
    }

    pub fn formatted(&mut self, i: usize, formatted: impl Into<Formatted>) -> char {
        let formatted = formatted.into();
        let input = formatted.content;
        for j in 0..10 {
            let input = (input as u8).wrapping_add(self.chars[i] as u8).wrapping_add(j);
            self.chars[i] = input as char;
        }
        for formatter in &formatted.formatters {
            formatter.apply(&mut self.chars[i..i+1]);
        }
        self.chars[i]
    }
}



#[cfg(test)]
mod tests {
    use crate::*;
    use test::Bencher;


    #[bench]
    fn raw(b: &mut Bencher) {
        let mut c = Container::new(15);

        b.iter(|| { c.raw(0, 'a') })
    }

    #[bench]
    fn formatted(b: &mut Bencher) {
        let mut c = Container::new(15);

        b.iter(|| { c.formatted(0, 'a') })
    }
}