![](images/title.png)

A simple ascii terminal integrated into bevy's ecs framework. The api was designed to be as simple and straightforward as possible. 

You can set a single tile or group of tiles to any ascii characters and can set foreground and background colors per tile. The main functions of the terminal are:

`clear`: Clear the terminal to default tiles (empty tiles with a black background)

`put_char`: Write an ascii character to a certain tile

`put_string`: Write a string to a set of tiles

`put_char_color`: Write an ascii character with the given foreground and background colors

`clear_box`: Clear the specified area of the terminal to default tiles

`draw_box_single`: Draw a box with a single-line-border

`draw_border_single`: Draw a single-line-border around the entire terminal.

## Dependencies

This project is fully integrated with [bevy](https://bevyengine.org/) and depends on it for rendering. 

While not a direct dependency, I would *highly* recommend importing [bevy_pixel_camera](https://crates.io/crates/bevy_pixel_camera) when you use this terminal. Bevy's default camera isn't set up to properly handle scaling up very low resolution pixel art (like terminal glyphs) without noticable artifacts due to pixels being misaligned. All the examples from this crate use bevy_pixel_camera.

## Getting Started

First add the plugin and spawn a bundle with a camera. 

```rust
fn spawn_terminal(mut commands: Commands) {
    let (w, h) = (80, 25);
    commands.spawn_bundle(TerminalBundle::with_size(w, h));

    commands.spawn_bundle(PixelCameraBundle::from_resolution(
        w as i32 * 12, // 12 is the size of the default font
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

Then you can write to the terminal from a query:

```rust
fn write_to_terminal(mut q: Query<&mut Terminal>) {
    let term = q.single_mut().unwrap();
    term.clear();
    term.draw_border();
    term.put_char(1,0, '☺');
    term.put_string(2,1, "451");
}
```

There are several examples in the crate demonstrating usage.

## How It Works
At it's lowest level the terminal renderer builds a dynamic mesh that bevy renders. It should be fast enough to clear and re-write to every frame, and it won't rebuild the mesh unless you make a change to the terminal.

## Other Features

There are several components you can modify to alter how the terminal is drawn. To name a few:

`Transform`: The terminal is a normal bevy entity, you can move and transform it at will.

`TerminalTileScaling`: Alters how tiles are sized when building the mesh.

`TerminalFont`: Changes which texture the terminal is using to draw glyphs.

`TerminalPivot`: Normalized value that gets offset from the terminal mesh when it gets built. This defaults to (0.5,0.5), meaning the mesh will be nicely centered on the screen regardless of size if it and the camera are at the same position.

`TilePivot`: Similar to `TerminalPivot` but for tiles - this defaults to (0.0,0.0) so a tile's bottom left corner sits on the pivot. 


## Feedback

This is my first real rust project - I'm open to any and all feedback, particularly with regards to improving the api or project structure. Thanks!