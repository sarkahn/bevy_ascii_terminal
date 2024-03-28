use bevy::{app::Plugin, ecs::system::Resource};

mod material;
mod mesh;
mod uv_mapping;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TileScaling {
    Pixels,
    World,
}

#[derive(Resource)]
pub struct TerminalRenderSettings {
    pub tile_scaling: TileScaling,
}

pub struct TerminalRendererPlugin;

impl Plugin for TerminalRendererPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TerminalRenderSettings {
            tile_scaling: TileScaling::World,
        });
    }
}