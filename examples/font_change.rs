use bevy::{
    prelude::*,
    reflect::enums::{DynamicVariant, Enum},
    window::WindowMode,
};
use bevy_ascii_terminal::{color::ColorPalette, *};

// Define custom named colors for the tagged strings
const COLORS: &[(&str, LinearRgba)] = &[
    ("BORDER", color::from_hex_string("#f0f0f0")),
    ("BG1", color::from_hex_string("#1920b0")),
    ("BG2", color::from_hex_string("#3030b0")),
    ("BG3", color::from_hex_string("#2828a8")),
    ("BG4", color::from_hex_string("#2020a0")),
    ("TITLE", color::from_hex_string("#f80505")),
    ("HEADER", color::from_hex_string("#f8f8f8")),
    ("TEXT1", color::from_hex_string("#ffffff")),
    ("TEXT2", color::from_hex_string("#acfa70")),
    ("TEXT3", color::from_hex_string("#00d493")),
    ("TEXT4", color::from_hex_string("#00bca1")),
];

const BORDER_COL: LinearRgba = color::from_hex_string("#f8f8f8");
const BG_COL: LinearRgba = color::from_hex_string("#3838b8");

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, (input, update))
        .run();
}

fn setup(mut commands: Commands) {
    let palette = ColorPalette::from_colors(COLORS.iter().cloned());

    let big_string = r#"
<fg=TEXT1><bg=BG1>.☺☻♥♦♣♠•◘○◙♂♀♪♫☼ ►◄↕‼¶§▬↨↑↓→←∟↔▲▼
!"\#$%&'()*+,-./ 0123456789:;/<=>?
<fg=TEXT2>@ABCDEFGHIJKLMNO PQRSTUVWXYZ[\]^_
`abcdefghijklmno pqrstuvwxyz{|}~⌂
<fg=TEXT3><bg=BG2>ÇüéâäàåçêëèïîìÄÅ ÉæÆôöòûùÿÖÜ¢£¥₧ƒ
áíóúñÑªº¿⌐¬½¼¡«» ░▒▓│┤╡╢╖╕╣║╗╝╜╛┐
<fg=TEXT4>└┴┬├─┼╞╟╚╔╩╦╠═╬╧ ╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀
αßΓπΣσµτΦΘΩδ∞φε∩ ≡±≥≤⌠⌡÷≈°∙·√ⁿ²■□"#;

    let term = Terminal::new([35, 14])
        .with_bg_clear_color(BG_COL)
        .with_border(BoxStyle::SINGLE_LINE)
        .with_color_palette(palette)
        .with_string(
            [0, 0],
            "<fg=HEADER>Press spacebar to change fonts.
            The quick brown fox jumps over the lazy dog.",
        )
        .with_string([0, 3], big_string);
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
            WindowMode::Windowed => WindowMode::BorderlessFullscreen(MonitorSelection::Current),
            WindowMode::BorderlessFullscreen(_) => WindowMode::Windowed,
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
        term.put_border(BoxStyle::SINGLE_LINE.with_fg_color(BORDER_COL));
        term.put_title(format!(
            " <fg=BORDER>[<fg=TITLE>{}<fg=BORDER>]",
            font.variant_name()
        ));
    }
}
