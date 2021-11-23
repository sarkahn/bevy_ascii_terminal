[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/bevy_ascii_terminal)](https://crates.io/crates/bevy_ascii_terminal/)
[![docs](https://docs.rs/bevy_ascii_terminal/badge.svg)](https://docs.rs/bevy_ascii_terminal/)

# `Bevy Ascii Terminal`

A simple ascii terminal integrated into bevy's ecs framework.

---
![](images/title.png)

---

The goal of this crate is to provide a simple, straightforward, and hopefully fast method for rendering crisp colorful ascii in bevy. It was made with "traditional roguelikes" in mind, but should serve as a simple UI tool if needed. 

## Rendering
In order to render the terminal you must add the `TerminalPlugin` via your bevy `App`. You then need a camera to display it. Though not a direct dependency, this crate uses [TiledCamera](https://crates.io/crates/bevy_tiled_camera) to render it's examples.

It's recommended to use this or [some other similar camera](https://crates.io/crates/bevy_pixel_camera) for rendering, as bevy's default orthographic camera is not a good fit for how the
terminal is displayed. 

## Example

```rs
use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;

fn setup(mut commands: Commands) {
    let size = (20, 3);

    let mut term_bundle = TerminalBundle::new().with_size(size);
    let terminal = &mut term_bundle.terminal;

    terminal.draw_border_single();
    terminal.put_string((1, 1), "Hello world!");

    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(TiledCameraBundle::new()
        .with_tile_count(size));
}

fn main () {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .add_plugin(TiledCameraPlugin)
    .insert_resource(ClearColor(Color::BLACK))
    .add_startup_system(setup.system())
    .run();
}
```

**You can check the [examples](examples) and the [documentation](https://docs.rs/bevy_ascii_terminal/) for more.**