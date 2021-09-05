use bevy::prelude::*;

use bevy_ascii_terminal::{TerminalBundle, TerminalPlugin};
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

fn make_terminal(
    commands: &mut Commands,
    pos: Vec3,
    size: (usize, usize),
    pivot: Vec2,
    string: &str,
) {
    let mut term_bundle = TerminalBundle::with_size(size.0, size.1);
    term_bundle.renderer.terminal_pivot.0 = pivot;

    term_bundle.terminal.draw_border_single();
    term_bundle.terminal.put_string(1, 1, string);

    term_bundle.transform.translation = pos;

    commands.spawn_bundle(term_bundle);
}

fn setup(mut commands: Commands) {
    let term_size = (15, 3);
    let view_size = (40, 21);

    let origin = Vec3::ZERO;
    let right = Vec3::X * view_size.0 as f32 * 12.0;
    let up = Vec3::Y * view_size.1 as f32 * 12.0;

    let pivot_bl = Vec2::ZERO;
    let pivot_right = Vec2::X;
    let pivot_top = Vec2::Y;

    make_terminal(&mut commands, origin, term_size, pivot_bl, "BottomLeft");

    make_terminal(&mut commands, origin + up, term_size, pivot_top, "TopLeft");

    make_terminal(
        &mut commands,
        origin + up + right,
        term_size,
        pivot_right + pivot_top,
        "TopRight",
    );

    make_terminal(
        &mut commands,
        origin + right,
        term_size,
        pivot_right,
        "BottomRight",
    );

    let (view_x, view_y) = (view_size.0 * 12, view_size.1 * 12);

    let mut cam = PixelCameraBundle::from_resolution(view_x, view_y);

    let x = view_x as f32 / 2.0;
    let y = view_y as f32 / 2.0;

    cam.transform = Transform::from_xyz(x, y, 0.0);
    commands.spawn_bundle(cam);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_plugin(PixelCameraPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup.system())
        .run()
}
