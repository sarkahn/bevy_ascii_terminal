![](images/title.png)

A simple ascii terminal integrated into bevy's ecs framework.

The api was designed to be as simple and straightforward as possible. First add the plugin and spawn a bundle with a camera:

```rust
fn spawn_terminal(mut commands: Commands) {
    let mut term_bundle = TerminalBundle::with_size(10,3);
    term_bundle.terminal.put_string(1,1, "Hello");
    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
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
    for mut term in q.iter_mut() {
        term.put_char(0,0, 'a');
    }
}
```

You can change fonts at any time by setting the `TerminalRendererFont` component to the name of the font you want to load:
```
fn change_font(mut q: Query<&mut TerminalRendererFont>) {
    for font term in q.iter_mut() {
        font.0 = String::from("zx_evolution_8x8.png");
    }
}
```

At it's lowest level the renderer builds a dynamic mesh that bevy renders any time the terminal changes. It should be fast enough to clear and re-write to every frame, and it won't rebuild the mesh unless you make a change to the terminal.