//! Terminal component for translating between world positions and terminal
//! grid coordinates.

use bevy::{
    app::{Plugin, PostStartup, PostUpdate},
    asset::{AssetEvent, Assets, Handle},
    ecs::{
        change_detection::DetectChangesMut,
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::{Changed, With},
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Commands, Query, Res},
    },
    math::{IVec2, Rect, UVec2, Vec2, Vec3, Vec3Swizzles},
    prelude::{Added, Or},
    render::texture::Image,
    transform::{components::Transform, TransformSystem},
};

use crate::{
    grid::size::GridSize,
    renderer::{
        mesh::RebuildTerminalMeshVerts, TerminalCamera, TerminalFontScaling, TerminalMaterial,
        TerminalMeshPivot, TerminalSystemMeshRebuild, UpdateTerminalViewportEvent,
    },
    GridPoint, GridRect, Pivot, Terminal, TerminalGridSettings, Tile,
};
pub struct TerminalTransformPlugin;

// /// [TerminalTransform] system for updating the entity transform based on [TerminalTransform].
// /// Runs in [PostUpdate].
// #[derive(Debug, Clone, Hash, PartialEq, Eq, SystemSet)]
// pub struct TerminalSystemTransformPositionUpdate;

/// [TerminalTransform] system for caching terminal mesh and size data. Runs in [PostUpdate].
#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemSet)]
pub struct TerminalSystemTransformCacheData;

