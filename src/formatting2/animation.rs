use std::{marker::PhantomData, iter::Once};

use bevy::prelude::*;


// All terminal changes driven by TerminalOperation - a single object defining changes to the terminal.
// Changable properties are: Glyph, FG Color, BG Color, Vert Color, Vertex, Uvs
// Enum? Trait?
// ANimations work by generating operations before other inputs. Then other inputs generate their own operations.
// Then terminal can decide if it needs to render (update mesh) based on actual changes.

// IE: Animation wait(5).change_glyph([0,0], 5). For the first five seconds no terminal updates should occur?
//                                               This has other implications - does animation state get discard during wait?
//                                               For glyph change Iwould expect not, for colors I would expect so?


pub trait AnimationDriver<'a, T: IntoIterator<Item=char>> {
    fn animation(self) -> Animator<'a, T>;
}

pub struct Animator<'a, T: IntoIterator<Item=char>> {
    content: T,
    animation: Vec<AnimationCommand<'a>>,
}


pub enum AnimationCommand<'a> {
    Wait(f32),
    ModifyFgColor(&'a mut Color),
}

pub struct Animation<'a> {
    commands: Vec<AnimationCommand<'a>>,
}

impl<'a> Animation<'a> {
    pub fn from_command(command: AnimationCommand<'a>) -> Self {
        Self {
            commands: vec![command],
        }
    }
}

impl<'a> AnimationDriver<'a, Once<char>> for char {
    fn animation(self) -> Animator<'a, Once<char>> {
        Animator {
            content: core::iter::once(self),
            animation: Vec::new(),
        }
    }
}

// impl<'a> Animator<'a> for char {
//     fn wait(self, wait_seconds: f32) -> Animation<'a> {
//         Animation::from_command(AnimationCommand::Wait(wait_seconds))
//     }

//     fn modify_fg_color(self, color: &'a mut Color) -> AnimationDriver<'a> {
//         todo!()
//     }
// }

#[test]
fn anim_test() {
    'a'.animation().wait(0.5);
    //'a'.wait(0.5).wait(0.3);
}