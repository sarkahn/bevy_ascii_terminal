


fn main() {
    // App::new()
    // .add_plugins(DefaultPlugins)
    // .add_plugin(TerminalPlugin)
    // .add_startup_system(setup)
    // .run();
}

// fn setup(
//     mut commands: Commands
// ) {
//     let terms = [
//         Terminal::new([10,10])
//             .with_pivot(Pivot::BottomLeft)
//             .with_border(Border::SINGLE_LINE),
//         Terminal::new([10,10]).with_pivot(Pivot::TopLeft),
//         Terminal::new([10,10]).with_pivot(Pivot::BottomRight),
//         Terminal::new([10,10]).with_pivot(Pivot::TopRight),
//     ];

//     for (i, mut term) in terms.into_iter().enumerate() {
//         term.put_string([0,0], "Hello");
//         commands.spawn((
//             TerminalBundle::from(term).with_depth(i as i32),
//             AutoCamera
//         ));
//     }
// }