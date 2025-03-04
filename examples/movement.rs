use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_ascii_terminal::*;

#[derive(Component, Deref, DerefMut)]
pub struct Position(IVec2);

#[derive(Component, Deref, DerefMut)]
pub struct Movement(IVec2);

pub const KEY_REPEAT: f32 = 0.01;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(
            PreUpdate,
            (
                input,
                movement.run_if(on_timer(Duration::from_secs_f32(KEY_REPEAT))),
            )
                .chain(),
        )
        .add_systems(PostUpdate, render)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Terminal::new([50, 50]));
    commands.spawn(TerminalCamera::new());

    commands.spawn((Position(IVec2::new(25, 25)), Movement(IVec2::ZERO)));
}

fn input(key: Res<ButtonInput<KeyCode>>, mut q_move: Query<&mut Movement>) {
    let left = -(key.any_pressed([KeyCode::Numpad1, KeyCode::Numpad4, KeyCode::Numpad7]) as i32);
    let up = key.any_pressed([KeyCode::Numpad7, KeyCode::Numpad8, KeyCode::Numpad9]) as i32;
    let down = -(key.any_pressed([KeyCode::Numpad1, KeyCode::Numpad2, KeyCode::Numpad3]) as i32);
    let right = key.any_pressed([KeyCode::Numpad3, KeyCode::Numpad6, KeyCode::Numpad9]) as i32;

    let movement = IVec2::new(right + left, up + down);
    if movement.x == 0 && movement.y == 0 {
        return;
    }
    q_move.single_mut().0 = movement;
}

fn movement(q_term: Query<&Terminal>, mut q_guy: Query<(&mut Movement, &mut Position)>) {
    let (mut mov, mut pos) = q_guy.single_mut();
    if mov.0 == IVec2::ZERO {
        return;
    }
    let term = q_term.single();
    let next = mov.0 + pos.0;
    if !term.size().contains_point(next) {
        return;
    }
    if next == pos.0 {
        return;
    }
    pos.0 = next;
    mov.0 = IVec2::ZERO;
}

fn render(
    mut q_term: Query<&mut Terminal>,
    q_guy: Query<&Position, Changed<Position>>,
    time: Res<Time>,
) {
    let Ok(guy) = q_guy.get_single() else {
        return;
    };
    let mut term = q_term.single_mut();
    term.clear();
    let t = Tile::default().with_char('.');
    term.fill(t);
    term.put_char(guy.0, '@');
    println!("Changing terminal: {}", time.elapsed_secs());
}
