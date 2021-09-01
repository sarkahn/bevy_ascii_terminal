use bevy::prelude::*;

use bevy_terminal::{TerminalBundle, TerminalPlugin, terminal::Terminal};


fn spawn_terminal(
    mut commands: Commands
) {
    let mut term_bundle = TerminalBundle::with_size(20,3);

    term_bundle.terminal.draw_border_single();
    term_bundle.terminal.put_string(1,1, "Press spacebar");
    commands.spawn_bundle(term_bundle);
    
    let mut cam = PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    };
    cam.transform.translation += Vec3::new(10.0,1.5,0.0);

    commands.spawn_bundle(cam);
}

fn hello_world(
    keys: Res<Input<KeyCode>>,
    mut q: Query<&mut Terminal>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for mut term in q.iter_mut() {
            term.clear();
            term.draw_border_single();
            term.put_string(1,1, "Hello, world!");
        }
    }
}

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .add_startup_system(spawn_terminal.system())
    .add_system(hello_world.system())
    .run()
}