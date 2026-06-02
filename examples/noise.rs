//! An interactive ui to display noise using the fastnoise-lite crate.
use bevy::{
    app::AppExit,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    time::common_conditions::on_timer,
    window::WindowMode,
};
use bevy_ascii_terminal::*;
use fastnoise_lite::*;

#[derive(Component)]
pub struct ControlsTerminal;

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

impl State {
    fn value(&self, name: &str) -> Option<f32> {
        self.values
            .iter()
            .find(|v| v.name.eq_ignore_ascii_case(name))
            .map(|v| v.value)
    }
}

const DRIFT_INIT: Vec2 = Vec2::new(5.0, -1.0);
const COLOR_RAMP: &[LinearRgba] = &[
    color::from_hex_string("#000000"),
    color::from_hex_string("#000a0a"),
    color::from_hex_string("#001a0f"),
    color::from_hex_string("#003c1e"),
    color::from_hex_string("#006432"),
    color::from_hex_string("#228b22"),
    color::from_hex_string("#00c864"),
    color::from_hex_string("#00ff7f"),
    color::from_hex_string("#96ff96"),
];

const CHAR_RAMP2: &str = " .:-=+*#%@";
const CHAR_RAMP: &str = " .'`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

fn noise_value<T: Copy>(noise: f32, ramp: &[T]) -> T {
    let t = noise.clamp(0.0, 1.0);
    let i = (t * (ramp.len() - 1) as f32) as usize;
    ramp[i]
}

fn setup(mut commands: Commands) {
    assert!(CHAR_RAMP.is_ascii());
    assert!(CHAR_RAMP2.is_ascii());

    commands.spawn((Terminal::new([110, 60]), TerminalMeshPivot::LeftTop));
    commands.spawn((
        Terminal::new([28, 16]),
        SetTerminalGridPosition(IVec2::new(0, -1)),
        SetTerminalLayerPosition(1),
        TerminalMeshPivot::LeftTop,
        ControlsTerminal,
    ));
    commands.spawn(TerminalCamera::new());
}

fn make_some_noise(
    mut q_term: Query<&mut Terminal, Without<ControlsTerminal>>,
    controls: Res<State>,
    time: Res<Time>,
    mut drift: Local<Vec2>,
    diag: Res<DiagnosticsStore>,
) {
    let mut term = q_term.single_mut().unwrap();
    let mut noise = FastNoiseLite::new();
    noise.set_noise_type(Some(controls.noise_type));
    noise.set_fractal_type(Some(controls.fractal_type));

    noise.set_seed(controls.value("Seed").map(|v| v as i32));
    noise.set_fractal_octaves(controls.value("octaves").map(|v| v.max(1.0) as i32));
    noise.set_frequency(controls.value("frequency"));
    noise.set_fractal_lacunarity(controls.value("lacunarity"));
    noise.set_fractal_gain(controls.value("gain"));
    noise.set_fractal_weighted_strength(controls.value("weighted strength"));

    let xdrift = controls.value("driftx").expect("Driftx");
    let ydrift = controls.value("drifty").expect("Drifty");

    drift.x += xdrift * time.delta_secs();
    drift.y += ydrift * time.delta_secs();

    let scale = 11.0;

    for (p, t) in term.iter_xy_mut() {
        let dx = p.x as f32 * scale;
        let dy = p.y as f32 * scale * 0.5;

        let noise = noise.get_noise_2d(dx + drift.x, dy + drift.y);
        let noise = (noise + 1.0) / 2.0;

        let col = noise_value(noise, COLOR_RAMP);

        let glyph = if (p.x + p.y) % 2 == 0 {
            noise_value(noise, CHAR_RAMP.as_bytes())
        } else {
            noise_value(noise, CHAR_RAMP2.as_bytes())
        };

        t.glyph = glyph as char;

        t.fg_color = col;
    }
    term.put_string(
        [0, 0],
        format!(
            "[Noise:<fg=blue>{:?}</fg> | Fractal:<fg=blue>{:?}</fg>]",
            controls.noise_type, controls.fractal_type
        ),
    );

    let Some(fps) = diag
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
    else {
        return;
    };
    term.put_string(
        [0, 0].pivot(Pivot::RightTop),
        format!("FPS: <fg=yellow>{}", fps.round() as i32),
    );
}

