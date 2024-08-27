use bevy::{
    app::{Last, Plugin, PostUpdate},
    asset::{AssetEvent, Assets, Handle},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::{Changed, Or, With},
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Query, Res},
    },
    math::{IVec2, Mat4, Rect, UVec2, Vec2},
    render::{
        camera::{Camera, CameraUpdateSystem, OrthographicProjection, ScalingMode, Viewport},
        texture::Image,
    },
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window, WindowResized},
};

use crate::{
    renderer::TerminalMaterial, GridRect, GridSize, Terminal, TerminalGridSettings,
    TerminalTransform, TileScaling,
};

pub struct TerminalCameraPlugin;

/// [TerminalCamera] systems for caching camera data. Runs in [PostUpdate].
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalSystemCameraCacheData;

/// [TerminalCamera] systems for updating the camera viewport.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalSystemCameraViewportUpdate;

impl Plugin for TerminalCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UpdateTerminalViewportEvent>()
            .add_systems(
                PostUpdate,
                (cache_camera_data, cache_cursor_data)
                    .chain()
                    .in_set(TerminalSystemCameraCacheData)
                    .after(CameraUpdateSystem),
            )
            .add_systems(
                Last,
                (on_window_resized, on_font_changed, update_viewport)
                    .chain()
                    .in_set(TerminalSystemCameraViewportUpdate),
            );
    }
}

#[derive(Event)]
pub struct UpdateTerminalViewportEvent;

/// Settings for how the [TerminalCamera] should set up the viewport for terminal
/// rendering.
#[derive(Debug, PartialEq, Default)]
pub enum TerminalCameraViewport {
    /// The [TerminalCamera] will automatically manage the camera's viewport,
    /// changing it to try and render any active terminals.
    ///
    /// Note for multiple terminals with different font resolutions this could
    /// cause rendering artifacts.
    #[default]
    Auto,
    /// The [TerminalCamera] will manage the camera's viewport, adjusting it to
    /// given target resolution.
    TargetResolutionPixels {
        /// Target pixel resolution. The viewport will be scaled up as much as
        /// possible to fit within the window while maintaining this resolution.
        target_res: UVec2,
        /// The pixel size of a single terminal tile. This defines how many
        /// terminal tiles will fit inside the viewport with the given target
        /// resolution.
        pixels_per_tile: UVec2,
    },
    /// The [TerminalCamera] will manage the camera's viewport, adjusting it to
    /// given tile resolution.
    TargetResolutionTiles {
        tile_count: UVec2,
        pixels_per_tile: UVec2,
    },
    /// The [TerminalCamera] will not adjust the viewport.
    DontModify,
}

/// A camera component to assist in rendering terminals and translating
/// cursor coordinates to and from terminal grid coordinates.
#[derive(Default, Component)]
pub struct TerminalCamera {
    pub viewport_type: TerminalCameraViewport,
    pub track_cursor: bool,
    cam_data: Option<CachedCameraData>,
    cursor_data: Option<CachedCursorData>,
}

impl TerminalCamera {
    /// Returns the world position of the main window cursor using the last
    /// cached camera data.
    ///
    /// Will return [None] if the camera data has not been initialized.
    ///
    /// For accurate results this should be called after [TerminalSystemCameraCacheData]
    /// which runs in [PostUpdate].
    pub fn cursor_world_pos(&self) -> Option<Vec2> {
        self.cursor_data.as_ref().map(|v| v.world_pos)
    }

    /// The viewport position of the main window cursor as of the last camera
    /// update.
    ///
    /// Will return [None] if the camera data has not been initialized.
    ///
    /// For accurate results this should be called after [TerminalSystemCameraCacheData]
    /// which runs in [PostUpdate].
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
    // function, but adjusted to account for a manually resized viewport.
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

