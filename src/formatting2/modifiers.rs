use std::{iter::{once, Once}, str::Chars};

use arrayvec::ArrayVec;
use bevy::prelude::*;

use crate::Pivot;

pub enum ModifyTileCommand {
    SetFgColor(Color),
    SetBgColor(Color),
    SetPivot(Pivot),
}

pub struct TerminalContent<T> {
    content: T,
    tile_modifiers: ArrayVec<ModifyTileCommand,3>,
    mesh_modifiers: Vec<MeshModifier>,
}

impl<T> TerminalContent<T> {
    fn with_modifier(content: T, modifier: ModifyTileCommand) -> TerminalContent<T> {
        let mut tile_modifiers = ArrayVec::new();
        tile_modifiers.push(modifier);
        TerminalContent {
            content,
            tile_modifiers,
            mesh_modifiers: Vec::new(),
        }
    }

    fn with_mesh_modifier(content: T, modifier: MeshModifier) -> TerminalContent<T> {
        TerminalContent {
            content,
            tile_modifiers: ArrayVec::new(),
            mesh_modifiers: vec![modifier],
        }
    }
}



pub trait Mod<T: IntoIterator<Item=char>> {
    fn into_iter(self) -> T;
    fn fg_color(self, color: Color) -> TerminalContent<T>;
    fn bg_color(self, color: Color) -> TerminalContent<T>;
    fn set_pivot(self, pivot: Pivot) -> TerminalContent<T>;
    fn jumble(self) -> TerminalContent<T>;
    fn flip_horizontal(self) -> TerminalContent<T>;
}

macro_rules! impl_tile_modifier {
    ($fn_name:ident, $iter_type:ty, $command:ident, $arg:ty) => {
        fn $fn_name(self, arg: $arg) -> TerminalContent<$iter_type> {
            TerminalContent::with_modifier(self.into_iter(), ModifyTileCommand::$command(arg))
        }
    }
}

macro_rules! impl_mesh_modifier {
    ($func_name:ident, $iter_type:ty, $mod_type:ident, $modifier:ident $(, $arg_name:ident:$arg_type:ty)? ) => {
        fn $func_name(self $(, $arg_name:ident:$arg_type:ty)? ) -> TerminalContent<$iter_type> {
            let modifier = MeshModifier::$modifier(Box::new($mod_type( $($arg_name)? )));
            TerminalContent::with_mesh_modifier(self.into_iter(), modifier)
        }
    }
}

macro_rules! impl_modifiers {
    ($iter_type:ty, $mod_type:ty) => {
        impl_tile_modifier!(fg_color, $iter_type, SetFgColor, Color);
        impl_tile_modifier!(bg_color, $iter_type, SetBgColor, Color);
        impl_tile_modifier!(set_pivot, $iter_type, SetPivot, Pivot);
        impl_mesh_modifier!(jumble, $iter_type, Jumble, VertexModifier);
        impl_mesh_modifier!(flip_horizontal, $iter_type, FlipHorizontal, UvModifier);
    }
}


impl Mod<Once<char>> for char {
    fn into_iter(self) -> Once<char> { once(self) }
    impl_modifiers!(Once<char>, char);
}

impl<'a> Mod<Chars<'a>> for &'a str {
    fn into_iter(self) -> Chars<'a> { self.chars() }
    impl_modifiers!(Chars<'a>, &'a str);
}

macro_rules! impl_tile_modifier_for_content {
    ($fn_name:ident, $iter_type:ty, $command:ident, $arg:ty) => {
        fn $fn_name(mut self, arg: $arg) -> TerminalContent<$iter_type> {
            self.tile_modifiers.push(ModifyTileCommand::$command(arg));
            self
        }
    }
}

macro_rules! impl_mesh_modifier_for_content {
    ($func_name:ident, $iter_type:ty, $mod_type:ident, $modifier:ident $(, $arg_name:ident:$arg_type:ty)? ) => {
        fn $func_name(mut self $(, $arg_name:ident:$arg_type:ty)? ) -> TerminalContent<$iter_type> {
            let modifier = MeshModifier::$modifier(Box::new($mod_type( $($arg_name)? )));
            self.mesh_modifiers.push(modifier);
            self
        }
    }
}

impl<T: IntoIterator<Item=char>> Mod<T> for TerminalContent<T> {
    fn into_iter(self) -> T {
        self.content
    }

    impl_tile_modifier_for_content!(fg_color, T, SetFgColor, Color);
    impl_tile_modifier_for_content!(bg_color, T, SetBgColor, Color);
    impl_tile_modifier_for_content!(set_pivot, T, SetPivot, Pivot);

    impl_mesh_modifier_for_content!(jumble, T, Jumble, VertexModifier);
    impl_mesh_modifier_for_content!(flip_horizontal, T, FlipHorizontal, UvModifier);
}

impl From<char> for TerminalContent<Once<char>> {
    fn from(ch: char) -> Self {
        TerminalContent {
            content: once(ch),
            tile_modifiers: ArrayVec::new(),
            mesh_modifiers: Vec::new(),
        }
    }
}



impl<'a> From<&'a str> for TerminalContent<Chars<'a>> {
    fn from(string: &'a str) -> Self {
        TerminalContent {
            content: string.chars(),
            tile_modifiers: ArrayVec::new(),
            mesh_modifiers: Vec::new(),
        }
    }
}

pub trait VertexModifier: Send + Sync {
    fn modify_verts(&self, verts: &mut [[f32;3]]);
}

pub struct Jumble();

impl VertexModifier for Jumble {
    fn modify_verts(&self, verts: &mut [[f32;3]]) {
    }
}



#[derive(Default)]
pub struct Term {
    chars: Vec<char>,
}

impl Term {
    pub fn write<T>(&mut self, content: impl Into<TerminalContent<T>>)
        where T: IntoIterator<Item=char>
    {
        let content: TerminalContent<T> = content.into();
        for c in content.content {
            
        }
    }
}
#[test]
fn test_writer() {
    let mut term = Term::default();
    term.write('a'.fg_color(Color::BLUE));
    term.write("hello");
}

pub enum MeshModifier {
    VertexModifier(Box<dyn VertexModifier>),
    UvModifier(Box<dyn UvModifier>),
}

#[derive(Component)]
pub struct VertexModifiers {
    modifiers: Vec<Box<dyn VertexModifier>>,
}

pub trait UvModifier: Send + Sync {
    fn modify(&self, uvs: &mut [[f32;2]]);
}

pub struct FlipHorizontal();
impl UvModifier for FlipHorizontal {
    fn modify(&self, uvs: &mut [[f32;2]]) {
        todo!()
    }
}

#[derive(Component)]
pub struct UVModifiers {
    modifiers: Vec<Box<dyn UvModifier>>,
}


#[test]
fn test() {
    //'a'.jumble().jumble();
}

pub struct Writer();

impl Writer {
    pub fn write<T: Iterator<Item=char>, C: Into<TerminalContent<T>>>(&self, content: C) {
        let content: TerminalContent<T> = content.into();
        let iter = content.content;

        for ch in iter {

        }
    }
}

#[test]
fn writer_test() {
    let writer = Writer();

    writer.write("hi".fg_color(Color::BLUE).flip_horizontal());
    writer.write('a'.jumble());
}