use bevy::{prelude::*, window::WindowMode};
use bevy_ascii_terminal::*;
use rand::{Rng, thread_rng};

const INITIAL_TICK_DELAY: f32 = 0.14;
const TICK_ACCELERATION: f32 = 0.005;
const MIN_TICK_DELAY: f32 = 0.1;

const HEAD_GRAPHICS: char = '@';
const BODY_GRAPHICS: char = '█';
const FOOD_GRAPHICS: char = '☼';

const GAME_AREA: UVec2 = UVec2::new(30, 18);

const HEAD_COLOR: LinearRgba = color::css::LIME;
const BODY_COLOR: LinearRgba = color::css::GREEN;
const FOOD_COLOR: LinearRgba = color::from_hex_string("#FFbb00");
const BORDER_COLOR: LinearRgba = color::css::DARK_GREEN;

#[derive(Resource, Default)]
pub struct Game {
    food: IVec2,
    snake: Snake,
    tick_delay: f32,
}

#[derive(Debug, States, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    Play,
    #[default]
    Start,
    Dead,
    Win,
}

#[derive(Default)]
pub struct Snake {
    segments: Vec<IVec2>,
    dir: IVec2,
    next_dir: IVec2,
}

impl Snake {
    pub fn head(&self) -> IVec2 {
        *self.segments.first().unwrap()
    }

    pub fn tail(&self) -> IVec2 {
        *self.segments.last().unwrap()
    }

    fn overlaps(&self, p: IVec2, include_head: bool) -> bool {
        let i = if include_head { 0 } else { 1 };
        self.segments[i..].contains(&p)
    }

    fn vroom(&mut self) {
        self.dir = self.next_dir;

        let next = self.head() + self.dir;

        for i in (1..self.segments.len()).rev() {
            self.segments[i] = self.segments[i - 1];
        }

        self.segments[0] = next;
    }
}

impl Game {
    fn place_food(&mut self) {
        let mut rng = thread_rng();
        let mut rand_point = || {
            let x = rng.gen_range(0..GAME_AREA.x) as i32;
            let y = rng.gen_range(0..GAME_AREA.y) as i32;
            IVec2::new(x, y)
        };
        let mut f = rand_point();
        while self.snake.overlaps(f, true) {
            f = rand_point();
        }
        self.food = f;
    }

    pub fn score(&self) -> usize {
        self.snake.segments.len() - 3
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, startup)
        .insert_state(GameState::Start)
        .insert_resource(Game::default())
        .add_systems(Update, input)
        .add_systems(Update, game_start.run_if(in_state(GameState::Start)))
        .add_systems(Update, draw.run_if(in_state(GameState::Play)))
        .add_systems(FixedUpdate, update.run_if(in_state(GameState::Play)))
        .add_systems(OnEnter(GameState::Play), reset)
        .add_systems(Update, dead.run_if(in_state(GameState::Dead)))
        .add_systems(Update, win.run_if(in_state(GameState::Win)))
        .insert_resource(Time::<Fixed>::from_seconds(INITIAL_TICK_DELAY as f64))
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}

fn game_start(
    input: Res<ButtonInput<KeyCode>>,
    mut term: Single<&mut Terminal>,
    mut commands: Commands,
) {
    term.clear();
    term.put_border(BoxStyle::SINGLE_LINE);
    term.put_title(" [<fg=lawn_green>Snake</fg>]");

    term.put_string(
        [0, 0].pivot(Pivot::Center),
        format!(
            "<fg={0}>WASD</fg> or <fg={0}>Arrows</fg> to move
         
        <fg={0}>F</fg> to toggle fullscreen
        
        <fg={0}>Esc</fg> to quit
        
        <fg={0}>Space</fg> to start",
            "lime"
        ),
    );

    if input.just_pressed(KeyCode::Space) {
        commands.set_state(GameState::Play);
    }
}

fn reset(mut commands: Commands, mut game: ResMut<Game>) {
    let head = (GAME_AREA / 2).as_ivec2();
    game.tick_delay = INITIAL_TICK_DELAY;
    game.snake.segments.clear();
    game.snake.segments.push(head);
    game.snake.segments.push(head - IVec2::new(1, 0));
    game.snake.segments.push(head - IVec2::new(2, 0));

    game.snake.dir = IVec2::new(1, 0);
    game.snake.next_dir = IVec2::new(1, 0);

    game.place_food();

    commands.insert_resource(Time::<Fixed>::from_seconds(game.tick_delay as f64));
}

