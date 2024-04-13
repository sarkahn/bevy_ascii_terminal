use bevy::{
    app::{Plugin, PostUpdate},
    asset::{AssetEvent, Assets, Handle},
    ecs::{
        change_detection::DetectChangesMut,
        component::Component,
        event::EventReader,
        query::Changed,
        schedule::{IntoSystemConfigs, SystemSet},
        system::{Query, Res},
    },
    math::{IVec2, Rect, UVec2, Vec2, Vec3},
    render::texture::Image,
    transform::{components::Transform, TransformSystem},
};

use crate::{
    border::Border,
    direction::Dir4,
    renderer::{TerminalFontScaling, TerminalMaterial, TerminalMeshPivot},
    GridPoint, GridRect, Pivot, Terminal, TerminalGridSettings, Tile,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq, SystemSet)]
pub struct TerminalTransformSystems;

pub struct TerminalTransformPlugin;

impl Plugin for TerminalTransformPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PostUpdate,
            (on_image_load, on_mat_change, update_transform)
                .chain()
                .in_set(TerminalTransformSystems)
                .before(TransformSystem::TransformPropagate),
        );
    }
}

/// Component for transforming between terminal and world space.
///
/// Setting the `grid_position` of this component will alter the terminal's position
/// in world space during the next transform update, based on terminal's font size
/// and the global [TerminalGridSettings]
#[derive(Default, Component)]
pub struct TerminalTransform {
    /// The grid position for the terminal. Setting this value will override the
    /// entity [Transform] during the next [PostUpdate].
    ///
    /// The final world position of the terminal is determined by the global [TerminalGridSettings]
    pub grid_position: IVec2,
    /// The world size of a terminal tile, based on the global [crate::TileScaling],
    /// the terminal's [crate::TerminalFont] and the terminal's [TerminalFontScaling]
    world_tile_size: Vec2,
    term_size: IVec2,
    local_mesh_bounds: Rect,
    world_pos: Vec3,
    pixels_per_tile: UVec2,
}

impl TerminalTransform {
    pub fn new(size: impl GridPoint) -> Self {
        Self {
            term_size: size.as_ivec2(),
            ..Default::default()
        }
    }

    /// Convert a world position into a local 2d tile index
    pub fn world_to_tile(&self, world_pos: Vec2) -> Option<IVec2> {
        let min = self.world_pos() + self.local_mesh_bounds.min;
        let pos = ((world_pos - min) / self.world_tile_size)
            .floor()
            .as_ivec2();
        if pos.cmplt(IVec2::ZERO).any() || pos.cmpge(self.term_size).any() {
            return None;
        }
        Some(pos)
    }

    /// The world bounds of the terminal.
    pub fn world_bounds(&self) -> Rect {
        let min = self.world_pos() + self.local_mesh_bounds.min;
        let max = min + self.local_mesh_bounds.size();
        Rect::from_corners(min, max)
    }

    /// World position, as calculated from the last transform update
    pub fn world_pos(&self) -> Vec2 {
        self.world_pos.truncate()
    }

    pub fn world_tile_size(&self) -> Vec2 {
        self.world_tile_size
    }

    /// Update cached transform data.
    fn updata_cached_data(
        &mut self,
        term_size: IVec2,
        world_tile_size: Vec2,
        mesh_pivot: Pivot,
        pixels_per_tile: UVec2,
        border: Option<(&Border, Tile)>,
    ) {
        let mut term_rect = GridRect::new([0, 0], term_size);
        if let Some((border, clear_tile)) = border {
            let edges = border.edge_opacity(clear_tile, term_size);
            let border_rect = GridRect::from_points([-1, -1], term_size);

            if edges[Dir4::Left.as_index()] {
                term_rect.envelope_point([border_rect.left(), 0]);
            }
            if edges[Dir4::Up.as_index()] {
                term_rect.envelope_point([0, border_rect.top()]);
            }
            if edges[Dir4::Right.as_index()] {
                term_rect.envelope_point([border_rect.right(), 0]);
            }
            if edges[Dir4::Down.as_index()] {
                term_rect.envelope_point([0, border_rect.bottom()]);
            }
        }

        self.term_size = term_size;
        self.world_tile_size = world_tile_size;
        self.pixels_per_tile = pixels_per_tile;

        // Calculate mesh bounds
        let bounds_size = term_rect.size.as_vec2() * world_tile_size;
        let normalized_pivot = mesh_pivot.normalized();
        let min = -bounds_size * normalized_pivot;
        let max = min + bounds_size;
        self.local_mesh_bounds = Rect::from_corners(min, max);
    }