impl Plugin for TerminalTransformPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            //.add_systems(
            //     PostUpdate,
            //     update_transform_position
            //         .in_set(TerminalSystemTransformPositionUpdate)
            //         .before(TransformSystem::TransformPropagate),
            // )
            .add_systems(
                PostUpdate,
                (
                    on_image_load,
                    on_mat_change,
                    on_terminal_resize,
                    cache_transform_data,
                )
                    .chain()
                    .in_set(TerminalSystemTransformCacheData)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_systems(
                PostStartup,
                cache_transform_data.after(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct CacheTransformData;

/// Component for transforming between world positions and terminal grid
/// coordinates.
///
/// Setting the `grid_position` of this component will alter the terminal's position
/// in world space during the next transform update, based the global [crate::TileScaling]
/// and [TerminalGridSettings].
#[derive(Debug, Component)]
pub struct TerminalTransform {
    /// The grid position for the terminal. Setting this value will override the
    /// entity [Transform] during the next [PostUpdate].
    ///
    /// The final world position of the terminal is determined by the global
    /// [TerminalGridSettings] and [crate::TileScaling].
    pub grid_position: IVec2,
    /// Snap the terminal mesh to the 'tile grid' based on the world size
    /// of the terminal's tiles. This ensures terminal tiles from different terminals
    /// will always align regardless of mesh pivot and terminal size.
    ///
    /// Defaults to true.
    pub snap_to_grid: bool,
    cached_data: Option<CachedTransformData>,
    has_border: bool,
}

#[derive(Debug, Default)]
pub(crate) struct CachedTransformData {
    /// The world size of a terminal tile, based on the global [crate::TileScaling],
    /// the terminal's [crate::TerminalFont] and the terminal's [TerminalFontScaling].
    pub world_tile_size: Vec2,
    /// The number of tiles on each axis excluding the terminal border
    pub terminal_size: UVec2,
    /// The number of tiles on each axis including the terminal border
    pub terminal_size_with_border: UVec2,
    /// The local bounds of the terminal mesh, accounting for the [TerminalMeshPivot].
    pub local_mesh_bounds: Rect,
    /// The world position of the terminal as of the last [TerminalTransform] update.
    pub world_pos: Vec3,
    /// The pixels per tile for the terminal based on the terminal's current font.
    pub pixels_per_tile: UVec2,
}

impl CachedTransformData {
    pub fn world_mesh_bounds(&self) -> Rect {
        let r = self.local_mesh_bounds;
        Rect::from_corners(r.min + self.world_pos.truncate(), r.size())
    }
}

impl Default for TerminalTransform {
    fn default() -> Self {
        Self {
            grid_position: Default::default(),
            snap_to_grid: true,
            cached_data: Default::default(),
            has_border: Default::default(),
        }
    }
}

impl TerminalTransform {
    pub fn new(size: impl GridSize) -> Self {
        Self {
            cached_data: Some(CachedTransformData {
                terminal_size: size.as_uvec2(),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    /// Convert a world position into a local 2d tile index.
    ///
    /// For accurate results this should be called after
    /// [TerminalSystemTransformCacheData] which runs in [PostUpdate].
    pub fn world_to_tile(&self, world_pos: Vec2) -> Option<IVec2> {
        let Some(data) = &self.cached_data else {
            return None;
        };
        let min = data.world_pos.truncate() + data.local_mesh_bounds.min;
        let pos = ((world_pos - min) / data.world_tile_size)
            .floor()
            .as_ivec2();
        if pos.cmplt(IVec2::ZERO).any() || pos.cmpge(data.terminal_size.as_ivec2()).any() {
            return None;
        }
        Some(pos)
    }

    pub fn transform_data(&self) -> Option<&CachedTransformData> {
        self.cached_data.as_ref()
    }

    // /// The world bounds of the terminal as of the last [TerminalTransform] update.
    // pub fn world_mesh_bounds(&self) -> Option<Rect> {
    //     let Some(data) = &self.cached_data else {
    //         return None;
    //     };
    //     let min = data.world_pos.truncate() + data.local_mesh_bounds.min;
    //     let max = min + data.local_mesh_bounds.size();
    //     Some(Rect::from_corners(min, max))
    // }

    // /// The local bounds of the terminal mesh as of the last [TerminalTransform]
    // /// update.
    // pub fn local_mesh_bounds(&self) -> Option<Rect> {
    //     self.cached_data.as_ref().map(|data| data.local_mesh_bounds)
    // }

    // /// World position of the terminal entity, as of the last [TerminalTransform]
    // /// update.
    // pub fn world_pos(&self) -> Option<Vec2> {
    //     self.cached_data
    //         .as_ref()
    //         .map(|data| data.world_pos.truncate())
    // }

    // /// The size of a single tile of the terminal in world space, as calculated
    // /// during the last transform update.
    // pub fn world_tile_size(&self) -> Option<Vec2> {
    //     self.cached_data.as_ref().map(|data| data.world_tile_size)
    // }

    // /// The pixels per tile of the terminal based on the terminal's current font,
    // /// as calculated during the last transform update.
    // pub fn pixels_per_tile(&self) -> Option<UVec2> {
    //     self.cached_data.as_ref().map(|data| data.pixels_per_tile)
    // }
}

// fn update_transform_position(
//     mut q_term: Query<(&mut Transform, &mut TerminalTransform), Changed<TerminalTransform>>,
// ) {
//     for (mut transform, mut term_transform) in &mut q_term {
//         let tile_size = term_transform.world_tile_size();
//         let p = term_transform.grid_position.as_vec2() * tile_size;
//         transform.translation = p.extend(transform.translation.z);
//         term_transform.bypass_change_detection().world_pos = transform.translation;
//     }
// }

fn on_image_load(
    mut q_term: Query<(Entity, &Handle<TerminalMaterial>)>,
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
        for (entity, mat_handle) in q_term.iter_mut() {
            let mat = materials
                .get(mat_handle)
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
    mut q_term: Query<(Entity, &Handle<TerminalMaterial>)>,
    mut mat_evt: EventReader<AssetEvent<TerminalMaterial>>,
    mut commands: Commands,
) {
    for evt in mat_evt.read() {
        let changed_material_id = match evt {
            AssetEvent::Modified { id } => id,
            _ => continue,
        };
        for (entity, mat_handle) in &mut q_term {
            if mat_handle.id() == *changed_material_id {
                commands.entity(entity).insert(CacheTransformData);
            }
        }
    }
}

fn on_terminal_resize(
    mut q_term: Query<(Entity, &Terminal, &mut TerminalTransform)>,
    q_cam: Query<&TerminalCamera>,
    mut vp_evt: EventWriter<UpdateTerminalViewportEvent>,
    mut commands: Commands,
) {
    // TODO: Handle terminal resize?
    // for (e, term, mut term_transform) in &mut q_term {
    //     if let Some(data) = term_transform.cached_data {
    //         if term.size() = data.terminal_size
    //     }
    //     if term_transform.tile_count_2d != term.size()
    //     // || term.get_border().is_some() != term_transform.has_border
    //     {
    //         term_transform.tile_count_2d = term.size();
    //         // term_transform.has_border = term.get_border().is_some();
    //         commands.entity(e).insert(RebuildTerminalMeshVerts);
    //         commands.entity(e).insert(CacheTransformData);
    //         for cam in q_cam.iter() {
    //             if cam.is_managing_viewport() {
    //                 vp_evt.send(UpdateTerminalViewportEvent);
    //             }
    //         }
    //     }
    // }
}

/// Calculate the terminal mesh size and cache the data used when translating
/// coordinates between world and terminal space. Reads terminal size, border,
/// mesh and font size, as well as global terminal grid settings.
#[allow(clippy::type_complexity)]
fn cache_transform_data(
    mut q_term: Query<
        (
            Entity,
            &mut Transform,
            &mut TerminalTransform,
            &TerminalMeshPivot,
            &mut Terminal,
            &Handle<TerminalMaterial>,
            &TerminalFontScaling,
        ),
        Or<(Added<TerminalTransform>, With<CacheTransformData>)>,
    >,
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
    settings: Res<TerminalGridSettings>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut term_transform, pivot, term, mat_handle, scaling) in &mut q_term
    {
        let snap = term_transform.snap_to_grid;
        let grid_pos = term_transform.grid_position;

        let data = term_transform
            .cached_data
            .get_or_insert(CachedTransformData::default());
        data.world_pos = transform.translation;
        data.terminal_size = term.size();

        let Some(image) = materials
            .get(mat_handle)
            .and_then(|mat| mat.texture.as_ref().and_then(|image| images.get(image)))
        else {
            continue;
        };

        let ppu = image.size() / 16;
        let world_tile_size = settings
            .tile_scaling
            .calculate_world_tile_size(ppu, Some(scaling.0));

        let p = grid_pos.as_vec2() * world_tile_size;
        transform.translation = p.extend(transform.translation.z);
        data.world_pos = transform.translation;

        data.world_tile_size = world_tile_size;
        data.pixels_per_tile = ppu;

        // Calculate mesh bounds
        let bounds_size = term.size().as_vec2() * world_tile_size;
        let normalized_pivot = pivot.0.normalized();
        let mut min = -bounds_size * normalized_pivot;
        // To align all terminal tiles to the same grid, an offset must be applied
        // to account for terminals with odd sizes and different mesh pivots.
        if snap {
            min = (min / world_tile_size).floor() * world_tile_size;
        }
        let max = min + bounds_size;
        data.local_mesh_bounds = Rect::from_corners(min, max);

        commands.entity(entity).remove::<CacheTransformData>();
    }
}
