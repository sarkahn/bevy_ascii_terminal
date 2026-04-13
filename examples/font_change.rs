use bevy::{
    prelude::*,
    reflect::enums::{DynamicVariant, Enum},
    window::WindowMode,
};
use bevy_ascii_terminal::*;

const BORDER_COL: &str = "#f0f0f0";
const TITLE_COL: &str = "#f80505";
const BG_COLS: &[&str] = &["#3838b8", "#3030b0", "#2828a8", "#2020a0"];
const TEXT_COLS: &[&str] = &["#f8f8f8", "#ffffff", "#acfa70", "#00d493", "#00bca1"];

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, (input, update))
        .run();
}

fn setup(mut commands: Commands) {
    let bg_col =
        color::parse_color_string(BG_COLS[0]).expect("Color string should be in hex format");

    let big_string = format!(
        r#"
<fg={0}><bg={4}>.☺☻♥♦♣♠•◘○◙♂♀♪♫☼ ►◄↕‼¶§▬↨↑↓→←∟↔▲▼
!"\#$%&'()*+,-./ 0123456789:;/<=>?
<fg={1}>@ABCDEFGHIJKLMNO PQRSTUVWXYZ[\]^_
`abcdefghijklmno pqrstuvwxyz{{|}}~⌂
<fg={2}><bg={5}>ÇüéâäàåçêëèïîìÄÅ ÉæÆôöòûùÿÖÜ¢£¥₧ƒ
áíóúñÑªº¿⌐¬½¼¡«» ░▒▓│┤╡╢╖╕╣║╗╝╜╛┐
<fg={3}>└┴┬├─┼╞╟╚╔╩╦╠═╬╧ ╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀
αßΓπΣσµτΦΘΩδ∞φε∩ ≡±≥≤⌠⌡÷≈°∙·√ⁿ²■□"#,
        TEXT_COLS[1], TEXT_COLS[2], TEXT_COLS[3], TEXT_COLS[4], BG_COLS[2], BG_COLS[3]
    );

    let size = [36, 14];
    let term = Terminal::new(size)
        .with_bg_clear_color(bg_col)
        .with_border(BoxStyle::SINGLE_LINE)
        .with_string(
            [0, 0],
            format!(
                "<fg={}>Press spacebar to change fonts.
            The quick brown fox jumps over the lazy dog.",
                TEXT_COLS[0],
            ),
        )
        .with_string([0, 3], big_string.as_str());
    commands.spawn(term);
    commands.spawn(TerminalCamera::new());
}

fn input(
    input: Res<ButtonInput<KeyCode>>,
    mut q_term: Query<&mut TerminalFont>,
    mut exit: MessageWriter<AppExit>,
    mut window: Single<&mut Window>,
) {
    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    if input.just_pressed(KeyCode::KeyF) {
        window.mode = match window.mode {
            bevy::window::WindowMode::Windowed => {
                WindowMode::BorderlessFullscreen(MonitorSelection::Current)
            }
            bevy::window::WindowMode::BorderlessFullscreen(_) => WindowMode::Windowed,
            _ => window.mode,
        };
    }
    if input.just_pressed(KeyCode::Space) {
        let mut font = q_term.single_mut().unwrap();
        let info = font
            .get_represented_type_info()
            .expect("Error getting terminal font enum info");
        let info = match info {
            bevy::reflect::TypeInfo::Enum(info) => info,
            _ => unreachable!(),
        };
        // Exclude custom variant
        let max = info.variant_len() - 2;
        let i = font.variant_index();
        let i = (i + 1).rem_euclid(max);
        let mut dynamic = font.to_dynamic_enum();
        dynamic.set_variant_with_index(i, info.variant_names()[i], DynamicVariant::Unit);
        font.apply(&dynamic);
    }
}

fn update(mut q_term: Query<(&mut Terminal, &TerminalFont), Changed<TerminalFont>>) {
    if let Ok((mut term, font)) = q_term.single_mut() {
        let title_col = TITLE_COL;
        let bracket_col = BORDER_COL;
        let border_col = color::parse_color_string(bracket_col).unwrap();

        let new_string = format!(
            " <fg={0}>[</fg><fg={1}>{2}</fg><fg={0}>]</fg>",
            bracket_col,
            title_col,
            font.variant_name()
        );
        term.put_border(BoxStyle::SINGLE_LINE.with_fg_color(border_col));
        term.put_title(new_string.as_str());
    }
}
