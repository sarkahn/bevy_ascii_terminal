use bevy::{sprite::Sprite, utils::HashMap};
use vec_option::VecOption;

#[derive(Default)]
struct TerminalSpriteData {
    fg_sprites: VecOption<Sprite>,
    bg_sprites: Vec<Sprite>,
}

impl From<TerminalSpriteData> for VecOption<Sprite> {
    fn from(fsd: TerminalSpriteData) -> Self {
        fsd.fg_sprites
    }
}

impl TerminalSpriteData {
    fn new(&self, width: usize, height: usize) -> Self {
        let len = width * height;

        let mut fg = VecOption::with_capacity(len);
        fg.extend_none(len);

        Self {
            fg_sprites: fg,
            bg_sprites:  vec![Sprite::default();len],
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::SpriteSheetBundle;

    use super::*;

    #[test]
    fn blah() {
        let sprites = SpriteSheetBundle {
            ..Default::default()
        };
        let mut v = VecOption::new();
        v.push(10);
    }
}