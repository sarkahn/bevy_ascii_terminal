use bevy::prelude::{UVec2, Vec2};

pub fn terminal_mesh_origin(
    term_size: UVec2,
    term_pivot: Vec2,
    tile_size: Vec2,
    tile_pivot: Vec2,
) -> Vec2 {
    let term_size = term_size.as_vec2();
    let term_offset = -(term_size * tile_size * term_pivot);
    let tile_offset = -(tile_size * tile_pivot);
    term_offset + tile_offset
}
