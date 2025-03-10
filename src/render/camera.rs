use bevy::{
    app::{First, Plugin},
    asset::{AssetEvent, Assets},
    ecs::{
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::{Changed, Or, With},
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Query, Res},
    },
    image::Image,
    math::{Mat4, UVec2, Vec2},
    prelude::Camera2d,
    render::camera::{Camera, Projection, ScalingMode, Viewport},
    sprite::MeshMaterial2d,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window, WindowResized},
};

use crate::{Terminal, transform::TerminalTransform};

use super::{TerminalMaterial, TerminalMeshWorldScaling};

pub struct TerminalCameraPlugin;

/// [TerminalCamera] systems for updating the camera viewport.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalSystemsUpdateCamera;

impl Plugin for TerminalCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UpdateTerminalViewportEvent>().add_systems(
            First,
            (
                cache_cursor_data,
                cache_camera_data,
                on_window_resized,
                on_font_changed,
                update_viewport,
            )
                .chain()
                .in_set(TerminalSystemsUpdateCamera),
        );
    }
}

#[derive(Event)]
pub struct UpdateTerminalViewportEvent;

/// A camera component to assist in rendering terminals and translating
/// cursor coordinates to and from terminal grid coordinates.
#[derive(Component)]
#[require(Camera2d, Transform(cam_transform))]
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
        let ndc_to_world = data.cam_transform.compute_matrix() * data.proj_matrix.inverse();
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
    mut resize_events: EventReader<WindowResized>,
    mut vp_evt: EventWriter<UpdateTerminalViewportEvent>,
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
    mut img_evt: EventReader<AssetEvent<Image>>,
    mut mat_evt: EventReader<AssetEvent<TerminalMaterial>>,
    mut vp_evt: EventWriter<UpdateTerminalViewportEvent>,
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

fn update_viewport(
    q_term: Query<&TerminalTransform>,
    mut q_cam: Query<(&mut Camera, &mut Transform, &mut Projection), With<TerminalCamera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    scaling: Res<TerminalMeshWorldScaling>,
    mut update_evt: EventReader<UpdateTerminalViewportEvent>,
) {
    if update_evt.is_empty() {
        return;
    }

    let Ok((mut cam, mut cam_transform, mut proj)) = q_cam.single_mut() else {
        return;
    };
    let Ok(window) = q_window.single() else {
        return;
    };

    // TODO: Calculate this from the lowest common multiple?
    // Determine our canonical 'pixels per unit' from the terminal
    // with the largest font.
    let Some(ppu) = q_term
        .iter()
        .filter_map(|t| t.cached_data.as_ref().map(|d| d.pixels_per_tile))
        .reduce(UVec2::max)
    else {
        // The camera system runs first, so this will return immediately at least once.
        // Furthermore the transform data won't be cached until the terminal font
        // is done loading.
        return;
    };
    // Determine our canonical tile size from the largest of all terminals.
    let Some(tile_size) = q_term
        .iter()
        .filter_map(|t| t.cached_data.as_ref().map(|d| d.world_tile_size))
        .reduce(Vec2::max)
    else {
        // We can probably just unwrap?
        return;
    };

    // Invalid terminal image size, images could still be loading.
    if ppu.cmpeq(UVec2::ZERO).any() {
        return;
    }

    // The total bounds of all terminal meshes in world space
    let Some(mesh_bounds) = q_term
        .iter()
        .filter_map(|t| t.cached_data.as_ref().map(|d| d.world_mesh_bounds))
        .reduce(|a, b| a.union(b))
    else {
        // We can probably just unwrap?
        return;
    };

    let z = cam_transform.translation.z;
    cam_transform.translation = mesh_bounds.center().extend(z);

    let tile_count = (mesh_bounds.size() / tile_size).as_ivec2();

    let ortho_size = match *scaling {
        TerminalMeshWorldScaling::Pixels => tile_count.y as f32 * ppu.y as f32,
        TerminalMeshWorldScaling::World => tile_count.y as f32,
    };

    let target_res = tile_count.as_vec2() * ppu.as_vec2();

    let window_res = UVec2::new(window.physical_width(), window.physical_height()).as_vec2();
    let zoom = (window_res / target_res).floor().min_element().max(1.0);

    let vp_size = (target_res * zoom).max(Vec2::ONE);
    let vp_pos = if window_res.cmple(target_res).any() {
        Vec2::ZERO
    } else {
        (window_res / 2.0) - (vp_size / 2.0)
    }
    .floor();

    if vp_size.cmpgt(window_res).any() {
        cam.viewport = None;
    } else {
        cam.viewport = Some(Viewport {
            physical_position: vp_pos.as_uvec2(),
            physical_size: vp_size.as_uvec2(),
            ..Default::default()
        });
    }

    if let Projection::Orthographic(proj) = proj.as_mut() {
        proj.scaling_mode = ScalingMode::FixedVertical {
            viewport_height: ortho_size,
        };
    }

    update_evt.clear();
}
