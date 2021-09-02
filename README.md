![](images/title.png)

A simple ascii terminal integrated into bevy's ecs framework.

The api was designed to be as simple and straightforward as possible. First add the plugin and spawn a bundle with a camera:

```rust
fn spawn_terminal(mut commands: Commands) {
    let mut term_bundle = TerminalBundle::with_size(10,3);
    term_bundle.terminal.put_string(1,1, "Hello");
    commands.spawn_bundle(term_bundle);

    let mut cam = OrthographicCameraBundle::new_2d();
    cam.orthographic_projection.scale = 1.0 / 2.0;
    commands.spawn_bundle(cam);
}

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .add_startup_system(spawn_terminal.system())
    .run()
}
```

Then you can write to the terminal from a query:

```rust
fn write_to_terminal(mut q: Query<&mut Terminal>) {
    let term = q.single_mut().unwrap();
    term.put_char(0,0, 'a');
}
```

You can change fonts at any time by setting the `TerminalRendererFont` component to the name of the font you want to load:
```rust
fn change_font(mut q: Query<&mut TerminalRendererFont>) {
    for font term in q.iter_mut() {
        font.0 = String::from("zx_evolution_8x8.png");
    }
}
```

By default the terminal will render such that each pixel in a tile equals one pixel in world space. This is in line with the default settings for bevy's orthographic camera. You can change it so that 1 tile will occupy 1 unit in world space by modifying the `TerminalTileScaling` component (this can be changed at runtime as well):

```rust
    let mut term_bundle = TerminalBundle::with_size(10,3);
    term_bundle.renderer.scaling = TerminalTileScaling::Window;
```

At it's lowest level the renderer builds a dynamic mesh that bevy renders any time the terminal changes. It should be fast enough to clear and re-write to every frame, and it won't rebuild the mesh unless you make a change to the terminal.