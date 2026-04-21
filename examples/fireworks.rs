use std::f32;
use std::f32::consts::TAU;
use std::ops::RangeInclusive;

use bevy::math::ops::{powf, sin};
use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;

use enum_ordinalize::Ordinalize;
use rand::Rng;
use rand::seq::SliceRandom;

struct Rocket {
    color: LinearRgba,
    vel: Vec2,
    pos: Vec2,
    life: f32,
}

struct Particle {
    color: LinearRgba,
    vel: Vec2,
    pos: Vec2,
    life: f32,
}

struct Star {
    pos: IVec2,
    phase: f32,
}

#[derive(Resource, Default)]
struct State {
    rockets: Vec<Rocket>,
    particles: Vec<Particle>,
    stars: Vec<Star>,
    term_scale: u32,
    last_scale: u32,
    last_size: Vec2,
    show_text: bool,
}

#[derive(Reflect, Ordinalize)]
enum Shape {
    Ring,
    DoubleRing,
    Burst,
    Star,
    Spiral,
    Willow,
    SawtoothSpiral,
}

const COLORS: &[LinearRgba] = &[
    color::from_hex_string("#ff5050"),
    color::from_hex_string("#ff8c60"),
    color::from_hex_string("#ffc850"),
    color::from_hex_string("#50c8ff"),
    color::from_hex_string("#78a0ff"),
    color::from_hex_string("#8c78ff"),
    color::from_hex_string("#c878ff"),
    color::from_hex_string("#78ff8c"),
    color::from_hex_string("#50ffc8"),
    color::from_hex_string("#ff78b4"),
    color::from_hex_string("#ffa0ff"),
    color::from_hex_string("#ffffff"),
    color::from_hex_string("#c8dcff"),
];

const GRAVITY: f32 = 9.8;
const STAR_DENSITY: u32 = 16;
const ROCKET_DRIFT: f32 = 3.0;
const ROCKET_VEL_RANGE: RangeInclusive<f32> = 30.0..=45.0;
const ROCKET_LIFE_RANGE: RangeInclusive<f32> = 1.5..=6.0;

const PARTICLE_COUNT_RANGE: RangeInclusive<i32> = 30..=160;

fn main() {
    let state = State {
        rockets: Vec::with_capacity(150),
        particles: Vec::with_capacity(15000),
        stars: Vec::with_capacity(3000),
        term_scale: 1,
        last_scale: 0,
        last_size: Vec2::ZERO,
        show_text: true,
    };
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, init)
        .add_systems(Update, handle_input)
        .add_systems(FixedUpdate, fixed_update)
        .add_systems(Update, draw)
        .insert_resource(state)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

fn init_stars(stars: &mut Vec<Star>, max: IVec2) {
    let mut rng = rand::thread_rng();
    let count = max.element_product() / STAR_DENSITY as i32;
    for _ in 0..count {
        stars.push(Star {
            pos: IVec2::new(
                rng.gen_range(0..=max.x) as i32,
                rng.gen_range(0..=max.y) as i32,
            ),
            phase: rng.gen_range(0.0..=1.0) * TAU,
        })
    }
}

fn spawn_rocket(max: Vec2) -> Rocket {
    let mut rng = rand::thread_rng();

    let hor = rng.gen_range(-ROCKET_DRIFT..=ROCKET_DRIFT);
    let ver = rng.gen_range(ROCKET_VEL_RANGE);

    let x: f32 = rng.gen_range(0.0..=max.x);
    let y: f32 = 0.0;

    let life: f32 = rng.gen_range(ROCKET_LIFE_RANGE);

    Rocket {
        color: *COLORS.choose(&mut rng).unwrap(),
        pos: Vec2::new(x, y),
        vel: Vec2::new(hor, ver),
        life,
    }
}

fn explode(particles: &mut Vec<Particle>, xy: Vec2, up_vel: f32, base_color: LinearRgba) {
    let mut rng = rand::thread_rng();

    let count = rng.gen_range(PARTICLE_COUNT_RANGE);
    let rot = rng.gen_range(0.0..=TAU);

    let shape = Shape::VARIANTS.choose(&mut rng).unwrap();

    for i in 0..count {
        if particles.len() >= particles.capacity() {
            break;
        }

        let t = i as f32 / count as f32;
        let mut angle = t as f32 * TAU;
        let speed: f32;

        match shape {
            Shape::Ring => {
                speed = 10.0;
                angle += rot;
            }
            Shape::DoubleRing => {
                angle += rot;
                speed = if rng.gen_range(0..=100) < 50 {
                    6.0
                } else {
                    12.0
                };
            }
            Shape::Burst => speed = rng.gen_range(4.0..=16.0),
            Shape::Star => {
                let spikes = 5.0;
                let spike = powf((angle * spikes).sin(), 3.0);
                speed = 8.0 + spike * 4.0;
                angle += rot;
            }
            Shape::Spiral => {
                let turns = 2.5;
                angle = t * turns * TAU;
                speed = 4.0 + t * 10.0;
                angle += rot;
            }
            Shape::Willow => speed = rng.gen_range(2.0..=8.0),
            Shape::SawtoothSpiral => {
                let spikes = 5.0;
                let spike = 2.0 * (t * spikes - (0.5 + t * spikes).floor());
                speed = 8.0 + spike * 4.0;
                angle += rot;
            }
        }

        let dir = Vec2::new(angle.cos(), angle.sin());
        let p = Particle {
            color: base_color,
            life: rng.gen_range(1.0..=3.0),
            vel: Vec2::new(dir.x * speed, dir.y * speed + up_vel * 0.5),
            pos: xy,
        };
        particles.push(p);
    }
}

