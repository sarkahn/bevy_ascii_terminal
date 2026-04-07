use bevy::{
    camera::{ScalingMode, Viewport},
    math::ops::powf,
    prelude::*,
    window::WindowMode,
};
use bevy_ascii_terminal::*;
use enum_ordinalize::Ordinalize;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, controls)
        //.add_systems(PostUpdate, draw_cursor.after(TransformSystems::Propagate))
        .add_systems(
            PostUpdate,
            fit_to_terminal.before(TransformSystems::Propagate),
        )
        .run();
}

fn setup(mut commands: Commands, window: Single<&Window>) {
    let window_size = window.resolution.physical_size().as_vec2();

    // Initialize centered, non-window-filling viewport
    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: (window_size * 0.125).as_uvec2(),
                physical_size: (window_size * 0.75).as_uvec2(),
                ..default()
            }),
            clear_color: ClearColorConfig::Custom(Color::linear_rgb(0.01, 0.01, 0.01)),
            ..default()
        },
    ));

    commands.spawn((
        Terminal::new([24, 10]).with_border(BoxStyle::SINGLE_LINE),
        TerminalMeshPivot::RightBottom,
    ));

    commands.insert_resource(TerminalMeshWorldScaling::World);
}

fn fit_to_terminal(
    mut term: Single<(&mut Terminal, &TerminalMeshPivot)>,
    mesh_scaling: Res<TerminalMeshWorldScaling>,
    mut q_cam: Single<(&mut Camera, &mut Projection, &mut Transform)>,
) {
    let (mut cam, mut proj, mut transform) = q_cam.into_inner();
    let (mut term, term_pivot) = term.into_inner();

    let term_size = term.size();
    let vp_size = cam.physical_viewport_size().unwrap();

    let pixels_per_tile = 8; // TODO: Derive from texture

    let tiles_fit = (vp_size.as_vec2() / pixels_per_tile as f32)
        .floor()
        .as_uvec2();

    let target_size = match mesh_scaling.as_ref() {
        TerminalMeshWorldScaling::World => term_size,
        TerminalMeshWorldScaling::Pixels => term_size * pixels_per_tile,
    };

    let max_scale = (vp_size.as_vec2() / target_size.as_vec2())
        .floor()
        .as_uvec2()
        .min_element()
        .max(1);

    let visible_tiles = vp_size / max_scale;
    // if let Projection::Orthographic(proj) = proj.as_mut() {
    //     proj.scaling_mode = ScalingMode::Fixed {
    //         width: visible_tiles.x as f32,
    //         height: visible_tiles.y as f32,
    //     };
    //     proj.viewport_origin = term_pivot.normalized();
    // }

    let ortho_size = vp_size.y as f32 / max_scale as f32;
    if let Projection::Orthographic(proj) = proj.as_mut() {
        proj.scaling_mode = ScalingMode::FixedVertical {
            viewport_height: ortho_size,
        };
        proj.viewport_origin = term_pivot.normalized();
    }

    // let vp = cam.viewport.clone().unwrap();
    // cam.viewport = Some(Viewport {
    //     physical_position: vp.physical_position,
    //     physical_size: target_size,
    //     depth: vp.depth,
    // });

    term.clear();
    term.put_string(
        [0, 0],
        format!(
            "VP Size: {}
Tar size: {}
Pivot: {:?}
Scaling: {:?}",
            vp_size, target_size, term_pivot, *mesh_scaling,
        ),
    );
}

fn controls(
    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
    mut window: Single<&mut Window>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
    mut world_scaling: ResMut<TerminalMeshWorldScaling>,
    mut term_pivot: Single<&mut TerminalMeshPivot>,
    mut exit: MessageWriter<AppExit>,
) {
    if input.just_pressed(KeyCode::Tab) {
        let mut i = term_pivot.ordinal();
        i = (i + 1).rem_euclid(TerminalMeshPivot::VARIANT_COUNT as i8);
        **term_pivot = TerminalMeshPivot::from_ordinal(i).unwrap();
    }

    if input.just_pressed(KeyCode::KeyF) {
        let fullscreen = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
        let windowed = WindowMode::Windowed;
        window.mode = match window.mode {
            WindowMode::Windowed => fullscreen,
            _ => windowed,
        };
    }

    if input.just_pressed(KeyCode::Space) {
        *world_scaling = match *world_scaling.as_ref() {
            TerminalMeshWorldScaling::World => TerminalMeshWorldScaling::Pixels,
            TerminalMeshWorldScaling::Pixels => TerminalMeshWorldScaling::World,
        };
    }

    if input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
        return;
    }

    let (mut camera, mut transform, mut projection) = camera_query.into_inner();

    let fspeed = 600.0 * time.delta_secs();
    let uspeed = fspeed as u32;
    let window_size = window.resolution.physical_size();

    // Camera zoom controls
    if let Projection::Orthographic(projection2d) = &mut *projection {
        if input.pressed(KeyCode::Comma) {
            projection2d.scale *= powf(4.0f32, time.delta_secs());
        }

        if input.pressed(KeyCode::Period) {
            projection2d.scale *= powf(0.25f32, time.delta_secs());
        }
    }

    if let Some(viewport) = camera.viewport.as_mut() {
        // Reset viewport size on window resize
        if viewport.physical_size.x > window_size.x || viewport.physical_size.y > window_size.y {
            viewport.physical_size = (window_size.as_vec2() * 0.75).as_uvec2();
        }

        // Viewport movement controls
        if input.pressed(KeyCode::KeyI) {
            viewport.physical_position.y = viewport.physical_position.y.saturating_sub(uspeed);
        }
        if input.pressed(KeyCode::KeyJ) {
            viewport.physical_position.y += uspeed;
        }
        if input.pressed(KeyCode::KeyK) {
            viewport.physical_position.x = viewport.physical_position.x.saturating_sub(uspeed);
        }
        if input.pressed(KeyCode::KeyL) {
            viewport.physical_position.x += uspeed;
        }

        // Bound viewport position so it doesn't go off-screen
        viewport.physical_position = viewport
            .physical_position
            .min(window_size - viewport.physical_size);

        // Viewport size controls
        if input.pressed(KeyCode::KeyW) {
            viewport.physical_size.y = viewport.physical_size.y.saturating_sub(uspeed);
        }
        if input.pressed(KeyCode::KeyS) {
            viewport.physical_size.y += uspeed;
        }
        if input.pressed(KeyCode::KeyA) {
            viewport.physical_size.x = viewport.physical_size.x.saturating_sub(uspeed);
        }
        if input.pressed(KeyCode::KeyD) {
            viewport.physical_size.x += uspeed;
        }

        // Bound viewport size so it doesn't go off-screen
        viewport.physical_size = viewport
            .physical_size
            .min(window_size - viewport.physical_position)
            .max(UVec2::new(20, 20));
    }
}
