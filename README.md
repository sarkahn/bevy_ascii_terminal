[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/bevy_ascii_terminal)](https://crates.io/crates/bevy_ascii_terminal/)
[![docs](https://docs.rs/bevy_ascii_terminal/badge.svg)](https://docs.rs/bevy_ascii_terminal/)

# `Bevy Ascii Terminal`

A simple ascii terminal integrated into bevy's ecs framework.

---
![](images/title.png)

---

The goal of this crate is to provide a simple, straightforward, and hopefully
fast method for rendering colorful ascii in bevy. It was made with "traditional
roguelikes" in mind, but should serve as a simple UI tool if needed.

# Example

```rust
use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn setup(mut commands: Commands) {
    // Create the terminal and write to it
    let mut terminal = Terminal::with_size([20,3]);
    terminal.draw_border(BorderGlyphs::single_line());
    // Draw a blue "Hello world!" to the terminal
    terminal.put_string([1, 1], "Hello world!".fg(Color::BLUE));

    // Spawn the terminal bundle from our terminal
    commands.spawn_bundle(TerminalBundle::from(terminal))
    // Automatically set up the camera for this terminal
    .insert(AutoCamera);
}

fn main () {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .add_startup_system(setup)
    .run();
}
```

## Versions
| bevy | bevy_ascii_terminal |
| --- | --- |
| 0.8 | 0.11 |
| 0.7 | 0.9 |