//! Demonstrates how the [TerminalCamera] will automatically adjust the viewport
//! to render all visible terminals.

use bevy::{
    app::AppExit,
    color::palettes::css::{BLUE, RED},
    prelude::*,
    time::common_conditions::on_timer,
};
use bevy_ascii_terminal::*;
use sark_grids::Pivot;

const FADED: f32 = 0.65;
const BRIGHT: f32 = 1.0;

#[derive(Resource, Default)]
struct Terminals(Vec<Entity>);

#[derive(Resource, Default)]
struct Current(usize);

const TERM_STRINGS: &[&str] = &[
    "WASD to change size",
    "Tab to change active terminal",
    "Space to toggle border",
];

fn main() {
    let key_repeat = std::time::Duration::from_secs_f32(0.1);
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .init_resource::<Terminals>()
        .init_resource::<Current>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_just_pressed,
                handle_pressed.run_if(on_timer(key_repeat)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands, mut terminals: ResMut<Terminals>) {
    commands.spawn(TerminalCamera::new());

    let v = vec![
        (
            make_terminal([10, 10], BRIGHT).with_string([0, 0], TERM_STRINGS[0]),
            TerminalMeshPivot::BottomRight,
            TerminalBorder::single_line(),
        ),
        (
            make_terminal([10, 10], FADED).with_string([0, 0], TERM_STRINGS[1]),
            TerminalMeshPivot::BottomLeft,
            TerminalBorder::single_line(),
        ),
        (
            make_terminal([12, 12], FADED)
                .with_string([0, 0].pivot(Pivot::TopCenter), TERM_STRINGS[2]),
            TerminalMeshPivot::TopCenter,
            TerminalBorder::single_line(),
        ),
    ];
    terminals.0 = Vec::from_iter(v.into_iter().map(|t| commands.spawn(t).id()));
}

fn make_terminal(size: impl GridSize, lightness: f32) -> Terminal {
    let mut term = Terminal::new(size);
    set_brightness(&mut term, lightness);
    term
}

fn set_brightness(term: &mut Terminal, lightness: f32) {
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

fn handle_just_pressed(
    mut q_term: Query<&mut Terminal>,
    input: Res<ButtonInput<KeyCode>>,
    terminals: Res<Terminals>,
    q_border: Query<&TerminalBorder>,
    mut current: ResMut<Current>,
    mut evt_quit: EventWriter<AppExit>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::Tab) {
        current.0 = (current.0 + 1) % terminals.0.len();
        for (i, e) in terminals.0.iter().enumerate() {
            let mut term = q_term.get_mut(*e).unwrap();
            let lightness = if current.0 == i { BRIGHT } else { FADED };
            set_brightness(&mut term, lightness)
        }
    }
    if input.just_pressed(KeyCode::Escape) {
        evt_quit.send(AppExit::Success);
    }
    if input.just_pressed(KeyCode::Space) {
        let e = terminals.0[current.0];
        if q_border.get(e).is_ok() {
            commands.entity(e).remove::<TerminalBorder>();
        } else {
            commands.entity(e).insert(TerminalBorder::single_line());
        }
    }
}

fn handle_pressed(
    mut q_term: Query<&mut Terminal>,
    input: Res<ButtonInput<KeyCode>>,
    terminals: Res<Terminals>,
    current: Res<Current>,
) {
    let mut term = q_term.get_mut(terminals.0[current.0]).unwrap();

    let hor = input.pressed(KeyCode::KeyD) as i32 - input.pressed(KeyCode::KeyA) as i32;
    let ver = input.pressed(KeyCode::KeyW) as i32 - input.pressed(KeyCode::KeyS) as i32;

    let size = IVec2::new(hor, ver);
    if !size.cmpeq(IVec2::ZERO).all() {
        let curr = term.size().as_ivec2();
        term.resize((curr + size).max(IVec2::ONE).as_uvec2());
        let pivot = if current.0 == 2 {
            Pivot::TopCenter
        } else {
            Pivot::TopLeft
        };
        term.put_string([0, 0].pivot(pivot), TERM_STRINGS[current.0]);
        set_brightness(&mut term, BRIGHT);
    }
}
