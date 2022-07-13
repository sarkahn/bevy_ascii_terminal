use bevy::prelude::*;
use bevy_tiled_camera::{*};

use crate::Terminal;

use super::{PixelsPerTile, TileScaling, TERMINAL_UPDATE_SIZE, TERMINAL_INIT};

pub struct TerminalCameraPlugin;

impl Plugin for TerminalCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system(init_camera.before(TERMINAL_INIT))
        .add_system(update_from_new.after(TERMINAL_UPDATE_SIZE))
        .add_system(update_from_terminal_change.after(TERMINAL_UPDATE_SIZE));
    }
}

/// Will track changes to the target terminal and update the viewport so the 
/// entire terminal can be visible.
#[derive(Default, Debug,Component)]
pub struct TerminalCamera {
    terminal: Option<Entity>,
}

impl TerminalCamera {
    pub fn with_terminal_entity(terminal: Entity) -> Self {
        Self {
            terminal: Some(terminal)
        }
    }

    /// The terminal that this camera is tracking.
    pub fn terminal(&self) -> Option<Entity> {
        self.terminal
    }
}

fn init_camera(
    mut commands: Commands,
    q_term: 
        Query<Entity, 
        (
            With<Terminal>,
            With<TerminalCamera>,
        )>,
    mut q_cam_with: Query<&mut TerminalCamera, With<TiledCamera>>,
    q_cam_without: Query<Entity,
        (
            Without<TerminalCamera>,
            With<TiledCamera>,
        )>,
) {
    for term_entity in q_term.iter() {
        commands.entity(term_entity).remove::<TerminalCamera>();
        
        // Try to find any camera
        if let Some(mut tcam) = q_cam_with.iter_mut().next() {
            tcam.terminal = Some(term_entity);
        } else if let Some(cam_entity) = q_cam_without.iter().next() {
            commands.entity(cam_entity).insert(TerminalCamera {
                terminal: Some(term_entity)
            });
        // Couldn't find any cameras - so let's make one
        } else {
            commands.spawn_bundle(TiledCameraBundle::new())
            .insert(TerminalCamera {
                terminal: Some(term_entity)
            });
        }
    }
}

fn update_from_new(
    mut q_cam: Query<(&mut TiledCamera, &TerminalCamera), 
        (
            Changed<TerminalCamera>,
            With<TiledCamera>,
        )
        >,
        q_term: Query<(&Terminal,&PixelsPerTile,&TileScaling)>,
) {
    if q_cam.is_empty() || q_term.is_empty() {
        return;
    }

    for (mut cam, tcam) in q_cam.iter_mut() {
        if let Some(term) = tcam.terminal {
            if let Ok((term,ppt,scaling)) = q_term.get(term) {
                cam.tile_count = term.size();
                cam.pixels_per_tile = **ppt;
                match scaling {
                    TileScaling::World => cam.set_world_space(WorldSpace::Units),
                    TileScaling::Pixels => cam.set_world_space(WorldSpace::Pixels),
                }
            }
        }
    }
}

fn update_from_terminal_change(
    q_term: Query<
        (&Terminal,&PixelsPerTile,&TileScaling), 
        Or<(
            Changed<PixelsPerTile>,
            Changed<TileScaling>
        )>
    >,
    mut q_cam: Query<(&mut TiledCamera, &TerminalCamera)>, 
) {
    // Check if any terminals changed
    if q_term.is_empty() {
        return;
    }

    for (mut cam, term) in q_cam.iter_mut() {
        if let Some(term) = term.terminal {
            if let Ok((term,ppt,scaling)) = q_term.get(term) {
                cam.tile_count = term.size();
                cam.pixels_per_tile = **ppt;
                match scaling {
                    TileScaling::World => cam.set_world_space(WorldSpace::Units),
                    TileScaling::Pixels => cam.set_world_space(WorldSpace::Pixels),
                }
            }
        }
    }
}
