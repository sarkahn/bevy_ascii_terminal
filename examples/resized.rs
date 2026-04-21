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

fn update(mut q_term: Query<&mut Terminal>, time: Res<Time>) {
    let mut term = q_term.single_mut().unwrap();
    let size = (time.elapsed_secs().cos() * 20.0) as u32 % 20 + 15;
    term.resize([size, size]);

    term.clear();
    term.set_pivot(Pivot::LeftTop);
    term.put_string([0, 0], "TopL");

    term.set_pivot(Pivot::CenterTop);
    term.put_string([0, 0], "TopC");

    term.set_pivot(Pivot::RightTop);
    term.put_string([0, 0], "TopR");

    term.set_pivot(Pivot::LeftCenter);
    term.put_string([0, 0], "LefC");

    term.set_pivot(Pivot::Center);
    term.put_string([0, 0], "C");

    term.set_pivot(Pivot::RightCenter);
    term.put_string([0, 0], "RigC");

    term.set_pivot(Pivot::LeftBottom);
    term.put_string([0, 0], "BotL");

    term.set_pivot(Pivot::CenterBottom);
    term.put_string([0, 0], "BotC");

    term.set_pivot(Pivot::RightBottom);
    term.put_string([0, 0], "BotR");
}