    pub fn pixels_per_unit(&self) -> UVec2 {
        self.pixels_per_tile
    }
}

fn update_transform(
    mut q_term: Query<(&mut Transform, &mut TerminalTransform), Changed<TerminalTransform>>,
) {
    for (mut transform, mut term_transform) in &mut q_term {
        // let tile_size = grid.world_grid_tile_size().unwrap_or(term_transform.world_tile_size());
        let tile_size = term_transform.world_tile_size();
        let xyz = term_transform.grid_position.as_vec2() * tile_size;
        let z = transform.translation.z;
        transform.translation = xyz.extend(z);
        term_transform.bypass_change_detection().world_pos = xyz.extend(z);
    }
}

#[allow(clippy::type_complexity)]
fn on_image_load(
    mut q_term: Query<(
        &mut TerminalTransform,
        &Terminal,
        &TerminalMeshPivot,
        &Handle<TerminalMaterial>,
        &TerminalFontScaling,
    )>,
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
    mut img_evt: EventReader<AssetEvent<Image>>,
    settings: Res<TerminalGridSettings>,
) {
    for evt in img_evt.read() {
        let loaded_image_id = match evt {
            AssetEvent::LoadedWithDependencies { id } => id,
            _ => continue,
        };
        for (mut transform, term, pivot, mat_handle, scaling) in q_term.iter_mut() {
            let mat = materials
                .get(mat_handle)
                .expect("Error getting terminal material");
            let Some(image) = mat
                .texture
                .as_ref()
                .filter(|image| image.id() == *loaded_image_id)
                .and_then(|image| images.get(image))
            else {
                continue;
            };
            let ppu = image.size() / 16;
            let world_tile_size = settings
                .tile_scaling
                .calculate_world_tile_size(ppu, Some(scaling.0));
            transform.updata_cached_data(
                term.size(),
                world_tile_size,
                pivot.0,
                ppu,
                term.get_border().map(|b| (b, term.clear_tile())),
            );
        }
    }
}

#[allow(clippy::type_complexity)]
fn on_mat_change(
    mut q_term: Query<(
        &mut TerminalTransform,
        &TerminalMeshPivot,
        &Terminal,
        &Handle<TerminalMaterial>,
        &TerminalFontScaling,
    )>,
    mut mat_evt: EventReader<AssetEvent<TerminalMaterial>>,
    materials: Res<Assets<TerminalMaterial>>,
    images: Res<Assets<Image>>,
    settings: Res<TerminalGridSettings>,
) {
    for evt in mat_evt.read() {
        let changed_material_id = match evt {
            AssetEvent::Modified { id } => id,
            _ => continue,
        };
        for (mut transform, pivot, term, mat_handle, scaling) in &mut q_term {
            if mat_handle.id() != *changed_material_id {
                continue;
            }

            let mat = materials
                .get(mat_handle.clone())
                .expect("Error getting terminal material");

            if let Some(image) = mat.texture.as_ref().and_then(|image| images.get(image)) {
                let ppu = image.size() / 16;
                let world_tile_size = settings
                    .tile_scaling
                    .calculate_world_tile_size(ppu, Some(scaling.0));
                transform.updata_cached_data(
                    term.size(),
                    world_tile_size,
                    pivot.0,
                    ppu,
                    term.get_border().map(|b| (b, term.clear_tile())),
                );
            }
        }
    }
}
