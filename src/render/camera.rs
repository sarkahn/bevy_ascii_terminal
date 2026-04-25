use std::ops::Mul;

use bevy::{
    app::{Plugin, PostUpdate},
    asset::{AssetEvent, Assets},
    camera::{Camera, Projection, ScalingMode},
    ecs::{
        component::Component,
        entity::Entity,
        message::{Message, MessageReader, MessageWriter},
        query::{Changed, Or, With, Without},
        schedule::{IntoScheduleConfigs, SystemSet},
        system::{Query, Res, Single},
    },
    image::Image,
    math::{Mat4, UVec2, Vec2},
    prelude::Camera2d,
    sprite_render::MeshMaterial2d,
    transform::{
        self,
        components::{GlobalTransform, Transform},
    },
    window::{PrimaryWindow, Window, WindowResized},
};

use crate::{Terminal, TerminalMeshPivot};

use super::{TerminalMaterial, TerminalMeshWorldScaling};

pub struct TerminalCameraPlugin;

/// [TerminalCamera] systems for updating the camera viewport.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalSystemsUpdateCamera;

impl Plugin for TerminalCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_message::<UpdateTerminalViewportEvent>()
            .add_systems(
                PostUpdate,
                (
                    cache_cursor_data,
                    cache_camera_data,
                    on_window_resized,
                    on_font_changed,
                    on_viewport_changed,
                    fit_to_terminal,
                )
                    .chain()
                    .in_set(TerminalSystemsUpdateCamera)
                    .before(transform::TransformSystems::Propagate),
            );
    }
}

#[derive(Message)]
pub struct UpdateTerminalViewportEvent;

/// A camera component to assist in rendering terminals and translating
/// cursor coordinates to and from terminal grid coordinates.
#[derive(Component)]
#[require(Camera2d, Transform = cam_transform())]
pub struct TerminalCamera {
    pub track_cursor: bool,
    cam_data: Option<CachedCameraData>,
    cursor_data: Option<CachedCursorData>,
}

fn cam_transform() -> Transform {
    Transform::from_xyz(0., 0., 100.0)
}

impl Default for TerminalCamera {
    fn default() -> Self {
        Self {
            track_cursor: true,
            cam_data: Default::default(),
            cursor_data: Default::default(),
        }
    }
}

impl TerminalCamera {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the world position of the main window cursor using the last
    /// cached camera data.
    ///
    /// Will return [None] if the camera data has not been initialized.
    ///
    /// For accurate results this should be called after [TerminalSystemsUpdateCamera]
    /// which runs in the [First] schedule.
    pub fn cursor_world_pos(&self) -> Option<Vec2> {
        self.cursor_data.as_ref().map(|v| v.world_pos)
    }

    /// The viewport position of the main window cursor as of the last camera
    /// update.
    ///
    /// Will return [None] if the camera data has not been initialized.
    ///
    /// For accurate results this should be called after [TerminalSystemsUpdateCamera]
    /// which runs in [First].
    pub fn cursor_viewport_pos(&self) -> Option<Vec2> {
        self.cursor_data.as_ref().map(|v| v.viewport_pos)
    }

    /// Transform a viewport position to it's corresponding world position using
    /// the last cached camera data.
    ///
    /// If you are attempting to translate the cursor position to/from terminal
    /// grid coordinates, consider using [TerminalCamera::cursor_world_pos] along with
    /// [TerminalTransform::world_to_tile] instead.
    //
    // Note this is more or less a copy of the existing bevy viewport transform
    // function, but adjusted to account for a manually resized viewport which
    // the original function did not do.
    pub fn viewport_to_world(&self, mut viewport_position: Vec2) -> Option<Vec2> {
        let data = self.cam_data.as_ref()?;
        let target_size = data.target_size?;
        if let Some(vp_offset) = data.vp_offset {
            viewport_position -= vp_offset;
        };
        // Flip the Y co-ordinate origin from the top to the bottom.
        viewport_position.y = target_size.y - viewport_position.y;
        let ndc = viewport_position * 2. / target_size - Vec2::ONE;
        let ndc_to_world = data.cam_transform.to_matrix() * data.proj_matrix.inverse();
        let world_space_coords = ndc_to_world.project_point3(ndc.extend(1.));
        (!world_space_coords.is_nan()).then_some(world_space_coords.truncate())
    }
}

