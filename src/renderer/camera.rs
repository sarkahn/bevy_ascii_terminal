//! An optional utility for automatically adjusting the camera to properly
//! view a terminal.
use crate::Terminal;

use super::TerminalLayout;

use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::Added;
use bevy::prelude::Changed;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::IntoSystemDescriptor;
use bevy::prelude::Plugin;
use bevy::prelude::Query;
use bevy::prelude::Transform;
use bevy::prelude::With;
use bevy::prelude::info;
use bevy::prelude::{App, CoreStage};
pub use bevy_tiled_camera::TiledCamera;
pub use bevy_tiled_camera::TiledCameraBundle;
use bevy_tiled_camera::TiledCameraPlugin;

/// This component can be added to terminal entities as a simple way to have
/// have the camera render the terminals. The camera viewport will automatically
/// be resized to contain all terminal entities with this component.
///
/// If no camera exists, one will be automatically created. If a camera exists,
/// the first one found will be used.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
/// use bevy_ascii_terminal::*;
///
/// fn setup(mut commands: Commands) {
///     let mut term = Terminal::with_size([7,3]);
///     term.put_string([1,1], "Hello");
///
///     commands.spawn((
///         TerminalBundle::from(term),
///         AutoCamera
///     ));
/// }
/// ```
#[derive(Component)]
pub struct AutoCamera;

/// Will track changes to a terminal and update the viewport so the
/// entire terminal can be visible.
#[derive(Default, Debug, Component)]
struct TerminalCamera;

pub(crate) struct TerminalCameraPlugin;

impl Plugin for TerminalCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TiledCameraPlugin);
        app.add_system_to_stage(CoreStage::First, init_camera)
        .add_system_to_stage(CoreStage::Last, 
            update
            .with_run_criteria(update_cam_conditions)
            .after(super::TERMINAL_LAYOUT_CHANGE)
        );
    }
}

fn init_camera(
    mut commands: Commands,
    q_term: Query<Entity, (With<Terminal>, With<AutoCamera>)>,
    q_cam: Query<Entity, With<TiledCamera>>,
    q_term_cam: Query<&TerminalCamera>,
) {
    // Found a terminal with an autocamera
    if !q_term.is_empty() {
        // Camera not set up yet, create one
        if q_cam.is_empty() {
            //println!("Spawning auto camera");
            commands.spawn((
                TiledCameraBundle::new(),
                TerminalCamera
            ));
        } else {
            // Use the first camera we can find
            let ecam = q_cam.iter().next().unwrap();

            // Found camera but it's missing our TerminalCamera component
            if !q_term_cam.get(ecam).is_ok() {
                commands.entity(ecam).insert(TerminalCamera);
            }
        }
    }
}

fn update(
    q_terminals: Query<&TerminalLayout, With<AutoCamera>>,
    mut q_cam: Query<(&mut TiledCamera, &mut Transform), With<TerminalCamera>>,
) { 
    if let Ok((mut cam, mut transform)) = q_cam.get_single_mut() {
        let mut iter = q_terminals.iter().map(|layout| layout.bounds_with_border());
        for rect in q_terminals.iter().map(|layout| layout.bounds_with_border()) {
            info!("Rect {:?}, Min {}, max {}", rect, rect.min_i(), rect.max_i());
        }
        if let Some(mut rect) = iter.next() {
            while let Some(next) = iter.next() {
                rect.envelope_rect(next);
            }
    
            info!("Updating camera bounds. Final Rect {}", rect);
            cam.tile_count = rect.size().as_uvec2();
            let z = transform.translation.z;
            transform.translation = rect.center.as_vec2().extend(z);
        }
    }
}

fn update_cam_conditions(
    q_cam_added: Query<Entity, (With<TiledCamera>, Added<TerminalCamera>)>,
    q_layout_changed: Query<&TerminalLayout, Changed<TerminalLayout>>,
) -> ShouldRun {
    if !q_cam_added.is_empty() || !q_layout_changed.is_empty() {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}