![](images/title.png)

A simple ascii terminal integrated into bevy's ecs framework. 

## What Is It

bevy_ascii_terminal is a utility to easily render colorful ascii in bevy. It was made  with "[traditional roguelikes](http://roguebasin.com/index.php/Main_Page)" in mind, but should serve effectively as a simple, no fuss UI tool if needed. The API is designed to be as simple and straightforward as possible. 

## What Can It Do

 It can set a single tile or group of tiles to any ascii character that's been mapped, and can set foreground and background colors per tile.
 
 As far as what it can render, it currently supports a fixed size tileset and by default expects a [code page 437 layout](https://en.wikipedia.org/wiki/Code_page_437) on the textures, though this can be changed via a configuration file. There are [plenty more fonts available](https://dwarffortresswiki.org/Tileset_repository) around the internet, and [changing fonts](#changing-fonts) is as simple as setting a string.

## Dependencies

This project is fully integrated with [bevy](https://bevyengine.org/) and depends on it for rendering. 

While not a direct dependency, I would *highly* recommend importing [bevy_pixel_camera](https://crates.io/crates/bevy_pixel_camera) when you use this terminal. Bevy's default camera isn't set up to properly handle scaling up very low resolution pixel art (like terminal glyphs) without noticable artifacts due to pixels being misaligned. All the examples from this crate use bevy_pixel_camera.

## Getting Started

Include [bevy_ascii_terminal](https://crates.io/crates/bevy_ascii_terminal) as a dependency in your `Cargo.toml`, then add the plugin and spawn a bundle with a camera. 

```rust
fn spawn_terminal(mut commands: Commands) {
    let (w, h) = (80, 25);
    let mut term_bundle = TerminalBundle::with_size(w, h);
    term_bundle.terminal.put_string(1,1, "Hello, world!");
    commands.spawn_bundle(term_bundle);

    // 12 is the size of the default terminal font. This setting 
    // will ensure the camera scales up the viewport so the 
    // terminal takes up as much space as possible on the screen
    // given it's current size, while still remaining pixel-perfect
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
    .insert_resource(ClearColor(Color::BLACK))
    .add_startup_system(spawn_terminal.system())
    .run()
}
```

And that's it. You can write to the terminal when you create it, or from a query:

```rust
fn write_to_terminal(mut q: Query<&mut Terminal>) {
    let term = q.single_mut().unwrap();
    term.clear();
    term.draw_border_single();
    term.put_char(1,1, '☺');
    term.put_string(2,1, "451");
}
```

You can check the [examples](examples) for more.

**Note:** For all terminal functions the y coordinate is flipped in the terminal, so `put_char(0,0,'x')` refers to the top left corner of the terminal, and `put_char(width - 1, height - 1, 'x')` refers to the bottom right corner. This was done because the terminal is meant to display readable text from left to right, top to bottom.

## Changing Fonts

If you want to change fonts, you should [find a new cp437 texture atlas online](https://dwarffortresswiki.org/Tileset_repository) or make one yourself, then put it into the assets/textures folder. To load your new font you just need to modify the `TerminalFont` component attached to the terminal:
```rust
fn change_font(
    keys: Res<Input<KeyCode>>,
    mut q: Query<&mut TerminalRendererFont>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for mut font in q.iter_mut() {
            font.font_name = String::from("zx_evolution_8x8.png");
            font.clip_color = Color::BLACK;
        }
    }
}
```

Notice the `clip_color` field above - this refers to the "background" color of the texture. The shader will swap this color out for whatever background color you set for a given tile in the terminal. For the included fonts, this should be black. Many fonts you'll find online use magenta as the background color.

## Other Features

There are several other components you can modify to alter how the terminal is drawn. To name a few:

`Transform`: The terminal is a normal bevy entity, you can move and transform it at will.

`TerminalTileScaling`: Alters how tiles are sized when building the mesh.

`TerminalPivot`: Determines how the terminal mesh is aligned. Defaults to (0.5,0.5) which will nicely center the terminal mesh regardless of size.

`TilePivot`: Similar to `TerminalPivot` but for tiles - this defaults to (0.0,0.0), so a tile's bottom left corner sits on the pivot. 

## Feedback

This is my first real rust project - I'm open to any and all [feedback and suggestions](https://github.com/sarkahn/bevy_ascii_terminal/issues), particularly with regards to improving the api or project structure. Thanks!