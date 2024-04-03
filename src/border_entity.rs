use std::collections::BTreeMap;

use bevy::ecs::component::Component;

use crate::Tile;

#[derive(Component)]
pub struct TerminalBorder {
    tiles: BTreeMap<(i32, i32), Tile>,
}
