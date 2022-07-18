//! An optional component for converting positions between "terminal space"
//! and world space.

use bevy::{prelude::*, render::camera::RenderTarget};
use sark_grids::GridPoint;

use crate::{
    renderer::{PixelsPerTile, TerminalPivot, TilePivot, TileScaling},
    Terminal,
};

pub struct ToWorldPlugin;

impl Plugin for ToWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_from_terminal)
            .add_system(update_from_camera);
    }
}

/// A component for converting positions between World Space and
/// "Terminal Space".
///
/// When you add this to a terminal it will track the various properties of the
/// terminal and camera, provide functions for converting positions.
#[derive(Default, Component)]
pub struct ToWorld {
    term_size: UVec2,
    term_pos: Vec3,
    term_pivot: Vec2,
    tile_pivot: Vec2,
    tile_scaling: TileScaling,
    pixels_per_unit: UVec2,
    camera_entity: Option<Entity>,
    ndc_to_world: Mat4,
    camera_pos: Vec3,
    viewport_pos: Vec2,
    viewport_size: Option<Vec2>,
}

impl ToWorld {
    /// Convert a tile position (bottom left corner) to it's corresponding
    /// world position.
    pub fn tile_to_world(&self, tile: impl GridPoint) -> Vec3 {
        let term_pos = self.term_pos.truncate();
        let term_offset = self.term_size.as_vec2() * self.term_pivot;
        let tile_offset = self.world_unit() * self.tile_pivot;
        (tile.as_vec2() + term_pos - term_offset - tile_offset).extend(self.term_pos.z)
    }

    /// Convert a tile center to it's corresponding world position.
    pub fn tile_center_to_world(&self, tile: impl GridPoint) -> Vec3 {
        let center_offset = (self.world_unit() / 2.0).extend(0.0);
        self.tile_to_world(tile) + center_offset
    }

    pub fn world_to_tile(&self, world: Vec2) -> IVec2 {
        let term_pos = self.term_pos.truncate();
        let term_offset = self.term_size.as_vec2() * self.term_pivot;
        let tile_offset = self.world_unit() * self.tile_pivot;
        let xy = world - term_pos + term_offset + tile_offset;
        xy.floor().as_ivec2()
    }

    /// The size of a single world unit, accounting for `TileScaling`.
    pub fn world_unit(&self) -> Vec2 {
        match self.tile_scaling {
            TileScaling::World => Vec2::ONE,
            TileScaling::Pixels => self.pixels_per_unit.as_vec2(),
        }
    }

    /// Convert a position from screen space (ie: Cursor position) to world space.
    pub fn screen_to_world(&self, screen_pos: Vec2) -> Option<Vec2> {
        if let Some(viewport_size) = self.viewport_size {
            let screen_pos = screen_pos - self.viewport_pos;
            // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
            let ndc = (screen_pos / viewport_size) * 2.0 - Vec2::ONE;

            // use it to convert ndc to world-space coordinates
            let world_pos = self.ndc_to_world.project_point3(ndc.extend(-1.0));

            // reduce it to a 2D value
            Some(world_pos.truncate())
        } else {
            None
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_from_terminal(
    mut q_term: Query<
        (
            &mut ToWorld,
            &Terminal,
            &GlobalTransform,
            &TerminalPivot,
            &TilePivot,
            &TileScaling,
            &PixelsPerTile,
        ),
        Or<(
            Changed<Terminal>,
            Changed<TerminalPivot>,
            Changed<TilePivot>,
            Changed<TileScaling>,
        )>,
    >,
) {
    for (mut to_world, term, transform, term_pivot, tile_pivot, tile_scaling, ppu) in
        q_term.iter_mut()
    {
        to_world.term_size = term.size();
        to_world.term_pivot = term_pivot.0;
        to_world.tile_pivot = tile_pivot.0;
        to_world.tile_scaling = *tile_scaling;
        to_world.term_pos = transform.translation();
        to_world.pixels_per_unit = ppu.0;
    }
}

#[allow(clippy::type_complexity)]
fn update_from_camera(
    q_cam: Query<
        (Entity, &Camera, &GlobalTransform),
        Or<(Changed<Camera>, Changed<GlobalTransform>)>,
    >,
    mut q_to_world: Query<&mut ToWorld>,
    windows: Res<Windows>,
    images: Res<Assets<Image>>,
) {
    if q_cam.is_empty() {
        return;
    }

    for mut tw in q_to_world.iter_mut() {
        // If no camera is explicitly set, choose the first camera we can find
        if tw.camera_entity.is_none() {
            tw.camera_entity = Some(q_cam.iter().next().unwrap().0);
        }

        for (cam_entity, cam, t) in q_cam.iter() {
            if cam_entity != tw.camera_entity.unwrap() {
                continue;
            }

            tw.camera_pos = t.translation();
            tw.ndc_to_world = t.compute_matrix() * cam.projection_matrix().inverse();

            if let Some(vp) = &cam.viewport {
                tw.viewport_pos = vp.physical_position.as_vec2();
                tw.viewport_size = Some(vp.physical_size.as_vec2());
            } else {
                tw.viewport_pos = Vec2::ZERO;
                let res = match &cam.target {
                    RenderTarget::Window(win_id) => {
                        windows
                            .get(*win_id)
                            .map(|window| Vec2::new(window.width(), window.height()))
                        // if let Some(window) = windows.get(*win_id) {
                        //     Some(Vec2::new(window.width(), window.height()))
                        // } else {
                        //     None
                        // }
                    }
                    RenderTarget::Image(image) => {
                        images.get(image).map(|image| image.size())
                        // if let Some(image) = images.get(image) {
                        //     Some(image.size())
                        // } else {
                        //     None
                        // }
                    }
                };
                tw.viewport_size = res;
            }
        }
    }
}
