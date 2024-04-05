use bevy::{
    app::Plugin,
    asset::{AssetEvent, Assets, Handle},
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Query, Res},
    },
    render::texture::Image,
    window::{PrimaryWindow, WindowResized},
};

use crate::{renderer::TerminalMaterial, Terminal, TerminalTransform};

pub struct TerminalCameraPlugin;

impl Plugin for TerminalCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        todo!()
    }
}

#[derive(Event)]
pub struct UpdateViewportEvent;

#[derive(Default)]
pub struct TerminalCamera {
    track_cursor: bool,
    manage_viewport: bool,
}

impl TerminalCamera {
    pub fn auto() -> Self {
        Self {
            manage_viewport: true,
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
) {
    if update_evt.is_empty() {
        return;
    }
    update_evt.clear();
}
