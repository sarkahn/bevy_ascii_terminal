use bevy::{
    app::{Last, Plugin}, asset::{AssetEvent, Assets, Handle}, core_pipeline::core_2d::Camera2dBundle, ecs::{
        bundle::Bundle, component::Component, entity::Entity, event::{Event, EventReader, EventWriter}, query::With, schedule::IntoSystemConfigs, system::{Query, Res}
    }, math::Vec2, render::texture::Image, window::{PrimaryWindow, WindowResized}
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
    mut update_evt: EventReader<UpdateViewportEvent>,
    grid: Res<TerminalGridSettings>,
) {
    if update_evt.is_empty() {
        return;
    }
    update_evt.clear();

    for t in &q_term {
        let tile_size = grid.world_grid_tile_size().unwrap_or(t.world_tile_size());
        let round_to_tile = |p: Vec2| (p / tile_size).round() * tile_size;
        let ceil_to_tile = |p: Vec2| (p / tile_size).floor() * tile_size;
        let min = round_to_tile(t.world_bounds().min).as_ivec2();
        let max = ceil_to_tile(t.world_bounds().max).as_ivec2();

        let grid_rect = GridRect::from_points(min, max);
        println!("GRID RECT: {:?}", grid_rect);
    }
}
