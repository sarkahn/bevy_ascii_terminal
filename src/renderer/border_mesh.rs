//use std::collections::BTreeMap;

use bevy::{
    prelude::{
        Added, Assets, BuildChildren, Changed, Children, Commands, Component, CoreSet, Entity,
        Handle, IVec2, IntoSystemConfig, Plugin, Query, Res, Vec2,
    },
    utils::HashMap,
};
use sark_grids::GridPoint;

use crate::{Edge, TerminalLayout, Tile};

use super::{
    mesh_data::{TileData, UvMesher, VertData, VertMesher},
    uv_mapping::UvMapping,
    TerminalInit, TerminalRender, TerminalRenderBundle, TerminalUpdateTiles,
};

#[derive(Debug, Default, PartialEq)]
pub struct BorderTile {
    pub pos: IVec2,
    pub tile: Tile,
}

#[derive(Component)]
pub struct BorderMesh {
    tiles: HashMap<IVec2, BorderTile>,
    size: IVec2,
    tile_size: Vec2,
    clear_tile: Tile,
}

impl Default for BorderMesh {
    fn default() -> Self {
        Self {
            tiles: Default::default(),
            size: Default::default(),
            tile_size: Vec2::ONE,
            clear_tile: Default::default(),
        }
    }
}

impl BorderMesh {
    pub fn new(layout: &TerminalLayout) -> Self {
        Self {
            size: layout.term_size().as_ivec2(),
            tile_size: layout.tile_size,
            clear_tile: layout.clear_tile(),
            ..Default::default()
        }
    }

    pub fn clear(&mut self) {
        self.tiles.clear();
    }

    pub fn put_tile(&mut self, xy: impl GridPoint, tile: Tile) {
        let tile = BorderTile {
            pos: xy.as_ivec2(),
            tile,
        };
        self.tiles.insert(xy.as_ivec2(), tile);
    }
}

fn init(mut q: Query<(Entity, &TerminalLayout), Added<TerminalLayout>>, mut commands: Commands) {
    for (term_entity, layout) in q.iter_mut() {
        let border = commands
            .spawn((TerminalRenderBundle::default(), BorderMesh::new(layout)))
            .id();

        commands.entity(term_entity).push_children(&[border]);
    }
}

fn update(
    mut q_border: Query<&mut BorderMesh>,
    q_term: Query<(&TerminalLayout, &Children), Changed<TerminalLayout>>,
) {
    for (layout, children) in &q_term {
        for child in children {
            if let Ok(mut mesh) = q_border.get_mut(*child) {
                //if layout.

                mesh.clear();
                if !layout.has_border() {
                    return;
                }
                //println!("FOUND BORDER. Inserting border tiles");

                mesh.size = layout.term_size().as_ivec2() + 2;
                mesh.tile_size = layout.tile_size;
                mesh.clear_tile = layout.clear_tile();
                let w = mesh.size.x - 1;
                let h = mesh.size.y - 1;

                let tile = get_tile(Edge::BottomLeft, layout);
                mesh.put_tile([0, 0], tile);

                let tile = get_tile(Edge::TopLeft, layout);
                mesh.put_tile([0, h], tile);

                let tile = get_tile(Edge::TopRight, layout);
                mesh.put_tile([w, h], tile);

                let tile = get_tile(Edge::BottomRight, layout);
                mesh.put_tile([w, 0], tile);

                let top = get_tile(Edge::Top, layout);
                let bot = get_tile(Edge::Bottom, layout);
                for x in 1..w {
                    mesh.put_tile([x, h], top);
                    mesh.put_tile([x, 0], bot);
                }
                let left = get_tile(Edge::Left, layout);
                let right = get_tile(Edge::Right, layout);
                for y in 1..h {
                    mesh.put_tile([0, y], left);
                    mesh.put_tile([w, y], right);
                }

                let border = layout.border().unwrap();

                for (edge, aligned_string) in border.edge_strings.iter() {
                    match edge {
                        Edge::Top => {
                            let align = aligned_string.align;
                            let string = &aligned_string.string;
                            let w = mesh.size.x - 2;
                            let len = string.chars().count();
                            let x = (align * w as f32).round() as i32;
                            let x = x - (len as f32 * align).round() as i32;

                            for (i, ch) in string.chars().enumerate() {
                                let i = i as i32 + 1;
                                let x = x + i;
                                let mut tile = mesh.clear_tile;
                                tile.glyph = ch;
                                if let Some(col) = aligned_string.fg_col {
                                    tile.fg_color = col;
                                }
                                if let Some(col) = aligned_string.bg_col {
                                    tile.bg_color = col;
                                }
                                mesh.put_tile([x, h], tile);
                            }

                            //let mut tile =
                            //mesh.put_tile(xy, tile)
                        }
                        Edge::Left => todo!(),
                        Edge::Right => todo!(),
                        Edge::Bottom => todo!(),
                        Edge::TopLeft => todo!(),
                        Edge::TopRight => todo!(),
                        Edge::BottomLeft => todo!(),
                        Edge::BottomRight => todo!(),
                    }
                }
            }
        }
    }
}

fn update_tile_data(
    mut q_mesh: Query<
        (
            &BorderMesh,
            &mut TileData,
            &mut VertData,
            &Handle<UvMapping>,
        ),
        Changed<BorderMesh>,
    >,
    mappings: Res<Assets<UvMapping>>,
) {
    for (bmesh, mut td, mut vd, mapping) in &mut q_mesh {
        td.clear();
        vd.clear();
        if bmesh.tiles.is_empty() {
            continue;
        }

        let origin = -(bmesh.size.as_vec2() / 2.0) * bmesh.tile_size;
        //println!("Update border tile data");
        let mapping = mappings.get(mapping).unwrap();
        let mut vmesher = VertMesher::new(origin, bmesh.tile_size, &mut vd);
        let mut tmesher = UvMesher::new(mapping, &mut td);

        //println!("border tiles {}", bmesh.tiles.len());
        for (p, t) in bmesh.tiles.iter() {
            let t = t.tile;
            vmesher.tile_verts_at(*p);
            tmesher.tile_uvs(t.glyph, t.fg_color, t.bg_color);
        }

        //println!("Vertcount {}, uvcount {}", vd.verts.len() / 4, td.uvs.len() / 4);
    }
}

fn get_tile(edge: Edge, layout: &TerminalLayout) -> Tile {
    let mut tile = layout.clear_tile();
    tile.glyph = layout.border().unwrap().edge_glyph(edge);
    tile
}

pub struct BorderMeshPlugin;

impl Plugin for BorderMeshPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(init.in_set(TerminalInit).in_base_set(CoreSet::PostUpdate))
            .add_system(
                update
                    .after(TerminalUpdateTiles)
                    .before(TerminalRender)
                    // The following comment is outdated. `with_run_criteria` was
                    // replaced with `run_if`.
                    //.with_run_criteria(should_update)
                    .in_base_set(CoreSet::Last),
            )
            .add_system(
                update_tile_data
                    .after(TerminalUpdateTiles)
                    .before(TerminalRender)
                    .in_base_set(CoreSet::Last),
            );
    }
}