fn input(
    mut window: Single<&mut Window>,
    mut exit: MessageWriter<AppExit>,
    input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
) {
    if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) && game.snake.dir.x != -1 {
        game.snake.next_dir = IVec2::new(1, 0);
    }
    if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) && game.snake.dir.x != 1 {
        game.snake.next_dir = IVec2::new(-1, 0);
    }
    if input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]) && game.snake.dir.y != -1 {
        game.snake.next_dir = IVec2::new(0, 1);
    }
    if input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]) && game.snake.dir.y != 1 {
        game.snake.next_dir = IVec2::new(0, -1);
    }

    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }

    if input.just_pressed(KeyCode::KeyF) {
        let fullscreen = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        let windowed = WindowMode::Windowed;
        window.mode = match window.mode {
            WindowMode::Windowed => fullscreen,
            _ => windowed,
        };
    }
}

fn update(mut commands: Commands, mut game: ResMut<Game>) {
    let tail = game.snake.tail();

    game.snake.vroom();

    if game.snake.head() == game.food {
        game.snake.segments.push(tail);

        game.place_food();

        if game.score() == 50 {
            commands.set_state(GameState::Win);
            return;
        }

        game.tick_delay = (game.tick_delay - TICK_ACCELERATION).max(MIN_TICK_DELAY);
        commands.insert_resource(Time::<Fixed>::from_seconds(game.tick_delay as f64));
    }

    let [x, y] = game.snake.head().to_array();
    let [xmax, ymax] = GAME_AREA.as_ivec2().to_array();
    let oob = x < 0 || x >= xmax || y < 0 || y >= ymax;
    let ate_urself = game.snake.overlaps(game.snake.head(), false);

    if oob || ate_urself {
        commands.set_state(GameState::Dead);
    }
}

fn dead(
    mut commands: Commands,
    game: Res<Game>,
    input: Res<ButtonInput<KeyCode>>,
    mut term: Single<&mut Terminal>,
) {
    draw_border(&mut term, game.score());

    term.put_string(
        [0, 0].pivot(Pivot::Center),
        "Game Over :(\nPress <fg=lime>Space</fg> to restart or <fg=lime>Escape</fg> to quit",
    );

    if input.pressed(KeyCode::Space) {
        commands.set_state(GameState::Play);
    }
}

fn draw(game: Res<Game>, mut term: Single<&mut Terminal>) {
    draw_border(&mut term, game.score());

    term.set_pivot(Pivot::LeftBottom);
    for p in &game.snake.segments {
        if let Some(tile) = term.try_tile_mut(*p) {
            tile.glyph = BODY_GRAPHICS;
            tile.fg_color = BODY_COLOR;
        }
    }

    if let Some(t) = term.try_tile_mut(game.snake.head()) {
        t.glyph = HEAD_GRAPHICS;
        t.fg_color = HEAD_COLOR;
    }

    let t = term.tile_mut(game.food);
    t.glyph = FOOD_GRAPHICS;
    t.fg_color = FOOD_COLOR;
}

fn draw_border(term: &mut Terminal, score: usize) {
    term.clear();
    term.put_border(BoxStyle::SINGLE_LINE);
    term.put_title(" [<fg=lawn_green>Snake</fg>]");
    term.set_padding(Padding::ZERO);
    term.put_string(
        [1, 0].pivot(Pivot::RightTop),
        format!("Score: <fg=lawn_green>{}", score),
    );
    term.set_padding(Padding::ONE);
}

fn win(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    game: Res<Game>,
    mut term: Single<&mut Terminal>,
) {
    draw_border(&mut term, game.score());

    term.put_string([0,0].pivot(Pivot::Center),
        "You Win! Congratulation!\n\nYou Are A Super Player <fg=yellow>:)</fg>\n\nPress <fg=lime>Space</fg> to Restart"
    );

    if input.just_pressed(KeyCode::Space) {
        commands.set_state(GameState::Play);
    }
}

fn startup(mut commands: Commands) {
    commands.spawn(Terminal::new(GAME_AREA + 2).with_clear_tile(Tile::new(
        ' ',
        BORDER_COLOR,
        color::css::BLACK,
    )));

    commands.spawn(TerminalCamera::new());

    commands.set_state(GameState::Start);
}
