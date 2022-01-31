use std::ops::Range;

use bevy::{prelude::*, core::Timer};

use crate::Terminal;


pub trait TerminalEffect {
    fn timer(&self) -> Timer;
}

// pub struct EffectTimer {

// }
// pub struct BlinkingEffect(Range<usize>);

// #[derive(Component)]
// pub struct BlinkingEffects(Vec<BlinkingEffect>);

// impl BlinkingEffect {
//     pub fn on_update(dt: f32, fg_color: &[Color], bg_color: &[Color]) {
        
//     }
// }

// fn blinking_effect(
//     q_effect: Query<(&Terminal, &BlinkingEffects)>
// ) {
//     for (term,effects) in q_effect.iter() {
//         for effect in &effects.0 {
//             let tiles = &term.tiles[effect.0];
//         }
//     }
// }