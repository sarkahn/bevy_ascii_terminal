//! Terminal component for translating between world positions and terminal
//! grid coordinates.

use bevy::{
    app::{Plugin, PostUpdate},
    asset::{AssetEvent, Assets},
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        query::{Changed, With},
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Commands, Query, Res},
    },
    image::Image,
    math::{IVec2, Rect, UVec2, Vec2, Vec3},
    prelude::{GlobalTransform, OnReplace, Or, Trigger},
    reflect::Reflect,
    sprite::MeshMaterial2d,
    transform::{components::Transform, TransformSystem},
};

use crate::{
    border::TerminalBorder,
    render::{TerminalFont, TerminalMaterial, TerminalMeshPivot, TerminalMeshTileScaling},
    GridPoint, Terminal, TerminalMeshWorldScaling,
};
pub(crate) struct TerminalTransformPlugin;

/// [TerminalTransform] system for caching terminal mesh and size data. Runs in [PostUpdate].
#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemSet)]
pub struct TerminalSystemsUpdateTransform;

impl Plugin for TerminalTransformPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_observer(on_border_replace);
        app.add_systems(
            PostUpdate,
            (
                on_image_load,
                on_mat_change,
                on_size_change,
                cache_transform_data,
                set_grid_position,
                set_layer_position,
            )
                .chain()
                .in_set(TerminalSystemsUpdateTransform)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

/// Instructs the terminal to cache transform data on the next update.
#[derive(Component, Default)]
#[component(storage = "SparseSet")]
struct CacheTransformData;

/// Component for transforming between world positions and terminal grid
/// coordinates.
///
/// Setting the `grid_position` of this component will alter the terminal's position
/// in world space during the next transform update, based the global [crate::TileScaling]
/// and [TerminalMeshWorldScaling].
#[derive(Debug, Component, Default, Reflect)]
#[require(CacheTransformData)]
pub struct TerminalTransform {
    pub(crate) cached_data: Option<CachedTransformData>,
}

/// A temporary component for setting the terminal to a fixed grid position
/// based on the terminal tile size. Runs in [PostUpdate].
///
/// Note that since the terminal tile size can only be calculated from the terminal
/// font there might be a delay before this gets applied while the terminal font
/// image is being loaded.
#[derive(Component, Debug, Default, Clone, Copy, Reflect)]
pub struct SetTerminalGridPosition(pub IVec2);

impl<T: GridPoint> From<T> for SetTerminalGridPosition {
    fn from(xy: T) -> Self {
        Self(xy.to_ivec2())
    }
}

/// A temporary component to set the terminal's layer position. Terminals on a higher layer
/// will be rendered on top of terminals on a lower layer. Runs in [PostUpdate].
#[derive(Component, Default, Clone, Copy)]
pub struct SetTerminalLayerPosition(pub i32);

#[derive(Debug, Default, Reflect)]
pub(crate) struct CachedTransformData {
    /// The world size of a terminal tile, based on the global [crate::TileScaling],
    /// the terminal's [crate::TerminalFont] and the terminal's [TerminalFontScaling].
    pub world_tile_size: Vec2,
    /// The number of tiles on each axis excluding the terminal border
    pub terminal_size: UVec2,
    /// The local bounds of the terminal's inner mesh excluding the terminal border.
    pub local_inner_mesh_bounds: Rect,

    /// The world bounds of the terminal mesh including the border if it has one.
    pub world_mesh_bounds: Rect,
    /// The world position of the terminal as of the last [TerminalTransform] update.
    pub world_pos: Vec3,
    /// The pixels per tile for the terminal based on the terminal's current font.
    pub pixels_per_tile: UVec2,
}

impl TerminalTransform {
    /// Convert a world position into a local 2d tile index.
    ///
    /// For accurate results this should be called after
    /// [TerminalSystemsUpdateTransform] which runs in [PostUpdate].
    pub fn world_to_tile(&self, world_pos: Vec2) -> Option<IVec2> {
        let Some(data) = &self.cached_data else {
            return None;
        };
        let min = data.world_pos.truncate() + data.local_inner_mesh_bounds.min;
        let pos = ((world_pos - min) / data.world_tile_size)
            .floor()
            .as_ivec2();
        if pos.cmplt(IVec2::ZERO).any() || pos.cmpge(data.terminal_size.as_ivec2()).any() {
            return None;
        }
        Some(pos)
    }
}

fn on_image_load(
    q_term: Query<(Entity, &MeshMaterial2d<TerminalMaterial>)>,
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
    mut img_evt: EventReader<AssetEvent<Image>>,
    mut commands: Commands,
) {
    for evt in img_evt.read() {
        let loaded_image_id = match evt {
            AssetEvent::LoadedWithDependencies { id } => id,
            _ => continue,
        };
        for (entity, mat_handle) in q_term.iter() {
            let mat = materials
                .get(&mat_handle.0)
                .expect("Error getting terminal material");
            let Some(_) = mat
                .texture
                .as_ref()
                .filter(|image| image.id() == *loaded_image_id)
                .and_then(|image| images.get(image))
            else {
                continue;
            };
            commands.entity(entity).insert(CacheTransformData);
        }
    }
}

fn on_mat_change(
    q_term: Query<(Entity, &MeshMaterial2d<TerminalMaterial>)>,
    mut mat_evt: EventReader<AssetEvent<TerminalMaterial>>,
    mut commands: Commands,
) {
    for evt in mat_evt.read() {
        let changed_material_id = match evt {
            AssetEvent::Modified { id } => id,
            _ => continue,
        };
        for (entity, mat_handle) in &q_term {
            if mat_handle.id() == *changed_material_id {
                commands.entity(entity).insert(CacheTransformData);
            }
        }
    }
}

fn on_size_change(
    q_term: Query<(Entity, &Terminal, &TerminalTransform), Changed<Terminal>>,
    mut commands: Commands,
) {
    for (entity, term, term_transform) in &q_term {
        if let Some(data) = &term_transform.cached_data {
            if data.terminal_size != term.size() {
                commands.entity(entity).insert(CacheTransformData);
            }
        }
    }
}
fn on_border_replace(on_replace: Trigger<OnReplace, TerminalBorder>, mut commands: Commands) {
    commands
        .entity(on_replace.entity())
        .insert(CacheTransformData);
}

/// Calculate the terminal mesh size and cache the data used when translating
/// coordinates between world and terminal space. Reads terminal size, border,
/// mesh and font size, as well as global terminal grid settings.
#[allow(clippy::type_complexity)]
fn cache_transform_data(
    mut q_term: Query<
        (
            Entity,
            &GlobalTransform,
            &mut TerminalTransform,
            &TerminalMeshPivot,
            &Terminal,
            &MeshMaterial2d<TerminalMaterial>,
            Option<&TerminalMeshTileScaling>,
            Option<&TerminalBorder>,
        ),
        Or<(
            Changed<Transform>,
            Changed<TerminalFont>,
            Changed<TerminalBorder>,
            With<CacheTransformData>,
        )>,
    >,
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
    scaling: Res<TerminalMeshWorldScaling>,
    mut commands: Commands,
) {
    for (entity, transform, mut term_transform, pivot, term, mat_handle, tile_scaling, border) in
        &mut q_term
    {
        let Some(image) = materials
            .get(&mat_handle.0)
            .and_then(|mat| mat.texture.as_ref().and_then(|image| images.get(image)))
        else {
            continue;
        };

        let data = term_transform
            .cached_data
            .get_or_insert(CachedTransformData::default());
        data.world_pos = transform.translation();
        data.terminal_size = term.size();

        let ppu = image.size() / 16;
        let world_tile_size = match *scaling {
            TerminalMeshWorldScaling::World => Vec2::new(ppu.x as f32 / ppu.y as f32, 1.0),
            TerminalMeshWorldScaling::Pixels => ppu.as_vec2(),
        };
        let world_tile_size = if let Some(tile_scaling) = tile_scaling.as_ref() {
            world_tile_size * tile_scaling.0
        } else {
            world_tile_size
        };

        data.world_tile_size = world_tile_size;
        data.pixels_per_tile = ppu;

        let border_offset = if let Some(border) = border.as_ref() {
            let left = border.has_left_side() as i32;
            let right = border.has_right_side() as i32;
            let top = border.has_top_side() as i32;
            let bottom = border.has_bottom_side() as i32;

            match pivot {
                TerminalMeshPivot::TopLeft => [left, -top],
                TerminalMeshPivot::TopCenter => [0, -top],
                TerminalMeshPivot::TopRight => [-right, -top],
                TerminalMeshPivot::LeftCenter => [left, 0],
                TerminalMeshPivot::RightCenter => [-right, 0],
                TerminalMeshPivot::BottomLeft => [left, bottom],
                TerminalMeshPivot::BottomCenter => [0, bottom],
                TerminalMeshPivot::BottomRight => [-right, bottom],
                TerminalMeshPivot::Center => [0, 0],
            }
        } else {
            [0, 0]
        }
        .to_vec2()
            * world_tile_size;

        // The size of the terminal mesh excluding the border bounds
        let inner_mesh_size = term.size().as_vec2() * world_tile_size;
        let inner_mesh_min = -inner_mesh_size * pivot.normalized();
        let local_min = inner_mesh_min + border_offset;
        let local_max = local_min + inner_mesh_size;
        data.local_inner_mesh_bounds = Rect::from_corners(local_min, local_max);

        let world_bounds = if let Some(border) = border.as_ref() {
            let bounds = border.bounds(term.size());
            // The size of the terminal mesh including the border bounds
            let total_world_size = bounds.size.as_vec2() * world_tile_size;
            let world_min =
                transform.translation().truncate() - total_world_size * pivot.normalized();
            let world_max = world_min + total_world_size;
            Rect::from_corners(world_min, world_max)
        } else {
            let world_min = transform.translation().truncate() + local_min;
            let world_max = world_min + inner_mesh_size;
            Rect::from_corners(world_min, world_max)
        };

        data.world_mesh_bounds = world_bounds;

        commands.entity(entity).remove::<CacheTransformData>();
    }
}

#[allow(clippy::type_complexity)]
fn set_grid_position(
    mut q_grid_pos: Query<(
        Entity,
        &SetTerminalGridPosition,
        &TerminalTransform,
        &mut Transform,
    )>,
    mut commands: Commands,
) {
    for (e, grid_pos, term_transform, mut transform) in &mut q_grid_pos {
        if let Some(data) = &term_transform.cached_data {
            let p = grid_pos.0.as_vec2() * data.world_tile_size;
            let z = transform.translation.z;
            transform.translation = p.extend(z);
            commands.entity(e).remove::<SetTerminalGridPosition>();
        } else {
            continue;
        }
    }
}

fn set_layer_position(
    mut q_grid_pos: Query<(Entity, &SetTerminalLayerPosition, &mut Transform)>,
    mut commands: Commands,
) {
    for (entity, layer, mut transform) in &mut q_grid_pos {
        let xy = transform.translation.truncate();
        transform.translation = xy.extend(layer.0 as f32);
        commands.entity(entity).remove::<SetTerminalLayerPosition>();
    }
}
