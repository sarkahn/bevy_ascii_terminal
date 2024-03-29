use bevy::{ecs::{component::Component, query::Changed, system::Query}, math::{bounding::Aabb2d, IVec2, Vec2}, transform::components::Transform};

use crate::renderer::TerminalMeshRenderer;


#[derive(Component)]
pub struct TerminalTransform {
    grid_pos: IVec2,
    world_tile_size: Vec2,
    term_size: IVec2,
    bounds: Aabb2d,
}

impl TerminalTransform {
    pub fn world_to_tile(&self, world_pos: Vec2) -> Option<IVec2> {
        let pos = self.grid_pos.as_vec2() * self.world_tile_size + self.bounds.min;
        let pos = ((world_pos - pos) / self.world_tile_size)
            .floor()
            .as_ivec2();
        if pos.cmplt(IVec2::ZERO).any() || pos.cmpge(self.term_size).any() {
            return None;
        }
        Some(pos)
    }
}

fn update_transform(
    mut q_term: Query<(&mut Transform, &TerminalMeshRenderer, &TerminalTransform), Changed<TerminalTransform>>
) {
    for (mut transform, renderer, term_transform) in &mut q_term {
        let xy = term_transform.grid_pos.as_vec2() * renderer.tile_size_world();
        let z = transform.translation.z;
        transform.translation = xy.extend(z);
    }
}