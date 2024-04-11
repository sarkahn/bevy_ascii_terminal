use std::ops::Sub;

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
        camera::{Camera, OrthographicProjection, ScalingMode, Viewport},
        texture::Image,
    },
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window, WindowResized},
};

use crate::{
    renderer::TerminalMaterial, terminal, transform::TerminalTransformSystems, GridRect, Terminal,
    TerminalGridSettings, TerminalTransform, TileScaling,
};

pub struct TerminalCameraPlugin;

impl Plugin for TerminalCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UpdateViewportEvent>()
            .add_systems(
                Last,
                (on_window_resized, on_font_changed, update_viewport)
                    .chain()
                    .in_set(TerminalViewportSystems)
                    .after(TerminalTransformSystems),
            )
            .add_systems(
                PostUpdate,
                (cache_camera_data, cache_cursor_data)
                    .chain()
                    .in_set(TerminalCameraSystems),
            );
    }
}

/// Systems for building the terminal mesh.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalViewportSystems;

/// Systems for building the terminal mesh.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct TerminalCameraSystems;

#[derive(Event)]
pub struct UpdateViewportEvent;

#[derive(Default, Component)]
pub struct TerminalCamera {
    pub manage_viewport: bool,
    pub track_cursor: bool,
    cam_data: Option<CachedCameraData>,
    cursor_data: Option<CachedCursorData>,
}

impl TerminalCamera {
    /// Returns the tile position of the main window cursor using the last
    /// cached camera data.
    ///
    /// Will return [None] if the camera data has not been initialized.
    ///
    /// For accurate results this should be called in [PostUpdate] after
    /// [TerminalCameraSystems].
    pub fn cursor_world_pos(&self) -> Option<Vec2> {
        self.cursor_data.as_ref().map(|v| v.world_pos)
    }

    pub fn cursor_viewport_pos(&self) -> Option<Vec2> {
        self.cursor_data.as_ref().map(|v| v.viewport_pos)
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
    /// A terminal camera will be created and will automatically manage
    /// the viewport to try and render any terminal entities.
    pub fn auto() -> Self {
        Self {
            term_cam: TerminalCamera {
                manage_viewport: true,
                track_cursor: false,
                cam_data: None,
                cursor_data: None,
            },
            ..Default::default()
        }
    }

    /// Enable cursor tracking for the [TerminalCamera].
    ///
    /// If cursor tracking is enabled the Terminal camera will cache viewport
    /// and cursor data every frame so cursor positions can easily be
    /// transformed to World/Terminal space.
    pub fn with_cursor_tracking(&mut self) -> &mut Self {
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
            proj_matrix: cam.projection_matrix(),
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
    mut vp_evt: EventWriter<UpdateViewportEvent>,
) {
    if q_win.is_empty() || resize_events.is_empty() {
        return;
    }
    let primary_window = q_win.single();
    for resize_event in resize_events.read() {
        if resize_event.window != primary_window {
            continue;
        }
        vp_evt.send(UpdateViewportEvent);
        return;
    }
}

fn on_font_changed(
    mut img_evt: EventReader<AssetEvent<Image>>,
    mut mat_evt: EventReader<AssetEvent<TerminalMaterial>>,
    mut vp_evt: EventWriter<UpdateViewportEvent>,
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
            vp_evt.send(UpdateViewportEvent);
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
            vp_evt.send(UpdateViewportEvent);
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
    mut update_evt: EventReader<UpdateViewportEvent>,
    grid: Res<TerminalGridSettings>,
) {
    if update_evt.is_empty() {
        return;
    }
    update_evt.clear();

    let Ok((term_cam, mut cam, mut cam_transform, mut proj)) = q_cam.get_single_mut() else {
        return;
    };

    if !term_cam.manage_viewport {
        return;
    }

    let Ok(window) = q_window.get_single() else {
        return;
    };
    let Some(ppu) = q_term
        .iter()
        .map(|t| t.pixels_per_unit())
        .reduce(UVec2::min)
    else {
        return;
    };

    // Transforms not updated yet, terminal images could still be loading.
    if ppu.cmpeq(UVec2::ZERO).any() {
        return;
    }

    let tile_size = grid.tile_scaling().calculate_world_tile_size(ppu, None);

    println!("Camera found ppu {}, tile size {}", ppu, tile_size);

    let min = q_term
        .iter()
        .map(|t| t.world_bounds().min)
        .reduce(Vec2::min)
        .unwrap()
        * tile_size;

    let max = q_term
        .iter()
        .map(|t| t.world_bounds().max)
        .reduce(Vec2::max)
        .unwrap()
        * tile_size;

    let pixel_rect = Rect::from_corners(min, max);
    let z = cam_transform.translation.z;
    let cam_pos = pixel_rect.center().extend(z);
    *cam_transform = Transform::from_translation(cam_pos);

    let window_res = UVec2::new(window.physical_width(), window.physical_height()).as_vec2();
    let zoom = (window_res / pixel_rect.size())
        .floor()
        .min_element()
        .max(1.0);

    let ortho_size = match grid.tile_scaling() {
        TileScaling::World => pixel_rect.height(),
        TileScaling::Pixels => pixel_rect.height() * ppu.y as f32,
    };

    proj.scaling_mode = ScalingMode::FixedVertical(ortho_size);

    let vp_size = pixel_rect.size() * zoom;
    let vp_pos = if window_res.cmple(pixel_rect.size()).any() {
        Vec2::ZERO
    } else {
        (window_res / 2.0) - (vp_size / 2.0)
    }
    .floor();

    cam.viewport = Some(Viewport {
        physical_position: vp_pos.as_uvec2(),
        physical_size: vp_size.as_uvec2(),
        ..Default::default()
    });

    println!(
        "PixelRect {:?}, Cam viewport: {:?}. Ortho size {}",
        pixel_rect, cam.viewport, ortho_size
    );

    // let intersect = |a: Rect, b: Rect| a.intersect(b);
    // let Some(bounds) = q_term.iter().map(|t| t.world_bounds()).reduce(intersect) else {
    //     return;
    // };

    // let min = (bounds.min / tile_size).round().as_ivec2();
    // let max = (bounds.max / tile_size).round().as_ivec2().sub(1);

    // let rect = GridRect::from_points(min, max);

    // let z = cam_transform.translation.z;
    // let cam_pos = bounds.center().extend(z);
    // *cam_transform = Transform::from_translation(cam_pos);

    // let target_res = ppu.as_vec2() * rect.size.as_vec2();
    // let window_res = UVec2::new(window.physical_width(), window.physical_height()).as_vec2();

    // let zoom = (window_res / target_res).floor().min_element().max(1.0);

    // let ortho_size = match grid.tile_scaling() {
    //     TileScaling::World => rect.height() as f32,
    //     TileScaling::Pixels => rect.height() as f32 * ppu.y as f32,
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

// fn gcd(mut a: u32, mut b: u32) -> u32 {
//     while b != 0 {
//         let temp = b;
//         b = a % b;
//         a = temp;
//     }
//     a
// }

// fn lcm(a: u32, b: u32) -> u32 {
//     (a * b) / gcd(a, b)
// }

// fn lcm_vec(a: UVec2, b: UVec2) -> UVec2 {
//     let x = lcm(a.x, b.x);
//     let y = lcm(a.y, b.y);
//     UVec2::new(x, y)
// }