#[derive(Default, Debug, Clone)]
struct CachedCameraData {
    cam_transform: GlobalTransform,
    proj_matrix: Mat4,
    target_size: Option<Vec2>,
    vp_offset: Option<Vec2>,
}

#[derive(Default, Debug, Clone)]
struct CachedCursorData {
    viewport_pos: Vec2,
    world_pos: Vec2,
}

#[allow(clippy::type_complexity)]
fn cache_camera_data(
    mut q_cam: Query<
        (&mut TerminalCamera, &GlobalTransform, &Camera),
        Or<(Changed<Camera>, Changed<GlobalTransform>)>,
    >,
) {
    for (mut terminal_cam, transform, cam) in &mut q_cam {
        if !terminal_cam.track_cursor {
            if terminal_cam.cam_data.is_some() {
                terminal_cam.cam_data = None;
            }
            continue;
        }
        terminal_cam.cam_data = Some(CachedCameraData {
            cam_transform: *transform,
            proj_matrix: cam.clip_from_view(),
            target_size: cam.logical_viewport_size(),
            vp_offset: cam.logical_viewport_rect().map(|vp| vp.min),
        });
    }
}

fn cache_cursor_data(
    mut q_cam: Query<&mut TerminalCamera>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let cursor_viewport_pos = window.single().ok().and_then(|w| w.cursor_position());
    for mut terminal_cam in &mut q_cam {
        if !terminal_cam.track_cursor {
            if terminal_cam.cursor_data.is_some() {
                terminal_cam.cursor_data = None;
            }
            continue;
        };

        let Some((viewport_pos, world_pos)) = cursor_viewport_pos
            .and_then(|vp| terminal_cam.viewport_to_world(vp).map(|wp| (vp, wp)))
        else {
            terminal_cam.cursor_data = None;
            continue;
        };
        terminal_cam.cursor_data = Some(CachedCursorData {
            viewport_pos,
            world_pos,
        });
    }
}

fn on_window_resized(
    q_win: Query<Entity, With<PrimaryWindow>>,
    mut resize_events: MessageReader<WindowResized>,
    mut vp_evt: MessageWriter<UpdateTerminalViewportEvent>,
) {
    if q_win.is_empty() || resize_events.is_empty() {
        return;
    }
    let primary_window = q_win.single().unwrap();
    for resize_event in resize_events.read() {
        if resize_event.window != primary_window {
            continue;
        }
        vp_evt.write(UpdateTerminalViewportEvent);
        return;
    }
}

fn on_font_changed(
    mut img_evt: MessageReader<AssetEvent<Image>>,
    mut mat_evt: MessageReader<AssetEvent<TerminalMaterial>>,
    mut vp_evt: MessageWriter<UpdateTerminalViewportEvent>,
    q_term: Query<&MeshMaterial2d<TerminalMaterial>, With<Terminal>>,
    mats: Res<Assets<TerminalMaterial>>,
) {
    if q_term.is_empty() || (img_evt.is_empty() && mat_evt.is_empty()) {
        return;
    }

    for evt in mat_evt.read() {
        let changed_mat_id = match evt {
            AssetEvent::Modified { id } => id,
            _ => continue,
        };
        if q_term.iter().any(|mat| mat.id() == *changed_mat_id) {
            vp_evt.write(UpdateTerminalViewportEvent);
            return;
        }
    }
    for evt in img_evt.read() {
        let loaded_image_id = match evt {
            AssetEvent::LoadedWithDependencies { id } => id,
            _ => continue,
        };
        if q_term
            .iter()
            .filter_map(|mat| mats.get(&mat.0).and_then(|mat| mat.texture.as_ref()))
            .any(|image| image.id() == *loaded_image_id)
        {
            vp_evt.write(UpdateTerminalViewportEvent);
            return;
        }
    }
}

