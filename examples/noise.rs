use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ascii_terminal::{
    render::TerminalTileScaling, terminal::Tile, Terminal, TerminalBundle, TerminalPlugin,
};

use bracket_noise::prelude::{FastNoise, NoiseType};
use bracket_random::prelude::*;

struct Noise {
    pub noise: FastNoise,
    types: Vec<NoiseType>,
    curr: usize,
    timer: f32,
}

impl Default for Noise {
    fn default() -> Self {
        let types = vec![
            NoiseType::Value,
            NoiseType::ValueFractal,
            NoiseType::Perlin,
            NoiseType::PerlinFractal,
            NoiseType::Simplex,
            NoiseType::SimplexFractal,
            NoiseType::Cellular,
            NoiseType::WhiteNoise,
            NoiseType::Cubic,
            NoiseType::CubicFractal,
        ];
        let default_type = NoiseType::Simplex;
        let i = types.iter().position(|&n| n == default_type).unwrap();
        Noise {
            noise: get_noise(types[i]),
            types,
            curr: i,
            timer: 0.0,
        }
    }
}

fn to_string(n: NoiseType) -> String {
    match n {
        NoiseType::Value => "Value".to_string(),
        NoiseType::ValueFractal => "Value Fractal".to_string(),
        NoiseType::Perlin => "Perlin".to_string(),
        NoiseType::PerlinFractal => "Perlin Fractal".to_string(),
        NoiseType::Simplex => "Simplex".to_string(),
        NoiseType::SimplexFractal => "Simplex Fractal".to_string(),
        NoiseType::Cellular => "Cellular".to_string(),
        NoiseType::WhiteNoise => "White Noise".to_string(),
        NoiseType::Cubic => "Cubic".to_string(),
        NoiseType::CubicFractal => "Cubic Fractal".to_string(),
    }
}

impl Noise {
    fn next(&mut self) {
        self.curr = (self.curr + 1) % self.types.len();
        self.noise = get_noise(self.types[self.curr]);
        self.timer = 0.0;
    }
}

fn get_noise(t: NoiseType) -> FastNoise {
    let mut noise = FastNoise::seeded(RandomNumberGenerator::new().next_u64());
    noise.set_noise_type(t);
    noise.set_frequency(0.1);
    noise
}

fn setup(mut commands: Commands) {
    let mut term = TerminalBundle::with_size(50, 50);
    term.renderer.scaling = TerminalTileScaling::Window;
    commands.spawn_bundle(term);

    let mut cam = OrthographicCameraBundle::new_2d();
    cam.orthographic_projection.scaling_mode = ScalingMode::FixedVertical;
    cam.orthographic_projection.scale = 25.0;
    cam.transform.translation += Vec3::new(25.0, 25.0, 0.0);

    commands.spawn_bundle(cam);
}

fn change_noise(keys: Res<Input<KeyCode>>, mut noise: ResMut<Noise>) {
    if keys.just_pressed(KeyCode::Space) {
        noise.next();
    }
}

fn noise(time: Res<Time>, mut noise: ResMut<Noise>, mut q: Query<&mut Terminal>) {
    let mut term = q.single_mut().unwrap();

    noise.timer += (time.delta().as_millis() as f32) / 1500.0;
    let t = noise.timer;

    noise.noise.set_frequency(t);

    let (width, height) = term.size;
    for (i, t) in term.iter_mut().enumerate() {
        let x = i % width;
        let y = i / height;

        let x = x as f32 - width as f32 / 2.0;
        let y = y as f32 - height as f32 / 2.0;

        let n = noise.noise.get_noise(x as f32, y as f32);
        let col = (n + 1.0) * 0.5;
        *t = Tile {
            glyph: '▒',
            fg_color: Color::rgb(col, col, col),
            bg_color: Color::BLACK,
        };
    }

    term.clear_box(0, 0, 30, 3);
    term.draw_box_single(0, 0, 30, 3);
    term.put_string(1, 1, "Press space to change noise");

    let t = noise.noise.get_noise_type();
    let string = to_string(t);
    let h = term.height() as i32;
    term.clear_box(0, h - 1, string.len(), 1);
    term.put_string(0, h - 1, &string);
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .init_resource::<Noise>()
        .add_startup_system(setup.system())
        .add_system(noise.system())
        .add_system(change_noise.system())
        .run();
}
