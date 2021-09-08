use bevy::prelude::*;
use bevy_ascii_terminal::{Terminal, TerminalBundle, TerminalPlugin};
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};


fn setup(
    mut commands: Commands
) {
    let (w,h) = (10,26);
    let mut bundle = TerminalBundle::with_size(w,h);
    draw_colors(&mut bundle.terminal);
    commands.spawn_bundle(bundle);

    commands.spawn_bundle(PixelCameraBundle::from_resolution(
        w as i32 * 8,
        h as i32 * 8
    ));
}

fn draw_colors(term: &mut Terminal) {
    let hue_count = 24;
    let hue_max = 360.0;
    let hue_step = hue_max / hue_count as f32;

    // Draw shaded hues
    for y in 0..hue_count {
        let hue = y as f32 * hue_step;
        let col = Color::hsl(hue, 1.0, 0.05);
        term.put_bg_color(0, y, col);
        for x in 1..10 {
            let col = Color::hsl(hue, 1.0, x as f32 / 10.0);
            term.put_bg_color(x, y, col);
        }
    }

    // Draw greys/sepia
    for x in 0..10 {
        let v = x as f32 / 10.0;
        let col = Color::rgb(v, v, v);
        term.put_bg_color(x, 24, col);

        let mut light_scale = 0.9;
        if x <= 5 {
            light_scale = 0.75;
        };
        let col = Color::hsl(35.0, 0.33, v * light_scale);
        term.put_bg_color(x, 25, col);
    }
}

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .add_plugin(PixelCameraPlugin)
    .insert_resource(ClearColor(Color::BLACK))
    .add_startup_system(setup.system())
    .run();
}