    pub fn is_managing_viewport(&self) -> bool {
        self.viewport_type != TerminalCameraViewport::DontModify
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

#[derive(Default, Bundle)]
pub struct TerminalCameraBundle {
    cam_bundle: Camera2dBundle,
    term_cam: TerminalCamera,
}

impl TerminalCameraBundle {
    /// Create a [TerminalCamera] set up to automatically manage the viewport to
    /// try to render any terminal entities.
    ///
    /// The viewport will be scaled up as much as possible while rendering all
    /// terminal entities.
    pub fn with_auto_resolution() -> Self {
        Self {
            term_cam: TerminalCamera {
                viewport_type: TerminalCameraViewport::Auto,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create a [TerminalCamera] set up to automatically manage the viewport to
    /// match a target tile resolution. The viewport will be scaled up as much
    /// as possible to fit within the window while matching the target resolution.
    pub fn with_tile_resolution(tile_count: impl GridSize, pixels_per_tile: impl GridSize) -> Self {
        let vp = TerminalCameraViewport::TargetResolutionTiles {
            tile_count: tile_count.as_uvec2(),
            pixels_per_tile: pixels_per_tile.as_uvec2(),
        };
        Self::with_viewport_type(vp)
    }

    /// Create a [TerminalCamera] with the given [TerminalCameraViewport].
    pub fn with_viewport_type(viewport_type: TerminalCameraViewport) -> Self {
        Self {
            term_cam: TerminalCamera {
                viewport_type,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Enable cursor tracking for the [TerminalCamera].
    ///
    /// If cursor tracking is enabled the Terminal camera will cache viewport
    /// and cursor data every frame so the cursor position can easily be
    /// retrieved via [TerminalCamera::cursor_world_pos].
    pub fn enable_cursor_tracking(&mut self) -> &mut Self {
        self.term_cam.track_cursor = true;
        self
    }
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
    let cursor_viewport_pos = window.get_single().ok().and_then(|w| w.cursor_position());
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
    let primary_window = q_win.single();
    for resize_event in resize_events.read() {
        if resize_event.window != primary_window {
            continue;
        }
        vp_evt.send(UpdateTerminalViewportEvent);
        return;
    }
}

fn on_font_changed(
    mut img_evt: EventReader<AssetEvent<Image>>,
    mut mat_evt: EventReader<AssetEvent<TerminalMaterial>>,
    mut vp_evt: EventWriter<UpdateTerminalViewportEvent>,
    q_term: Query<&Handle<TerminalMaterial>, With<Terminal>>,
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
            vp_evt.send(UpdateTerminalViewportEvent);
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
            .filter_map(|mat| mats.get(mat).and_then(|mat| mat.texture.as_ref()))
            .any(|image| image.id() == *loaded_image_id)
        {
            vp_evt.send(UpdateTerminalViewportEvent);
            return;
        }
    }
}

fn update_viewport(
    q_term: Query<&TerminalTransform>,
    mut q_cam: Query<(
        &TerminalCamera,
        &mut Camera,
        &mut Transform,
        &mut OrthographicProjection,
    )>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut update_evt: EventReader<UpdateTerminalViewportEvent>,
    grid: Res<TerminalGridSettings>,
) {
    if update_evt.is_empty() {
        return;
    }

    let Ok((term_cam, mut cam, mut cam_transform, mut proj)) = q_cam.get_single_mut() else {
        return;
    };
    let Ok(window) = q_window.get_single() else {
        return;
    };

    let (target_res, ortho_size) = match term_cam.viewport_type {
        TerminalCameraViewport::DontModify => return,
        TerminalCameraViewport::TargetResolutionPixels {
            target_res,
            pixels_per_tile,
        } => {
            let ortho_size = match grid.tile_scaling() {
                TileScaling::Pixels => target_res.y,
                TileScaling::World => target_res.y / pixels_per_tile.y,
            };
            (target_res.as_vec2(), ortho_size as f32)
        }
        TerminalCameraViewport::TargetResolutionTiles {
            tile_count,
            pixels_per_tile,
        } => {
            let target_res = tile_count * pixels_per_tile;
            let ortho_size = match grid.tile_scaling() {
                TileScaling::Pixels => target_res.y,
                TileScaling::World => tile_count.y,
            };
            (target_res.as_vec2(), ortho_size as f32)
        }
        TerminalCameraViewport::Auto => {
            // Determine our canonical 'pixels per unit' from the terminal
            // with the largest font.
            let Some(ppu) = q_term
                .iter()
                .map(|t| {
                    t.transform_data()
                        .expect("Terminal transform update should run before camera update")
                        .pixels_per_tile
                })
                .reduce(UVec2::max)
            else {
                return;
            };
            // Determine our canonical tile size from the largest of all terminals.
            let tile_size = q_term
                .iter()
                .map(|t| {
                    t.transform_data()
                        .expect("Terminal transform update should run before camera update")
                        .world_tile_size
                })
                .reduce(Vec2::max)
                .unwrap();

            // Invalid terminal image size, images could still be loading.
            if ppu.cmpeq(UVec2::ZERO).any() {
                return;
            }

            // The total bounds of all terminal meshes in world space
            let mesh_bounds = q_term
                .iter()
                .map(|t| {
                    t.transform_data()
                        .expect("Terminal transform update should run before camera update")
                        .world_mesh_bounds()
                })
                .reduce(|a, b| a.intersect(b))
                .unwrap();

            let z = cam_transform.translation.z;
            cam_transform.translation = mesh_bounds.center().extend(z);

            let tile_count = (mesh_bounds.size() / tile_size).as_ivec2();

            let ortho_size = match grid.tile_scaling() {
                TileScaling::Pixels => tile_count.y as f32 * ppu.y as f32,
                TileScaling::World => tile_count.y as f32,
            };

            let target_res = tile_count.as_vec2() * ppu.as_vec2();
            (target_res, ortho_size)
        }
    };
    update_evt.clear();

    let window_res = UVec2::new(window.physical_width(), window.physical_height()).as_vec2();
    let zoom = (window_res / target_res).floor().min_element().max(1.0);

    let vp_size = target_res * zoom;
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

    proj.scaling_mode = ScalingMode::FixedVertical(ortho_size);
}
