use bevy::prelude::*;
use bevy_ascii_terminal::ui::*;
use bevy_ascii_terminal::*;

#[derive(Component)]
pub struct ProgressBar {
    pos: IVec2,
    size: usize,
    value: i32,
    ui: UiProgressBar,
    name: String,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(spawn_terminal)
        .add_system(draw_bars)
        .run()
}

fn spawn_terminal(mut commands: Commands) {
    let mut term = Terminal::with_size([50, 20]);

    draw_boxes(&mut term);

    commands
        .spawn_bundle(TerminalBundle::from(term))
        .insert(AutoCamera);

    let initial_value = 0;
    let max = 100;
    commands.spawn().insert(ProgressBar {
        pos: IVec2::from([0, 10]),
        size: 15,
        value: initial_value,
        ui: UiProgressBar::transition_bar(initial_value, max)
            .color_fill(ColorFill::EmptyOrFilled(Color::GRAY, Color::BLUE)),
        name: "Transition".to_string(),
    });

    commands.spawn().insert(ProgressBar {
        pos: IVec2::from([0, 14]),
        size: 15,
        value: initial_value,
        ui: UiProgressBar::new(initial_value, max),
        name: "Default".to_string(),
    });
}

fn draw_boxes(term: &mut Terminal) {
    term.draw_box([0, 0], [20, 5], &UiBox::single_line());
    term.put_string([2, 5], "Single line box");

    term.draw_box([22, 0], [20, 5], &UiBox::double_line());
    term.put_string([24, 5], "Double line box");
}

fn draw_bars(time: Res<Time>, mut term_q: Query<&mut Terminal>, mut q: Query<&mut ProgressBar>) {
    let mut term = term_q.single_mut();

    for mut bar in q.iter_mut() {
        let t = time.time_since_startup().as_secs_f32() * 15.0;
        let t = clamp_reverse_repeat(t, 101.0);
        let val = t.round() as i32;
        bar.value = val;
        bar.ui.set_value(val);
        term.draw_progress_bar(bar.pos, bar.size, &bar.ui);

        term.put_string(
            bar.pos + IVec2::new(bar.size as i32 + 2, 0),
            &format!(
                "{} {}",
                bar.ui.value().to_string(),
                bar.ui.value_normalized().to_string()
            ),
        );
        term.put_string(bar.pos + IVec2::new(0, 1), &bar.name);
    }
}

/// Loops the value between 0 and max
#[inline]
pub fn repeat(val: f32, max: f32) -> f32 {
    f32::clamp(val - (val / max).floor() * max, 0.0, max)
}

#[inline]
/// Clamps the value from 0 to max, then max to 0, repeating.
pub fn clamp_reverse_repeat(val: f32, max: f32) -> f32 {
    let t = repeat(val, max * 2.0);
    max - (t - max).abs()
}
