use bevy::{
    app::{Plugin, PostUpdate},
    ecs::{
        component::Component,
        query::Changed,
        schedule::{IntoSystemConfigs, SystemSet},
        system::Query,
    },
    math::{IVec2, Rect, Vec2},
    transform::{components::Transform, TransformSystem},
};

use crate::{renderer::{TerminalRenderSystems, TerminalRenderer}, GridPoint};

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemSet)]
pub struct TerminalTransformSystems;
pub struct TerminalTransformPlugin;

impl Plugin for TerminalTransformPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (on_renderer_change, update_transform)
                .in_set(TerminalTransformSystems)
                .before(TransformSystem::TransformPropagate)
                .after(TerminalRenderSystems),
        );
    }
}

#[derive(Default, Component)]
pub struct TerminalTransform {
    /// The grid position for the terminal. Setting this value will override the
    /// entity [Transform] during the next [PostUpdate]
    pub grid_pos: IVec2,
    world_tile_size: Vec2,
    term_size: IVec2,
    mesh_bl: Vec2,
}

impl TerminalTransform {
    pub fn new(size: impl GridPoint) -> Self {
        Self {
            term_size: size.as_ivec2(),
            ..Default::default()
        }
    }

    pub fn world_to_tile(&self, world_pos: Vec2) -> Option<IVec2> {
        let pos = self.grid_pos.as_vec2() * self.world_tile_size + self.mesh_bl;
        let pos = ((world_pos - pos) / self.world_tile_size)
            .floor()
            .as_ivec2();
        if pos.cmplt(IVec2::ZERO).any() || pos.cmpge(self.term_size).any() {
            return None;
        }
        Some(pos)
    }

    pub fn world_bounds(&self) -> Rect {
        let min = self.world_pos() + self.mesh_bl;
        let max = min + self.term_size.as_vec2() * self.world_tile_size;
        Rect::from_corners(min, max)
    }

    pub fn world_pos(&self) -> Vec2 {
        self.grid_pos.as_vec2() * self.world_tile_size
    }
}

fn on_renderer_change(mut q_term: Query<(&TerminalRenderer, &mut TerminalTransform)>) {
    for (renderer, mut transform) in &mut q_term {
        transform.mesh_bl = renderer.mesh_bounds().min;
        transform.world_tile_size = renderer.tile_size_world();
    }
}

fn update_transform(
    mut q_term: Query<
        (&mut Transform, &TerminalTransform),
        Changed<TerminalTransform>,
    >,
) {
    for (mut transform, term_transform) in &mut q_term {
        let xy = term_transform.grid_pos.as_vec2() * term_transform.world_tile_size;
        let z = transform.translation.z;
        transform.translation = xy.extend(z);
    }
}
