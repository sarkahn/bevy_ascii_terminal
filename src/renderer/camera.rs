//! An optional utility for automatically adjusting the camera to properly
//! view a terminal.
use crate::Terminal;
use crate::TerminalMaterial;

use super::TerminalLayout;

use bevy::prelude::Added;
use bevy::prelude::AssetEvent;
use bevy::prelude::Assets;
use bevy::prelude::Changed;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::CoreSet;
use bevy::prelude::Entity;
use bevy::prelude::EventReader;
use bevy::prelude::Handle;
use bevy::prelude::Image;
use bevy::prelude::IntoSystemConfig;
use bevy::prelude::Plugin;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Transform;
use bevy::prelude::With;

use bevy::prelude::App;
pub use bevy_tiled_camera::TiledCamera;
pub use bevy_tiled_camera::TiledCameraBundle;
use bevy_tiled_camera::TiledCameraPlugin;

/// This component can be added to terminal entities as a simple way to have
/// have the camera render the terminals. The camera viewport will automatically
/// be resized to show all terminal entities with this component.
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
///     let mut term = Terminal::new([7,3]);
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
            commands.spawn((TiledCameraBundle::new(), TerminalCamera));
        } else {
            // Use the first camera we can find
            let ecam = q_cam.iter().next().unwrap();

            // Found camera but it's missing our TerminalCamera component
            if q_term_cam.get(ecam).is_err() {
                commands.entity(ecam).insert(TerminalCamera);
            }
        }
    }
}

fn update(
    q_terminals: Query<(&TerminalLayout, &Handle<TerminalMaterial>), With<AutoCamera>>,
    mut q_cam: Query<(&mut TiledCamera, &mut Transform), With<TerminalCamera>>,
    images: Res<Assets<Image>>,
    materials: Res<Assets<TerminalMaterial>>,
) {
    if let Ok((mut cam, mut transform)) = q_cam.get_single_mut() {
        //println!("UPDATING CAMERA");
        let mut iter = q_terminals.iter();

        if let Some((layout, material)) = iter.next() {
            // TODO: This doesn't account for mixing terminals with different
            // pixels per unit -  properly handling that would require
            // calculating a correct resolution to handle all ppu's without
            // pixel artifacts
            if let Some(material) = materials.get(material) {
                if let Some(image) = &material.texture {
                    if let Some(image) = images.get(image) {
                        let ppu = image.size().as_uvec2() / 16;
                        cam.pixels_per_tile = ppu;
                    }
                }
            }

            let mut rect = layout.bounds_with_border();
            for next in iter {
                rect.envelope_rect(next.0.bounds_with_border());
            }

            //println!("Updating camera bounds. Final Rect {}", rect);
            cam.tile_count = rect.size().as_uvec2();
            let z = transform.translation.z;
            transform.translation = rect.center.as_vec2().extend(z);
        }
    }
}

fn update_cam_conditions(
    q_cam_added: Query<Entity, (With<TiledCamera>, Added<TerminalCamera>)>,
    q_layout_changed: Query<&TerminalLayout, Changed<TerminalLayout>>,
    ev_asset: EventReader<AssetEvent<Image>>,
) -> bool {
    !q_cam_added.is_empty() || !q_layout_changed.is_empty() || !ev_asset.is_empty()
}

/// Will track changes to a terminal and update the viewport so the
/// entire terminal can be visible.
#[derive(Default, Debug, Component)]
struct TerminalCamera;

pub(crate) struct TerminalCameraPlugin;

impl Plugin for TerminalCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TiledCameraPlugin);
        app.add_system(init_camera.in_base_set(CoreSet::First))
            .add_system(
                update
                    .run_if(update_cam_conditions)
                    .after(super::TerminalLayoutChange)
                    .in_base_set(CoreSet::Last),
            );
    }
}
