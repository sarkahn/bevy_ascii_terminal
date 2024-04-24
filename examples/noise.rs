use bevy::{app::AppExit, prelude::*, time::common_conditions::on_timer};
use bevy_ascii_terminal::*;
use fastnoise_lite::*;

fn main() {
    let controls = Controls {
        current: 0,
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
        .add_plugins((DefaultPlugins, TerminalPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_key_repeat.run_if(on_timer(key_repeat)),
                handle_other_input,
                draw_controls,
                make_some_noise,
            )
                .chain(),
        )
        .run()
}

#[derive(Component)]
pub struct ControlsTerminal;

fn setup(mut commands: Commands) {
    commands.spawn(TerminalCameraBundle::auto());

    commands.spawn(
        TerminalBundle::new([80, 60])
            .with_border(Border::single_line())
            .with_mesh_pivot(Pivot::TopLeft),
    );
    commands.spawn((
        TerminalBundle::new([30, 30])
            .with_border(Border::single_line())
            .with_mesh_pivot(Pivot::TopRight),
        ControlsTerminal,
    ));
}

pub struct Control {
    name: String,
    value: f32,
    step: f32,
}

#[derive(Resource)]
struct Controls {
    current: usize,
    noise_type: NoiseType,
    fractal_type: FractalType,
    values: Vec<Control>,
}

fn handle_key_repeat(input: Res<ButtonInput<KeyCode>>, mut controls: ResMut<Controls>) {
    let hor = input.pressed(KeyCode::KeyD) as i32 - input.pressed(KeyCode::KeyA) as i32;
    if hor != 0 {
        let curr = controls.current;
        let step = controls.values[curr].step;
        controls.values[curr].value += step * hor as f32;
    }
}

fn handle_other_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controls: ResMut<Controls>,
    mut evt_quit: EventWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Escape) {
        evt_quit.send(AppExit);
    }
    let ver = input.just_pressed(KeyCode::KeyS) as i32 - input.just_pressed(KeyCode::KeyW) as i32;
    if ver != 0 {
        let mut value = controls.current as i32;
        value = (value + ver).rem_euclid(controls.values.len() as i32);
        controls.current = value as usize;
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

fn draw_controls(
    mut q_term: Query<&mut Terminal, With<ControlsTerminal>>,
    controls: Res<Controls>,
) {
    if !controls.is_changed() {
        return;
    }

    let mut term = q_term.single_mut();
    term.clear();
    term.put_string([0, 0], "WASD to change noise values");
    term.put_string([0, 1], "Space to change noise type");
    term.put_string([0, 2], "Tab to change fractal type");
    term.put_string([0, 3], "Escape to quit");
    term.put_string([0, 4], "------------------------------");
    for (i, control) in controls.values.iter().enumerate() {
        let value = (control.value * 1000.0).round() / 1000.0;
        let control_string = if i == controls.current {
            format!("{}: {} <--", control.name, value)
        } else {
            format!("{}: {}", control.name, value)
        };
        term.put_string([0, i + 5], &control_string);
    }
}

fn make_some_noise(
    mut q_term: Query<&mut Terminal, Without<ControlsTerminal>>,
    controls: Res<Controls>,
) {
    if !controls.is_changed() {
        return;
    }

    let mut term = q_term.single_mut();
    term.border_mut().put_title(
        format!(
            "Noise:{:?}---Fractal:{:?}",
            controls.noise_type, controls.fractal_type
        )
        .fg(Color::ANTIQUE_WHITE)
        .bg(Color::GRAY)
        .ignore_spaces(),
    );
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(controls.noise_type));
    noise.set_fractal_type(Some(controls.fractal_type));

    noise.set_seed(Some(controls.values[0].value as i32));
    noise.set_fractal_octaves(Some(controls.values[1].value as i32));
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
        t.bg_color.set_l(noise);
    }
}
