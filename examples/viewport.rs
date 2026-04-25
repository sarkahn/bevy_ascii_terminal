use std::ops::Mul;

use bevy::{camera::Viewport, math::ops::powf, prelude::*, window::WindowMode};
use bevy_ascii_terminal::{render::TerminalMaterial, *};

#[derive(Component)]
struct DrawTerminal;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, controls)
        .add_systems(PostUpdate, draw)
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
        TerminalCamera::new(),
    ));

    commands.spawn((
        Terminal::new([28, 16]).with_border(BoxStyle::SINGLE_LINE),
        TerminalMeshPivot::RightTop,
        DrawTerminal,
    ));

    commands.spawn((
        Terminal::new([12, 5])
            .with_border(BoxStyle::DOUBLE_LINE)
            .with_string([0, 0], "Terminal 2"),
        TerminalMeshPivot::LeftTop,
    ));

    commands.insert_resource(TerminalMeshWorldScaling::World);
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn draw(
    mut q_term: Query<(
        &mut Terminal,
        &TerminalMeshPivot,
        &Transform,
        &MeshMaterial2d<TerminalMaterial>,
        Option<&DrawTerminal>,
    )>,
    mesh_scaling: Res<TerminalMeshWorldScaling>,
    q_cam: Single<&Camera, Without<Terminal>>,
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
) {
    let cam = q_cam.into_inner();

    // Determine a canonical pixels per unit based on the largest of
    // all terminals. Probably not the best way to do that
    let mut pixels_per_tile = UVec2::new(8, 8);
    for (_, _, _, mat, _) in q_term.iter_mut() {
        let Some(ppt) = materials
            .get(&mat.0)
            .and_then(|mat| mat.texture.as_ref().and_then(|image| images.get(image)))
            .map(|i| i.size() / 16)
        // Assuming 16/16 tiled textures
        else {
            continue;
        };
        pixels_per_tile = pixels_per_tile.max(ppt);
    }

    // The size of a single terminal mesh tile in world space
    let world_tile = match *mesh_scaling {
        TerminalMeshWorldScaling::World => {
            Vec2::new(pixels_per_tile.x as f32 / pixels_per_tile.y as f32, 1.0)
        }
        TerminalMeshWorldScaling::Pixels => pixels_per_tile.as_vec2(),
    };
    // Size of a pixel in world space
    let world_pixel = world_tile / pixels_per_tile.as_vec2();

    let mut mesh_world_bl = Vec2::MAX;
    let mut mesh_world_tr = Vec2::MIN;
    for (term, mesh_pivot, term_transform, _, _) in q_term.iter() {
        let mesh_world_size = term.size().as_vec2() * world_tile;
        let mesh_pivot_offset = mesh_pivot.normalized() * mesh_world_size;
        let mesh_pos = term_transform.translation.truncate();
        let mesh_bl = mesh_pos - mesh_pivot_offset;
        let mesh_tr = mesh_bl + mesh_world_size;
        mesh_world_bl = mesh_world_bl.min(mesh_bl);
        mesh_world_tr = mesh_world_tr.max(mesh_tr);
    }

    let tile_count = ((mesh_world_tr - mesh_world_bl) / world_tile)
        .round()
        .as_uvec2();

    let vp_size = cam.physical_viewport_size().unwrap();
    let target_resolution = (tile_count * pixels_per_tile).as_vec2();

    let scale = (vp_size.as_vec2() / target_resolution)
        .floor()
        .as_uvec2()
        .min_element()
        .max(1);

    let scaled_res = target_resolution * scale as f32;
    let edge_pixels = (vp_size.as_vec2() - scaled_res).mul(0.5).floor();
    let center_offset = edge_pixels / scale as f32 * world_pixel;

    for (mut term, _, _, _, drawterm) in q_term.iter_mut() {
        if drawterm.is_none() {
            continue;
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
        put_line(format!("World Tile:  {:.2}", world_tile));
        put_line(format!("World Pixel: {:.2}", world_pixel));
        put_line(format!("Pixel off:   {:.2}", edge_pixels));
        put_line(format!("World off:   {:.2}", center_offset));

        put_line("".to_string());
        put_line(format!("Scaling: {:?}", *mesh_scaling));
        //put_line(format!("Pivot:   {:?}", *mesh_pivot));
    }
}

fn controls(
    camera_query: Single<(&mut Camera, &mut Transform, &mut Projection)>,
    mut window: Single<&mut Window>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
    mut world_scaling: ResMut<TerminalMeshWorldScaling>,
    mut exit: MessageWriter<AppExit>,
) {
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
