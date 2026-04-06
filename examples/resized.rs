use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_ascii_terminal::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            update.run_if(on_timer(Duration::from_secs_f32(0.01))),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Terminal::new([30, 30]));
    commands.spawn(TerminalCamera::new());
}

#[allow(deprecated)]
fn update(mut q_term: Query<&mut Terminal>, time: Res<Time>) {
    let mut term = q_term.single_mut().unwrap();
    let size = (time.elapsed_secs().cos() * 20.0) as u32 % 20 + 15;
    term.resize([size, size]);

    term.clear();
    term.put_string([0, 0].pivot(Pivot::LeftTop), "TopL");
    term.put_string([0, 0].pivot(Pivot::CenterTop), "TopC");
    term.put_string([0, 0].pivot(Pivot::RightTop), "TopR");
    term.put_string([0, 0].pivot(Pivot::LeftCenter), "LefC");
    term.put_string([0, 0].pivot(Pivot::Center), "C");
    term.put_string([0, 0].pivot(Pivot::RightCenter), "RigC");
    term.put_string([0, 0].pivot(Pivot::LeftBottom), "BotL");
    term.put_string([0, 0].pivot(Pivot::CenterBottom), "BotC");
    term.put_string([0, 0].pivot(Pivot::RightBottom), "BotR");
}
