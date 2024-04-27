use bevy::{prelude::*, reflect::Enum};
use bevy_ascii_terminal::*;

fn main() {}

fn spawn_terminal(mut commands: Commands) {
    let size = [47, 13];
    let font = TerminalFont::default();
    let font_name = font.variant_name().to_uppercase().to_owned();
    let clear_tile = *Tile::default().fg(Color::WHITE).bg(Color::MIDNIGHT_BLUE);
    let term = TerminalBundle::new(size)
        //.with_border_title(Border::single_line(), &font_name)
        .with_clear_tile(clear_tile);
    // let mut term = Terminal::new(size)
    //     .with_clear_tile(' '.fg(Color::WHITE).bg(Color::MIDNIGHT_BLUE))
    //     .with_border(
    //         Border::single_line().with_title(
    //             font.variant_name()
    //                 .to_uppercase()
    //                 .aligned(0.1)
    //                 .fg_col(Color::RED),
    //         ),
    //     );

    // //draw_title(&mut term, font.variant_name());
    // term.put_string(
    //     [0, 1].pivot(Pivot::TopLeft),
    //     "Press spacebar to change fonts",
    // );
    // term.put_string([0, 3].pivot(Pivot::TopLeft), "!@#$%^&*()_+=-`~");
    // term.put_string(
    //     [0, 5].pivot(Pivot::TopLeft),
    //     "The quick brown fox jumps over the lazy dog.",
    // );
    // term.put_string(
    //     [0, 7].pivot(Pivot::TopLeft),
    //     "☺☻♥♦♣♠•'◘'○'◙'♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼",
    // );
    // term.put_string(
    //     [0, 9].pivot(Pivot::TopLeft),
    //     "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞",
    // );
    // commands.spawn((TerminalBundle::from(term), AutoCamera));
}
