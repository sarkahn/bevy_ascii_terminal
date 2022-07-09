use bevy::prelude::*;

use bevy_ascii_terminal::{ui::BorderGlyphs, TerminalBundle, TerminalPlugin};
use bevy_tiled_camera::*;

fn main() {
    App::new()
        // Must add TiledCameraPlugin first: https://github.com/bevyengine/bevy/issues/1255
        .add_plugin(TiledCameraPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)
        .run()
}

fn make_terminal(
    commands: &mut Commands,
    pos: Vec3,
    size: [u32; 2],
    pivot: [f32; 2],
    string: &str,
) {
    let mut term_bundle = TerminalBundle::new().with_size(size);
    term_bundle.renderer.terminal_pivot.0 = Vec2::from(pivot);

    term_bundle
        .terminal
        .draw_border(BorderGlyphs::single_line());
    term_bundle.terminal.put_string([1, 1], string);

    term_bundle.transform.translation = pos;

    commands.spawn_bundle(term_bundle);
}

fn setup(mut commands: Commands) {
    let term_size = [15, 3];
    let view_size = [40, 21];

    let origin = Vec3::ZERO;
    let right = Vec3::X * view_size[0] as f32;
    let up = Vec3::Y * view_size[1] as f32;

    let pivot_bl = Vec2::ZERO;
    let pivot_right = Vec2::X;
    let pivot_top = Vec2::Y;

    make_terminal(
        &mut commands,
        origin,
        term_size,
        pivot_bl.into(),
        "BottomLeft",
    );

    make_terminal(
        &mut commands,
        origin + up,
        term_size,
        pivot_top.into(),
        "TopLeft",
    );

    make_terminal(
        &mut commands,
        origin + up + right,
        term_size,
        (pivot_right + pivot_top).into(),
        "TopRight",
    );

    make_terminal(
        &mut commands,
        origin + right,
        term_size,
        pivot_right.into(),
        "BottomRight",
    );

    let view_pos = UVec2::from(view_size) / 2;

    commands.spawn_bundle(
        TiledCameraBundle::new()
            .with_pixels_per_tile([8, 8])
            .with_tile_count(view_size)
            .with_camera_position(view_pos.as_vec2()),
    );
}
