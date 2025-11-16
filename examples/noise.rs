//! An interactive ui to display noise using the fastnoise-lite crate.

use bevy::{app::AppExit, prelude::*, time::common_conditions::on_timer};
use bevy_ascii_terminal::*;
use fastnoise_lite::*;

fn main() {
    let controls = State {
        current_control: 0,
        noise_type: NoiseType::OpenSimplex2,
        fractal_type: FractalType::FBm,
        values: vec![
            Control {
                name: "Seed".to_string(),
                value: 0.0,
                step: 1.0,
            },
            Control {
                name: "Octaves".to_string(),
                value: 3.0,
                step: 1.0,
            },
            Control {
                name: "Frequency".to_string(),
                value: 0.1,
                step: 0.005,
            },
            Control {
                name: "Lacunarity".to_string(),
                value: 2.0,
                step: 0.02,
            },
            Control {
                name: "Gain".to_string(),
                value: 0.5,
                step: 0.01,
            },
            Control {
                name: "Weighted Strength".to_string(),
                value: 0.0,
                step: 0.03,
            },
        ],
    };
    let key_repeat = std::time::Duration::from_secs_f32(0.1);
    App::new()
        .insert_resource(controls)
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_key_repeat.run_if(on_timer(key_repeat)),
                handle_other_input,
                draw_controls.run_if(resource_changed::<State>),
                make_some_noise.run_if(resource_changed::<State>),
            )
                .chain(),
        )
        .run();
}

#[derive(Component)]
pub struct ControlsTerminal;

fn setup(mut commands: Commands) {
    commands.spawn((Terminal::new([80, 60]), TerminalMeshPivot::TopLeft));
    commands.spawn((
        Terminal::new([30, 30]),
        TerminalMeshPivot::TopRight,
        ControlsTerminal,
    ));
    commands.spawn(TerminalCamera::new());
}

pub struct Control {
    name: String,
    value: f32,
    step: f32,
}

#[derive(Resource)]
struct State {
    current_control: usize,
    noise_type: NoiseType,
    fractal_type: FractalType,
    values: Vec<Control>,
}

fn handle_key_repeat(input: Res<ButtonInput<KeyCode>>, mut controls: ResMut<State>) {
    let hor = input.pressed(KeyCode::KeyD) as i32 - input.pressed(KeyCode::KeyA) as i32;
    if hor != 0 {
        let curr = controls.current_control;
        let step = controls.values[curr].step;
        controls.values[curr].value += step * hor as f32;
    }
}

fn handle_other_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controls: ResMut<State>,
    mut evt_quit: MessageWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        evt_quit.write(AppExit::Success);
    }
    let ver = input.just_pressed(KeyCode::KeyS) as i32 - input.just_pressed(KeyCode::KeyW) as i32;
    if ver != 0 {
        let mut value = controls.current_control as i32;
        value = (value + ver).rem_euclid(controls.values.len() as i32);
        controls.current_control = value as usize;
    }
    if input.just_pressed(KeyCode::Tab) {
        let curr = controls.fractal_type;
        controls.fractal_type = match curr {
            FractalType::None => FractalType::FBm,
            FractalType::FBm => FractalType::Ridged,
            FractalType::Ridged => FractalType::PingPong,
            FractalType::PingPong => FractalType::None,
            _ => FractalType::FBm,
        };
    }

    if input.just_pressed(KeyCode::Space) {
        let curr = controls.noise_type;
        controls.noise_type = match curr {
            NoiseType::OpenSimplex2 => NoiseType::OpenSimplex2S,
            NoiseType::OpenSimplex2S => NoiseType::Cellular,
            NoiseType::Cellular => NoiseType::Perlin,
            NoiseType::Perlin => NoiseType::ValueCubic,
            NoiseType::ValueCubic => NoiseType::Value,
            NoiseType::Value => NoiseType::OpenSimplex2,
        };
    }
}

fn draw_controls(mut q_term: Query<&mut Terminal, With<ControlsTerminal>>, controls: Res<State>) {
    let mut term = q_term.single_mut().unwrap();
    term.clear();
    term.put_string([0, 0], "WASD to change noise values");
    term.put_string([0, 1], "Space to change noise type");
    term.put_string([0, 2], "Tab to change fractal type");
    term.put_string([0, 3], "Escape to quit");
    term.put_string([0, 4], "-----------------------------");
    for (i, control) in controls.values.iter().enumerate() {
        let value = (control.value * 1000.0).round() / 1000.0;
        let control_string = format!("{}: {}", control.name, value);
        term.put_string([0, i + 5], control_string.as_str());

        if i == controls.current_control {
            term.put_string(
                [control_string.len() + 1, i + 5],
                "<--".fg(LinearRgba::GREEN),
            );
        }
    }
}

fn make_some_noise(
    mut q_term: Query<&mut Terminal, Without<ControlsTerminal>>,
    controls: Res<State>,
) {
    let mut term = q_term.single_mut().unwrap();
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(controls.noise_type));
    noise.set_fractal_type(Some(controls.fractal_type));

    noise.set_seed(Some(controls.values[0].value as i32));
    noise.set_fractal_octaves(Some((controls.values[1].value as i32).max(1)));
    noise.set_frequency(Some(controls.values[2].value));
    noise.set_fractal_lacunarity(Some(controls.values[3].value));
    noise.set_fractal_gain(Some(controls.values[4].value));
    noise.set_fractal_weighted_strength(Some(controls.values[5].value));

    for (p, t) in term.iter_xy_mut() {
        let noise = noise.get_noise_2d(p.x as f32, p.y as f32);
        let noise = (noise + 1.0) / 2.0;
        let glyph = if noise < 0.25 {
            Glyph::ShadeLight
        } else if noise < 0.5 {
            Glyph::ShadeMedium
        } else if noise < 0.75 {
            Glyph::ShadeDark
        } else {
            Glyph::BlockFull
        };
        t.glyph = glyph.to_char();
        t.bg_color = Hsla::from(t.bg_color).with_lightness(noise).into();
    }
    term.put_string(
        [0, 0],
        format!(
            "[Noise:{:?} | Fractal:{:?}]",
            controls.noise_type, controls.fractal_type
        )
        .clear_colors(),
    );
}
