use bevy::{app::AppExit, color::palettes::basic, prelude::*};
use bevy_ascii_terminal::*;

const FADED: f32 = 0.65;
const BRIGHT: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin::default()))
        .init_resource::<Terminals>()
        .add_systems(Startup, setup)
        .add_systems(Update, (input, on_update))
        .run();
}

#[derive(Resource, Default)]
struct Terminals(Vec<Entity>);

const TERM_STRINGS: &[&str] = &[
    "WASD to change size",
    "Tab to change active terminal",
    "Space to toggle border",
];

fn setup(mut commands: Commands, mut terminals: ResMut<Terminals>) {
    commands.spawn(TerminalCameraBundle::auto());

    let v = vec![
        TerminalBundle::from(make_terminal([TERM_STRINGS[0].len() + 4, 5], BRIGHT))
            .with_mesh_pivot(Pivot::BottomRight)
            .put_string([1, 1], TERM_STRINGS[0])
            .with_border(Border::single_line()),
        TerminalBundle::from(make_terminal([TERM_STRINGS[1].len() + 4, 7], FADED))
            .with_mesh_pivot(Pivot::BottomLeft)
            .put_string([1, 1], TERM_STRINGS[1])
            .with_border(Border::single_line()),
        TerminalBundle::from(make_terminal([TERM_STRINGS[2].len() + 4, 6], FADED))
            .with_mesh_pivot(Pivot::TopCenter)
            .put_string([0, 1].pivot(Pivot::TopCenter), TERM_STRINGS[2])
            .with_border(Border::single_line()),
    ];
    terminals.0 = Vec::from_iter(v.into_iter().map(|t| commands.spawn(t).id()));
}

fn make_terminal(size: impl GridPoint, lightness: f32) -> Terminal {
    let mut term = Terminal::new(size);
    set_brightness(&mut term, lightness);
    term
}

fn set_brightness(term: &mut Terminal, lightness: f32) {
    for (p, t) in term.iter_xy_mut() {
        let grid_color = if (p.x + p.y) % 2 == 0 {
            Hsla::from(basic::BLUE)
                .with_lightness(lightness - 0.5)
                .with_saturation(0.5)
        } else {
            Hsla {
                lightness: lightness - 0.5,
                saturation: 0.5,
                ..basic::RED.into()
            }
        };
        t.fg_color = Hsla::from(t.fg_color).with_lightness(lightness).into();
        t.bg_color = grid_color.into();
    }
}

fn on_update(
    mut q_term: Query<&mut Terminal>,
    input: Res<ButtonInput<KeyCode>>,
    terminals: Res<Terminals>,
    mut current: Local<usize>,
) {
    if input.just_pressed(KeyCode::Tab) {
        *current = (*current + 1) % terminals.0.len();
        for (i, e) in terminals.0.iter().enumerate() {
            let mut term = q_term.get_mut(*e).unwrap();
            let lightness = if *current == i { BRIGHT } else { FADED };
            set_brightness(&mut term, lightness)
        }
    }
    let mut term = q_term.get_mut(terminals.0[*current]).unwrap();

    let hor = input.just_pressed(KeyCode::KeyD) as i32 - input.just_pressed(KeyCode::KeyA) as i32;
    let ver = input.just_pressed(KeyCode::KeyW) as i32 - input.just_pressed(KeyCode::KeyS) as i32;

    if input.just_pressed(KeyCode::Space) {
        // match term.get_border() {
        //     Some(_) => term.set_border(None),
        //     None => {
        //         term.put_border(Border::single_line());
        //     }
        // };
    }

    let size = IVec2::new(hor, ver);
    if !size.cmpeq(IVec2::ZERO).all() {
        let curr = term.size();
        term.resize((curr + size).max(IVec2::ONE));
        let pivot = if *current == 2 {
            Pivot::TopCenter
        } else {
            Pivot::TopLeft
        };
        term.put_string([1, 1].pivot(pivot), TERM_STRINGS[*current]);
        set_brightness(&mut term, BRIGHT);
        term.put_border(Border::single_line());
    }
}

fn input(input: Res<ButtonInput<KeyCode>>, mut evt_quit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        evt_quit.send(AppExit::Success);
    }
}
