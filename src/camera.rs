use bevy::{
    app::{Last, Plugin}, asset::{AssetEvent, Assets, Handle}, core_pipeline::core_2d::Camera2dBundle, ecs::{
        bundle::Bundle, component::Component, entity::Entity, event::{Event, EventReader, EventWriter}, query::With, schedule::IntoSystemConfigs, system::{Query, Res}
    }, math::{Rect, UVec2, Vec2}, render::{camera::Camera, texture::Image}, transform::components::Transform, window::{PrimaryWindow, Window, WindowResized}
};

use crate::{renderer::TerminalMaterial, GridRect, Terminal, TerminalGridSettings, TerminalTransform};

pub struct TerminalCameraPlugin;

impl Plugin for TerminalCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Last,
        (on_window_resized,
        on_font_changed,
        update_viewport).chain());
    }
}

#[derive(Event)]
pub struct UpdateViewportEvent;

#[derive(Default, Component)]
pub struct TerminalCamera {
    track_cursor: bool,
    manage_viewport: bool,
}

#[derive(Default, Bundle)]
pub struct TerminalCameraBundle {
    cam_bundle: Camera2dBundle,
    term_cam: TerminalCamera,
}

impl TerminalCameraBundle {
    pub fn auto() -> Self {
        Self {
            term_cam: TerminalCamera {
                manage_viewport: true,
                ..Default::default()
            },
            ..Default::default()
        }
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
    let ewin = q_win.single();
    for evt in resize_events.read() {
        if evt.window != ewin {
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
        let material_id = match evt {
            AssetEvent::Modified { id } => id,
            _ => continue,
        };
        if q_term.iter().any(|mat| mat.id() == *material_id) {
            vp_evt.send(UpdateViewportEvent);
            return;
        }
    }
    for evt in img_evt.read() {
        let image_id = match evt {
            AssetEvent::LoadedWithDependencies { id } => id,
            _ => continue,
        };
        if q_term
            .iter()
            .filter_map(|mat| mats.get(mat).and_then(|mat| mat.texture.as_ref()))
            .any(|image| image.id() == *image_id)
        {
            vp_evt.send(UpdateViewportEvent);
            return;
        }
    }
}

fn update_viewport(
    q_term: Query<&TerminalTransform>,
    mut q_cam: Query<&mut Transform, With<Camera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut update_evt: EventReader<UpdateViewportEvent>,
    grid: Res<TerminalGridSettings>,
) {
    if update_evt.is_empty() {
        return;
    }
    update_evt.clear();

    let Ok(window) = q_window.get_single() else {
        return;
    };

    let Some(ppu) = q_term.iter().map(|t| t.pixels_per_unit()).reduce(UVec2::min) else {
        return;
    };

    let tile_size = grid.tile_scaling().calculate_world_tile_size(ppu, None);

    let intersect = |a: Rect, b: Rect| a.intersect(b);
    let Some(bounds) = q_term.iter().map(|t|t.world_bounds()).reduce(intersect) else {
        return;
    };

    let min = (bounds.min / tile_size).floor() * tile_size;
    let max = (bounds.max / tile_size).ceil() * tile_size;

    let total_grid = GridRect::from_points((min / tile_size).as_ivec2(), (max / tile_size).as_ivec2());

    let Ok(mut cam) = q_cam.get_single_mut() else {
        return;
    };
    
    let z = cam.translation.z;
    let cam_pos = bounds.center().extend(z);
    *cam = Transform::from_translation(cam_pos);
    

    let target_res = bounds.size();
    let window_res = UVec2::new(window.physical_width(), window.physical_height()).as_vec2();

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


fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u32, b: u32) -> u32 {
    (a * b) / gcd(a, b)
}

fn lcm_vec(a: UVec2, b: UVec2) -> UVec2 {
    let x = lcm(a.x, b.x);
    let y = lcm(a.y, b.y);
    UVec2::new(x,y)
}
