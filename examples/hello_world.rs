use bevy::{prelude::*, render::camera::ScalingMode};

use bevy_ascii_terminal::{terminal::Terminal, TerminalBundle, TerminalPlugin};

fn spawn_terminal(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>) {
    let mut term_bundle = TerminalBundle::with_size(20, 3);

    term_bundle.terminal.draw_border_single();
    term_bundle.terminal.put_string(1, 1, "Press spacebar");
    commands.spawn_bundle(term_bundle);

    let mut cam = OrthographicCameraBundle::new_2d();
    cam.orthographic_projection.scaling_mode = ScalingMode::FixedVertical;
    cam.orthographic_projection.scale = 6.0;
    cam.transform.translation += Vec3::new(0.0, 1.5, 0.0);

    commands.spawn_bundle(cam);

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube{ size: 2.0})),
        transform: Transform::from_xyz(-1.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn hello_world(keys: Res<Input<KeyCode>>, mut q: Query<&mut Terminal>) {
    if keys.just_pressed(KeyCode::Space) {
        for mut term in q.iter_mut() {
            term.clear();
            term.draw_border_single();
            term.put_string(1, 1, "Hello, world!");
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
