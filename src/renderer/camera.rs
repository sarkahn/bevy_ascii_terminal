use crate::{transform::TerminalTransform, GridRect, TerminalGrid};
use bevy::{
    app::PostStartup,
    ecs::{
        bundle::Bundle,
        query::Changed,
        schedule::{IntoSystemConfigs, SystemSet},
    },
    math::{bounding::BoundingVolume, IVec2, Mat4},
    prelude::{
        AssetEvent, Assets, Camera, Camera2dBundle, Commands, Component, Entity, Event,
        EventReader, EventWriter, First, Handle, Image, Last, OrthographicProjection, Plugin,
        PostUpdate, PreUpdate, Query, Res, UVec2, Vec2, With, Without,
    },
    render::camera::{CameraUpdateSystem, ScalingMode, Viewport},
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window, WindowResized},
};

use crate::Terminal;

use super::{material::TerminalMaterial, mesh::TerminalRenderer};

pub(crate) struct TerminalCameraPlugin;

impl Plugin for TerminalCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UpdateViewportEvent>()
            .add_systems(
                PostStartup,
                cache_camera_data
                    .in_set(TerminalCameraSystems)
                    .after(CameraUpdateSystem),
            )
            .add_systems(
                PostUpdate,
                (cache_camera_data, on_renderer_change, on_window_resized)
                    .chain()
                    .in_set(TerminalCameraSystems)
                    .after(CameraUpdateSystem),
            )
            .add_systems(Last, update_viewport);
    }
}

/// System for tracking camera and cursor data.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalCameraSystems;

#[derive(Bundle)]
pub struct TerminalCameraBundle {
    term_cam: TerminalCamera,
    cam_2d: Camera2dBundle,
}

impl Default for TerminalCameraBundle {
    fn default() -> Self {
        Self::auto()
    }
}

impl TerminalCameraBundle {
    /// Set up an auto terminal camera. The viewport will be automatically
    /// adjusted any time the window changes to try and render all existing
    /// terminals without any artifacts.
    pub fn auto() -> Self {
        Self {
            term_cam: TerminalCamera {
                auto_resize_viewport: true,
                ..Default::default()
            },
            cam_2d: Default::default(),
        }
    }

    /// Enable cursor tracking for the [TerminalCamera].
    ///
    /// If cursor tracking is enabled the Terminal camera will cache viewport
    /// and cursor data every frame so cursor positions can easily be
    /// transformed to World/Terminal space.
    pub fn with_cursor_tracking(mut self) -> Self {
        self.term_cam.track_cursor = true;
        self
    }
}

#[derive(Default, Debug, Clone)]
struct CachedCameraData {
    cam_transform: GlobalTransform,
    proj_matrix: Mat4,
    target_size: Option<Vec2>,
    cursor: Option<Vec2>,
    vp_offset: Option<Vec2>,
}

/// A component for easy terminal rendering.
#[derive(Default, Debug, Clone, Component)]
pub struct TerminalCamera {
    /// If set to true, the Terminal camera will cache viewport and cursor
    /// data every frame so the cursor position can easily be transformed to
    /// Terminal tile coordinates via the camera's `cursor_to_tile` function.
    pub track_cursor: bool,
    /// If set to true the camera viewport will be adjusted automatically
    /// any time the window or a terminal is resized to try and render
    /// all terminals correctly with no artifacts.
    pub auto_resize_viewport: bool,
    cam_data: Option<CachedCameraData>,
}

impl TerminalCamera {
    /// Returns the tile position of the main window cursor using the last
    /// cached camera data.
    ///
    /// Will return [None] if the camera data has not been initialized or if the
    /// cursor is out of the bounds of the given terminal.
    ///
    /// For accurate results this should be called in [PostUpdate] after
    /// [TerminalCameraSystem].
    pub fn cursor_to_tile(&self, transform: &TerminalTransform) -> Option<IVec2> {
        self.cam_data
            .as_ref()
            .and_then(|d| d.cursor)
            .and_then(|p| self.viewport_to_tile(p, transform))
    }

    /// Converts a window viewport position (IE: the cursor) to it's
    /// corresponding tile position in the given terminal.
    ///
    /// Will return [None] if the position is outside the bounds of the
    /// terminal.
    pub fn viewport_to_tile(
        &self,
        viewport_position: Vec2,
        transform: &TerminalTransform,
    ) -> Option<IVec2> {
        self.viewport_to_world(viewport_position)
            .and_then(|p| transform.world_to_tile(p))
    }

    /// Transform a viewport position to it's corresponding world position using
    /// the last cached camera data.
    ///
    // Note this is more or less a copy of the existing bevy viewport transform
    // function, but adjusted to account for a resized viewport.
    pub fn viewport_to_world(&self, mut viewport_position: Vec2) -> Option<Vec2> {
        let data = self.cam_data.as_ref()?;
        let target_size = data.target_size?;
        if let Some(vp_offset) = data.vp_offset {
            viewport_position -= vp_offset;
        };
        // Flip the Y co-ordinate origin from the top to the bottom.
        viewport_position.y = target_size.y - viewport_position.y;
        let ndc = viewport_position * 2. / target_size - Vec2::ONE;
        let ndc_to_world = data.cam_transform.compute_matrix() * data.proj_matrix.inverse();
        let world_space_coords = ndc_to_world.project_point3(ndc.extend(1.));
        (!world_space_coords.is_nan()).then_some(world_space_coords.truncate())
    }
}

#[derive(Event)]
pub struct UpdateViewportEvent;

