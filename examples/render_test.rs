use bevy::prelude::*;
use bevy_terminal::{TerminalBundle, TerminalPlugin};

fn setup(
    mut commands: Commands,
) {
    commands.spawn_bundle(TerminalBundle::with_size(40,40));

    let mut cam = PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 80.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };
    cam.transform.translation += Vec3::new(20.0, 20.0, 0.0);
    commands.spawn_bundle(cam);
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(setup.system())
        .run();
}