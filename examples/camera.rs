//! Demonstrates how the [TerminalCamera] will automatically adjust the viewport
//! to render all visible terminals.

// TODO: Fix top left mesh getting clipped...probably transform issue
// TODO: Crash if terminal is too small - should be handled in put_string

use bevy::{
    app::AppExit,
    color::palettes::css::{BLUE, RED},
    prelude::*,
    time::common_conditions::on_timer,
};
use bevy_ascii_terminal::*;

const FADED: f32 = 0.65;
const BRIGHT: f32 = 1.0;

#[derive(Resource, Default)]
struct Current(usize);

/// It's necessary to store the strings externally since the terminals may be
/// resized.
#[derive(Component)]
pub struct TermString(String);

fn main() {
    let key_repeat = std::time::Duration::from_secs_f32(0.1);
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .init_resource::<Current>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, put_strings)
        .add_systems(
            Update,
            (
                handle_just_pressed,
                handle_pressed.run_if(on_timer(key_repeat)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(TerminalCamera::new());

    commands.spawn((
        make_terminal([10, 10], BRIGHT),
        TerminalMeshPivot::RightBottom,
        TermString("WASD to change size".to_string()),
    ));
    commands.spawn((
        make_terminal([10, 10], FADED),
        TerminalMeshPivot::LeftBottom,
        TermString("Tab to change active terminal".to_string()),
    ));
    commands.spawn((
        make_terminal([12, 12], FADED),
        TerminalMeshPivot::CenterTop,
        TermString("Space to toggle border".to_string()),
    ));
}

fn make_terminal(size: impl Into<UVec2>, lightness: f32) -> Terminal {
    let mut term = Terminal::new(size.into());
    draw_grid(&mut term, lightness);
    term
}

fn draw_grid(term: &mut Terminal, lightness: f32) {
    for (p, t) in term.iter_xy_mut() {
        let grid_color = if (p.x + p.y) % 2 == 0 {
            BLUE.with_luminance(lightness - 0.5)
        } else {
            RED.with_luminance(lightness - 0.5)
        };
        t.fg_color = t.fg_color.with_luminance(lightness);
        t.bg_color = grid_color.into();
    }
}

fn put_strings(mut q_term: Query<(&mut Terminal, &TermString)>) {
    for (mut term, string) in &mut q_term {
        term.put_string([0, 0], string.0.as_str());
    }
}

fn handle_just_pressed(
    mut q_term: Query<(Entity, &mut Terminal, &TermString)>,
    input: Res<ButtonInput<KeyCode>>,
    mut current: ResMut<Current>,
    mut evt_quit: MessageWriter<AppExit>,
) {
    // If we're accessing a terminal by index we need to make sure they're
    // always in the same order
    let mut terminals: Vec<_> = q_term.iter_mut().sort::<Entity>().rev().collect();
    if input.just_pressed(KeyCode::Tab) {
        current.0 = (current.0 + 1) % terminals.len();
        for (i, (_, term, string)) in terminals.iter_mut().enumerate() {
            let lightness = if current.0 == i { BRIGHT } else { FADED };
            draw_grid(term, lightness);
            if term.padding() == Padding::ONE {
                term.put_border(BoxStyle::SINGLE_LINE);
            }
            term.put_string([0, 0], string.0.as_str());
        }
    }

    if input.just_pressed(KeyCode::Space) {
        for (i, (_, term, string)) in terminals.iter_mut().enumerate() {
            if current.0 == i {
                if term.padding() == Padding::ONE {
                    term.set_padding(Padding::ZERO);
                    term.clear();
                    draw_grid(term, BRIGHT);
                    term.put_string([0, 0], string.0.as_str());
                } else {
                    term.clear();
                    draw_grid(term, BRIGHT);
                    term.put_border(BoxStyle::SINGLE_LINE);
                    term.put_string([0, 0], string.0.as_str());
                }
            }
        }
    }

    if input.just_pressed(KeyCode::Escape) {
        evt_quit.write(AppExit::Success);
    }
}

fn handle_pressed(
    mut q_term: Query<(&mut Terminal, &TermString)>,
    input: Res<ButtonInput<KeyCode>>,
    current: Res<Current>,
) {
    let hor = input.pressed(KeyCode::KeyD) as i32 - input.pressed(KeyCode::KeyA) as i32;
    let ver = input.pressed(KeyCode::KeyW) as i32 - input.pressed(KeyCode::KeyS) as i32;

    let size = IVec2::new(hor, ver);
    if !size.cmpeq(IVec2::ZERO).all() {
        // You can sort by entity even if Entity isn't explicitly in the query
        let mut terminals: Vec<_> = q_term.iter_mut().sort::<Entity>().rev().collect();
        let string = terminals[current.0].1;
        let term = &mut terminals[current.0].0;

        let curr_size = term.size().as_ivec2();
        term.resize((curr_size + size).max(IVec2::ONE).as_uvec2());
        term.clear();
        draw_grid(term, BRIGHT);
        if term.padding() == Padding::ONE {
            term.put_border(BoxStyle::SINGLE_LINE);
        }
        term.put_string([0, 0], string.0.as_str());
    }
}