#[allow(clippy::type_complexity)]
fn cache_camera_data(
    mut q_cam: Query<(&mut TerminalCamera, &GlobalTransform, &Camera)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let cursor = window.get_single().ok().and_then(|w| w.cursor_position());

    for (mut terminal_cam, transform, cam) in &mut q_cam {
        if !terminal_cam.track_cursor {
            if terminal_cam.cam_data.is_some() {
                terminal_cam.cam_data = None;
            }
            continue;
        }
        terminal_cam.cam_data = Some(CachedCameraData {
            cam_transform: *transform,
            proj_matrix: cam.projection_matrix(),
            target_size: cam.logical_viewport_size(),
            cursor,
            vp_offset: cam.logical_viewport_rect().map(|vp| vp.min),
        });
    }
}

fn on_window_resized(
    q_win: Query<Entity, With<PrimaryWindow>>,
    mut resize_events: EventReader<WindowResized>,
    mut vp_evt: EventWriter<UpdateViewportEvent>,
) {
    if q_win.is_empty() || resize_events.is_empty() {
        return;
    }
    let window_entity = q_win.single();
    for evt in resize_events.read() {
        if evt.window != window_entity {
            continue;
        }
        vp_evt.send(UpdateViewportEvent);
        return;
    }
}

fn on_renderer_change(
    q_term: Query<&TerminalRenderer, Changed<TerminalRenderer>>,
    mut vp_evt: EventWriter<UpdateViewportEvent>,
) {
    if !q_term.is_empty() {
        vp_evt.send(UpdateViewportEvent);
    }
}

fn update_viewport(
    mut evt_vp_update: EventReader<UpdateViewportEvent>,
    q_term: Query<(
        &GlobalTransform,
        &Terminal,
        &Handle<TerminalMaterial>,
        &TerminalRenderer,
    )>,
    mut q_cam: Query<(
        &mut Camera,
        &mut Transform,
        &mut OrthographicProjection,
        &mut TerminalCamera,
    )>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    images: Res<Assets<Image>>,
    materials: Res<Assets<TerminalMaterial>>,
    render_settings: Res<TerminalGrid>,
) {
    if evt_vp_update.is_empty() || q_term.is_empty() || q_cam.is_empty() || q_window.is_empty() {
        return;
    }
    evt_vp_update.read();

    let (mut cam, mut cam_transform, mut proj, term_cam) = q_cam.single_mut();
    if !term_cam.auto_resize_viewport {
        return;
    }

    let ppu = q_term
        .iter()
        .map(|(_, _, _, renderer)| renderer.pixels_per_tile())
        .reduce(IVec2::max)
        .unwrap();

    // For a tilescaling::world camera, every tile is 1 world
    // unit vertically.

    // let grid_rect = q_term
    //     .iter()
    //     .map(|(t, term, _, renderer)| {
    //         let p = t.translation().truncate();
    //         let size = term.size().as_vec2();
    //         let bl = p - (size * renderer.pivot.normalized()).floor();
    //         GridRect::new(bl.as_ivec2(), size.as_ivec2())
    //     })
    //     .reduce(|a, b| a.merged(b))
    //     .unwrap();

    // // We'll use the largest PPU from all terminals as our baseline.
    // //
    // // TODO: Handle different font sizes with multiple terminals.
    // let ppu = q_term
    //     .iter()
    //     .filter_map(|(_, _, handle, _)| {
    //         materials
    //             .get(handle)
    //             .and_then(|m| m.texture.clone())
    //             .and_then(|i| images.get(i))
    //             .map(|i| i.size().as_ivec2() / 16)
    //     })
    //     .reduce(IVec2::max);

    // // Check if no terminals have an image.
    // let Some(ppu) = ppu else {
    //     return;
    // };

    // // Bounds encompassing all terminal meshes in world space.
    // let mesh_bounds = q_term
    //     .iter()
    //     .map(|(_, _, _, renderer)| renderer.bounds())
    //     .reduce(|a, b| a.merge(&b));
    // let Some(mesh_bounds) = mesh_bounds else {
    //     return;
    // };

    // evt_vp_update.clear();

    // let window = q_window.single();

    // *cam_transform =
    //     Transform::from_translation(mesh_bounds.center().extend(cam_transform.translation.z));

    // let target_res = (grid_rect.size * ppu).as_vec2();
    // let window_res = UVec2::new(window.physical_width(), window.physical_height()).as_vec2();

    // let zoom = (window_res / target_res).floor().min_element().max(1.0);

    // let ortho_size = match render_settings.scaling {
    //     TileScaling::WorldUnits => grid_rect.height() as f32,
    //     TileScaling::Pixels => grid_rect.height() as f32 * ppu.y as f32,
    // };

    // proj.scaling_mode = ScalingMode::FixedVertical(ortho_size);

    // let vp_size = target_res * zoom;
    // let vp_pos = if window_res.cmple(target_res).any() {
    //     Vec2::ZERO
    // } else {
    //     (window_res / 2.0) - (vp_size / 2.0)
    // }
    // .floor();

    // cam.viewport = Some(Viewport {
    //     physical_position: vp_pos.as_uvec2(),
    //     physical_size: vp_size.as_uvec2(),
    //     ..Default::default()
    // });
}

#[cfg(test)]
mod tests {
    fn round_to_step(value: f32, step: f32) -> f32 {
        step * (value / step).round()
    }

    #[test]
    fn step() {
        let step = 1.75;

        let res = round_to_step(-3.33, step);
        println!("Res {}", res);
    }
}
