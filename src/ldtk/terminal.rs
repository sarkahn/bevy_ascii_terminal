use bevy::prelude::*;

use crate::Terminal;

use super::LdtkAsset;

pub struct LdtkTerminalPlugin;

impl Plugin for LdtkTerminalPlugin {
    fn build(&self, app: &mut App) {
    }
}

pub struct BuildFromLdtk();

fn build_from_ldtk(
    q_term: Query<(&mut Terminal, &Handle<LdtkAsset>)>,
) {
    
}

