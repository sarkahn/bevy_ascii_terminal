use bevy::prelude::*;
use bevy_ascii_terminal::{*, ui::UiBox};
use bevy_tiled_camera::*;

use bracket_noise::prelude::{FastNoise, NoiseType};
use bracket_random::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_plugin(TiledCameraPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Noise>()
        .add_startup_system(setup)
        .add_system(noise)
        .add_system(change_noise)
        .run();
}

// Original example from the bracket noise library
// https://github.com/amethyst/bracket-lib/blob/master/bracket-noise/examples/value.rs

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
    let size = [40, 40];

    commands.spawn_bundle(TerminalBundle::new().with_size(size));
    commands.spawn_bundle(TiledCameraBundle::new().with_tile_count(size));
}

fn change_noise(keys: Res<Input<KeyCode>>, mut noise: ResMut<Noise>) {
    if keys.just_pressed(KeyCode::Space) {
        noise.next();
    }
}

fn noise(time: Res<Time>, mut noise: ResMut<Noise>, mut query: Query<&mut Terminal>) {
    for mut term in query.iter_mut() {
        noise.timer += (time.delta().as_millis() as f32) / 1500.0;
        let t = noise.timer;

        noise.noise.set_frequency(t);

        let (width, height) = term.size().into();
        let half_width = width as f32 / 2.0;
        let half_height = height as f32 / 2.0;
        for (i, t) in term.iter_mut().enumerate() {
            let x = (i % width as usize) as f32;
            let y = (i / width as usize) as f32;

            let x = x - half_width;
            let y = y - half_height;

            let noise = noise.noise.get_noise(x, y);
            let col = (noise + 1.0) * 0.5;
            *t = Tile {
                glyph: '▒',
                fg_color: Color::rgb(col, col, col),
                bg_color: Color::BLACK,
            };
        }

        term.clear_box([0, 0], [30, 3]);
        term.draw_box([0, 0], [30, 3], &UiBox::single_line());
        term.put_string([1, 1], "Press space to change noise");

        let t = noise.noise.get_noise_type();
        let string = to_string(t);
        let h = term.height();
        term.clear_box([0, h as i32 - 1], [string.len() as u32, 1]);
        term.put_string([0, h as i32 - 1], &string);
    }
}