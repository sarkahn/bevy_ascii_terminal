![](images/title.png)

A simple ascii terminal integrated into bevy's ecs framework. The api was designed to be as simple and straightforward as possible. 

You can set a single tile or group of tiles to any ascii characters and can set foreground and background colors per tile. Some of the important ones:

`clear()`: Clear the terminal to default tiles (empty tiles with a black background)

`put_char(x, y, char)`: Write an ascii character to a certain tile.

`put_string(x, y, string)`: Write a string to the terminal.

`put_char_color(x, y, string, fg_color, bg_color`: Write an ascii character with the given foreground and background colors.

`draw_box_single(x, y, width, height)`: Draw a box with a single-line-border.

`draw_border_single()`: Draw a single-line-border around the entire terminal.

`clear_box(x, y, width, height)`: Clear the specified area of the terminal to default tiles.


**Note:** For all functions the y coordinate is flipped in the terminal, so `put_char(0,0,'x')` refers to the top left corner of the terminal, and `put_char(width - 1, height - 1, 'x')` refers to the bottom right corner. This was done because the terminal is meant to display readable text from left to right, top to bottom.

## Dependencies

This project is fully integrated with [bevy](https://bevyengine.org/) and depends on it for rendering. 

While not a direct dependency, I would *highly* recommend importing [bevy_pixel_camera](https://crates.io/crates/bevy_pixel_camera) when you use this terminal. Bevy's default camera isn't set up to properly handle scaling up very low resolution pixel art (like terminal glyphs) without noticable artifacts due to pixels being misaligned. All the examples from this crate use bevy_pixel_camera.

## Getting Started

First add the plugin and spawn a bundle with a camera. 

```rust
fn spawn_terminal(mut commands: Commands) {
    let (w, h) = (80, 25);
    let mut term_bundle = TerminalBundle::with_size(w, h);
    term_bundle.terminal.put_char(0,0, "Hello, world!");
    commands.spawn_bundle(term_bundle);

    // 12 is the size of the default terminal font. This setting 
    // will ensure the camera scales up the viewport so the 
    // terminal takes up as much space as possible while still 
    // remaining pixel-perfect
    commands.spawn_bundle(PixelCameraBundle::from_resolution(
        w as i32 * 12, 
        h as i32 * 12,
    ));
}

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .add_plugin(PixelCameraPlugin)
    .add_plugin(PixelBorderPlugin{ color: Color::BLACK }
    .add_startup_system(spawn_terminal.system())
    .run()
}
```

You can write to the terminal when you create it, or from a query:

```rust
fn write_to_terminal(mut q: Query<&mut Terminal>) {
    let term = q.single_mut().unwrap();
    term.clear();
    term.draw_border();
    term.put_char(1,1, '☺');
    term.put_string(2,1, "451");
}
```

You can check [the examples](https://github.com/sarkahn/bevy_ascii_terminal/tree/main/examples) for more.

## How It Works
At it's lowest level the terminal renderer builds a dynamic mesh that bevy renders. It should be fast enough to clear and re-write to every frame, and it won't rebuild the mesh unless you make a change to the terminal.

## Other Features

There are several components you can modify to alter how the terminal is drawn. To name a few:

`Transform`: The terminal is a normal bevy entity, you can move and transform it at will.

`TerminalTileScaling`: Alters how tiles are sized when building the mesh.

`TerminalFont`: Changes which texture the terminal is using to draw glyphs.

`TerminalPivot`: Determines how the terminal mesh is aligned. Defaults to (0.5,0.5) which will nicely center the terminal mesh regardless of size.

`TilePivot`: Similar to `TerminalPivot` but for tiles - this defaults to (0.0,0.0) so a tile's bottom left corner sits on the pivot. 

## Feedback

This is my first real rust project - I'm open to any and all [feedback](https://github.com/sarkahn/bevy_ascii_terminal/issues), particularly with regards to improving the api or project structure. Thanks!