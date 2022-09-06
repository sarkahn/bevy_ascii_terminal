use bevy_ascii_terminal::*;
use bevy::prelude::*;
use sark_grids::point::Point2d;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .init_resource::<State>()
    .add_startup_system(setup)
    .add_system(input)
    .add_system(update)
    .run();
}

#[derive(Default)]
struct State {
    align: Vec2,
}

fn setup(
    mut commands: Commands,
) {
    let term = Terminal::with_size([80,35]);

    commands.spawn_bundle(TerminalBundle::from(term))
    .insert(AutoCamera);
}

fn aligned_string(term: &mut Terminal, xy: impl GridPoint, rect_size: impl Size2d, string: &str, align: impl Point2d) {

    let strw = string.lines().map(|l|l.chars().count()).max().unwrap() as i32;
    let strh = string.lines().count() as i32;
    let strsz = Vec2::new(strw as f32, strh as f32);
    let recsz = rect_size.as_vec2();
    let xy = xy.as_vec2();
    let align = align.as_vec2();

    let xy = ((xy + recsz * align) - (strsz * align)).floor().as_ivec2();

    let boxw = rect_size.width() as i32;
    let boxh = rect_size.height() as i32;

    let ymax = xy.y.min(boxh - 1);
    let ymin = (ymax - strh).max(0);
    let xmin = xy.x.max(0);
    let xmax = (ymin + strw).min(boxw - 1);

    let mut lines = string.lines();
    for y in ymin..ymax {
        if let Some(line) = lines.next() {
            for (x, glyph) in (xmin..xmax).zip(line.chars()) {
                term.put_char([x,y], glyph);
            }
        }
    }
}

fn input(
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State>,
) {
    if input.just_pressed(KeyCode::Left) {
        state.align.x = (state.align.x - 0.2).max(0.0);
    }

    if input.just_pressed(KeyCode::Right) {
        state.align.x = (state.align.x + 0.2).min(0.0);
    }
    
    if input.just_pressed(KeyCode::Up) {
        state.align.y = (state.align.y - 0.2).max(0.0);
    }

    if input.just_pressed(KeyCode::Down) {
        state.align.y = (state.align.y + 0.2).min(0.0);
    }
}

fn update(
    mut q_term: Query<&mut Terminal>,
    state: Res<State>,
) {
    if state.is_changed() {
        let mut term = q_term.single_mut();
        //aligned_string(&mut term, [0,0], [20,5], "Hello to you my dear\nWhat is up?", state.align);
        term.put_char([0,0],'a');
    }
}