fn on_viewport_changed(
    q_cam: Query<&Camera, (With<TerminalCamera>, Changed<Camera>)>,
    mut update: MessageWriter<UpdateTerminalViewportEvent>,
) {
    if q_cam.is_empty() {
        return;
    }

    update.write(UpdateTerminalViewportEvent);
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn fit_to_terminal(
    mut q_term: Query<(
        &mut Terminal,
        &TerminalMeshPivot,
        &Transform,
        &MeshMaterial2d<TerminalMaterial>,
    )>,
    mesh_scaling: Res<TerminalMeshWorldScaling>,
    q_cam: Single<
        (&Camera, &mut Projection, &mut Transform),
        (Without<Terminal>, With<TerminalCamera>),
    >,
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
    mut update_evt: MessageReader<UpdateTerminalViewportEvent>,
) {
    // TODO: Remove once tracking asset changes becomes more ergonomic
    if update_evt.is_empty() {
        return;
    }

    let (cam, mut proj, mut cam_transform) = q_cam.into_inner();

    // Determine a canonical pixels per unit based on the largest of
    // all terminals. Probably not the best way to do that
    let mut pixels_per_tile = UVec2::new(8, 8);
    for (_, _, _, mat) in q_term.iter_mut() {
        let Some(ppt) = materials
            .get(&mat.0)
            .and_then(|mat| mat.texture.as_ref().and_then(|image| images.get(image)))
            .map(|i| i.size() / 16)
        // Assuming 16/16 tiled textures
        else {
            continue;
        };
        pixels_per_tile = pixels_per_tile.max(ppt);
    }

    // The size of a single terminal mesh tile in world space
    let world_tile = match *mesh_scaling {
        TerminalMeshWorldScaling::World => {
            Vec2::new(pixels_per_tile.x as f32 / pixels_per_tile.y as f32, 1.0)
        }
        TerminalMeshWorldScaling::Pixels => pixels_per_tile.as_vec2(),
    };
    // Size of a pixel in world space
    let world_pixel = world_tile / pixels_per_tile.as_vec2();

    let mut mesh_world_bl = Vec2::MAX;
    let mut mesh_world_tr = Vec2::MIN;
    for (term, mesh_pivot, term_transform, _) in q_term.iter() {
        let mesh_world_size = term.size().as_vec2() * world_tile;
        let mesh_pivot_offset = mesh_pivot.normalized() * mesh_world_size;
        let mesh_pos = term_transform.translation.truncate();
        let mesh_bl = mesh_pos - mesh_pivot_offset;
        let mesh_tr = mesh_bl + mesh_world_size;
        mesh_world_bl = mesh_world_bl.min(mesh_bl);
        mesh_world_tr = mesh_world_tr.max(mesh_tr);
    }

    let tile_count = ((mesh_world_tr - mesh_world_bl) / world_tile)
        .round()
        .as_uvec2();

    let vp_size = cam.physical_viewport_size().unwrap();
    let target_resolution = (tile_count * pixels_per_tile).as_vec2();

    let scale = (vp_size.as_vec2() / target_resolution)
        .floor()
        .as_uvec2()
        .min_element()
        .max(1);

    let vp_height = vp_size.y;
    let vp_world_height = match mesh_scaling.as_ref() {
        TerminalMeshWorldScaling::Pixels => vp_height as f32 / scale as f32,
        TerminalMeshWorldScaling::World => vp_height as f32 / (scale * pixels_per_tile.y) as f32,
    };

    if let Projection::Orthographic(proj) = proj.as_mut() {
        proj.scaling_mode = ScalingMode::FixedVertical {
            viewport_height: vp_world_height,
        };
        proj.viewport_origin = Vec2::ZERO;
    }

    let scaled_res = target_resolution * scale as f32;
    let edge_pixels = (vp_size.as_vec2() - scaled_res).mul(0.5).floor();
    let center_offset = edge_pixels / scale as f32 * world_pixel;

    let cam_z = cam_transform.translation.z;
    cam_transform.translation = (mesh_world_bl - center_offset).extend(cam_z);

    update_evt.clear();
}
