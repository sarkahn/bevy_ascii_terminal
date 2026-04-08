use std::ops::Mul;

use bevy::{
    camera::{ScalingMode, Viewport},
    math::ops::powf,
    prelude::*,
    window::WindowMode,
};
use bevy_ascii_terminal::{render::TerminalMaterial, *};
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
        Terminal::new([28, 14]).with_border(BoxStyle::SINGLE_LINE),
        TerminalMeshPivot::LeftBottom,
        TerminalFont::TaritusCurses8x12,
    ));

    commands.insert_resource(TerminalMeshWorldScaling::World);
}

fn fit_to_terminal(
    mut term: Single<(
        &mut Terminal,
        &TerminalMeshPivot,
        &mut Transform,
        &MeshMaterial2d<TerminalMaterial>,
    )>,
    mesh_scaling: Res<TerminalMeshWorldScaling>,
    mut q_cam: Single<(&mut Camera, &mut Projection, &GlobalTransform)>,
    input: Res<ButtonInput<KeyCode>>,
    window: Single<&Window>,
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
) {
    let (
        mut cam,
        mut proj,
        //mut cam_transform,
        cam_global_transform,
    ) = q_cam.into_inner();
    let (mut term, mesh_pivot, mut term_transform, mat) = term.into_inner();

    let tile_count = term.size();
    let vp_size = cam.physical_viewport_size().unwrap();

    let pixels_per_tile = materials
        .get(&mat.0)
        .and_then(|mat| mat.texture.as_ref().and_then(|image| images.get(image)))
        .map(|i| i.size() / 16)
        .unwrap_or(UVec2::new(8, 8)); // Assume 8x8 with no image 

    let target_resolution = (tile_count * pixels_per_tile).as_vec2();

    let scale = (vp_size.as_vec2() / target_resolution)
        .floor()
        .as_uvec2()
        .min_element()
        .max(1);

    let vp_height = vp_size.y;
    let vp_world_height = match mesh_scaling.as_ref() {
        TerminalMeshWorldScaling::Pixels => vp_height as f32 / scale as f32,
        TerminalMeshWorldScaling::World => vp_height as f32 / (scale * pixels_per_tile.y) as f32,
    };

    if let Projection::Orthographic(proj) = proj.as_mut() {
        proj.scaling_mode = ScalingMode::FixedVertical {
            viewport_height: vp_world_height,
        };
        proj.viewport_origin = Vec2::ZERO;
    }

    let world_pixel = Vec2::splat(vp_world_height / vp_size.y as f32) * scale as f32;
    let world_unit = world_pixel * pixels_per_tile.as_vec2();

    let scaled_res = target_resolution * scale as f32;
    let pixels_offset = (vp_size.as_vec2() - scaled_res).mul(0.5).floor();
    let world_offset = pixels_offset / scale as f32 * world_pixel;

    if let Some(cam_bl) = cam.ndc_to_world(cam_global_transform, Vec3::new(-1.0, -1.0, 0.0)) {
        let cam_bl = cam_bl.truncate();
        let mesh_pivot_offset = mesh_pivot.normalized() * term.size().as_vec2() * world_unit;

        term_transform.translation = (cam_bl + mesh_pivot_offset + world_offset).extend(0.0);
    }

    if input.pressed(KeyCode::ShiftLeft) {
        // Viewport size controls
        if input.just_pressed(KeyCode::KeyW) {
            let mut size = term.size();
            size.y += 1;
            term.resize(size);
        }
        if input.just_pressed(KeyCode::KeyS) {
            let mut size = term.size();
            size.y = (size.y - 1).max(1);
            term.resize(size);
        }
        if input.just_pressed(KeyCode::KeyA) {
            let mut size = term.size();
            size.x = (size.x - 1).max(1);
            term.resize(size);
        }
        if input.just_pressed(KeyCode::KeyD) {
            let mut size = term.size();
            size.x += 1;
            term.resize(size);
        }
    }

    term.clear();

    let mut line = 0;
    let mut put_line = |s: String| {
        term.put_string([0, line], s);
        line += 1;
    };

    put_line(format!("VP Size:     {}", vp_size));
    put_line(format!("Term Size:   {}", tile_count));
    put_line(format!("Tar Res:     {}", target_resolution));
    put_line(format!("Scale:       {}", scale));
    // put_line(format!("BotLeft:     {}", bl));
    put_line(format!("World Unit:  {:.2}", world_unit));
    put_line(format!("World Pixel: {:.2}", world_pixel));
    put_line(format!("Pixel off:   {:.2}", pixels_offset));
    put_line(format!("World off:   {:.2}", world_offset));

    put_line("".to_string());
    put_line(format!("Scaling: {:?}", *mesh_scaling));
    put_line(format!("Pivot:   {:?}", *mesh_pivot));
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

    let offset = Vec2::splat(fspeed);
    if input.just_pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= offset.x;
    }
    if input.just_pressed(KeyCode::ArrowRight) {
        transform.translation.x += offset.x;
    }
    if input.just_pressed(KeyCode::ArrowUp) {
        transform.translation.y += offset.y;
    }
    if input.just_pressed(KeyCode::ArrowDown) {
        transform.translation.y -= offset.y;
    }

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
        if input.pressed(KeyCode::KeyK) {
            viewport.physical_position.y += uspeed;
        }
        if input.pressed(KeyCode::KeyJ) {
            viewport.physical_position.x = viewport.physical_position.x.saturating_sub(uspeed);
        }
        if input.pressed(KeyCode::KeyL) {
            viewport.physical_position.x += uspeed;
        }

        // Bound viewport position so it doesn't go off-screen
        viewport.physical_position = viewport
            .physical_position
            .min(window_size - viewport.physical_size);

        if !input.pressed(KeyCode::ShiftLeft) {
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
        }
        // Bound viewport size so it doesn't go off-screen
        viewport.physical_size = viewport
            .physical_size
            .min(window_size - viewport.physical_position)
            .max(UVec2::new(20, 20));
    }
}