fn init(mut commands: Commands) {
    commands.spawn(Terminal::new([120, 70]));
    commands.spawn(TerminalCamera::new());
}

fn handle_input(
    mut exit: MessageWriter<AppExit>,
    mut window: Single<&mut Window>,
    input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<State>,
    mut term: Single<&mut Terminal>,
) {
    if input.just_pressed(KeyCode::KeyH) {
        state.show_text = !state.show_text;
    }

    if input.just_pressed(KeyCode::KeyF) {
        let fullscreen = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        let windowed = WindowMode::Windowed;
        window.mode = match window.mode {
            WindowMode::Windowed => fullscreen,
            _ => windowed,
        };
    }
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
        return;
    }

    if input.just_pressed(KeyCode::Equal) {
        state.term_scale += 1;
    }

    if input.just_pressed(KeyCode::Minus) {
        state.term_scale = state.term_scale.max(2) - 1;
    }

    if state.last_size != window.size() || state.last_scale != state.term_scale {
        state.last_size = window.size();
        state.last_scale = state.term_scale;

        let max = (window.size() / (Vec2::new(8.0, 8.0) * state.term_scale as f32)).floor();
        term.resize(max.as_uvec2());
        state.stars.clear();
        init_stars(
            &mut state.stars,
            IVec2::new(term.width() as i32 - 1, term.height() as i32 - 1),
        )
    }
}

fn fixed_update(mut state: ResMut<State>, time: Res<Time>, term: Single<&Terminal>) {
    let mut rng = rand::thread_rng();

    if rng.gen_range(0..=100) < 6 {
        let r = spawn_rocket(term.size().as_vec2());
        state.rockets.push(r);
    }

    let dt = time.delta_secs();
    let mut rng = rand::thread_rng();

    let mut i = state.rockets.len();
    while i > 0 {
        i -= 1;
        let particlecount = state.particles.len();
        let particlecap = state.particles.capacity();

        let r = &mut state.rockets[i];

        r.vel.y -= GRAVITY * dt;
        r.pos += r.vel * dt;
        r.life -= dt;

        let col = r.color;
        let vel = r.vel;
        let pos = r.pos;

        if r.life <= 0.0 {
            explode(&mut state.particles, pos, vel.y, col);
            state.rockets.swap_remove(i);
        }

        // Trail particles
        if particlecount < particlecap {
            let p = Particle {
                pos,
                vel: Vec2::new(
                    vel.x * rng.gen_range(0.25..=0.45),
                    vel.y * rng.gen_range(0.25..=0.45),
                ),
                life: 0.7,
                color: col,
            };
            state.particles.push(p);
        }
    }

    i = state.particles.len();
    while i > 0 {
        i -= 1;

        let p = &mut state.particles[i];

        p.vel.y -= GRAVITY * dt;
        p.pos += p.vel * dt;
        p.life -= dt;

        if p.life <= 0.0 {
            state.particles.swap_remove(i);
        }
    }
}

fn draw(mut term: Single<&mut Terminal>, state: Res<State>, time: Res<Time>) {
    term.clear();
    term.set_pivot(Pivot::LeftBottom);

    for p in &state.particles {
        let Some(t) = term.try_tile_mut(p.pos.as_ivec2()) else {
            continue;
        };
        let brightness = p.life.clamp(0.0, 1.0) * 255.0;

        let col = LinearRgba::new(
            p.color.red * brightness / 255.0,
            p.color.green * brightness / 255.0,
            p.color.blue * brightness / 255.0,
            1.0,
        );

        let ch = match brightness as u8 {
            181..=255 => '*',
            101..=180 => '+',
            40..=100 => '.',
            _ => ' ',
        };

        t.glyph = ch;
        t.fg_color = col;
    }

    for r in &state.rockets {
        let Some(t) = term.try_tile_mut(r.pos.as_ivec2()) else {
            continue;
        };
        t.glyph = if r.vel.y >= 0.0 { '^' } else { 'v' };
        t.fg_color = r.color;
    }

    let time = time.elapsed_secs();
    let max_height = term.height();

    for s in &state.stars {
        let wave = sin(time * 0.01 + s.phase);

        // only allow twinkle near the peak
        let mut twinkle = 0.0;
        if wave > 0.99 {
            twinkle = (wave - 0.98) / 0.02;
        }

        let mut brightness = 5 + (20.0 * (twinkle * twinkle)) as u8;

        // Pulse
        if twinkle > 0.998 {
            brightness = 105
        }

        let height_t = s.pos.y as f32 / max_height as f32;
        let brightness = (brightness as f32 * height_t.powf(0.25)) as u8;

        let col = color::srgba_bytes(
            brightness,
            brightness,
            brightness.saturating_add(50), // slight blue bias
            255,
        );

        let ch = if brightness < 100 { '.' } else { '+' };

        let Some(t) = term.try_tile_mut(s.pos) else {
            continue;
        };
        t.glyph = ch;
        t.fg_color = col;
    }

    if state.show_text {
        term.set_pivot(Pivot::LeftTop);

        #[rustfmt::skip]
        let keys_string = format!(
"[<fg={0}>+/-</fg>]: Zoom
[<fg={0}>H</fg>]: Toggle text
[<fg={0}>F</fg>]: Toggle fullscreen",
"dodger_blue" );
        term.put_string([0, 0], keys_string);

        term.set_pivot(Pivot::RightTop);
        term.put_string(
            [0, 0],
            format!(
                "Particles: {}",
                state.particles.len() + state.rockets.len() + state.stars.len()
            ),
        );
    }
}