fn handle_key_repeat(input: Res<ButtonInput<KeyCode>>, mut controls: ResMut<State>) {
    let right = [KeyCode::KeyD, KeyCode::ArrowRight];
    let left = [KeyCode::KeyA, KeyCode::ArrowLeft];

    let hor = input.any_pressed(right) as i32 - input.any_pressed(left) as i32;
    if hor != 0 {
        let curr = controls.current_control;
        let step = controls.values[curr].step;
        controls.values[curr].value += step * hor as f32;
    }
}

fn handle_other_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controls: ResMut<State>,
    mut exit: MessageWriter<AppExit>,
    mut window: Single<&mut Window>,
) {
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    let ver = input.any_just_pressed([KeyCode::KeyS, KeyCode::ArrowDown]) as i32
        - input.any_just_pressed([KeyCode::KeyW, KeyCode::ArrowUp]) as i32;
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
            FractalType::PingPong => FractalType::DomainWarpIndependent,
            FractalType::DomainWarpIndependent => FractalType::DomainWarpProgressive,
            FractalType::DomainWarpProgressive => FractalType::FBm,
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

    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    if input.just_pressed(KeyCode::KeyF) {
        if window.mode == WindowMode::BorderlessFullscreen(MonitorSelection::Current) {
            window.mode = WindowMode::Windowed;
        } else {
            window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        }
    }
}

fn draw_controls(mut q_term: Query<&mut Terminal, With<ControlsTerminal>>, controls: Res<State>) {
    let mut term = q_term.single_mut().unwrap();
    term.clear();
    term.put_string(
        [0, 0],
        "<fg=lawn_green>WASD/Arrows</fg> to change values
<fg=lawn_green>Space</fg> to change noise
<fg=lawn_green>Tab</fg> to change fractal
<fg=lawn_green>Escape</fg> to quit",
    );
    term.put_string([0, 4], "-----------------------");
    for (i, control) in controls.values.iter().enumerate() {
        let value = (control.value * 1000.0).round() / 1000.0;
        let control_string = format!("{}: <fg=blue>{}</fg>", control.name, value);
        term.put_string([0, i + 5], control_string.as_str());

        if i == controls.current_control
            && let Ok((_, len, _)) = strings::wrap_tagged_string(&control_string, 30, false)
        {
            term.put_string([len + 1, i + 5], "<fg=lime>/<--"); // Note the escape character for left brace
        }
    }
}

fn update_terminal_size(
    window: Single<&Window>,
    mut term: Single<&mut Terminal, Without<ControlsTerminal>>,
) {
    let res = window.physical_size().as_vec2();
    let max = (res / 8.0).floor().as_uvec2();

    if term.size() != max {
        term.resize(max);
    }
}

fn main() {
    let fnl = FastNoiseLite::default();
    let controls = State {
        current_control: 0,
        noise_type: NoiseType::OpenSimplex2,
        fractal_type: fnl.fractal_type,
        values: vec![
            Control {
                name: "Seed".to_string(),
                value: 0.0,
                step: 1.0,
            },
            Control {
                name: "Octaves".to_string(),
                value: fnl.octaves as f32,
                step: 1.0,
            },
            Control {
                name: "Frequency".to_string(),
                value: fnl.frequency,
                step: 0.0005,
            },
            Control {
                name: "Lacunarity".to_string(),
                value: fnl.lacunarity,
                step: 0.02,
            },
            Control {
                name: "Gain".to_string(),
                value: fnl.gain,
                step: 0.01,
            },
            Control {
                name: "Weighted Strength".to_string(),
                value: fnl.weighted_strength,
                step: 0.03,
            },
            Control {
                name: "DriftX".to_string(),
                value: DRIFT_INIT.x,
                step: 0.5,
            },
            Control {
                name: "DriftY".to_string(),
                value: DRIFT_INIT.y,
                step: 0.5,
            },
        ],
    };
    let key_repeat = std::time::Duration::from_secs_f32(0.1);
    App::new()
        .insert_resource(controls)
        .add_plugins((
            DefaultPlugins,
            TerminalPlugins,
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_key_repeat.run_if(on_timer(key_repeat)),
                handle_other_input,
                draw_controls.run_if(resource_changed::<State>),
                make_some_noise,
            )
                .chain(),
        )
        .add_systems(Update, update_terminal_size)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
